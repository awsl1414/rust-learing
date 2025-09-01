// 15_standard_library.rs - 常用标准库函数与实用宏
// 本文件展示 Rust 标准库中的常用功能和实用宏

// ========== 自定义宏定义 ==========

// 简单打印宏
macro_rules! say_hello {
    () => {
        println!("Hello, World!");
    };
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
    ($name:expr, $age:expr) => {
        println!("Hello, {}! You are {} years old.", $name, $age);
    };
}

// 计算宏
macro_rules! calculate {
    ($expr:expr) => {
        {
            let result = $expr;
            println!("计算 {} = {}", stringify!($expr), result);
            result
        }
    };
}

// 向量创建宏
macro_rules! my_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

fn main() {
    println!("=== Rust 标准库函数与实用宏 ===\n");
    
    // ========== 常用宏 ==========
    println!("========== 常用宏 ==========");
    
    // 1. 格式化宏
    println!("\n1. 格式化宏：");
    
    let name = "Alice";
    let age = 30;
    let score = 98.5;
    
    // println! 宏族
    println!("基本输出: 姓名: {}, 年龄: {}", name, age);
    print!("不换行输出: ");
    println!("继续输出");
    
    // 格式化字符串
    let formatted = format!("用户: {} ({}岁), 分数: {:.1}", name, age, score);
    println!("格式化字符串: {}", formatted);
    
    // eprintln! 错误输出
    eprintln!("这是错误信息: 用户 {} 分数过低", name);
    
    // 2. 调试宏
    println!("\n2. 调试宏：");
    
    let numbers = vec![1, 2, 3, 4, 5];
    let point = (10, 20);
    
    // dbg! 宏
    let doubled = numbers.iter().map(|x| x * 2).collect::<Vec<_>>();
    dbg!(&numbers);
    dbg!(&doubled);
    dbg!(point);
    
    // 3. 条件编译宏
    println!("\n3. 条件编译：");
    
    debug_print("这是调试信息");
    
    #[cfg(debug_assertions)]
    println!("这是调试模式");
    
    #[cfg(not(debug_assertions))]
    println!("这是发布模式");
    
    // ========== 集合相关函数 ==========
    println!("\n========== 集合相关函数 ==========");
    
    // 4. Vec 常用方法
    println!("\n4. Vec 常用方法：");
    
    let mut vec = vec![1, 2, 3, 4, 5];
    println!("原始向量: {:?}", vec);
    
    // 基本操作
    vec.push(6);
    println!("添加元素后: {:?}", vec);
    
    if let Some(last) = vec.pop() {
        println!("弹出的元素: {}", last);
    }
    
    vec.insert(0, 0);
    println!("在开头插入0: {:?}", vec);
    
    let removed = vec.remove(1);
    println!("删除索引1的元素 {}: {:?}", removed, vec);
    
    // 5. 迭代器方法
    println!("\n5. 迭代器方法：");
    
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    
    // map, filter, collect
    let evens: Vec<i32> = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .collect();
    println!("偶数的平方: {:?}", evens);
    
    // fold 和 reduce
    let sum = numbers.iter().fold(0, |acc, &x| acc + x);
    println!("累加和: {}", sum);
    
    let product = numbers.iter().copied().reduce(|acc, x| acc * x);
    println!("累乘积: {:?}", product);
    
    // find 和 position
    let found = numbers.iter().find(|&&x| x > 5);
    println!("第一个大于5的数: {:?}", found);
    
    let pos = numbers.iter().position(|&x| x == 7);
    println!("数字7的位置: {:?}", pos);
    
    // ========== 字符串处理 ==========
    println!("\n========== 字符串处理 ==========");
    
    // 6. String 和 &str 方法
    println!("\n6. 字符串方法：");
    
    let text = "  Hello, Rust World!  ";
    println!("原文本: '{}'", text);
    
    // 基本操作
    println!("去空白: '{}'", text.trim());
    println!("转大写: '{}'", text.to_uppercase());
    println!("转小写: '{}'", text.to_lowercase());
    
    // 查找和替换
    println!("包含'Rust': {}", text.contains("Rust"));
    println!("以'Hello'开始: {}", text.trim().starts_with("Hello"));
    println!("以'!'结束: {}", text.trim().ends_with("!"));
    
    let replaced = text.replace("Rust", "Programming");
    println!("替换后: '{}'", replaced);
    
    // 分割
    let words: Vec<&str> = text.split_whitespace().collect();
    println!("单词列表: {:?}", words);
    
    let parts: Vec<&str> = text.split(',').collect();
    println!("按逗号分割: {:?}", parts);
    
    // ========== 数学函数 ==========
    println!("\n========== 数学函数 ==========");
    
    // 7. 数学运算
    println!("\n7. 数学运算：");
    
    let x = 16.0f64;
    let y = -3.5f64;
    
    println!("绝对值: |{}| = {}", y, y.abs());
    println!("平方根: √{} = {}", x, x.sqrt());
    println!("立方根: ∛{} = {}", x, x.cbrt());
    println!("2的幂: 2^{} = {}", x, x.exp2());
    println!("向上取整: ⌈{}⌉ = {}", y, y.ceil());
    println!("向下取整: ⌊{}⌋ = {}", y, y.floor());
    println!("四舍五入: round({}) = {}", y, y.round());
    
    // 三角函数
    let angle = std::f64::consts::PI / 4.0; // 45度
    println!("sin(π/4) = {:.4}", angle.sin());
    println!("cos(π/4) = {:.4}", angle.cos());
    println!("tan(π/4) = {:.4}", angle.tan());
    
    // ========== Option 和 Result 处理 ==========
    println!("\n========== Option 和 Result 处理 ==========");
    
    // 8. Option 方法
    println!("\n8. Option 方法：");
    
    let some_value = Some(5);
    let none_value: Option<i32> = None;
    
    // map 和 and_then
    let doubled = some_value.map(|x| x * 2);
    println!("Some(5) * 2 = {:?}", doubled);
    
    let chained = some_value
        .and_then(|x| Some(x * 2))
        .and_then(|x| Some(x + 1));
    println!("链式调用结果: {:?}", chained);
    
    // unwrap_or 和 unwrap_or_else
    println!("unwrap_or: {}", none_value.unwrap_or(42));
    println!("unwrap_or_else: {}", none_value.unwrap_or_else(|| 100));
    
    // 9. Result 方法
    println!("\n9. Result 方法：");
    
    let good_result: Result<i32, &str> = Ok(42);
    let bad_result: Result<i32, &str> = Err("出错了");
    
    // map 和 map_err
    let doubled_result = good_result.map(|x| x * 2);
    println!("Ok(42) * 2 = {:?}", doubled_result);
    
    let mapped_error = bad_result.map_err(|e| format!("错误: {}", e));
    println!("映射错误: {:?}", mapped_error);
    
    // unwrap_or 和 unwrap_or_else
    println!("Result unwrap_or: {}", bad_result.unwrap_or(0));
    
    // ========== 时间和日期 ==========
    println!("\n========== 时间和日期 ==========");
    
    // 10. 时间处理
    println!("\n10. 时间处理：");
    
    use std::time::{Duration, Instant, SystemTime, UNIX_EPOCH};
    
    let start = Instant::now();
    
    // 模拟一些工作
    std::thread::sleep(Duration::from_millis(10));
    
    let elapsed = start.elapsed();
    println!("操作耗时: {:?}", elapsed);
    
    // 系统时间
    let now = SystemTime::now();
    let since_epoch = now.duration_since(UNIX_EPOCH).expect("时间倒退了！");
    println!("Unix 时间戳: {} 秒", since_epoch.as_secs());
    
    // ========== 文件和 I/O ==========
    println!("\n========== 文件和 I/O ==========");
    
    // 11. 文件操作
    println!("\n11. 文件操作：");
    
    use std::fs;
    
    // 写入文件
    let content = "Hello, Rust!\n这是测试内容。\n";
    match fs::write("test.txt", content) {
        Ok(()) => println!("文件写入成功"),
        Err(e) => println!("文件写入失败: {}", e),
    }
    
    // 读取文件
    match fs::read_to_string("test.txt") {
        Ok(content) => println!("文件内容:\n{}", content),
        Err(e) => println!("文件读取失败: {}", e),
    }
    
    // 删除文件
    match fs::remove_file("test.txt") {
        Ok(()) => println!("文件删除成功"),
        Err(e) => println!("文件删除失败: {}", e),
    }
    
    // ========== 环境变量和命令行 ==========
    println!("\n========== 环境变量和命令行 ==========");
    
    // 12. 环境变量
    println!("\n12. 环境变量：");
    
    use std::env;
    
    // 获取环境变量
    match env::var("HOME") {
        Ok(home) => println!("HOME 目录: {}", home),
        Err(_) => println!("HOME 环境变量未设置"),
    }
    
    // 设置环境变量
    unsafe {
        env::set_var("MY_VAR", "Hello from Rust");
    }
    println!("MY_VAR: {}", env::var("MY_VAR").unwrap_or_else(|_| "未设置".to_string()));
    
    // 获取所有环境变量
    println!("环境变量数量: {}", env::vars().count());
    
    // 命令行参数
    let args: Vec<String> = env::args().collect();
    println!("命令行参数: {:?}", args);
    
    // ========== 随机数和哈希 ==========
    println!("\n========== 随机数和哈希 ==========");
    
    // 13. 随机数（需要 rand crate）
    println!("\n13. 随机数生成：");
    
    use rand::{rng, Rng, prelude::IndexedRandom};
    
    let mut rng = rng();
    
    println!("随机整数: {}", rng.random::<i32>());
    println!("0-100随机数: {}", rng.random_range(0..=100));
    println!("随机布尔值: {}", rng.random::<bool>());
    println!("随机浮点数: {:.3}", rng.random::<f64>());
    
    // 随机选择
    let choices = vec!["apple", "banana", "cherry"];
    if let Some(choice) = choices.choose(&mut rng) {
        println!("随机选择: {}", choice);
    }
    
    // ========== 实用工具函数 ==========
    println!("\n========== 实用工具函数 ==========");
    
    // 14. 实用工具
    println!("\n14. 实用工具：");
    
    // 交换变量
    let mut a = 10;
    let mut b = 20;
    println!("交换前: a = {}, b = {}", a, b);
    std::mem::swap(&mut a, &mut b);
    println!("交换后: a = {}, b = {}", a, b);
    
    // 替换值
    let mut x = vec![1, 2, 3];
    let old_x = std::mem::replace(&mut x, vec![4, 5, 6]);
    println!("新值: {:?}, 旧值: {:?}", x, old_x);
    
    // 类型大小
    println!("i32 大小: {} 字节", std::mem::size_of::<i32>());
    println!("String 大小: {} 字节", std::mem::size_of::<String>());
    println!("Vec<i32> 大小: {} 字节", std::mem::size_of::<Vec<i32>>());
    
    // ========== 自定义宏示例 ==========
    println!("\n========== 自定义宏示例 ==========");
    
    // 15. 自定义宏
    println!("\n15. 自定义宏：");
    
    // 使用自定义宏
    say_hello!();
    say_hello!("Alice");
    say_hello!("Bob", 25);
    
    // 计算宏
    let result = calculate!(10 + 5 * 2);
    println!("计算结果: {}", result);
    
    // 向量创建宏
    let my_vec = my_vec![1, 2, 3, 4, 5];
    println!("自定义向量宏: {:?}", my_vec);
    
    println!("\n=== 标准库函数与实用宏学习完成 ===");
}

// ========== 自定义宏定义 ==========

// 简单打印宏
macro_rules! say_hello {
    () => {
        println!("Hello, World!");
    };
    ($name:expr) => {
        println!("Hello, {}!", $name);
    };
    ($name:expr, $age:expr) => {
        println!("Hello, {}! You are {} years old.", $name, $age);
    };
}

// 计算宏
macro_rules! calculate {
    ($expr:expr) => {
        {
            let result = $expr;
            println!("计算 {} = {}", stringify!($expr), result);
            result
        }
    };
}

// 向量创建宏
macro_rules! my_vec {
    ( $( $x:expr ),* ) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    };
}

// ========== 条件编译函数 ==========

#[cfg(debug_assertions)]
fn debug_print(msg: &str) {
    println!("[DEBUG] {}", msg);
}

#[cfg(not(debug_assertions))]
fn debug_print(_msg: &str) {
    // 在发布模式下不做任何事
}

// ========== 实用工具函数 ==========

// 重试函数
fn retry_operation<F, T, E>(mut operation: F, max_attempts: usize) -> Result<T, E>
where
    F: FnMut() -> Result<T, E>,
{
    let mut attempts = 0;
    loop {
        attempts += 1;
        match operation() {
            Ok(result) => return Ok(result),
            Err(e) if attempts >= max_attempts => return Err(e),
            Err(_) => continue,
        }
    }
}

// 计时器函数
fn time_it<F, T>(operation: F) -> (T, std::time::Duration)
where
    F: FnOnce() -> T,
{
    let start = std::time::Instant::now();
    let result = operation();
    let elapsed = start.elapsed();
    (result, elapsed)
}

// 安全除法
fn safe_divide(a: f64, b: f64) -> Option<f64> {
    if b != 0.0 {
        Some(a / b)
    } else {
        None
    }
}

// 范围检查
fn clamp(value: f64, min: f64, max: f64) -> f64 {
    if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}

/* 
重要概念总结：

常用宏：
- println!, print!, format!：格式化输出
- dbg!：调试输出，显示表达式和值
- eprintln!：错误输出到标准错误流
- vec!：创建向量的便捷宏

集合操作：
- Vec：动态数组，push, pop, insert, remove
- 迭代器：map, filter, fold, reduce, find
- 链式调用：强大的函数式编程支持

字符串处理：
- 基本操作：trim, to_uppercase, to_lowercase
- 查找：contains, starts_with, ends_with
- 分割和替换：split, replace

数学函数：
- 基本运算：abs, sqrt, pow, exp
- 舍入：ceil, floor, round
- 三角函数：sin, cos, tan

Option 和 Result：
- map, and_then：链式转换
- unwrap_or, unwrap_or_else：提供默认值
- 错误处理的函数式方法

时间处理：
- Instant：性能测量
- SystemTime：系统时间
- Duration：时间间隔

文件 I/O：
- fs::read_to_string, fs::write：简单文件操作
- 错误处理：使用 Result 类型

环境变量：
- env::var：获取环境变量
- env::args：命令行参数
- 跨平台环境访问

实用工具：
- mem::swap：交换变量
- mem::replace：替换值
- mem::size_of：类型大小

自定义宏：
- macro_rules!：声明式宏
- 模式匹配：支持多种输入格式
- 代码生成：编译时展开

最佳实践：
- 善用标准库提供的功能
- 合理使用迭代器链式调用
- 优先使用 Result 和 Option
- 适当使用宏简化重复代码

编译运行：
cargo run --bin 15_standard_library
*/
