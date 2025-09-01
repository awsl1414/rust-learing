// 08_enums_and_pattern_matching.rs - 枚举与模式匹配
// 本文件展示 Rust 中枚举的定义、使用和强大的模式匹配功能

fn main() {
    println!("=== Rust 枚举与模式匹配 ===\n");
    
    // ========== 基本枚举 ==========
    println!("========== 基本枚举 ==========");
    
    // 1. 简单枚举
    println!("\n1. 简单枚举：");
    let ipv4 = IpAddrKind::V4;
    let ipv6 = IpAddrKind::V6;
    
    println!("IPv4: {:?}", ipv4);
    println!("IPv6: {:?}", ipv6);
    
    route(ipv4);
    route(ipv6);
    
    // 2. 带数据的枚举
    println!("\n2. 带数据的枚举：");
    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
    
    println!("家庭IP: {:?}", home);
    println!("回环IP: {:?}", loopback);
    
    // 3. 复杂枚举
    println!("\n3. 复杂枚举：");
    let msg1 = Message::Quit;
    let msg2 = Message::Move { x: 10, y: 20 };
    let msg3 = Message::Write(String::from("Hello, Rust!"));
    let msg4 = Message::ChangeColor(255, 0, 0);
    
    process_message(msg1);
    process_message(msg2);
    process_message(msg3);
    process_message(msg4);
    
    // ========== Option 枚举 ==========
    println!("\n========== Option 枚举 ==========");
    
    // 4. Option 的使用
    println!("\n4. Option 枚举：");
    let some_number = Some(5);
    let some_string = Some("一个字符串");
    let absent_number: Option<i32> = None;
    
    println!("有值的数字: {:?}", some_number);
    println!("有值的字符串: {:?}", some_string);
    println!("无值: {:?}", absent_number);
    
    // Option 与 match
    let x: i8 = 5;
    let y: Option<i8> = Some(5);
    
    println!("x + 1 = {}", plus_one_regular(x));
    
    match plus_one_option(y) {
        Some(value) => println!("y + 1 = {}", value),
        None => println!("无法计算"),
    }
    
    match plus_one_option(None) {
        Some(value) => println!("None + 1 = {}", value),
        None => println!("None + 1 = None"),
    }
    
    // ========== match 表达式 ==========
    println!("\n========== match 表达式 ==========");
    
    // 5. 基本 match
    println!("\n5. 基本 match：");
    let coin = Coin::Quarter(UsState::Alaska);
    let value = value_in_cents(coin);
    println!("硬币价值: {} 美分", value);
    
    // 不同类型的硬币
    let coins = vec![
        Coin::Penny,
        Coin::Nickel,
        Coin::Dime,
        Coin::Quarter(UsState::Alaska),
        Coin::Quarter(UsState::Alabama),
    ];
    
    for coin in coins {
        println!("硬币价值: {} 美分", value_in_cents(coin));
    }
    
    // 6. match 的穷尽性
    println!("\n6. match 的穷尽性：");
    let numbers = vec![Some(1), Some(3), Some(5), None, Some(7)];
    
    for num in numbers {
        match num {
            Some(i) if i % 2 == 0 => println!("{} 是偶数", i),
            Some(i) => println!("{} 是奇数", i),
            None => println!("没有值"),
        }
    }
    
    // ========== if let 语法糖 ==========
    println!("\n========== if let 语法糖 ==========");
    
    // 7. if let 简化代码
    println!("\n7. if let 语法：");
    let config_max = Some(3u8);
    
    // 使用 match
    match config_max {
        Some(max) => println!("最大值配置为 {}", max),
        _ => (),
    }
    
    // 使用 if let（更简洁）
    if let Some(max) = config_max {
        println!("if let: 最大值配置为 {}", max);
    }
    
    // if let 与 else
    let favorite_color: Option<&str> = None;
    let is_tuesday = false;
    let age: Result<u8, _> = "34".parse();
    
    if let Some(color) = favorite_color {
        println!("使用你最喜欢的颜色 {} 作为背景", color);
    } else if is_tuesday {
        println!("星期二是绿色的日子！");
    } else if let Ok(age) = age {
        if age > 30 {
            println!("使用紫色作为背景色");
        } else {
            println!("使用橙色作为背景色");
        }
    } else {
        println!("使用蓝色作为背景色");
    }
    
    // ========== while let 循环 ==========
    println!("\n========== while let 循环 ==========");
    
    // 8. while let 循环
    println!("\n8. while let 循环：");
    let mut stack = Vec::new();
    
    stack.push(1);
    stack.push(2);
    stack.push(3);
    
    println!("栈内容: {:?}", stack);
    print!("弹出顺序: ");
    while let Some(top) = stack.pop() {
        print!("{} ", top);
    }
    println!();
    
    // ========== 复杂模式匹配 ==========
    println!("\n========== 复杂模式匹配 ==========");
    
    // 9. 解构枚举
    println!("\n9. 解构枚举：");
    let data = vec![
        DataType::Integer(42),
        DataType::Float(3.14),
        DataType::Text(String::from("Hello")),
        DataType::Boolean(true),
        DataType::Array(vec![1, 2, 3, 4, 5]),
    ];
    
    for item in data {
        analyze_data(item);
    }
    
    // 10. 嵌套枚举匹配
    println!("\n10. 嵌套枚举匹配：");
    let events = vec![
        Event::UserAction(UserAction::Click { x: 100, y: 200 }),
        Event::UserAction(UserAction::KeyPress('a')),
        Event::SystemEvent(SystemEvent::Shutdown),
        Event::SystemEvent(SystemEvent::NetworkError(String::from("连接超时"))),
    ];
    
    for event in events {
        handle_event(event);
    }
    
    // ========== 高级模式匹配特性 ==========
    println!("\n========== 高级模式匹配特性 ==========");
    
    // 11. 范围匹配
    println!("\n11. 范围匹配：");
    let numbers = vec![1, 5, 15, 25, 50, 100];
    
    for num in numbers {
        match num {
            1..=10 => println!("{} 是小数字", num),
            11..=50 => println!("{} 是中等数字", num),
            51..=100 => println!("{} 是大数字", num),
            _ => println!("{} 是超大数字", num),
        }
    }
    
    // 12. 字符匹配
    println!("\n12. 字符匹配：");
    let letters = vec!['a', 'B', '5', '中', '!'];
    
    for letter in letters {
        match letter {
            'a'..='z' => println!("'{}' 是小写字母", letter),
            'A'..='Z' => println!("'{}' 是大写字母", letter),
            '0'..='9' => println!("'{}' 是数字", letter),
            _ => println!("'{}' 是其他字符", letter),
        }
    }
    
    // 13. 绑定值
    println!("\n13. 绑定值：");
    let msg = MyMessage::Hello { id: 5 };
    
    match msg {
        MyMessage::Hello { id: id_variable @ 3..=7 } => {
            println!("找到一个在范围内的id: {}", id_variable);
        }
        MyMessage::Hello { id: 10..=12 } => {
            println!("找到一个在另一个范围内的id");
        }
        MyMessage::Hello { id } => {
            println!("找到一些其他的id: {}", id);
        }
    }
    
    println!("\n=== 枚举与模式匹配学习完成 ===");
}

// ========== 枚举定义 ==========

// 简单枚举
#[derive(Debug)]
enum IpAddrKind {
    V4,
    V6,
}

// 带数据的枚举
#[derive(Debug)]
enum IpAddr {
    V4(u8, u8, u8, u8),
    V6(String),
}

// 复杂枚举
#[derive(Debug)]
enum Message {
    Quit,
    Move { x: i32, y: i32 },
    Write(String),
    ChangeColor(i32, i32, i32),
}

// 为枚举实现方法
impl Message {
    fn call(&self) {
        println!("调用消息方法: {:?}", self);
    }
}

// 硬币枚举
#[derive(Debug)]
enum Coin {
    Penny,
    Nickel,
    Dime,
    Quarter(UsState),
}

#[derive(Debug)]
enum UsState {
    Alabama,
    Alaska,
    // ... 其他州
}

// 数据类型枚举
#[derive(Debug)]
enum DataType {
    Integer(i32),
    Float(f64),
    Text(String),
    Boolean(bool),
    Array(Vec<i32>),
}

// 事件系统枚举
#[derive(Debug)]
enum Event {
    UserAction(UserAction),
    SystemEvent(SystemEvent),
}

#[derive(Debug)]
enum UserAction {
    Click { x: i32, y: i32 },
    KeyPress(char),
}

#[derive(Debug)]
enum SystemEvent {
    Shutdown,
    NetworkError(String),
}

// 带绑定的枚举
#[derive(Debug)]
enum MyMessage {
    Hello { id: i32 },
}

// ========== 辅助函数 ==========

fn route(ip_kind: IpAddrKind) {
    match ip_kind {
        IpAddrKind::V4 => println!("路由IPv4地址"),
        IpAddrKind::V6 => println!("路由IPv6地址"),
    }
}

fn process_message(msg: Message) {
    match msg {
        Message::Quit => {
            println!("退出程序");
        }
        Message::Move { x, y } => {
            println!("移动到坐标 ({}, {})", x, y);
        }
        Message::Write(text) => {
            println!("写入文本: {}", text);
        }
        Message::ChangeColor(r, g, b) => {
            println!("改变颜色为 RGB({}, {}, {})", r, g, b);
        }
    }
}

fn plus_one_regular(x: i8) -> i8 {
    x + 1
}

fn plus_one_option(x: Option<i8>) -> Option<i8> {
    match x {
        None => None,
        Some(i) => Some(i + 1),
    }
}

fn value_in_cents(coin: Coin) -> u8 {
    match coin {
        Coin::Penny => {
            println!("幸运便士！");
            1
        }
        Coin::Nickel => 5,
        Coin::Dime => 10,
        Coin::Quarter(state) => {
            println!("来自 {:?} 州的25美分硬币!", state);
            25
        }
    }
}

fn analyze_data(data: DataType) {
    match data {
        DataType::Integer(value) => {
            println!("整数: {}, 是否为偶数: {}", value, value % 2 == 0);
        }
        DataType::Float(value) => {
            println!("浮点数: {}, 向上取整: {}", value, value.ceil());
        }
        DataType::Text(text) => {
            println!("文本: '{}', 长度: {}", text, text.len());
        }
        DataType::Boolean(flag) => {
            println!("布尔值: {}", if flag { "真" } else { "假" });
        }
        DataType::Array(arr) => {
            let sum: i32 = arr.iter().sum();
            println!("数组: {:?}, 元素和: {}", arr, sum);
        }
    }
}

fn handle_event(event: Event) {
    match event {
        Event::UserAction(UserAction::Click { x, y }) => {
            println!("用户在 ({}, {}) 位置点击", x, y);
        }
        Event::UserAction(UserAction::KeyPress(key)) => {
            println!("用户按下了 '{}' 键", key);
        }
        Event::SystemEvent(SystemEvent::Shutdown) => {
            println!("系统正在关闭");
        }
        Event::SystemEvent(SystemEvent::NetworkError(error)) => {
            println!("网络错误: {}", error);
        }
    }
}

/* 
重要概念总结：

枚举基础：
- enum 关键字定义枚举类型
- 可以有简单变体、带数据变体
- 每个变体可以有不同类型和数量的关联数据

Option 枚举：
- Rust 标准库中的重要枚举：Option<T>
- Some(T) 表示有值，None 表示无值
- 替代其他语言中的 null/nil

match 表达式：
- 强大的模式匹配控制流结构
- 必须穷尽所有可能的模式
- 可以绑定值、使用守卫表达式
- 是表达式，可以返回值

模式匹配语法糖：
- if let：处理单一模式的简化语法
- while let：结合循环的模式匹配
- 在某些情况下比 match 更简洁

高级模式匹配：
- 范围匹配：1..=5
- 绑定：variable @ pattern
- 守卫：pattern if condition
- 解构：提取枚举中的数据

最佳实践：
- 使用枚举建模状态和数据变体
- 优先使用 Option<T> 而不是 null
- 利用编译器检查确保处理所有情况
- 合理使用 if let 简化单一模式匹配

编译运行：
cargo run --bin 08_enums_and_pattern_matching
*/
