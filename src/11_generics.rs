// 11_generics.rs - 泛型与 Traits
// 本文件展示 Rust 中泛型和 Trait 的使用

fn main() {
    println!("=== 泛型与 Traits ===\n");

    // 1. 泛型函数
    println!("1. 泛型函数：");
    let numbers = vec![1, 2, 3, 4, 5];
    let strings = vec!["hello", "world", "rust"];

    let largest_number = largest(&numbers);
    let largest_string = largest(&strings);

    println!("最大数字: {}", largest_number);
    println!("最长字符串: {}", largest_string);

    // 2. 泛型结构体
    println!("\n2. 泛型结构体：");
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };

    println!("整数点: ({}, {})", integer_point.x, integer_point.y);
    println!("浮点点: ({}, {})", float_point.x, float_point.y);

    // 3. Trait 定义与实现
    println!("\n3. Trait 示例：");
    let tweet = Tweet {
        username: String::from("rustacean"),
        content: String::from("Rust 很棒！"),
    };

    println!("摘要: {}", tweet.summarize());

    // 4. Trait 作为参数
    println!("\n4. Trait 作为参数：");
    notify(&tweet);

    println!("\n=== 泛型与 Traits 学习完成 ===");
}

// 泛型函数
fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item;
        }
    }

    largest
}

// 泛型结构体
#[derive(Debug)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

// Trait 定义
pub trait Summary {
    fn summarize(&self) -> String;
}

pub struct Tweet {
    pub username: String,
    pub content: String,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

// Trait 作为参数
pub fn notify(item: &impl Summary) {
    println!("重要通知! {}", item.summarize());
}
