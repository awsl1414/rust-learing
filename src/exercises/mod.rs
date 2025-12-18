// exercises/mod.rs - Rust 练习合集
// 包含所有阶段的综合练习

use std::env;

fn main() {
    println!("🦀 Rust 练习合集");
    println!("=================");

    // 解析命令行参数
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        show_menu();
        return;
    }

    match args[1].as_str() {
        "basic" => basic_exercises(),
        "ownership" => ownership_exercises(),
        "generics" => generics_exercises(),
        "async" => async_exercises(),
        "all" => run_all_exercises(),
        _ => {
            eprintln!("❌ 未知练习类型: {}", args[1]);
            show_menu();
        }
    }
}

fn show_menu() {
    println!("📚 可用练习:");
    println!("  basic      - 基础语法练习");
    println!("  ownership  - 所有权系统练习");
    println!("  generics   - 泛型和Trait练习");
    println!("  async      - 异步编程练习");
    println!("  all        - 运行所有练习");
    println!();
    println!("💡 使用方法:");
    println!("  cargo run --bin exercises -- basic");
}

fn basic_exercises() {
    println!("🔹 基础语法练习");
    println!("涵盖变量、数据类型、函数、控制流等基础概念");
    println!();

    // 练习 1: 变量和可变性
    println!("📝 练习 1: 变量和可变性");
    exercise_variables();
    println!();

    // 练习 2: 数据类型和转换
    println!("📝 练习 2: 数据类型和转换");
    exercise_types();
    println!();

    // 练习 3: 函数定义和调用
    println!("📝 练习 3: 函数定义和调用");
    exercise_functions();
    println!();

    // 练习 4: 控制流和模式匹配
    println!("📝 练习 4: 控制流和模式匹配");
    exercise_control_flow();
    println!();

    println!("✅ 基础练习完成");
}

fn ownership_exercises() {
    println!("🔹 所有权系统练习");
    println!("涵盖所有权、借用、生命周期等核心概念");
    println!();

    // 练习 5: 所有权基础
    println!("📝 练习 5: 所有权基础");
    exercise_ownership_basics();
    println!();

    // 练习 6: 引用和借用
    println!("📝 练习 6: 引用和借用");
    exercise_references();
    println!();

    // 练习 7: 切片操作
    println!("📝 练习 7: 切片操作");
    exercise_slices();
    println!();

    println!("✅ 所有权练习完成");
}

fn generics_exercises() {
    println!("🔹 泛型和Trait练习");
    println!("涵盖泛型、Trait、生命周期等高级特性");
    println!();

    // 练习 8: 结构体和方法
    println!("📝 练习 8: 结构体和方法");
    exercise_structs();
    println!();

    // 练习 9: 枚举和模式匹配
    println!("📝 练习 9: 枚举和模式匹配");
    exercise_enums();
    println!();

    // 练习 10: 集合类型
    println!("📝 练习 10: 集合类型");
    exercise_collections();
    println!();

    // 练习 11: 错误处理
    println!("📝 练习 11: 错误处理");
    exercise_error_handling();
    println!();

    // 练习 12: 泛型和Trait
    println!("📝 练习 12: 泛型和Trait");
    exercise_generics();
    println!();

    println!("✅ 泛型练习完成");
}

fn async_exercises() {
    println!("🔹 异步编程练习");
    println!("涵盖async/await、并发等异步编程概念");
    println!();

    // 练习 13: 生命周期
    println!("📝 练习 13: 生命周期");
    exercise_lifetimes();
    println!();

    // 练习 14: 模块系统
    println!("📝 练习 14: 模块系统");
    exercise_modules();
    println!();

    // 练习 15: 异步编程
    println!("📝 练习 15: 异步编程");
    exercise_async();
    println!();

    // 练习 16: 标准库综合应用
    println!("📝 练习 16: 标准库综合应用");
    exercise_std_lib();
    println!();

    println!("✅ 异步练习完成");
}

fn run_all_exercises() {
    println!("🏃 运行所有练习");
    basic_exercises();
    ownership_exercises();
    generics_exercises();
    async_exercises();
    println!("🎉 所有练习完成！");
}

// 练习函数实现

// 基础练习函数
fn exercise_variables() {
    println!("  🎯 任务: 修复变量相关的编译错误");

    // 练习 1.1: 可变性
    let mut x = 5;
    println!("  初始值: {}", x);
    x = 10;
    println!("  修改后: {}", x);

    // 练习 1.2: 变量遮蔽
    let y = 15;
    println!("  外层 y: {}", y);
    {
        let y = y + 5;
        println!("  内层 y: {}", y);
    }
    println!("  回到外层 y: {}", y);

    // 练习 1.3: 常量定义
    const MAX_POINTS: u32 = 100_000;
    println!("  常量 MAX_POINTS: {}", MAX_POINTS);

    println!("  ✅ 变量练习完成！");
}

fn exercise_types() {
    println!("  🎯 任务: 完成类型转换和运算");

    // 练习 2.1: 数值类型
    let integer: i32 = 42;
    let float: f64 = 3.14159;
    let boolean: bool = true;
    let character: char = '🦀';

    println!(
        "  整数: {}, 浮点数: {}, 布尔值: {}, 字符: {}",
        integer, float, boolean, character
    );

    // 练习 2.2: 复合类型
    let tuple: (i32, f64, bool) = (500, 6.4, false);
    let (a, b, c) = tuple;
    println!("  元组解包: a={}, b={}, c={}", a, b, c);

    let array: [i32; 5] = [1, 2, 3, 4, 5];
    println!("  数组第一个元素: {}", array[0]);

    // 练习 2.3: 类型转换
    let decimal = 65.4321_f32;
    let integer_part = decimal as u8;
    let char_from_int = integer_part as char;
    println!(
        "  类型转换链: {} -> {} -> '{}'",
        decimal, integer_part, char_from_int
    );

    println!("  ✅ 类型练习完成！");
}

fn exercise_functions() {
    println!("  🎯 任务: 实现各种函数功能");

    // 练习 3.1: 基本函数
    fn greet(name: &str) -> String {
        format!("你好, {}! 欢迎学习 Rust!", name)
    }

    println!("  {}", greet("学习者"));

    // 练习 3.2: 表达式和语句
    fn add_one(x: i32) -> i32 {
        x + 1 // 表达式，没有分号
    }

    let result = add_one(5);
    println!("  add_one(5) = {}", result);

    // 练习 3.3: 递归函数
    fn factorial(n: u32) -> u32 {
        if n <= 1 { 1 } else { n * factorial(n - 1) }
    }

    println!("  5的阶乘: {}", factorial(5));

    // 练习 3.4: 闭包
    let multiply = |x: i32, y: i32| x * y;
    println!("  闭包乘法: 3 * 4 = {}", multiply(3, 4));

    println!("  ✅ 函数练习完成！");
}

fn exercise_control_flow() {
    println!("  🎯 任务: 使用控制流解决问题");

    // 练习 4.1: 条件语句
    let number = 42;
    match number {
        0 => println!("  数字是 0"),
        1..=9 => println!("  数字是个位数"),
        10..=99 => println!("  数字是两位数"),
        _ => println!("  数字很大"),
    }

    // 练习 4.2: 循环
    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 3 {
            break counter * 2;
        }
    };
    println!("  loop 结果: {}", result);

    // 练习 4.3: while 循环
    let mut number = 3;
    while number != 0 {
        println!("  {}!", number);
        number -= 1;
    }
    println!("  发射! 🚀");

    // 练习 4.4: for 循环
    let a = [10, 20, 30, 40, 50];
    println!("  数组元素:");
    for element in a.iter() {
        println!("    {}", element);
    }

    // 练习 4.5: 模式匹配
    let some_value: Option<i32> = Some(5);
    if let Some(value) = some_value {
        println!("  Option 中的值: {}", value);
    }

    println!("  ✅ 控制流练习完成！");
}

// 所有权练习函数
fn exercise_ownership_basics() {
    println!("  🎯 任务: 理解所有权转移");

    // 练习 5.1: 移动语义
    let s1 = String::from("hello");
    let s2 = s1; // s1 的所有权移动到 s2
    // println!("{}", s1);  // 这行会报错！
    println!("  s2 = {}", s2);

    // 练习 5.2: 克隆
    let s3 = String::from("world");
    let s4 = s3.clone(); // 深拷贝
    println!("  s3 = {}, s4 = {}", s3, s4);

    // 练习 5.3: Copy 类型
    let x = 5;
    let y = x; // 复制而不是移动
    println!("  x = {}, y = {}", x, y);

    // 练习 5.4: 函数参数的所有权
    let s = String::from("函数参数测试");
    takes_ownership(s); // s 的所有权被转移
    // println!("{}", s);  // 这行会报错！

    let x = 5;
    makes_copy(x); // x 被复制
    println!("  函数调用后 x = {}", x);

    fn takes_ownership(some_string: String) {
        println!("  函数接收到: {}", some_string);
    }

    fn makes_copy(some_integer: i32) {
        println!("  函数接收到: {}", some_integer);
    }

    // 练习 5.5: 返回值和所有权
    let s1 = gives_ownership();
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2);
    println!("  s1 = {}, s3 = {}", s1, s3);

    fn gives_ownership() -> String {
        let some_string = String::from("give ownership");
        some_string
    }

    fn takes_and_gives_back(a_string: String) -> String {
        a_string
    }

    println!("  ✅ 所有权基础练习完成！");
}

fn exercise_references() {
    println!("  🎯 任务: 使用引用避免所有权转移");

    // 练习 6.1: 不可变引用
    let s1 = String::from("引用测试");
    let len = calculate_length(&s1); // 传递引用
    println!("  '{}' 的长度是 {}", s1, len); // s1 仍然有效

    fn calculate_length(s: &String) -> usize {
        s.len()
    }

    // 练习 6.2: 可变引用
    let mut s2 = String::from("hello");
    change(&mut s2);
    println!("  修改后: {}", s2);

    fn change(some_string: &mut String) {
        some_string.push_str(", world");
    }

    // 练习 6.3: 多个不可变引用
    let s3 = String::from("多个引用");
    let r1 = &s3;
    let r2 = &s3;
    println!("  r1: {}, r2: {}", r1, r2);

    // 练习 6.4: 引用作用域
    let mut s4 = String::from("作用域测试");
    {
        let r1 = &s4;
        println!("  内层引用: {}", r1);
    } // r1 在这里离开作用域
    let r2 = &mut s4;
    r2.push_str(" (修改后)");
    println!("  可变引用: {}", r2);

    // 练习 6.5: 悬垂引用（编译器会防止）
    let reference_to_nothing = dangle();
    println!("  悬垂引用测试: {}", reference_to_nothing);

    fn dangle() -> String {
        let s = String::from("dangle test");
        s // 返回 String 而不是引用
    }

    println!("  ✅ 引用练习完成！");
}

fn exercise_slices() {
    println!("  🎯 任务: 使用切片操作字符串和数组");

    // 练习 7.1: 字符串切片
    let s = String::from("你好世界，Rust 很棒！");
    // 修复: 使用字符边界来切片中文字符串
    let hello = &s[0..12]; // "你好世界" (3 * 4 = 12 bytes)
    let rust_start = s.find("Rust").unwrap_or(0);
    let rust_end = rust_start + 4;
    let rust = &s[rust_start..rust_end]; // "Rust"
    println!("  原字符串: {}", s);
    println!("  切片1: {}", hello);
    println!("  切片2: {}", rust);

    // 练习 7.2: 数组切片
    let a = [1, 2, 3, 4, 5];
    let slice = &a[1..4];
    println!("  数组: {:?}", a);
    println!("  切片: {:?}", slice);

    // 练习 7.3: 函数参数中的切片
    let mut s = String::from("测试字符串切片");
    let word = first_word(&s);
    println!("  第一个单词: {}", word);

    s.clear(); // 清空字符串
    // println!("  第一个单词: {}", word);  // 这会导致未定义行为！

    fn first_word(s: &String) -> &str {
        let bytes = s.as_bytes();

        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }

        &s[..]
    }

    // 练习 7.4: 字符串字面量作为切片
    let s_literal = "字符串字面量也是切片";
    println!("  字面量: {}", s_literal);

    // 练习 7.5: 安全的字符切片
    let safe_hello = "你好世界";
    println!("  安全的中文切片: {}", safe_hello);

    println!("  ✅ 切片练习完成！");
}

// 结构体和枚举练习函数
fn exercise_structs() {
    println!("  🎯 任务: 创建和使用结构体");

    // 练习 8.1: 定义和实例化结构体
    #[derive(Debug)]
    struct User {
        username: String,
        email: String,
        age: u32,
        active: bool,
    }

    let user1 = User {
        username: String::from("张三"),
        email: String::from("zhangsan@example.com"),
        age: 25,
        active: true,
    };

    println!("  用户信息: {:?}", user1);

    // 练习 8.2: 结构体方法
    impl User {
        fn new(username: String, email: String, age: u32) -> User {
            User {
                username,
                email,
                age,
                active: true,
            }
        }

        fn is_adult(&self) -> bool {
            self.age >= 18
        }

        fn deactivate(&mut self) {
            self.active = false;
        }
    }

    let mut user2 = User::new(String::from("李四"), String::from("lisi@example.com"), 17);

    println!("  {} 是成年人: {}", user2.username, user2.is_adult());
    user2.deactivate();
    println!("  {} 激活状态: {}", user2.username, user2.active);

    // 练习 8.3: 元组结构体
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let black = Color(0, 0, 0);
    let origin = Point(0, 0, 0);

    println!("  黑色 RGB: ({}, {}, {})", black.0, black.1, black.2);
    println!("  原点坐标: ({}, {}, {})", origin.0, origin.1, origin.2);

    println!("  ✅ 结构体练习完成！");
}

fn exercise_enums() {
    println!("  🎯 任务: 使用枚举进行类型安全的编程");

    // 练习 9.1: 基本枚举
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String),
    }

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));

    fn display_ip(ip: &IpAddr) {
        match ip {
            IpAddr::V4(a, b, c, d) => println!("  IPv4: {}.{}.{}.{}", a, b, c, d),
            IpAddr::V6(addr) => println!("  IPv6: {}", addr),
        }
    }

    display_ip(&home);
    display_ip(&loopback);

    // 练习 9.2: 带有关联数据的枚举
    #[derive(Debug)]
    enum Message {
        Quit,
        Move { x: i32, y: i32 },
        Write(String),
        ChangeColor(i32, i32, i32),
    }

    let messages = [
        Message::Move { x: 10, y: 20 },
        Message::Write(String::from("你好")),
        Message::ChangeColor(255, 0, 0),
        Message::Quit,
    ];

    for msg in &messages {
        match msg {
            Message::Move { x, y } => println!("  移动到: ({}, {})", x, y),
            Message::Write(text) => println!("  写入: {}", text),
            Message::ChangeColor(r, g, b) => println!("  颜色: RGB({}, {}, {})", r, g, b),
            Message::Quit => println!("  退出"),
        }
    }

    // 练习 9.3: Option 枚举
    let some_number = Some(5);
    let absent_number: Option<i32> = None;

    fn plus_one(x: Option<i32>) -> Option<i32> {
        match x {
            None => None,
            Some(i) => Some(i + 1),
        }
    }

    let five = Some(5);
    let six = plus_one(five);
    let none = plus_one(None);

    println!("  plus_one(Some(5)) = {:?}", six);
    println!("  plus_one(None) = {:?}", none);

    // 练习 9.4: if let 语法糖
    if let Some(num) = some_number {
        println!("  有数字: {}", num);
    }

    let mut count = 0;
    if let None = absent_number {
        count += 2;
    } else {
        count += 1;
    }
    println!("  计数: {}", count);

    println!("  ✅ 枚举练习完成！");
}

fn exercise_collections() {
    println!("  🎯 任务: 使用集合类型存储和操作数据");

    // 练习 10.1: Vec 动态数组
    let mut v: Vec<i32> = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    println!("  向量: {:?}", v);

    let third = &v[2];
    println!("  第三个元素: {}", third);

    match v.get(2) {
        Some(third) => println!("  第三个元素: {}", third),
        None => println!("  没有第三个元素"),
    }

    // 练习 10.2: 迭代和修改
    for i in &mut v {
        *i *= 2;
    }
    println!("  翻倍后的向量: {:?}", v);

    // 练习 10.3: String 字符串
    let mut s1 = String::new();
    s1.push_str("hello");
    s1.push(' ');
    s1 += "world";

    println!("  字符串: {}", s1);

    let s2 = "  ".to_string();
    let s3 = String::from("初始值");

    // 练习 10.4: 字符串拼接
    let s4 = format!("{}-{}-{}", s1, s2, s3);
    println!("  格式化字符串: {}", s4);

    // 练习 10.5: 字符串切片
    let hello = "Здравствуйте";
    let s = &hello[0..4];
    println!("  俄语切片: {}", s);

    // 练习 10.6: HashMap 哈希映射
    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("蓝队"), 10);
    scores.insert(String::from("黄队"), 50);

    println!("  分数表: {:?}", scores);

    let team_name = String::from("蓝队");
    let score = scores.get(&team_name);

    match score {
        Some(s) => println!("  {} 的分数: {}", team_name, s),
        None => println!("  找不到 {}", team_name),
    }

    // 练习 10.7: 条件插入
    scores.entry(String::from("红队")).or_insert(30);
    scores.entry(String::from("蓝队")).or_insert(40); // 蓝队已存在

    println!("  更新后的分数表: {:?}", scores);

    // 练习 10.8: 根据旧值更新
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("  单词统计: {:?}", map);

    println!("  ✅ 集合类型练习完成！");
}

fn exercise_error_handling() {
    println!("  🎯 任务: 实现健壮的错误处理");

    // 练习 11.1: 可恢复错误 - Result
    fn divide(numerator: f64, denominator: f64) -> Result<f64, String> {
        if denominator == 0.0 {
            Err(String::from("除数不能为零"))
        } else {
            Ok(numerator / denominator)
        }
    }

    let result1 = divide(10.0, 2.0);
    let result2 = divide(10.0, 0.0);

    match result1 {
        Ok(value) => println!("  10 / 2 = {}", value),
        Err(e) => println!("  错误: {}", e),
    }

    match result2 {
        Ok(value) => println!("  10 / 0 = {}", value),
        Err(e) => println!("  错误: {}", e),
    }

    // 练习 11.2: 使用 ? 运算符
    fn read_file_content(filename: &str) -> Result<String, std::io::Error> {
        use std::fs::File;
        use std::io::Read;

        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    let filename = "example.txt";
    match read_file_content(filename) {
        Ok(contents) => println!("  文件内容长度: {} 字符", contents.len()),
        Err(e) => println!("  读取文件 {}: {}", filename, e),
    }

    // 练习 11.3: 使用 unwrap_or 和 unwrap_or_else
    let safe_result = divide(10.0, 0.0).unwrap_or(-1.0);
    println!("  安全除法结果: {}", safe_result);

    let default_result = divide(10.0, 0.0).unwrap_or_else(|e| {
        println!("  使用默认值，因为: {}", e);
        0.0
    });
    println!("  默认结果: {}", default_result);

    // 练习 11.4: 自定义错误类型（简化版）
    #[derive(Debug)]
    enum AppError {
        DivisionByZero,
        InvalidNumber(String),
    }

    fn safe_divide_with_custom_error(numerator: i32, denominator: i32) -> Result<i32, AppError> {
        if denominator == 0 {
            Err(AppError::DivisionByZero)
        } else {
            Ok(numerator / denominator)
        }
    }

    match safe_divide_with_custom_error(10, 0) {
        Ok(result) => println!("  除法结果: {}", result),
        Err(AppError::DivisionByZero) => println!("  自定义错误: 除零错误"),
        Err(AppError::InvalidNumber(msg)) => println!("  自定义错误: {}", msg),
    }

    println!("  ✅ 错误处理练习完成！");
}

fn exercise_generics() {
    println!("  🎯 任务: 使用泛型编写可重用的代码");

    // 练习 12.1: 泛型函数
    fn largest<T: PartialOrd + Copy>(list: &[T]) -> T {
        let mut largest = list[0];

        for &item in list.iter() {
            if item > largest {
                largest = item;
            }
        }

        largest
    }

    let numbers = vec![34, 50, 25, 100, 65];
    let chars = vec!['y', 'm', 'a', 'q'];

    println!("  最大数字: {}", largest(&numbers));
    println!("  最大字符: {}", largest(&chars));

    // 练习 12.2: 泛型结构体
    #[derive(Debug)]
    struct Point<T, U> {
        x: T,
        y: U,
    }

    let integer_and_float = Point { x: 5, y: 10.4 };
    let float_and_char = Point { x: 3.2, y: 'A' };
    let integer_and_integer = Point { x: 5, y: 10 };

    println!("  整数浮点点: {:?}", integer_and_float);
    println!("  浮点字符点: {:?}", float_and_char);
    println!("  整数整点: {:?}", integer_and_integer);

    // 练习 12.3: 为泛型实现方法
    impl<T, U> Point<T, U> {
        fn mixup<V, W>(self, other: Point<V, W>) -> Point<T, W> {
            Point {
                x: self.x,
                y: other.y,
            }
        }
    }

    let p1 = Point { x: 5, y: 10.4 };
    let p2 = Point { x: "Hello", y: 'c' };
    let p3 = p1.mixup(p2);
    println!("  混合点: {:?}", p3);

    // 练习 12.4: Trait 定义
    trait Summary {
        fn summarize(&self) -> String;
    }

    #[derive(Debug, Clone)]
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
    }

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

    let tweet = Tweet {
        username: String::from("horse_ebooks"),
        content: String::from("当然，就像您可能知道的，"),
        reply: false,
        retweet: false,
    };

    println!("  推文摘要: {}", tweet.summarize());

    // 练习 12.5: Trait 作为参数
    fn notify(item: &impl Summary) {
        println!("  突发新闻! {}", item.summarize());
    }

    notify(&tweet);

    // 练习 12.6: Trait bound 语法
    fn notify_bound<T: Summary>(item: &T) {
        println!("  突发新闻(约束版)! {}", item.summarize());
    }

    notify_bound(&tweet);

    // 练习 12.7: 多个 Trait 约束
    use std::fmt::Display;

    fn some_function<T: Display + Clone, U: Clone + Summary>(t: &T, u: &U) -> i32 {
        println!("  T: {}, U: {}", t, u.summarize());
        42
    }

    let article = NewsArticle {
        headline: String::from("重大新闻"),
        location: String::from("北京"),
        author: String::from("记者"),
        content: String::from("内容..."),
    };

    some_function(&5, &article);

    println!("  ✅ 泛型和 Trait 练习完成！");
}

// 高级特性练习函数
fn exercise_lifetimes() {
    println!("  🎯 任务: 理解和使用生命周期");

    // 练习 13.1: 基本生命周期注解
    fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
        if x.len() > y.len() { x } else { y }
    }

    let string1 = String::from("长字符串");
    let string2 = "短";

    let result = longest(&string1, string2);
    println!("  较长的字符串是: {}", result);

    // 练习 13.2: 结构体中的生命周期
    #[derive(Debug)]
    struct ImportantExcerpt<'a> {
        part: &'a str,
    }

    let novel = String::from("叫我以实玛利。几年前...");
    let first_sentence = novel.split('.').next().expect("找不到句号");

    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };

    println!("  重要摘录: {:?}", excerpt);

    // 练习 13.3: 生命周期省略规则
    fn first_word_ann<'a>(s: &'a str) -> &'a str {
        let bytes = s.as_bytes();
        for (i, &item) in bytes.iter().enumerate() {
            if item == b' ' {
                return &s[0..i];
            }
        }
        &s[..]
    }

    // 练习 13.4: 方法定义中的生命周期
    impl<'a> ImportantExcerpt<'a> {
        fn level(&self) -> i32 {
            3
        }

        // 简化的生命周期示例
        fn get_part(&self) -> &str {
            self.part
        }
    }

    let level = excerpt.level();
    println!("  重要级别: {}", level);

    let part = excerpt.get_part();
    println!("  获取的部分: {}", part);

    // 练习 13.5: 静态生命周期
    let s: &'static str = "这是一个静态生命周期的字符串切片";
    println!("  静态字符串: {}", s);

    // 练习 13.6: 泛型、Trait 和生命周期的组合
    use std::fmt::Display;

    struct Ref<'a, T: 'a> {
        value: &'a T,
    }

    fn print_ref<T: Display>(r: Ref<T>) {
        println!("  引用的值: {}", r.value);
    }

    let x = 42;
    let r = Ref { value: &x };
    print_ref(r);

    println!("  ✅ 生命周期练习完成！");
}

fn exercise_modules() {
    println!("  🎯 任务: 使用模块组织代码");

    // 练习 14.1: 基本模块定义
    mod front_of_house {
        pub mod hosting {
            pub fn add_to_waitlist() {
                println!("  添加到等待名单");
            }

            pub fn seat_at_table() {
                println!("  安排座位");
            }
        }

        mod serving {
            fn take_order() {
                println!("  接受订单");
            }

            fn serve_order() {
                println!("  服务订单");
            }

            fn take_payment() {
                println!("  收取付款");
            }
        }
    }

    // 练习 14.2: 使用 use 关键字
    use front_of_house::hosting;

    hosting::add_to_waitlist();

    // 练习 14.3: 嵌套路径
    use front_of_house::hosting::add_to_waitlist;

    add_to_waitlist();

    // 练习 14.4: 全局导入
    use std::collections::*;

    let mut map = HashMap::new();
    map.insert("key", "value");
    println!("  HashMap: {:?}", map);

    // 练习 14.5: 自定义模块结构
    mod math {
        pub mod basic {
            pub fn add(a: i32, b: i32) -> i32 {
                a + b
            }

            pub fn multiply(a: i32, b: i32) -> i32 {
                a * b
            }
        }

        pub mod advanced {
            pub fn factorial(n: u32) -> u64 {
                match n {
                    0 | 1 => 1,
                    n => n as u64 * factorial(n - 1),
                }
            }

            pub fn fibonacci(n: u32) -> u64 {
                match n {
                    0 => 0,
                    1 => 1,
                    n => fibonacci(n - 1) + fibonacci(n - 2),
                }
            }
        }

        pub fn constants() {
            const PI: f64 = 3.14159265359;
            const E: f64 = 2.71828182846;
            println!("  PI: {}, E: {}", PI, E);
        }
    }

    println!("  5 + 3 = {}", math::basic::add(5, 3));
    println!("  5 * 3 = {}", math::basic::multiply(5, 3));
    println!("  5! = {}", math::advanced::factorial(5));
    println!("  fibonacci(7) = {}", math::advanced::fibonacci(7));
    math::constants();

    // 练习 14.6: 重新导出
    mod utils {
        pub use self::string_utils::*;

        mod string_utils {
            pub fn reverse(s: &str) -> String {
                s.chars().rev().collect()
            }

            pub fn is_palindrome(s: &str) -> bool {
                s == reverse(s)
            }
        }
    }

    let text = "racecar";
    println!("  '{}' 反转: '{}'", text, utils::reverse(text));
    println!("  '{}' 是回文: {}", text, utils::is_palindrome(text));

    // 练习 14.7: 模块作为自包含包
    mod my_crate {
        pub mod kinds {
            pub enum PrimaryColor {
                Red,
                Yellow,
                Blue,
            }

            pub enum SecondaryColor {
                Orange,
                Green,
                Purple,
            }
        }

        pub mod utils {
            use super::kinds::*;

            pub fn mix(c1: PrimaryColor, c2: PrimaryColor) -> SecondaryColor {
                match (c1, c2) {
                    (PrimaryColor::Red, PrimaryColor::Yellow)
                    | (PrimaryColor::Yellow, PrimaryColor::Red) => SecondaryColor::Orange,
                    (PrimaryColor::Yellow, PrimaryColor::Blue)
                    | (PrimaryColor::Blue, PrimaryColor::Yellow) => SecondaryColor::Green,
                    (PrimaryColor::Red, PrimaryColor::Blue)
                    | (PrimaryColor::Blue, PrimaryColor::Red) => SecondaryColor::Purple,
                    // 处理相同颜色的情况
                    (PrimaryColor::Red, PrimaryColor::Red)
                    | (PrimaryColor::Yellow, PrimaryColor::Yellow)
                    | (PrimaryColor::Blue, PrimaryColor::Blue) => SecondaryColor::Green, // 默认返回绿色
                }
            }
        }
    }

    use my_crate::kinds::*;

    let _orange = my_crate::utils::mix(PrimaryColor::Red, PrimaryColor::Yellow);
    println!("  混合颜色成功");

    println!("  ✅ 模块系统练习完成！");
}

fn exercise_async() {
    println!("  🎯 任务: 使用异步编程处理并发任务");
    println!("  注意: 异步练习需要在 tokio 运行时中执行");
    println!("  可以使用 'cargo run --bin exercises -- async' 来运行异步练习");

    // 这里只展示基本概念，实际的异步练习需要 tokio 运行时
    println!("  异步编程概念:");
    println!("  1. async fn - 定义异步函数");
    println!("  2. .await - 等待异步操作完成");
    println!("  3. tokio::join! - 并发执行多个异步任务");
    println!("  4. 异步迭代器 - 处理异步数据流");
    println!("  5. 错误处理 - 在异步上下文中处理错误");

    // 异步函数示例（实际运行需要 tokio）
    async fn example_async_function() -> &'static str {
        "异步函数示例"
    }

    println!("  示例异步函数已定义: example_async_function()");
    println!("  ✅ 异步编程概念介绍完成！");

    println!("  💡 提示: 要运行实际的异步代码，请创建专门的异步示例程序");
}

fn exercise_std_lib() {
    println!("  🎯 任务: 使用标准库的实用功能");

    // 练习 16.1: 环境变量和参数
    use std::env;

    println!("  当前程序: {}", env::current_exe().unwrap().display());
    println!("  当前目录: {}", env::current_dir().unwrap().display());

    // 练习 16.2: 文件 I/O
    use std::fs::File;
    use std::io::{BufRead, BufReader, Read, Write};
    use std::path::Path;

    // 创建临时文件
    let file_path = "temp_test_file.txt";
    let mut file = File::create(file_path).unwrap();
    writeln!(file, "Hello, Rust!").unwrap();
    writeln!(file, "这是测试文件").unwrap();
    writeln!(file, "1,2,3,4,5").unwrap();

    // 读取文件内容
    let mut file = File::open(file_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();
    println!("  文件内容:\n{}", contents);

    // 练习 16.3: 按行读取
    let file = File::open(file_path).unwrap();
    let reader = BufReader::new(file);

    println!("  按行读取:");
    for line in reader.lines() {
        let line = line.unwrap();
        println!("    {}", line);
    }

    // 练习 16.4: 路径操作
    use std::path::PathBuf;

    let path = Path::new("/home/user/documents/test.txt");
    println!("  路径存在: {}", path.exists());
    println!("  是文件: {}", path.is_file());
    println!("  文件名: {:?}", path.file_name());
    println!("  扩展名: {:?}", path.extension());

    let mut path_buf = PathBuf::from("/tmp");
    path_buf.push("test");
    path_buf.set_extension("rs");
    println!("  构建的路径: {}", path_buf.display());

    // 练习 16.5: 时间处理
    use std::time::{SystemTime, UNIX_EPOCH};

    let now = SystemTime::now();
    let since_epoch = now.duration_since(UNIX_EPOCH).unwrap();
    println!("  Unix 时间戳: {} 秒", since_epoch.as_secs());

    // 练习 16.6: 数学和格式化
    use std::f64::consts;

    let pi = consts::PI;
    let e = consts::E;
    println!("  PI: {:.4}, E: {:.4}", pi, e);

    let number = 42.5;
    println!("  格式化数字:");
    println!("    {:.2}", number);
    println!("    {:08.2}", number);
    println!("    {:>+8.2}", number);

    // 练习 16.7: 迭代器链
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    let sum: i32 = numbers
        .iter()
        .filter(|&&x| x % 2 == 0)
        .map(|&x| x * x)
        .sum();

    println!("  偶数平方和: {}", sum);

    let text = "Hello, world! Rust is awesome!";
    let words: Vec<&str> = text.split_whitespace().collect();

    println!("  单词列表: {:?}", words);

    let word_lengths: Vec<usize> = words.iter().map(|word| word.len()).collect();

    println!("  单词长度: {:?}", word_lengths);

    // 练习 16.8: 错误传播链
    fn parse_and_double(s: &str) -> Result<i32, std::num::ParseIntError> {
        let num: i32 = s.parse()?;
        Ok(num * 2)
    }

    let numbers_str = vec!["10", "20", "invalid", "40"];

    for num_str in numbers_str {
        match parse_and_double(num_str) {
            Ok(result) => println!("  {} * 2 = {}", num_str, result),
            Err(e) => println!("  解析 '{}' 失败: {}", num_str, e),
        }
    }

    // 清理临时文件
    std::fs::remove_file(file_path).unwrap_or_else(|e| {
        println!("  删除文件失败: {}", e);
    });

    println!("  ✅ 标准库练习完成！");
}
