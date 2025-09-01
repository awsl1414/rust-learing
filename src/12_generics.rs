// 12_generics.rs - 泛型
// 本文件展示 Rust 中泛型的使用，包括泛型函数、结构体、枚举和方法

fn main() {
    println!("=== Rust 泛型 ===\n");
    
    // ========== 泛型函数 ==========
    println!("========== 泛型函数 ==========");
    
    // 1. 基本泛型函数
    println!("\n1. 基本泛型函数：");
    
    let number_list = vec![34, 50, 25, 100, 65];
    let largest_number = largest(&number_list);
    println!("最大的数字: {}", largest_number);
    
    let char_list = vec!['y', 'm', 'a', 'q'];
    let largest_char = largest(&char_list);
    println!("最大的字符: {}", largest_char);
    
    // 2. 多个泛型参数
    println!("\n2. 多个泛型参数：");
    
    let point1 = create_point(5, 10);
    let point2 = create_point(1.0, 4.0);
    let point3 = create_point("hello", "world");
    
    println!("整数点: {:?}", point1);
    println!("浮点数点: {:?}", point2);
    println!("字符串点: {:?}", point3);
    
    // 3. 泛型函数的约束
    println!("\n3. 泛型函数约束：");
    
    let numbers = vec![1, 2, 3, 4, 5];
    let sum = calculate_sum(&numbers);
    println!("数字求和: {}", sum);
    
    let floats = vec![1.1, 2.2, 3.3, 4.4, 5.5];
    let float_sum = calculate_sum(&floats);
    println!("浮点数求和: {}", float_sum);
    
    // ========== 泛型结构体 ==========
    println!("\n========== 泛型结构体 ==========");
    
    // 4. 基本泛型结构体
    println!("\n4. 泛型结构体：");
    
    let integer_point = Point { x: 5, y: 10 };
    let float_point = Point { x: 1.0, y: 4.0 };
    
    println!("整数点: {:?}", integer_point);
    println!("浮点数点: {:?}", float_point);
    
    // 5. 混合类型泛型结构体
    println!("\n5. 混合类型泛型结构体：");
    
    let mixed_point = MixedPoint { x: 5, y: 4.0 };
    println!("混合点: {:?}", mixed_point);
    
    // 6. 泛型结构体方法
    println!("\n6. 泛型结构体方法：");
    
    let p1 = Point { x: 5.0, y: 10.0 };
    let p2 = Point { x: 3.0, y: 4.0 };
    
    println!("点1到原点距离: {:.2}", p1.distance_from_origin());
    // println!("点1到点2距离: {:.2}", p1.distance_to(&p2));
    
    // 7. 结构体中的复杂泛型
    println!("\n7. 复杂泛型结构体：");
    
    let mut container = Container::new();
    container.add(42);
    container.add(100);
    container.add(25);
    
    println!("容器内容: {:?}", container.get_all());
    
    if let Some(value) = container.get(1) {
        println!("索引1的值: {}", value);
    }
    
    // ========== 泛型枚举 ==========
    println!("\n========== 泛型枚举 ==========");
    
    // 8. 自定义泛型枚举
    println!("\n8. 泛型枚举：");
    
    let success: MyResult<i32, &str> = MyResult::Success(42);
    let failure: MyResult<i32, &str> = MyResult::Failure("出错了");
    
    match success {
        MyResult::Success(value) => println!("成功: {}", value),
        MyResult::Failure(error) => println!("失败: {}", error),
    }
    
    match failure {
        MyResult::Success(value) => println!("成功: {}", value),
        MyResult::Failure(error) => println!("失败: {}", error),
    }
    
    // 9. Option 和 Result 的使用
    println!("\n9. 标准库泛型枚举：");
    
    let some_number = Some(5);
    let some_string = Some("一个字符串");
    let absent_number: Option<i32> = None;
    
    println!("Some number: {:?}", some_number);
    println!("Some string: {:?}", some_string);
    println!("Absent: {:?}", absent_number);
    
    // ========== 泛型方法 ==========
    println!("\n========== 泛型方法 ==========");
    
    // 10. 泛型方法
    println!("\n10. 泛型方法：");
    
    let p1 = MixedPoint { x: 5, y: 10.4 };
    let p2 = MixedPoint { x: "Hello", y: 'c' };
    
    println!("混合后的点1: x = {}, y = {}", p1.x, p1.y);
    println!("混合后的点2: x = {}, y = {}", p2.x, p2.y);
    
    // ========== 实际应用示例 ==========
    println!("\n========== 实际应用示例 ==========");
    
    // 11. 泛型数据结构
    println!("\n11. 泛型栈：");
    
    let mut int_stack = Stack::new();
    int_stack.push(1);
    int_stack.push(2);
    int_stack.push(3);
    
    println!("栈大小: {}", int_stack.size());
    
    while let Some(value) = int_stack.pop() {
        println!("弹出: {}", value);
    }
    
    let mut string_stack = Stack::new();
    string_stack.push(String::from("first"));
    string_stack.push(String::from("second"));
    string_stack.push(String::from("third"));
    
    println!("\n字符串栈:");
    while let Some(value) = string_stack.pop() {
        println!("弹出: {}", value);
    }
    
    // 12. 泛型缓存
    println!("\n12. 泛型缓存：");
    
    let mut cache = Cache::new();
    
    cache.set("key1", 42);
    cache.set("key2", 100);
    
    if let Some(value) = cache.get(&"key1") {
        println!("缓存命中 key1: {}", value);
    }
    
    if let Some(value) = cache.get(&"key3") {
        println!("缓存命中 key3: {}", value);
    } else {
        println!("缓存未命中 key3");
    }
    
    // 13. 泛型比较器
    println!("\n13. 泛型比较和查找：");
    
    let numbers = vec![64, 34, 25, 12, 22, 11, 90];
    println!("原数组: {:?}", numbers);
    
    if let Some(max) = find_max(&numbers) {
        println!("最大值: {}", max);
    }
    
    if let Some(min) = find_min(&numbers) {
        println!("最小值: {}", min);
    }
    
    let words = vec!["apple", "banana", "cherry", "date"];
    println!("单词数组: {:?}", words);
    
    if let Some(longest) = find_max(&words) {
        println!("字典序最大: {}", longest);
    }
    
    // 14. 泛型配对
    println!("\n14. 泛型配对：");
    
    let pair1 = Pair::new(42, "hello");
    println!("配对1: {:?}", pair1);
    
    let (first, second) = pair1.into_tuple();
    println!("解构: first = {}, second = {}", first, second);
    
    let pair2 = Pair::new(3.14, true);
    println!("配对2: {:?}", pair2);
    
    // 15. 泛型迭代器适配
    println!("\n15. 泛型转换：");
    
    let numbers = vec![1, 2, 3, 4, 5];
    let doubled: Vec<i32> = map_vec(&numbers, |x| x * 2);
    println!("原数组: {:?}", numbers);
    println!("翻倍后: {:?}", doubled);
    
    let words = vec!["hello", "world", "rust"];
    let lengths: Vec<usize> = map_vec(&words, |s| s.len());
    println!("单词: {:?}", words);
    println!("长度: {:?}", lengths);
    
    println!("\n=== 泛型学习完成 ===");
}

// ========== 泛型函数 ==========

fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
    let mut largest = list[0];
    
    for &item in list {
        if item > largest {
            largest = item;
        }
    }
    
    largest
}

fn create_point<T, U>(x: T, y: U) -> (T, U) {
    (x, y)
}

fn calculate_sum<T>(list: &[T]) -> T 
where 
    T: std::ops::Add<Output = T> + Copy + Default
{
    let mut sum = T::default();
    for &item in list {
        sum = sum + item;
    }
    sum
}

// ========== 泛型结构体 ==========

#[derive(Debug, Clone)]
struct Point<T> {
    x: T,
    y: T,
}

impl<T> Point<T> {
    fn new(x: T, y: T) -> Self {
        Point { x, y }
    }
    
    fn x(&self) -> &T {
        &self.x
    }
    
    fn y(&self) -> &T {
        &self.y
    }
}

// 为特定类型实现方法
impl Point<f64> {
    fn distance_from_origin(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

impl<T> Point<T> 
where 
    T: std::ops::Sub<Output = T> + std::ops::Mul<Output = T> + std::ops::Add<Output = T> + Copy,
    f64: From<T>
{
    fn distance_to(&self, other: &Point<T>) -> f64 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let distance_squared = dx * dx + dy * dy;
        f64::from(distance_squared).sqrt()
    }
}

// 混合类型泛型
#[derive(Debug)]
struct MixedPoint<T, U> {
    x: T,
    y: U,
}

// 删除有问题的 mixup 方法实现
// impl<T, U> Point<T> {
//     fn mixup<V, W>(self, other: Point<V>) -> Point<T> {
//         Point {
//             x: self.x,
//             y: self.y,
//         }
//     }
// }

// 容器结构体
#[derive(Debug)]
struct Container<T> {
    items: Vec<T>,
}

impl<T> Container<T> {
    fn new() -> Self {
        Container { items: Vec::new() }
    }
    
    fn add(&mut self, item: T) {
        self.items.push(item);
    }
    
    fn get(&self, index: usize) -> Option<&T> {
        self.items.get(index)
    }
    
    fn get_all(&self) -> &Vec<T> {
        &self.items
    }
    
    fn len(&self) -> usize {
        self.items.len()
    }
}

// ========== 泛型枚举 ==========

#[derive(Debug)]
enum MyResult<T, E> {
    Success(T),
    Failure(E),
}

impl<T, E> MyResult<T, E> {
    fn is_success(&self) -> bool {
        matches!(self, MyResult::Success(_))
    }
    
    fn is_failure(&self) -> bool {
        matches!(self, MyResult::Failure(_))
    }
}

// ========== 实际应用泛型结构 ==========

// 泛型栈
#[derive(Debug)]
struct Stack<T> {
    items: Vec<T>,
}

impl<T> Stack<T> {
    fn new() -> Self {
        Stack { items: Vec::new() }
    }
    
    fn push(&mut self, item: T) {
        self.items.push(item);
    }
    
    fn pop(&mut self) -> Option<T> {
        self.items.pop()
    }
    
    fn peek(&self) -> Option<&T> {
        self.items.last()
    }
    
    fn size(&self) -> usize {
        self.items.len()
    }
    
    fn is_empty(&self) -> bool {
        self.items.is_empty()
    }
}

// 泛型缓存
use std::collections::HashMap;

#[derive(Debug)]
struct Cache<K, V> 
where 
    K: std::hash::Hash + Eq
{
    data: HashMap<K, V>,
}

impl<K, V> Cache<K, V> 
where 
    K: std::hash::Hash + Eq
{
    fn new() -> Self {
        Cache {
            data: HashMap::new(),
        }
    }
    
    fn set(&mut self, key: K, value: V) {
        self.data.insert(key, value);
    }
    
    fn get(&self, key: &K) -> Option<&V> {
        self.data.get(key)
    }
    
    fn remove(&mut self, key: &K) -> Option<V> {
        self.data.remove(key)
    }
    
    fn contains_key(&self, key: &K) -> bool {
        self.data.contains_key(key)
    }
}

// 泛型配对
#[derive(Debug)]
struct Pair<T, U> {
    first: T,
    second: U,
}

impl<T, U> Pair<T, U> {
    fn new(first: T, second: U) -> Self {
        Pair { first, second }
    }
    
    fn first(&self) -> &T {
        &self.first
    }
    
    fn second(&self) -> &U {
        &self.second
    }
    
    fn into_tuple(self) -> (T, U) {
        (self.first, self.second)
    }
}

// ========== 泛型辅助函数 ==========

fn find_max<T: PartialOrd>(slice: &[T]) -> Option<&T> {
    if slice.is_empty() {
        return None;
    }
    
    let mut max = &slice[0];
    for item in slice.iter().skip(1) {
        if item > max {
            max = item;
        }
    }
    Some(max)
}

fn find_min<T: PartialOrd>(slice: &[T]) -> Option<&T> {
    if slice.is_empty() {
        return None;
    }
    
    let mut min = &slice[0];
    for item in slice.iter().skip(1) {
        if item < min {
            min = item;
        }
    }
    Some(min)
}

fn map_vec<T, U, F>(vec: &[T], f: F) -> Vec<U>
where
    F: Fn(&T) -> U,
{
    let mut result = Vec::new();
    for item in vec {
        result.push(f(item));
    }
    result
}

/* 
重要概念总结：

泛型基础：
- 使用 <T> 语法定义类型参数
- 可以在函数、结构体、枚举、方法中使用
- 编译时进行单态化（monomorphization）

泛型约束：
- trait bounds：指定泛型类型必须实现的 trait
- where 子句：复杂约束的清晰表达
- 常用约束：Clone, Copy, PartialEq, PartialOrd, Debug

泛型函数：
- fn function_name<T>(param: T) -> T
- 可以有多个类型参数：<T, U, V>
- 约束语法：<T: Trait> 或 where T: Trait

泛型结构体：
- struct Name<T> { field: T }
- 可以为特定类型实现方法
- 混合泛型和具体类型

泛型枚举：
- enum Name<T> { Variant(T) }
- Option<T> 和 Result<T, E> 是标准库例子

方法泛型：
- impl<T> 块为泛型结构体实现方法
- 方法可以引入新的泛型参数
- 约束可以在 impl 或方法级别

性能特性：
- 零成本抽象：编译时展开
- 单态化：为每个具体类型生成代码
- 没有运行时开销

最佳实践：
- 使用有意义的类型参数名
- 合理使用约束，避免过度泛化
- 优先使用标准库的泛型类型
- 考虑生命周期参数的需求

编译运行：
cargo run --bin 12_generics
*/
