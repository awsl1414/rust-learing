// 13_modules.rs - 模块系统和包管理
// 本文件展示 Rust 中的模块系统、可见性、use 关键字和包管理

fn main() {
    println!("=== Rust 模块系统和包管理 ===\n");

    // ========== 模块基础 ==========
    println!("========== 模块基础 ==========");

    // 1. 使用内联模块
    println!("\n1. 内联模块的使用：");

    // 调用模块中的函数
    front_of_house::hosting::add_to_waitlist();
    front_of_house::hosting::seat_at_table();
    front_of_house::serving::take_order();
    front_of_house::serving::serve_order();
    front_of_house::serving::take_payment();

    // 2. 绝对路径和相对路径
    println!("\n2. 绝对路径和相对路径：");

    // 绝对路径调用
    crate::front_of_house::hosting::add_to_waitlist();

    // 相对路径调用（从当前模块开始）
    front_of_house::hosting::add_to_waitlist();

    // ========== use 关键字 ==========
    println!("\n========== use 关键字 ==========");

    // 3. 使用 use 简化路径
    println!("\n3. use 关键字简化调用：");

    // 直接调用，无需完整路径
    hosting::add_to_waitlist();

    // 使用别名调用同样的函数
    network::add_to_waitlist();
    net::take_order();

    // 4. 重新导出
    println!("\n4. 重新导出：");

    // 通过重新导出的路径调用
    restaurant::add_to_waitlist();

    // ========== 标准库模块 ==========
    println!("\n========== 标准库模块 ==========");

    // 5. 使用标准库集合
    println!("\n5. 标准库集合的使用：");

    let mut map = HashMap::new();
    map.insert("key1", "value1");
    map.insert("key2", "value2");

    println!("HashMap: {:?}", map);

    let mut set = HashSet::new();
    set.insert("item1");
    set.insert("item2");
    set.insert("item1"); // 重复项不会被添加

    println!("HashSet: {:?}", set);

    // 6. 使用嵌套路径
    println!("\n6. 嵌套路径导入：");

    let mut rng = rng();
    let random_number: u32 = rng.random_range(1..=100);
    println!("随机数: {}", random_number);

    // ========== 自定义模块示例 ==========
    println!("\n========== 自定义模块示例 ==========");

    // 7. 数学模块的使用
    println!("\n7. 数学模块：");

    println!("5 + 3 = {}", math::basic::add(5, 3));
    println!("10 - 4 = {}", math::basic::subtract(10, 4));
    println!("6 × 7 = {}", math::basic::multiply(6, 7));
    println!("15 ÷ 3 = {}", math::basic::divide(15, 3));

    println!("圆周率: {}", math::geometry::PI);
    println!("半径5的圆面积: {:.2}", math::geometry::circle_area(5.0));
    println!(
        "长3宽4的矩形面积: {}",
        math::geometry::rectangle_area(3.0, 4.0)
    );

    // 8. 实用工具模块
    println!("\n8. 实用工具模块：");

    let text = "  Hello, Rust World!  ";
    println!("原文本: '{}'", text);
    println!("处理后: '{}'", utils::string_helpers::clean_string(text));

    let numbers = vec![3, 1, 4, 1, 5, 9, 2, 6];
    println!("原数组: {:?}", numbers);

    let stats = utils::math_helpers::calculate_stats(&numbers);
    println!("统计信息: {:#?}", stats);

    // 9. 错误处理模块
    println!("\n9. 错误处理模块：");

    match validation::validate_email("user@example.com") {
        Ok(email) => println!("有效邮箱: {}", email),
        Err(e) => println!("邮箱验证失败: {}", e),
    }

    match validation::validate_email("invalid-email") {
        Ok(email) => println!("有效邮箱: {}", email),
        Err(e) => println!("邮箱验证失败: {}", e),
    }

    match validation::validate_password("StrongPass123!") {
        Ok(_) => println!("密码强度足够"),
        Err(e) => println!("密码验证失败: {}", e),
    }

    match validation::validate_password("weak") {
        Ok(_) => println!("密码强度足够"),
        Err(e) => println!("密码验证失败: {}", e),
    }

    // ========== 模块组织最佳实践 ==========
    println!("\n========== 模块组织最佳实践 ==========");

    // 10. 配置模块
    println!("\n10. 配置管理：");

    let config = config::AppConfig::new();
    config.display();

    let db_config = config::DatabaseConfig::default();
    println!("数据库配置: {:?}", db_config);

    println!("\n=== 模块系统和包管理学习完成 ===");
}

// ========== 内联模块定义 ==========

// 餐厅模块
mod front_of_house {
    pub mod hosting {
        pub fn add_to_waitlist() {
            println!("添加到等待列表");
        }

        pub fn seat_at_table() {
            println!("安排就座");
        }
    }

    pub mod serving {
        pub fn take_order() {
            println!("接受订单");
        }

        pub fn serve_order() {
            println!("上菜");
        }

        pub fn take_payment() {
            println!("收款");
        }
    }
}

// ========== use 声明 ==========

// 引入 hosting 模块
use front_of_house::hosting;

// 使用别名
use front_of_house::hosting as network;
use front_of_house::serving as net;

// 重新导出
pub use front_of_house::hosting as restaurant;

// 标准库导入
use std::collections::{HashMap, HashSet};

// 嵌套路径导入
use rand::{Rng, rng};

// ========== 数学模块 ==========

mod math {
    pub mod basic {
        pub fn add(a: i32, b: i32) -> i32 {
            a + b
        }

        pub fn subtract(a: i32, b: i32) -> i32 {
            a - b
        }

        pub fn multiply(a: i32, b: i32) -> i32 {
            a * b
        }

        pub fn divide(a: i32, b: i32) -> i32 {
            if b != 0 {
                a / b
            } else {
                panic!("除数不能为0");
            }
        }
    }

    pub mod geometry {
        pub const PI: f64 = 3.14159265359;

        pub fn circle_area(radius: f64) -> f64 {
            PI * radius * radius
        }

        pub fn rectangle_area(width: f64, height: f64) -> f64 {
            width * height
        }

        pub fn triangle_area(base: f64, height: f64) -> f64 {
            0.5 * base * height
        }
    }
}

// ========== 实用工具模块 ==========

mod utils {
    pub mod string_helpers {
        pub fn clean_string(s: &str) -> String {
            s.trim().to_lowercase()
        }

        pub fn word_count(s: &str) -> usize {
            s.split_whitespace().count()
        }

        pub fn reverse_string(s: &str) -> String {
            s.chars().rev().collect()
        }
    }

    pub mod math_helpers {
        #[derive(Debug)]
        pub struct Statistics {
            pub sum: i32,
            pub average: f64,
            pub min: i32,
            pub max: i32,
            pub count: usize,
        }

        pub fn calculate_stats(numbers: &[i32]) -> Statistics {
            if numbers.is_empty() {
                return Statistics {
                    sum: 0,
                    average: 0.0,
                    min: 0,
                    max: 0,
                    count: 0,
                };
            }

            let sum: i32 = numbers.iter().sum();
            let count = numbers.len();
            let average = sum as f64 / count as f64;
            let min = *numbers.iter().min().unwrap();
            let max = *numbers.iter().max().unwrap();

            Statistics {
                sum,
                average,
                min,
                max,
                count,
            }
        }
    }
}

// ========== 验证模块 ==========

mod validation {
    #[derive(Debug)]
    pub enum ValidationError {
        InvalidEmail,
        WeakPassword,
    }

    impl std::fmt::Display for ValidationError {
        fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
            match self {
                ValidationError::InvalidEmail => write!(f, "邮箱格式无效"),
                ValidationError::WeakPassword => write!(f, "密码强度不够"),
            }
        }
    }

    impl std::error::Error for ValidationError {}

    pub fn validate_email(email: &str) -> Result<&str, ValidationError> {
        if email.contains('@') && email.contains('.') {
            Ok(email)
        } else {
            Err(ValidationError::InvalidEmail)
        }
    }

    pub fn validate_password(password: &str) -> Result<(), ValidationError> {
        if password.len() >= 8
            && password.chars().any(|c| c.is_uppercase())
            && password.chars().any(|c| c.is_lowercase())
            && password.chars().any(|c| c.is_numeric())
        {
            Ok(())
        } else {
            Err(ValidationError::WeakPassword)
        }
    }
}

// ========== 配置模块 ==========

mod config {
    #[derive(Debug)]
    pub struct AppConfig {
        pub app_name: String,
        pub version: String,
        pub debug: bool,
    }

    impl AppConfig {
        pub fn new() -> Self {
            AppConfig {
                app_name: "Rust学习应用".to_string(),
                version: "1.0.0".to_string(),
                debug: true,
            }
        }

        pub fn display(&self) {
            println!("应用配置:");
            println!("  名称: {}", self.app_name);
            println!("  版本: {}", self.version);
            println!("  调试模式: {}", self.debug);
        }
    }

    #[derive(Debug)]
    pub struct DatabaseConfig {
        pub host: String,
        pub port: u16,
        pub database: String,
        pub username: String,
    }

    impl Default for DatabaseConfig {
        fn default() -> Self {
            DatabaseConfig {
                host: "localhost".to_string(),
                port: 5432,
                database: "myapp".to_string(),
                username: "user".to_string(),
            }
        }
    }
}

/*
重要概念总结：

模块系统：
- mod 关键字定义模块
- pub 关键字控制可见性
- 模块可以嵌套和包含子模块
- 私有边界：默认情况下一切都是私有的

路径系统：
- 绝对路径：从 crate 根开始
- 相对路径：从当前模块开始
- super：引用父模块
- self：引用当前模块

use 关键字：
- 将路径引入作用域
- 支持别名：use path as alias
- 支持嵌套路径：use std::{cmp::Ordering, io}
- pub use：重新导出

模块文件组织：
- 同文件内：mod module_name { ... }
- 单独文件：mod module_name; (需要对应的.rs文件)
- 文件夹模块：mod.rs 或 module_name.rs

包和 Crate：
- Package：包含 Cargo.toml 的项目
- Crate：编译单元，可以是库或二进制
- 库 crate：src/lib.rs
- 二进制 crate：src/main.rs

可见性规则：
- 私有：模块外部不能访问
- 公有：pub 标记的项可以被访问
- 结构体字段需要单独标记 pub
- 枚举变体自动公有

最佳实践：
- 合理组织模块层次结构
- 使用 pub use 提供清晰的公共 API
- 将相关功能组织在同一模块中
- 避免过深的模块嵌套

编译运行：
cargo run --bin modules
*/
