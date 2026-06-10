use std::io::{BufRead, BufReader, Read, Write};
use std::thread::sleep;
use std::{char::ToUppercase, string};

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
use std::thread;
use std::time::Duration;

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
    io_test();
}
