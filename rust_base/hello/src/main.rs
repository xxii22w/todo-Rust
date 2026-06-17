use core::fmt;
use std::arch::x86_64::_mm256_mask_cmp_ps_mask;
use std::cell::{Cell, RefCell};
use std::error::Error;
use std::ffi::c_char;
use std::fmt::format;
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader, Read, Write};
use std::rc::Rc;
use std::sync::mpsc;
use std::thread::sleep;
use std::{char::ToUppercase, string};

use anyhow::{Context, Result, bail};
use hello::Frog;
use hello::spanish;
#[warn(unused_imports)]

fn greet_world() {
    let southern_germany = "Grüß Gott!";
    let chinese = "世界，你好";
    let english = "World, hello";
    let regions = [southern_germany, chinese, english];
    for region in regions.iter() {
        println!("{}", &region);
    }

    // 使用下划线告诉编译器不要警告未使用变量
    let _x = 5;
    // 变量解构
    let (a, mut b): (bool, bool) = (true, false);
    println!("a = {:?},b = {:?}", a, b);
    b = true;
    assert_eq!(a, b);

    // const MAX_POINT: i32 = 100_000;

    // let buf = [1, 2, 3];
    // let buff = [0; 3]; // 3个0
}

fn fib(n: u32) -> u32 {
    if n < 2 {
        return n;
    } else {
        return fib(n - 1) + fib(n - 2);
    }
}

fn test_fib() {
    let n = 10;
    println!("fib({n}) = {}", fib(n));
}

fn control_flow() {
    let mut count = 0;
    let mut bunnies = 2;

    loop {
        count += 1;
        bunnies *= 2;
        if bunnies > 500 {
            break;
        }
    }

    println!("{} And {}", count, bunnies);

    let mut sum = 0;
    for i in 7..=23 {
        sum += i;
    }

    println!("The Sum {}", sum);

    let mut fives: Vec<i32> = vec![];
    let mut number = 5;
    while fives.len() < 12 {
        fives.push(number);
        number += 5;
    }
    println!("{:?}", fives);

    let s = [[5, 6, 7], [8, 9, 10], [32, 25, 22]];
    let mut elements_searched = 0;
    let target_value = 10;
    'outer: for i in 0..=2 {
        for j in 0..=2 {
            elements_searched += 1;
            if s[i][j] == target_value {
                break 'outer;
            }
        }
    }
    println!("elements_searched: {}", elements_searched);
}

fn string_ex() {
    println!("\u{2728}");

    let mut favorite = String::new();
    favorite = "🍓".to_string();
    if favorite != "" {
        println!("Everyone's favvorite fruit is: {favorite}");
    }

    let saying = "Now is\nthe time\nfor all\ngreat men";
    println!("{saying}");

    // &str 是utf-8编码的字节切片 相当于C++的std::string_view
    // STring 是一个UTF-8编码字节的自由缓冲区 相当于C++的std::string 但是他没有小字符串优化，也就是在堆上
    let s1: &str = "World";
    println!("s1: {s1}");

    let mut s2: String = String::from("Hello ");
    println!("s2: {s2}");

    s2.push_str(s1);
    println!("s2: {s2}");

    let s3: &str = &s2[2..9];
    println!("s3: {s3}");
}

// fn do_stuff(s: String) -> String {
//     s
// }

fn do_stuff(s: &mut String) {
    s.insert_str(0, "Hi, ");
}

fn inspect(s: &String) {
    if s.ends_with("s") {
        println!("{} is plural", s);
    } else {
        println!("{} is singular", s);
    }
}

fn change(s: &mut String) {
    if !s.ends_with("s") {
        s.push_str("s");
    }
}

fn eat(s: String) -> bool {
    if s.starts_with("b") && s.contains("a") {
        return true;
    } else {
        return false;
    }
}

fn ownership() {
    // 1.Each value has an owner
    // 2. Only one owner
    // 3. Value gets dropped if its owner goes out scope
    // point    ->    a
    // len            b
    // capital        c
    let s1 = String::from("abc");
    let s2 = s1; // move, s1 destructor free heap pop stack
    let mut s3 = s2.clone(); // 深拷贝
    println!("{} {}", s2, s3);

    // s3 = do_stuff(s3);
    // println!("{}", s3);

    // brower 借用就是new一个指针指向字符串结构
    let mut s4 = String::from("abc");
    do_stuff(&mut s4);

    let mut arg: String = std::env::args().nth(1).unwrap_or_else(|| {
        println!("Please supply an argument to this program.");
        std::process::exit(-1);
    });

    inspect(&arg);

    change(&mut arg);
    println!("I have many {}", arg);

    if eat(arg) {
        println!("Migt be bananas");
    } else {
        println!("Not bananas");
    }
}

struct Polygon {
    name: String,
    sides: u32,
    visible: bool,
}

impl Polygon {
    fn new(name: String) -> Self {
        Self {
            name,
            sides: 3,
            visible: true,
        }
    }

    fn shape(&self) -> String {
        match self.sides {
            3 => "triangle".to_string(),
            4 => "squuare".to_string(),
            5 => "pentagon".to_string(),
            _ => "polygon".to_string(),
        }
    }

    fn increment_sides(&mut self) {
        self.sides += 1;
    }
}

fn struct_ex() {
    let mut polygon = Polygon::new("George".to_string());
    println!(
        "I see a {}-sided polygon named {}!",
        polygon.sides, polygon.name
    );

    println!(
        "The polygon named {} is a {}",
        polygon.name,
        polygon.shape()
    );

    for _ in 0..3 {
        polygon.increment_sides();
        println!(
            "The polygon now has {} sides and is the shape of a {}",
            polygon.sides,
            polygon.shape()
        );
    }
}

// 萃取就是接口，组合，类似interface
trait Colorful {
    fn color(&self) -> String;
}

struct Hat {
    size: i32,
}

impl Colorful for Hat {
    fn color(&self) -> String {
        match self.size {
            0..=5 => "red",
            6 | 7 => "green",
            _ => "blue",
        }
        .to_string()
    }
}

fn describe_three_hats(hat1: &Hat, hat2: &Hat, hat3: &Hat) {
    for hat in [hat1, hat2, hat3] {
        let largeness = if hat.size < 3 {
            "small"
        } else if hat.size < 9 {
            "medium"
        } else {
            "large"
        };
        println!("The {} hat is {}", largeness, hat.color())
    }
}

impl Colorful for i32 {
    fn color(&self) -> String {
        if self.is_even() { "orange" } else { "purple" }.to_string()
    }
}

trait EventOdd {
    fn is_even(&self) -> bool;
}

impl EventOdd for i32 {
    fn is_even(&self) -> bool {
        *self % 2 == 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Cake {
    Chocolate,
    MapleBacon,
    Spice,
}

#[derive(Debug)]
pub struct Party {
    pub at_restaurant: bool,
    pub num_people: u8,
    pub cake: Cake,
}

impl Default for Party {
    fn default() -> Self {
        Party {
            at_restaurant: true,
            num_people: 8,
            cake: Cake::Chocolate,
        }
    }
}

pub fn admire_cake(cake: Cake) {
    println!("What a nice {:?} cake! ", cake); // 使用{:?} 需要继承std::fmt::Debug特性
}

impl PartialEq for Party {
    fn eq(&self, other: &Self) -> bool {
        self.cake == other.cake
    }
}

fn traits_ex() {
    let small_hat = Hat { size: 2 };
    let medium_hat = Hat { size: 7 };
    let large_hat = Hat { size: 42 };
    describe_three_hats(&small_hat, &medium_hat, &large_hat);

    println!("4 is {}", 4.color());
    println!("5 is {}", 5.color());

    let cake = Cake::Spice;
    admire_cake(cake);

    // 添加copy,clone特性
    match cake {
        Cake::Chocolate => println!("The name's Chocolate. Dark...Chocolate."),
        Cake::MapleBacon => println!("Dreams do come true!"),
        Cake::Spice => println!("Great,let's spice it up"),
    }

    println!("The default Party is\n{:#?}", Party::default());

    // 添加fmt::Debug特性
    let party = Party {
        cake: Cake::MapleBacon,
        ..Default::default()
    };
    println!("Yes! My party has my favorite {:?} cake!", party.cake);

    // 添加equal特性
    let other_party = Party {
        at_restaurant: false,
        num_people: 235,
        cake: Cake::MapleBacon,
    };
    if party == other_party {
        println!("Your party is just like mine!");
    }
}

fn collection_ex() {
    let item = String::from("socks");
    let animal = "fox".to_string();
    let container = "box".to_owned();
    let material = "rocks".into();

    let mut things = vec![item, animal, container, material];
    println!("{:?}", things);
    println!("things has a length of {}", things.len());
    println!("what does the {} say?", things[1]);

    things.sort();
    println!("Sorted values: {things:?}");

    for thing in things {
        println!("{thing}");
    }
}

fn inspect_enum(item: Option<&str>) {
    if let Some(value) = item {
        println!("You passed in a {value}");
    }
}

fn do_math(x: i32) -> Result<i32, String> {
    if x == 1 {
        return Ok(100);
    }
    return Err(format!("I wanted the number 1 and you gave me a {} eye", x));
}

enum Snake {
    Apple,
    Cookies(u8),
    Sandwich { lettuce: bool, cheese: bool },
}

impl Snake {
    fn price(self) -> u8 {
        match self {
            Snake::Apple => 5,
            Snake::Cookies(num_cookies) => 2 * num_cookies,
            Snake::Sandwich { lettuce, cheese } => {
                let mut price = 10;
                if lettuce {
                    price += 1;
                }
                if cheese {
                    price += 2;
                }
                price
            }
        }
    }
}

fn enums_ex() {
    let maybe_fruit: Option<&str> = Some("apple");
    if maybe_fruit.is_some() {
        let fruit = maybe_fruit.unwrap();
        println!("{fruit}");
    }

    let maybe_plant: Option<&str> = None;
    let maybe_food: Option<&str> = Some("cake");
    inspect_enum(maybe_plant);
    inspect_enum(maybe_food);

    let numbers = vec![0, 1];
    for number in numbers {
        match do_math(number) {
            Ok(x) => println!("The result was {x}"),
            Err(msg) => println!("{msg}"),
        }
    }

    let healthy_snack = Snake::Apple;
    let sugary_snack = Snake::Cookies(18);
    let lunch = Snake::Sandwich {
        lettuce: false,
        cheese: true,
    };
    if let Snake::Apple = healthy_snack {
        println!("The healthy snack is an apple.")
    }
    if let Snake::Cookies(num_cookies) = sugary_snack {
        println!("The sugary snack is {} coookies", num_cookies);
    }
    if let Snake::Sandwich { lettuce, cheese } = lunch {
        let lattuce_msg = if lettuce { "does" } else { "does not" };
        let cheese_msg = if cheese { "does" } else { "does not" };
        println!(
            "The sandwich {} ave lattuce and {} have cheese.",
            lattuce_msg, cheese_msg
        );
    }

    println!("An apple costs ${}", healthy_snack.price());
    if let Snake::Cookies(number) = sugary_snack {
        println!("{} cookies cost ${}", number, sugary_snack.price());
    }
    if let Snake::Sandwich { lettuce, cheese } = lunch {
        let lattuce_message = if lettuce { " with lettuce" } else { "" };
        let cheese_message = if cheese { " with cheese" } else { "" };
        println!(
            "A sandwich{}{} costs ${}",
            lattuce_message,
            cheese_message,
            lunch.price()
        );
    }
}

fn closures_iterators() {
    // v.into_iter() consumes v, returns owned items  ex: for _ in v
    // v.iter() return immutable references           ex: for _ in &v
    // v.iter_mut() returns mutable reference         ex: for _ in &mut v

    let square = |x| x * x;
    println!("5 squared is {}", square(5));

    let pairs = vec![(0, 1), (2, 3), (4, 5)];
    pairs
        .into_iter()
        // .map(|t| (t.0 + 1, t.1))
        .map(|(x, y)| (x + 1, y))
        .for_each(|t| println!("{:?}", t));

    let mut numbers = vec![1, 2, 3, 4];
    for x in numbers.iter_mut() {
        *x *= 3;
    }
    println!("{:?}", numbers);

    let words = vec!["autobot", "beach", "car", "decepticon", "energon", "frothy"];
    let transformed = words
        .into_iter()
        .filter(|s| !s.contains("h"))
        .map(|w| w.to_uppercase())
        .collect::<Vec<_>>();
    println!("transformed: {:?}", transformed);
}

pub fn sploosh(x: i32, y: i32, z: i32) -> i32 {
    match (x, y, z) {
        (x, _, _) if x < 0 => 99,
        (1, 2, 3) => 4,
        (5, 6, 7) => 3,
        (x, y, z) => x + y - z,
    }
}

pub fn splish(a: i32, b: i32) -> i32 {
    -a + 3 * b
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_the_conditions() {
        assert_eq!(sploosh(1, 2, 3), 4);
        assert_ne!(sploosh(5, 6, 7), 4);
        assert_eq!(sploosh(-100, 1, 2), 99);
    }

    #[test]
    fn second_test() {
        assert!(splish(100, 10) < 0);
        assert!(splish(40, 20) > 0);
        assert!(splish(9, 3) == 0);
    }
}

fn logging() {
    env_logger::init();

    let mut skippy = Frog::new();
    skippy.hop();
    skippy.hop();
    skippy.hop();
    skippy.hop();
    skippy.hop();
    skippy.sleep();
}

use crossbeam::channel;
use log::set_max_level;
use std::thread;
use std::time::Duration;
use thiserror::Error;

fn sleep_ms(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}

fn expensive_sum(v: Vec<i32>) -> i32 {
    sleep_ms(500);
    println!("Child thread: just about finished");
    v.iter().filter(|&x| x % 2 == 0).map(|x| x * x).sum()
}

fn thread_channel() {
    let my_vector = vec![2, 5, 1, 0, 4, 3];

    let handle = thread::spawn(move || expensive_sum(my_vector));

    for letter in vec!["a", "b", "c", "d", "e", "f"] {
        println!("Main thread: Processing the letter '{}'", letter);
        sleep_ms(200);
    }

    let result = handle.join();
    let sum = result.unwrap();
    println!("The child thread's expensive sum is {}", sum);

    let (tx, rx) = channel::unbounded();
    let tx2 = tx.clone();
    let handle_a = thread::spawn(move || {
        sleep_ms(1000);
        tx2.send("Thread A: 1").unwrap();
        sleep_ms(200);
        tx2.send("Thread A: 2").unwrap();
    });
    sleep_ms(100);
    let handle_b = thread::spawn(move || {
        sleep_ms(0);
        tx.send("Thread B: 1").unwrap();
        sleep_ms(200);
        tx.send("Thread B: 2").unwrap();
    });

    for msg in rx {
        println!("Main thread: Received {}", msg);
    }

    handle_a.join().unwrap();
    let _ = handle_b.join();

    println!("Main thread: Exiting..")
}

fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
    let mut result = [[0; 3]; 3];
    for i in 0..3 {
        for j in 0..3 {
            result[j][i] = matrix[i][j];
        }
    }
    result
}

fn Arrays_test() {
    let matrix = [
        [101, 102, 103], // <-- the comment makes rustfmt add a newline
        [201, 202, 203],
        [301, 302, 303],
    ];

    println!("Original:");
    for row in matrix {
        println!("{row:?}");
    }

    let transposed = transpose(matrix);

    println!("\nTransposed:");
    for row in transposed {
        println!("{row:?}");
    }
}

fn shared_reference() {
    println!("shared references are read_only,and the referenced data connot change.");
    let a = 'A';
    let b = 'B';
    let mut r: &char = &a;
    println!("value: {}", r);

    r = &b;
    println!("value: {}", r);
}

fn exclusive_reference() {
    println!("exclusive reference allow changing the value they refer to. They have type &mut T.");
    let mut point = (1, 2);
    let x_coord = &mut point.0;
    *x_coord = 20;
    println!("point: {point:?}");
}

fn slices_tx() {
    let a: [i32; 6] = [10, 20, 30, 40, 50, 60];
    println!("a: {a:?}");

    let s: &[i32] = &a[2..4];
    println!("s: {s:?}");
}

// 静态变量在程序执行期间始终存在，因此不会被移动，拥有实际地址
static BANNER: &str = "Welcome to Rustos 3.14";

fn takes_tuple(tuple: (char, i32, bool)) {
    let a = tuple.0;
    let b = tuple.1;
    let c = tuple.2;

    let (a, b, c) = tuple;

    // _始终匹配任何值的模式，并丢弃匹配的值
    let (_, b, c) = tuple;
    // ..允许一次性忽略多个值
    let (.., c) = tuple;
}

fn pattern_matching() {
    takes_tuple(('a', 777, true));
}

fn pick<T>(cond: bool, left: T, right: T) -> T {
    if cond { left } else { right }
}

fn duplicate<T: Clone>(a: T) -> (T, T) {
    (a.clone(), a.clone())
}

#[derive(Debug)]
struct Foo(String);

impl From<u32> for Foo {
    fn from(from: u32) -> Foo {
        Foo(format!("Converted from integer: {from}"))
    }
}

impl From<bool> for Foo {
    fn from(from: bool) -> Foo {
        Foo(format!("Converted from bool: {from}"))
    }
}

struct Dog {
    name: String,
    age: i8,
}
struct Cat {
    lives: i8,
}

trait Pet {
    fn talk(&self) -> String;
}

impl Pet for Dog {
    fn talk(&self) -> String {
        format!("Woof, my name is {}!", self.name)
    }
}

impl Pet for Cat {
    fn talk(&self) -> String {
        String::from("Miau!")
    }
}

// Uses generics and static dispatch.
fn generic(pet: &impl Pet) {
    println!("Hello, who are you? {}", pet.talk());
}

// Uses type-erasure and dynamic dispatch. 虚函数表
fn dynamic(pet: &dyn Pet) {
    println!("Hello, who are you? {}", pet.talk());
}

fn Generics() {
    println!("picked a number: {:?}", pick(true, 222, 333));
    println!("picked a string: {:?}", pick(false, 'L', 'R'));

    let foo = String::from("foo");
    let pair = duplicate(foo);
    println!("{pair:?}");

    let from_int = Foo::from(123);
    let from_bool = Foo::from(true);
    println!("{:?}", from_int);
    println!("{:?}", from_bool);

    let cat = Cat { lives: 9 };
    let dog = Dog {
        name: String::from("Fido"),
        age: 5,
    };

    generic(&cat);
    generic(&dog);

    dynamic(&cat);
    dynamic(&dog);
}

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl std::ops::Add for Point {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

fn operator() {
    let p1 = Point { x: 1, y: 20 };
    let p2 = Point { x: 100, y: 200 };
    println!("{p1:?} + {p2:?} = {:?}", p1 + p2);
}

fn count_lines<R: Read>(reader: R) -> usize {
    let buf_reader = BufReader::new(reader);
    buf_reader.lines().count()
}

fn log<W: Write>(writer: &mut W, msg: &str) -> std::io::Result<()> {
    writer.write_all(msg.as_bytes())?;
    writer.write_all("\n".as_bytes())
}

fn io_test() -> std::io::Result<()> {
    let slice: &[u8] = b"foo\nbar\nbaz\n";
    println!("lines in slice: {}", count_lines(slice));

    let file = std::fs::File::open(std::env::current_exe()?)?;
    println!("lines in file: {}", count_lines(file));

    let mut buffer = Vec::new();
    log(&mut buffer, "Hello")?;
    log(&mut buffer, "World")?;
    println!("Logged: {buffer:?}");
    Ok(())
}

struct Droppable {
    name: &'static str,
}

impl Drop for Droppable {
    fn drop(&mut self) {
        println!("Dropping {}", self.name);
    }
}

// Box<T> 实现了Deref<Target = T> 这意味着直接从T在Box<T>上调用函数
#[derive(Debug)]
enum List<T> {
    Element(T, Box<List<T>>),
    Nil,
}

// 32 bytes
// String -> ptr (8)
//        -> cap (8)
//        -> len (8)
// i8 (1)
// 字节对齐 32
struct Dog1 {
    name: String,
    age: i8,
}

struct Cat1 {
    lives: i8,
}

trait Pet1 {
    fn talk(&self) -> String;
}

impl Pet1 for Dog1 {
    fn talk(&self) -> String {
        format!("Woof, my name is {}!", self.name)
    }
}

impl Pet1 for Cat1 {
    fn talk(&self) -> String {
        String::from("Miau!")
    }
}

fn memory_manager() {
    // Clone属性通常对值深拷贝
    // Copy属性可以将自定义类型添加到复制语义，或者用P1.clone()显示复制数据
    // Drop属性可以制定超出作用域时要运行的代码 类似 c++析构函数 打印dcba
    let a = Droppable { name: "a" };
    {
        let b = Droppable { name: "b" };
        {
            let c = Droppable { name: "c" };
            let d = Droppable { name: "d" };
        }
        println!("Exiting next block");
    }
    drop(a);
    println!("Existinig main");

    // Box是一个指向堆数据的私有指针 c++的unique_ptr
    //  编译时无法确定大小时使用
    //  需要转移大量数据的所有权，用指针
    let five = Box::new(55);
    println!("five {}", *five);

    let list: List<i32> = List::Element(1, Box::new(List::Element(2, Box::new(List::Nil))));
    println!("{list:?}");

    // Rc是一个引用计数指针 C++的shared_ptr
    //  Rc::string_count 检查引用次数
    //  Rc::downgrade 提供一个弱引用计数对象
    let a = Rc::new(10);
    let b = Rc::clone(&a);
    println!("a: {}", *a);
    println!("b: {}", *b);

    // 拥有特性的对象
    let pets: Vec<Box<dyn Pet1>> = vec![
        Box::new(Cat1 { lives: 9 }),
        Box::new(Dog1 {
            name: String::from("Fibo"),
            age: 5,
        }),
    ];
    for pet in pets {
        println!("Hello, who are you? {}", pet.talk());
    }
    println!(
        "{} {}",
        std::mem::size_of::<Dog1>(),
        std::mem::size_of::<Cat1>()
    );
    println!(
        "{} {}",
        std::mem::size_of::<&Dog1>(),
        std::mem::size_of::<&Cat1>()
    );
    // 数据指针 + 虚函数表指针 （16）
    println!("{}", std::mem::size_of::<&dyn Pet1>());
    println!("{}", std::mem::size_of::<Box<dyn Pet1>>());
}

#[derive(Debug)]
struct Point1(i32, i32);

fn add(p1: &Point1, p2: &Point1) -> Point1 {
    Point1(p1.0 + p2.0, p1.1 + p2.1)
}

fn borrowing() {
    let p1 = Point1(3, 4);
    let p2 = Point1(10, 20);
    let p3 = add(&p1, &p2);
    println!("{p1:?} + {p2:?} = {p3:?}");

    // Cell 封装了一个值，并且只允许使用对 Cell 共享引用来获取或设置该值。
    // 但是，它不允许对内部值进行任何引用。由于不存在引用，因此不会违反借用规则。
    // 在某些不可变的对象内部，需要修改某一个小数据”的情况。为了打破这个死板的限制，Rust 提供了 Cell 和 RefCell 这两个“作弊通道”
    // Cell 适合实现了Copy特征的基础类型
    // RefCell 适合复杂类型的内部修改 未实现Copy的
    let cell = Cell::new(5);
    cell.set(123);
    println!("{}", cell.get());

    let cell1 = RefCell::new(6);
    {
        let mut cell_ref = cell1.borrow_mut();
        *cell_ref = 123;
    }
    println!("{cell:?}");
}

fn identity(x: &i32) -> &i32 {
    x
}

fn Pick<'a>(c: bool, a: &'a i32, b: &'a i32) -> &'a i32 {
    if c { a } else { b }
}

fn find_nearest<'a>(points: &'a [Point1], query: &Point1) -> &'a Point1 {
    fn cab_distance(p1: &Point1, p2: &Point1) -> i32 {
        (p1.0 - p2.0).abs() + (p1.1 - p2.1).abs()
    }

    let mut nearest = None;
    for p in points {
        if let Some((_, nearest_dist)) = nearest {
            let dist = cab_distance(p, query);
            if dist < nearest_dist {
                nearest = Some((p, dist));
            }
        } else {
            nearest = Some((p, cab_distance(p, query)));
        };
    }

    nearest.map(|(p, _)| p).unwrap()
    // query // What happens if we do this instead?
}

#[derive(Debug)]
enum HighlightColor {
    Pink,
    Yellow,
}

#[derive(Debug)]
// & <'document> str 相当于c++的std::string_view
struct Highlight<'document> {
    // &'a 字符串切片
    slice: &'document str,
    color: HighlightColor,
}

fn Lifetimes() {
    let mut x = 123;
    let out = identity(&x);

    println!("{}", out);

    let mut a = 5;
    let mut b = 10;
    let r = Pick(true, &a, &b);

    println!("{}", r);

    let points = &[Point1(1, 0), Point1(1, 0), Point1(-1, 0), Point1(0, -1)];
    let query = Point1(0, 2);
    let nearest = find_nearest(points, &query);

    // `query` isn't borrowed at this point.
    drop(query);

    dbg!(nearest);

    // 如果数据类型存储借用的数据，则必须为其添加生命周期注释
    let doc = String::from("The quick brown fox jumps over the lazy dog.");
    let noun = Highlight {
        slice: &doc[16..19],
        color: HighlightColor::Yellow,
    };
    let verb = Highlight {
        slice: &doc[20..25],
        color: HighlightColor::Pink,
    };
    // drop(doc);
    dbg!(noun);
    dbg!(verb);
}

struct SliceIter<'s> {
    slice: &'s [i32],
    i: usize,
}

impl<'s> Iterator for SliceIter<'s> {
    type Item = &'s i32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.i == self.slice.len() {
            None
        } else {
            let next = &self.slice[self.i];
            self.i += 1;
            Some(next)
        }
    }
}

struct Grid {
    x_coords: Vec<u32>,
    y_coords: Vec<u32>,
}

struct GridIter {
    grid: Grid,
    i: usize,
    j: usize,
}

impl IntoIterator for Grid {
    type Item = (u32, u32);
    type IntoIter = GridIter;
    fn into_iter(self) -> GridIter {
        GridIter {
            grid: self,
            i: 0,
            j: 0,
        }
    }
}

impl Iterator for GridIter {
    type Item = (u32, u32);

    fn next(&mut self) -> Option<(u32, u32)> {
        if self.i >= self.grid.x_coords.len() {
            self.i = 0;
            self.j += 1;
            if self.j >= self.grid.y_coords.len() {
                return None;
            }
        }
        let res = Some((self.grid.x_coords[self.i], self.grid.y_coords[self.j]));
        self.i += 1;
        res
    }
}

fn iterator_ex() {
    let array = [2, 4, 6, 8];
    let mut i = 0;
    while i < array.len() {
        let elem = array[i];
        i += 1;
        println!("{}", elem);
    }

    // &取指针地址 iter迭代器
    let slice = &[2, 4, 6, 8];
    let iter = SliceIter { slice, i: 0 };
    for elem in iter {
        dbg!(elem);
    }

    // Iterator trait提供了70多个辅助方法，用于构建自定义迭代器
    let result: i32 = (1..=10).filter(|x| x % 2 == 0).map(|x| x * x).sum();

    println!(
        "The sum of squares of even numbers from 1 to 10 is {}",
        result
    );

    let primes = vec![2, 3, 5, 7];
    let prime_squares = primes.into_iter().map(|p| p * p).collect::<Vec<_>>();
    println!("prime_squares: {prime_squares:?}");

    let grid = Grid {
        x_coords: vec![3, 55, 7, 9],
        y_coords: vec![10, 20, 30, 40],
    };
    for (x, y) in grid {
        println!("point = {x},{y}");
    }
}

#[derive(Debug)]
enum ReadUsernameError {
    ioError(io::Error),
    EmptyUsername(String),
}

impl Error for ReadUsernameError {}

impl fmt::Display for ReadUsernameError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::ioError(e) => write!(f, "I/O error: {e}"),
            Self::EmptyUsername(path) => write!(f, "Found no username in {path}"),
        }
    }
}

impl From<io::Error> for ReadUsernameError {
    fn from(err: io::Error) -> Self {
        Self::ioError(err)
    }
}

fn read_username(path: &str) -> Result<String, ReadUsernameError> {
    let mut username = String::with_capacity(100);
    fs::File::open(path)?.read_to_string(&mut username)?;
    if username.is_empty() {
        return Err(ReadUsernameError::EmptyUsername(String::from(path)));
    }
    Ok(username)
}

// 动态错误类型 std::error::Error trait可以轻松创建一个可以包含任何错误的trait对象
fn read_count(path: &str) -> Result<i32, Box<dyn Error>> {
    let mut count_str = String::new();
    fs::File::open(path)?.read_to_string(&mut count_str)?;
    let count: i32 = count_str.parse()?;
    Ok(count)
}

// thiserror
#[derive(Debug, Error)]
enum readUsernameError {
    #[error("I/O error: {0}")]
    IoError(#[from] io::Error),
    #[error("Found no username in {0}")]
    EmptyUsername(String),
}

fn Read_username(path: &str) -> Result<String, readUsernameError> {
    let mut username = String::with_capacity(100);
    fs::File::open(path)?.read_to_string(&mut username)?;
    if username.is_empty() {
        return Err(readUsernameError::EmptyUsername(String::from(path)));
    }
    Ok(username)
}

// anyhow
// 提供了一个丰富的错误类型，支持携带额外的上下文信息，用于提供程序在发生错误之前所执行操作的语义跟踪
#[derive(Clone, Debug, Eq, Error, PartialEq)]
#[error("Found no username in {0}")]
struct EmptyUsernameError(String);

fn read_username2(path: &str) -> Result<String> {
    let mut username = String::with_capacity(100);
    fs::File::open(path)
        .with_context(|| format!("Failed to open {path}"))?
        .read_to_string(&mut username)
        .context("Failed to read")?;
    if username.is_empty() {
        bail!(EmptyUsernameError(path.to_string()));
    }
    Ok(username)
}

fn error_handle() {
    // panic
    // 1.针对无法恢复且意料之外的错误而触发的
    // 2. panic会展开堆栈，丢弃值，就像函数返回一样
    // 3.如果不不能接受崩溃，请使用不会引发恐慌的API

    // rust最主要错误处理机制是Result枚举
    let file: Result<File, std::io::Error> = File::open("diary.txt");
    match file {
        Ok(mut file) => {
            let mut contents = String::new();
            if let Ok(bytes) = file.read_to_string(&mut contents) {
                println!("Dear diary: {contents} ({bytes} bytes)");
            } else {
                println!("Could not read file content");
            }
        }
        Err(err) => {
            println!("The diary could not be opened: {err}");
        }
    }

    // FROM::from 调用表示我们尝试将错误类型转换为函数返回的类型
    let username = read_username("config.dat");
    println!("username or erroro: {username:?}");

    fs::write("count.dat", "1i3").unwrap();
    match read_count("count.dat") {
        Ok(count) => println!("Count: {count}"),
        Err(err) => println!("Error: {err}"),
    }

    match read_username("config.dat") {
        Ok(username) => println!("Username: {username}"),
        Err(err) => println!("Error: {err}"),
    }

    match read_username2("config.dat") {
        Ok(username) => println!("Username: {username}"),
        Err(err) => println!("Error: {err:?}"),
    }
}

static mut COUNTER: u32 = 0;

fn add_to_counter(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

#[repr(C)]
union MyUnion {
    i: u8,
    b: bool,
}

// 如果函数需要特定的前提条件以避免未定义行为，可以标记为unsafe
unsafe fn swap(a: *mut u8, b: *mut u8) {
    unsafe {
        let temp = *a;
        *a = *b;
        *b = temp;
    }
}

// 使用unsafe extern声明外部函数给rust内部访问，在extern中必须标记safe或unsafe
unsafe extern "C" {
    safe fn abs(input: i32) -> i32;

    unsafe fn strlen(s: *const c_char) -> usize;
}

fn unsafe_rust() {
    // 创建指针是安全的，但解引用需要使用unsafe
    let mut x = 10;
    let p1: *mut i32 = &raw mut x;
    let p2 = p1 as *const i32;
    unsafe {
        dbg!(*p1);
        *p1 = 6;
        dbg!(*p2);
    }

    // 读取不可变静态变量是安全的，可变量静态变量的读写是不安全的
    // 要合理使用可变静态变量，需要在不借助编译器帮助的情况下对并发性进行推理
    add_to_counter(42);
    unsafe {
        dbg!(COUNTER);
    }

    let u = MyUnion { i: 42 };
    println!("int: {}", unsafe { u.i });
    println!("bool: {}", unsafe { u.b });

    let mut a = 42;
    let mut b = 66;
    unsafe {
        swap(&mut a, &mut b);
    }

    println!("a = {},b = {}", a, b);

    println!("Absolute value of -3 according to C: {}", abs(-3));
    unsafe {
        println!("String length: {}", strlen(c"String".as_ptr()));
    }
}

fn concurrency() {
    let a = thread::spawn(|| {
        for i in 0..10 {
            println!("COunt in thread: {i}!");
            thread::sleep(Duration::from_millis(5));
        }
    });

    for i in 0..5 {
        println!("Main thread: {i}");
        thread::sleep(Duration::from_millis(5));
    }

    // 从环境中借用资源 需要用作用域线程2
    let s = String::from("Hello");
    thread::scope(|scope| {
        scope.spawn(|| {
            dbg!(s.len());
        });
    });

    // sender receivers mpsc代表多生产者单消费者
    let (tx, rx) = mpsc::channel();
    tx.send(10).unwrap();
    tx.send(20).unwrap();

    println!("Received: {:?}", rx.recv());
    println!("Received: {:?}", rx.recv());

    let tx2 = tx.clone();
    tx2.send(30).unwrap();
    println!("Received: {:?}", rx.recv());

    // 无界channel
    let (tx3, rx2) = mpsc::channel();
    thread::spawn(move || {
        let thread_id = thread::current().id();
        for i in 0..10 {
            tx3.send(format!("Message {i}")).unwrap();
            println!("{thread_id:?}: send Message {i}");
        }
        println!("{thread_id:?}: done");
    });

    for msg in rx2 {
        println!("Main: got {msg}");
    }

    // 有界channel 满了会阻塞
    let (tx4, rx3) = mpsc::sync_channel(3);

    thread::spawn(move || {
        let thread_id = thread::current().id();
        for i in 0..10 {
            tx4.send(format!("bounded channel Message {i}")).unwrap();
            println!("{thread_id:?}: bounded channel sent Message {i}");
        }
        println!("{thread_id:?}: bounded channel done");
    });
    thread::sleep(Duration::from_millis(100));

    for msg in rx3 {
        println!("Main: bounded channel got {msg}");
    }
}

fn main() {
    // greet_world();
    // test_fib();
    // spanish::greet();
    // control_flow();
    // string_ex();
    // ownership();
    // struct_ex();
    // traits_ex();
    // collection_ex();
    // enums_ex();
    // closures_iterators();
    // logging();
    // thread_channel();
    // Arrays_test();
    // shared_reference();
    // exclusive_reference();
    // slices_tx();
    // pattern_matching();
    // Generics();
    // operator();
    // io_test();
    // memory_manager();
    // borrowing();
    // Lifetimes();
    // iterator_ex();
    // error_handle();
    // unsafe_rust();
    concurrency();
}
