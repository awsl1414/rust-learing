// 10_errors.rs - 错误处理
// 本文件展示 Rust 中的错误处理机制，包括 panic!、Result、Option 等

fn main() {
    println!("=== Rust 错误处理 ===\n");

    // ========== panic! 宏 ==========
    println!("========== panic! 宏 ==========");

    // 1. 主动触发 panic（演示用，实际运行时取消注释）
    println!("\n1. panic! 宏的使用：");
    println!("注意：以下代码会触发 panic，在实际运行中被注释掉了");
    // panic!("程序崩溃了！");

    // 可恢复的错误演示
    println!("演示数组越界（会触发 panic）：");
    let v = vec![1, 2, 3];
    // let element = v[99]; // 这会触发 panic

    // 安全的访问方式
    match v.get(99) {
        Some(element) => println!("元素: {}", element),
        None => println!("索引越界，但程序继续运行"),
    }

    // ========== Result 类型 ==========
    println!("\n========== Result 类型 ==========");

    // 2. 基本 Result 使用
    println!("\n2. 基本 Result 使用：");

    // 文件操作示例
    match read_username_from_file("user.txt") {
        Ok(username) => println!("用户名: {}", username),
        Err(e) => println!("读取文件失败: {}", e),
    }

    // 字符串解析示例
    let number_str = "42";
    match number_str.parse::<i32>() {
        Ok(number) => println!("解析成功: {}", number),
        Err(e) => println!("解析失败: {}", e),
    }

    let invalid_str = "abc";
    match invalid_str.parse::<i32>() {
        Ok(number) => println!("解析成功: {}", number),
        Err(e) => println!("解析失败: {}", e),
    }

    // 3. unwrap 和 expect
    println!("\n3. unwrap 和 expect：");

    let good_number = "42".parse::<i32>().unwrap();
    println!("unwrap 成功: {}", good_number);

    let another_good = "100".parse::<i32>().expect("这应该是一个有效数字");
    println!("expect 成功: {}", another_good);

    // 以下代码会 panic，在演示中注释掉
    // let bad_number = "abc".parse::<i32>().unwrap();
    // let bad_expect = "xyz".parse::<i32>().expect("解析数字失败");

    // ========== ? 运算符 ==========
    println!("\n========== ? 运算符 ==========");

    // 4. ? 运算符的使用
    println!("\n4. ? 运算符：");

    match read_username_from_file_short("user.txt") {
        Ok(username) => println!("简化版读取用户名: {}", username),
        Err(e) => println!("简化版读取失败: {}", e),
    }

    match calculate_area_from_string("5.5") {
        Ok(area) => println!("圆形面积: {:.2}", area),
        Err(e) => println!("计算面积失败: {}", e),
    }

    match calculate_area_from_string("invalid") {
        Ok(area) => println!("圆形面积: {:.2}", area),
        Err(e) => println!("计算面积失败: {}", e),
    }

    // ========== 自定义错误类型 ==========
    println!("\n========== 自定义错误类型 ==========");

    // 5. 自定义错误
    println!("\n5. 自定义错误类型：");

    // 用户验证示例
    match validate_user("Alice", 25) {
        Ok(user) => println!("用户验证成功: {:?}", user),
        Err(e) => println!("用户验证失败: {}", e),
    }

    match validate_user("", 25) {
        Ok(user) => println!("用户验证成功: {:?}", user),
        Err(e) => println!("用户验证失败: {}", e),
    }

    match validate_user("Bob", 15) {
        Ok(user) => println!("用户验证成功: {:?}", user),
        Err(e) => println!("用户验证失败: {}", e),
    }

    // 6. 数学运算错误处理
    println!("\n6. 数学运算错误处理：");

    let operations = vec![
        (10, 2),
        (15, 3),
        (8, 0), // 除零错误
        (-5, 2),
    ];

    for (a, b) in operations {
        match safe_divide(a, b) {
            Ok(result) => println!("{} ÷ {} = {}", a, b, result),
            Err(e) => println!("{} ÷ {} 错误: {}", a, b, e),
        }
    }

    // ========== Option 类型深入 ==========
    println!("\n========== Option 类型深入 ==========");

    // 7. Option 的高级用法
    println!("\n7. Option 高级用法：");

    let numbers = vec![1, 2, 3, 4, 5];

    // 查找元素
    if let Some(found) = find_number(&numbers, 3) {
        println!("找到数字: {}", found);
    } else {
        println!("未找到数字");
    }

    // Option 链式调用
    let result = Some("42")
        .map(|s| s.parse::<i32>())
        .and_then(|r| r.ok())
        .map(|n| n * 2);

    match result {
        Some(value) => println!("链式调用结果: {}", value),
        None => println!("链式调用失败"),
    }

    // 8. Option 与 Result 转换
    println!("\n8. Option 与 Result 转换：");

    let maybe_number = Some(42);
    let result: Result<i32, &str> = maybe_number.ok_or("没有数字");
    println!("Option to Result: {:?}", result);

    let error_result: Result<i32, &str> = Err("计算错误");
    let maybe_value = error_result.ok();
    println!("Result to Option: {:?}", maybe_value);

    // ========== 错误传播和组合 ==========
    println!("\n========== 错误传播和组合 ==========");

    // 9. 复杂错误处理
    println!("\n9. 复杂错误处理：");

    let user_data = vec![
        ("Alice", "25", "alice@example.com"),
        ("Bob", "30", "bob@example.com"),
        ("Charlie", "abc", "charlie@example.com"), // 年龄无效
        ("David", "35", "invalid-email"),          // 邮箱无效
    ];

    for (name, age_str, email) in user_data {
        match process_user_registration(name, age_str, email) {
            Ok(user) => println!("注册成功: {:?}", user),
            Err(e) => println!("注册失败 {}: {}", name, e),
        }
    }

    // 10. 错误恢复策略
    println!("\n10. 错误恢复策略：");

    let file_paths = vec!["config.txt", "backup_config.txt", "default_config.txt"];

    match load_config_with_fallback(&file_paths) {
        Ok(config) => println!("配置加载成功: {}", config),
        Err(e) => println!("所有配置文件都加载失败: {}", e),
    }

    // ========== 最佳实践示例 ==========
    println!("\n========== 最佳实践示例 ==========");

    // 11. 网络请求模拟
    println!("\n11. 网络请求错误处理：");

    let urls = vec![
        "https://api.example.com/users",
        "https://invalid-url",
        "https://api.example.com/posts",
    ];

    for url in urls {
        match simulate_network_request(url) {
            Ok(response) => println!("请求成功 {}: {}", url, response),
            Err(e) => println!("请求失败 {}: {}", url, e),
        }
    }

    println!("\n=== 错误处理学习完成 ===");
}

// ========== 基本错误处理函数 ==========

use std::fs;
use std::io;

fn read_username_from_file(filename: &str) -> Result<String, io::Error> {
    let username_file_result = fs::read_to_string(filename);

    match username_file_result {
        Ok(content) => Ok(content.trim().to_string()),
        Err(e) => Err(e),
    }
}

fn read_username_from_file_short(filename: &str) -> Result<String, io::Error> {
    let content = fs::read_to_string(filename)?;
    Ok(content.trim().to_string())
}

fn calculate_area_from_string(radius_str: &str) -> Result<f64, Box<dyn std::error::Error>> {
    let radius: f64 = radius_str.parse()?;
    if radius < 0.0 {
        return Err("半径不能为负数".into());
    }
    Ok(std::f64::consts::PI * radius * radius)
}

// ========== 自定义错误类型 ==========

#[derive(Debug)]
enum UserValidationError {
    EmptyName,
    InvalidAge,
    TooYoung,
}

impl std::fmt::Display for UserValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            UserValidationError::EmptyName => write!(f, "用户名不能为空"),
            UserValidationError::InvalidAge => write!(f, "年龄格式无效"),
            UserValidationError::TooYoung => write!(f, "年龄必须大于18岁"),
        }
    }
}

impl std::error::Error for UserValidationError {}

#[derive(Debug)]
struct User {
    name: String,
    age: u32,
}

fn validate_user(name: &str, age: u32) -> Result<User, UserValidationError> {
    if name.is_empty() {
        return Err(UserValidationError::EmptyName);
    }

    if age < 18 {
        return Err(UserValidationError::TooYoung);
    }

    Ok(User {
        name: name.to_string(),
        age,
    })
}

// ========== 数学错误处理 ==========

#[derive(Debug)]
enum MathError {
    DivisionByZero,
    NegativeInput,
}

impl std::fmt::Display for MathError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            MathError::DivisionByZero => write!(f, "除数不能为零"),
            MathError::NegativeInput => write!(f, "输入不能为负数"),
        }
    }
}

impl std::error::Error for MathError {}

fn safe_divide(a: i32, b: i32) -> Result<f64, MathError> {
    if b == 0 {
        return Err(MathError::DivisionByZero);
    }

    Ok(a as f64 / b as f64)
}

// ========== Option 处理函数 ==========

fn find_number(numbers: &[i32], target: i32) -> Option<i32> {
    for &number in numbers {
        if number == target {
            return Some(number);
        }
    }
    None
}

// ========== 复杂错误处理 ==========

#[derive(Debug)]
enum RegistrationError {
    InvalidAge(std::num::ParseIntError),
    InvalidEmail,
    UserTooYoung,
}

impl std::fmt::Display for RegistrationError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            RegistrationError::InvalidAge(e) => write!(f, "年龄格式错误: {}", e),
            RegistrationError::InvalidEmail => write!(f, "邮箱格式无效"),
            RegistrationError::UserTooYoung => write!(f, "用户年龄不够"),
        }
    }
}

impl std::error::Error for RegistrationError {}

impl From<std::num::ParseIntError> for RegistrationError {
    fn from(error: std::num::ParseIntError) -> Self {
        RegistrationError::InvalidAge(error)
    }
}

#[derive(Debug)]
struct RegisteredUser {
    name: String,
    age: u32,
    email: String,
}

fn process_user_registration(
    name: &str,
    age_str: &str,
    email: &str,
) -> Result<RegisteredUser, RegistrationError> {
    let age: u32 = age_str.parse()?; // 使用 From trait 自动转换错误

    if age < 18 {
        return Err(RegistrationError::UserTooYoung);
    }

    if !email.contains('@') || !email.contains('.') {
        return Err(RegistrationError::InvalidEmail);
    }

    Ok(RegisteredUser {
        name: name.to_string(),
        age,
        email: email.to_string(),
    })
}

// ========== 错误恢复策略 ==========

fn load_config_with_fallback(file_paths: &[&str]) -> Result<String, io::Error> {
    for path in file_paths {
        match fs::read_to_string(path) {
            Ok(content) => {
                println!("成功从 {} 加载配置", path);
                return Ok(content);
            }
            Err(e) => {
                println!("从 {} 加载失败: {}", path, e);
                continue;
            }
        }
    }

    Err(io::Error::new(
        io::ErrorKind::NotFound,
        "所有配置文件都不存在",
    ))
}

// ========== 网络请求模拟 ==========

#[derive(Debug)]
enum NetworkError {
    InvalidUrl,
    ConnectionTimeout,
    ServerError(u16),
}

impl std::fmt::Display for NetworkError {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            NetworkError::InvalidUrl => write!(f, "无效的URL"),
            NetworkError::ConnectionTimeout => write!(f, "连接超时"),
            NetworkError::ServerError(code) => write!(f, "服务器错误: {}", code),
        }
    }
}

impl std::error::Error for NetworkError {}

fn simulate_network_request(url: &str) -> Result<String, NetworkError> {
    if !url.starts_with("https://") {
        return Err(NetworkError::InvalidUrl);
    }

    if url.contains("invalid") {
        return Err(NetworkError::ConnectionTimeout);
    }

    if url.contains("posts") {
        return Err(NetworkError::ServerError(500));
    }

    Ok("请求成功".to_string())
}

/*
重要概念总结：

错误分类：
1. 不可恢复错误：panic!，程序终止
2. 可恢复错误：Result<T, E>，可以处理继续执行

panic! 宏：
- 用于不可恢复的错误
- 会展开调用栈并清理数据
- 可以设置为直接终止程序

Result<T, E> 枚举：
- Ok(T)：成功情况，包含值
- Err(E)：错误情况，包含错误信息
- 必须显式处理两种情况

? 运算符：
- 错误传播的语法糖
- 如果是 Ok，解包值继续执行
- 如果是 Err，提前返回错误

unwrap 和 expect：
- unwrap：panic on error
- expect：panic with custom message
- 仅在确定不会错误时使用

Option<T> 枚举：
- Some(T)：有值
- None：无值
- 处理可能不存在的值

自定义错误类型：
- 实现 Display 和 Error trait
- 使用 From trait 进行错误转换
- 提供丰富的错误信息

最佳实践：
- 使用 Result 处理可能失败的操作
- 提供有意义的错误信息
- 合理使用错误传播
- 在适当的层级处理错误
- 考虑错误恢复策略

编译运行：
cargo run --bin errors
*/
