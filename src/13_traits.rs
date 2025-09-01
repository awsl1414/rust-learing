// 13_traits.rs - Trait 与 Trait Bound
// 本文件展示 Rust 中 trait 的定义、实现、trait bound 和高级特性

fn main() {
    println!("=== Rust Trait 与 Trait Bound ===\n");
    
    // ========== 基本 Trait ==========
    println!("========== 基本 Trait ==========");
    
    // 1. 基本 trait 实现
    println!("\n1. 基本 trait 使用：");
    
    let tweet = Tweet {
        username: String::from("alice"),
        content: String::from("今天学习了 Rust trait，非常有趣！"),
        reply: false,
        retweet: false,
    };
    
    let article = NewsArticle {
        headline: String::from("Rust 1.70 发布"),
        location: String::from("全球"),
        author: String::from("Rust 团队"),
        content: String::from("Rust 1.70 版本带来了许多新特性和改进..."),
    };
    
    println!("推文摘要: {}", tweet.summarize());
    println!("文章摘要: {}", article.summarize());
    
    // 2. 默认实现
    println!("\n2. 默认实现：");
    
    let book = Book {
        title: String::from("Rust 程序设计语言"),
        author: String::from("Steve Klabnik"),
        pages: 500,
    };
    
    println!("书籍摘要: {}", book.summarize());
    println!("书籍作者: {}", book.author_info());
    
    // ========== Trait 作为参数 ==========
    println!("\n========== Trait 作为参数 ==========");
    
    // 3. trait 作为参数
    println!("\n3. trait 作为参数：");
    
    notify(&tweet);
    notify(&article);
    notify(&book);
    
    // 4. trait bound 语法
    println!("\n4. trait bound 语法：");
    
    notify_bound(&tweet);
    notify_multiple_bounds(&tweet);
    
    // 5. where 子句
    println!("\n5. where 子句：");
    
    let numbers = vec![1, 2, 3, 4, 5];
    let result = some_complex_function(&numbers, &tweet);
    println!("复杂函数结果: {:?}", result);
    
    // ========== 多个 Trait ==========
    println!("\n========== 多个 Trait ==========");
    
    // 6. 实现多个 trait
    println!("\n6. 多个 trait 实现：");
    
    let person = Person {
        name: String::from("张三"),
        age: 30,
    };
    
    println!("人员信息: {}", person.display());
    println!("人员描述: {}", person.describe());
    person.greet();
    
    // 7. trait 对象
    println!("\n7. trait 对象：");
    
    let drawable_objects: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle { radius: 5.0 }),
        Box::new(Rectangle { width: 10.0, height: 20.0 }),
    ];
    
    for obj in drawable_objects {
        obj.draw();
        println!("面积: {:.2}", obj.area());
    }
    
    // ========== 关联类型 ==========
    println!("\n========== 关联类型 ==========");
    
    // 8. 关联类型
    println!("\n8. 关联类型：");
    
    let counter = Counter::new();
    
    for value in counter.take(5) {
        println!("计数: {}", value);
    }
    
    // 9. 类型转换 trait
    println!("\n9. 类型转换：");
    
    let temperature_c = TemperatureC(25.0);
    let temperature_f: TemperatureF = temperature_c.into();
    println!("25°C = {:.1}°F", temperature_f.0);
    
    let temp_back: TemperatureC = TemperatureF(77.0).into();
    println!("77.0°F = {:.1}°C", temp_back.0);
    
    // ========== 运算符重载 ==========
    println!("\n========== 运算符重载 ==========");
    
    // 10. 运算符重载
    println!("\n10. 运算符重载：");
    
    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    
    let p3 = p1 + p2;
    println!("点加法: {:?}", p3);
    
    let mut v1 = Vector2D { x: 1.0, y: 2.0 };
    let v2 = Vector2D { x: 3.0, y: 4.0 };
    
    println!("向量1: {:?}", v1);
    println!("向量2: {:?}", v2);
    
    v1 += v2;
    println!("相加后的向量1: {:?}", v1);
    
    // ========== 派生 Trait ==========
    println!("\n========== 派生 Trait ==========");
    
    // 11. 派生 trait
    println!("\n11. 派生 trait：");
    
    let color1 = Color::Red;
    let color2 = Color::Red;
    let color3 = Color::Blue;
    
    println!("颜色1: {:?}", color1);
    println!("颜色1 == 颜色2: {}", color1 == color2);
    println!("颜色1 == 颜色3: {}", color1 == color3);
    
    let mut colors = vec![Color::Red, Color::Green, Color::Blue];
    println!("排序前: {:?}", colors);
    colors.sort();
    println!("排序后: {:?}", colors);
    
    // ========== 高级 Trait 特性 ==========
    println!("\n========== 高级 Trait 特性 ==========");
    
    // 12. 孤儿规则和 newtype 模式
    println!("\n12. newtype 模式：");
    
    let wrapper = Wrapper(vec![1, 2, 3, 4, 5]);
    println!("包装器显示: {}", wrapper);
    
    // 13. trait 条件实现
    println!("\n13. 条件实现：");
    
    let pair = Pair::new(5, 10);
    pair.cmp_display(); // 只有当 T 实现了 Display + PartialOrd 才可用
    
    // ========== 实际应用示例 ==========
    println!("\n========== 实际应用示例 ==========");
    
    // 14. 序列化 trait
    println!("\n14. 序列化示例：");
    
    let user = User {
        id: 1,
        name: String::from("Alice"),
        email: String::from("alice@example.com"),
    };
    
    println!("JSON: {}", user.to_json());
    println!("XML: {}", user.to_xml());
    
    // 15. 验证 trait
    println!("\n15. 验证示例：");
    
    let valid_email = "user@example.com";
    let invalid_email = "invalid-email";
    
    println!("'{}' 有效: {}", valid_email, valid_email.is_valid());
    println!("'{}' 有效: {}", invalid_email, invalid_email.is_valid());
    
    let password = "StrongPass123!";
    println!("密码强度: {:?}", password.strength());
    
    println!("\n=== Trait 与 Trait Bound 学习完成 ===");
}

// ========== 基本 Trait 定义 ==========

trait Summary {
    fn summarize(&self) -> String;
    
    // 默认实现
    fn author_info(&self) -> String {
        String::from("(作者信息未知)")
    }
}

// ========== Trait 实现 ==========

#[derive(Debug)]
struct Tweet {
    username: String,
    content: String,
    reply: bool,
    retweet: bool,
}

impl Summary for Tweet {
    fn summarize(&self) -> String {
        format!("{}: {}", self.username, self.content)
    }
}

struct NewsArticle {
    headline: String,
    location: String,
    author: String,
    content: String,
}

impl Summary for NewsArticle {
    fn summarize(&self) -> String {
        format!("{}, by {} ({})", self.headline, self.author, self.location)
    }
    
    fn author_info(&self) -> String {
        format!("作者: {}", self.author)
    }
}

struct Book {
    title: String,
    author: String,
    pages: u32,
}

impl Summary for Book {
    fn summarize(&self) -> String {
        format!("《{}》- {} 页", self.title, self.pages)
    }
    
    fn author_info(&self) -> String {
        format!("作者: {}", self.author)
    }
}

// ========== Trait 作为参数 ==========

// 基本语法
fn notify(item: &impl Summary) {
    println!("通知: {}", item.summarize());
}

// trait bound 语法
fn notify_bound<T: Summary>(item: &T) {
    println!("通知 (bound): {}", item.summarize());
}

// 多个 trait bound
fn notify_multiple_bounds<T>(item: &T) 
where 
    T: Summary + std::fmt::Debug,
{
    println!("通知 (多重约束): {}", item.summarize());
}

// 复杂的 where 子句
fn some_complex_function<T, U>(t: &T, u: &U) -> Vec<String>
where
    T: std::fmt::Debug + Clone,
    U: Summary + std::fmt::Debug,
{
    vec![
        format!("T: {:?}", t),
        format!("U: {}", u.summarize()),
    ]
}

// ========== 多个 Trait ==========

trait Display {
    fn display(&self) -> String;
}

trait Describe {
    fn describe(&self) -> String;
}

trait Greet {
    fn greet(&self);
}

struct Person {
    name: String,
    age: u32,
}

impl Display for Person {
    fn display(&self) -> String {
        format!("{} ({}岁)", self.name, self.age)
    }
}

impl Describe for Person {
    fn describe(&self) -> String {
        format!("这是一个名叫{}的人，今年{}岁", self.name, self.age)
    }
}

impl Greet for Person {
    fn greet(&self) {
        println!("你好，我是{}", self.name);
    }
}

// ========== Trait 对象 ==========

trait Drawable {
    fn draw(&self);
    fn area(&self) -> f64;
}

struct Circle {
    radius: f64,
}

impl Drawable for Circle {
    fn draw(&self) {
        println!("绘制半径为 {} 的圆形", self.radius);
    }
    
    fn area(&self) -> f64 {
        std::f64::consts::PI * self.radius * self.radius
    }
}

struct Rectangle {
    width: f64,
    height: f64,
}

impl Drawable for Rectangle {
    fn draw(&self) {
        println!("绘制 {}×{} 的矩形", self.width, self.height);
    }
    
    fn area(&self) -> f64 {
        self.width * self.height
    }
}

// ========== 关联类型 ==========

trait Iterator {
    type Item; // 关联类型
    
    fn next(&mut self) -> Option<Self::Item>;
}

struct Counter {
    current: usize,
    max: usize,
}

impl Counter {
    fn new() -> Counter {
        Counter { current: 0, max: usize::MAX }
    }
}

impl Iterator for Counter {
    type Item = usize;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < self.max {
            let current = self.current;
            self.current += 1;
            Some(current)
        } else {
            None
        }
    }
}

// 为 Counter 实现标准库的 Iterator
impl std::iter::Iterator for Counter {
    type Item = usize;
    
    fn next(&mut self) -> Option<Self::Item> {
        if self.current < 10 { // 限制为 10 个元素以便演示
            let current = self.current;
            self.current += 1;
            Some(current)
        } else {
            None
        }
    }
}

// ========== 类型转换 ==========

struct TemperatureC(f64);
struct TemperatureF(f64);

impl From<TemperatureC> for TemperatureF {
    fn from(c: TemperatureC) -> Self {
        TemperatureF(c.0 * 9.0 / 5.0 + 32.0)
    }
}

impl From<TemperatureF> for TemperatureC {
    fn from(f: TemperatureF) -> Self {
        TemperatureC((f.0 - 32.0) * 5.0 / 9.0)
    }
}

// ========== 运算符重载 ==========

use std::ops::Add;

#[derive(Debug, Copy, Clone)]
struct Point {
    x: i32,
    y: i32,
}

impl Add for Point {
    type Output = Point;
    
    fn add(self, other: Point) -> Point {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

use std::ops::AddAssign;

#[derive(Debug, Copy, Clone)]
struct Vector2D {
    x: f64,
    y: f64,
}

impl AddAssign for Vector2D {
    fn add_assign(&mut self, other: Vector2D) {
        self.x += other.x;
        self.y += other.y;
    }
}

// ========== 派生 Trait ==========

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone)]
enum Color {
    Red,
    Green,
    Blue,
}

// ========== 孤儿规则和 newtype ==========

struct Wrapper(Vec<i32>);

impl std::fmt::Display for Wrapper {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}]", self.0.iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(", "))
    }
}

// ========== 条件实现 ==========

struct Pair<T> {
    x: T,
    y: T,
}

impl<T> Pair<T> {
    fn new(x: T, y: T) -> Self {
        Self { x, y }
    }
}

impl<T: std::fmt::Display + PartialOrd> Pair<T> {
    fn cmp_display(&self) {
        if self.x >= self.y {
            println!("最大值是 x = {}", self.x);
        } else {
            println!("最大值是 y = {}", self.y);
        }
    }
}

// ========== 实际应用示例 ==========

// 序列化 trait
trait Serialize {
    fn to_json(&self) -> String;
    fn to_xml(&self) -> String;
}

struct User {
    id: u32,
    name: String,
    email: String,
}

impl Serialize for User {
    fn to_json(&self) -> String {
        format!(
            r#"{{"id": {}, "name": "{}", "email": "{}"}}"#,
            self.id, self.name, self.email
        )
    }
    
    fn to_xml(&self) -> String {
        format!(
            "<user><id>{}</id><name>{}</name><email>{}</email></user>",
            self.id, self.name, self.email
        )
    }
}

// 验证 trait
trait Validate {
    fn is_valid(&self) -> bool;
}

impl Validate for str {
    fn is_valid(&self) -> bool {
        self.contains('@') && self.contains('.')
    }
}

#[derive(Debug)]
enum PasswordStrength {
    Weak,
    Medium,
    Strong,
}

trait PasswordValidation {
    fn strength(&self) -> PasswordStrength;
}

impl PasswordValidation for str {
    fn strength(&self) -> PasswordStrength {
        let has_uppercase = self.chars().any(|c| c.is_uppercase());
        let has_lowercase = self.chars().any(|c| c.is_lowercase());
        let has_digit = self.chars().any(|c| c.is_numeric());
        let has_special = self.chars().any(|c| !c.is_alphanumeric());
        
        match (self.len() >= 8, has_uppercase, has_lowercase, has_digit, has_special) {
            (true, true, true, true, true) => PasswordStrength::Strong,
            (true, _, true, true, _) => PasswordStrength::Medium,
            _ => PasswordStrength::Weak,
        }
    }
}

/* 
重要概念总结：

Trait 基础：
- 定义共享行为的接口
- 类似其他语言的接口或抽象类
- 使用 trait 关键字定义

Trait 实现：
- impl Trait for Type 语法
- 可以为任何类型实现 trait
- 可以提供默认实现

Trait 作为参数：
- impl Trait 语法：简洁的参数类型
- Trait bound：<T: Trait> 语法
- where 子句：复杂约束的清晰表达

Trait 对象：
- dyn Trait：动态分发
- 允许存储不同类型但实现相同 trait 的对象
- 运行时多态

关联类型：
- type Item 语法
- 与泛型参数的区别
- 每种实现只能有一个关联类型

运算符重载：
- 实现 std::ops 中的 trait
- Add, Sub, Mul, Div 等
- 使代码更直观

派生宏：
- #[derive(Debug, Clone, PartialEq)] 等
- 自动实现常用 trait
- 减少样板代码

孤儿规则：
- 要么拥有 trait，要么拥有类型
- newtype 模式绕过限制
- 避免上游依赖冲突

最佳实践：
- 优先使用标准库 trait
- 设计简洁的 trait 接口
- 合理使用默认实现
- 考虑 trait 对象的性能影响

编译运行：
cargo run --bin 13_traits
*/
