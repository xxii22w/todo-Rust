use std::{
    cell::{Cell, RefCell},
    rc::Rc,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use anyhow::Context;
use thiserror::Error;
use tokio::io::join;
// 1. thiserror 底层强类型错误定义
#[derive(Error, Debug)]
pub enum DataError {
    #[error("畸形数据错误: 无法处理 NaN (位置: {index})")]
    InvalidNumber { index: usize },

    #[error("通道传输异常: 接收端可能已关闭")]
    ChannelClosed,
}

// 2. unsafe & lifetime 裸内存缓冲区
struct RawBuffer {
    ptr: *mut f64,
    cap: usize,
}

unsafe impl Send for RawBuffer {}
unsafe impl Sync for RawBuffer {}

// 3. iterator & Cell 自定义数据流迭代器
struct DataStream<'a> {
    buffer: &'a RawBuffer, // borrowing & lifetime 借用buffer,生命周期绑定‘a
    cursor: Cell<usize>,   // 内存可变性
}

impl<'a> Iterator for DataStream<'a> {
    type Item = (usize, f64);

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.cursor.get();
        if current >= self.buffer.cap {
            return None;
        }

        // unsafe 通过裸指针直接偏移读取内存
        let val = unsafe { *self.buffer.ptr.add(current) };

        // Cell 就算没有&mut self,也可以用set修改内部状态
        self.cursor.set(current + 1);
        Some((current, val))
    }
}

// 4. async/await 异步核心计算任务
async fn async_calculate_task(id: usize, val: f64) -> Result<String, DataError> {
    // 模拟异步非阻塞IO延迟
    tokio::time::sleep(Duration::from_millis(15)).await;
    // 故意制造错误边界：如果是 NaN 触发 thiserror 异常
    if val.is_nan() {
        return Err(DataError::InvalidNumber { index: id });
    }

    let res = val.powi(2);
    Ok(format!("任务 #{} 处理成功: {} 的平方 = {}", id, val, res))
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut raw_data = vec![1.5, 3.0, f64::NAN, 4.5];
    let buffer = RawBuffer {
        ptr: raw_data.as_mut_ptr(),
        cap: raw_data.len(),
    };

    // [RC] [RefCell] 单线程异步的局部可变日志收集
    // Vec没有实现Copy,必须用RefCell运行时动态检查借用
    let local_logger = Rc::new(RefCell::new(Vec::<String>::new()));

    // [Arc] [Mutex] 跨多线程共享的全局安全状态计数器
    let global_counter = Arc::new(Mutex::new(0));

    // [tokio channel] 异步多生产者但消费者通道，避免阻塞异步主线程
    let (tx, mut rx) = tokio::sync::mpsc::channel(32);

    println!("=== [阶段 1] 启动多线程多生产者进行数据分发 ===");

    // [thread scope] 开启现代现场作用域
    // 允许内部并发子线程直接借用主线程栈上的变量 如：&buffer
    thread::scope(|s| {
        let mut stream = DataStream {
            buffer: &buffer,
            cursor: Cell::new(0),
        };

        // [iterator] 消费者定义迭代器
        for (task_id, value) in &mut stream {
            let tx_clone = tx.clone();
            let count_clone = Arc::clone(&global_counter);

            // 衍生并发子线程
            s.spawn(move || {
                // mutex 通过RAII 锁安全递增全局计数
                {
                    let mut lock = count_clone.lock().unwrap();
                    *lock += 1;
                }
                // channel 使用同步阻塞方式发生到异步通道 （在std线程中安全）
                tx_clone.blocking_send((task_id, value)).unwrap();
            });
        }
    });

    // 手动消除最初的tx,否则下面的rx.recv 异步循环将永远等待
    drop(tx);

    println!("=== [阶段 2] 收集通道数据并组装异步 Future 队列 ===");

    // [Box] & [Pin] 异步运行时调度准备
    // 异步产生的 Future 带有自引用结构（Self-referential），在内存中移动会导致指针失效
    // 我们必须用 Box 分配到堆上，再用 Pin 牢牢“钉住”内存地址，同时抹除匿名类型存入列表
    // 这里引入最新的 `tokio::task::JoinSet` 来管理并行的 Pinned Future，真正实现异步并发
    let mut join_set = tokio::task::JoinSet::new();

    while let Some((id, val)) = rx.recv().await {
        // box + pin 固化异步状态机，并投入异步并发执行组中
        let pinned_future = Box::pin(async_calculate_task(id, val));
        join_set.spawn(pinned_future);
    }

    println!("=== [阶段 3] 激活 Async 并发执行流与错误拦截 ===");

    // 使用 JoinSet 乱序并发接收结果，效率最大化
    while let Some(res) = join_set.join_next().await {
        // tokio join 的错误外层包裹了一层 JoinError，先解包
        let task_result = res.context("异步任务调度发生崩溃 (Panic)")?;

        // 【anyhow】: 使用 .context() 为底层的 thiserror 错误强行注入顶层业务线索
        match task_result.context("顶层数据清洗流水线发生严重熔断") {
            Ok(success_msg) => {
                println!("{}", success_msg);

                // 【RefCell】: 运行时动态借用，将日志塞入单线程共享的 Vec 中
                local_logger.borrow_mut().push(success_msg);
            }
            Err(anyhow_err) => {
                // 【errorhandling】: 打印包含完整 context 链条和底层 thiserror 的详细错误描述
                eprintln!("\n[报警中心拦截异常]\n{:?}", anyhow_err);
            }
        }
    }

    // 5.7 最终运行报告
    println!("\n=== [阶段 4] 最终统计报告 ===");
    println!(
        "Arc<Mutex> 全局多线程并发安全计数: {} 次",
        *global_counter.lock().unwrap()
    );
    println!(
        "Rc<RefCell> 局部单线程安全接收日志: {} 条",
        local_logger.borrow().len()
    );
    Ok(())
}
