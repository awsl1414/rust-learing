// 04_control.rs - 流程控制
// 本文件展示 Rust 中的条件语句、循环和模式匹配等流程控制结构

fn main() {
    println!("=== Rust 流程控制 ===\n");

    // ========== 条件语句 ==========
    println!("========== 条件语句 ==========");

    // 1. if 表达式
    println!("\n1. if 表达式：");
    let number = 6;

    if number % 4 == 0 {
        println!("{} 能被 4 整除", number);
    } else if number % 3 == 0 {
        println!("{} 能被 3 整除", number);
    } else if number % 2 == 0 {
        println!("{} 能被 2 整除", number);
    } else {
        println!("{} 不能被 2、3、4 整除", number);
    }

    // 2. if 作为表达式
    println!("\n2. if 作为表达式：");
    let condition = true;
    let number = if condition { 5 } else { 6 };
    println!("根据条件选择的数字: {}", number);

    // 复杂的条件表达式
    let x = 10;
    let result = if x > 0 {
        "正数"
    } else if x < 0 {
        "负数"
    } else {
        "零"
    };
    println!("{} 是 {}", x, result);

    // ========== 循环 ==========
    println!("\n========== 循环 ==========");

    // 3. loop 无限循环
    println!("\n3. loop 无限循环：");
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2; // 从循环中返回值
        }
    };

    println!("loop 循环结果: {}", result);

    // 4. while 条件循环
    println!("\n4. while 条件循环：");
    let mut number = 3;

    while number != 0 {
        println!("倒计时: {}", number);
        number -= 1;
    }

    println!("发射!");

    // 5. for 循环
    println!("\n5. for 循环：");

    // 遍历数组
    let arr = [10, 20, 30, 40, 50];
    println!("遍历数组:");
    for element in arr {
        println!("值: {}", element);
    }

    // 使用范围
    println!("\n使用范围 (1..4):");
    for number in 1..4 {
        println!("数字: {}", number);
    }

    println!("\n使用包含范围 (1..=4):");
    for number in 1..=4 {
        println!("数字: {}", number);
    }

    // 倒序遍历
    println!("\n倒序遍历:");
    for number in (1..4).rev() {
        println!("倒序: {}", number);
    }

    // 带索引遍历
    println!("\n带索引遍历:");
    for (index, value) in arr.iter().enumerate() {
        println!("索引 {}: 值 {}", index, value);
    }

    // ========== 嵌套循环和标签 ==========
    println!("\n========== 嵌套循环和标签 ==========");

    // 6. 嵌套循环和循环标签
    println!("\n6. 嵌套循环和循环标签：");
    'outer: loop {
        println!("进入外层循环");

        'inner: loop {
            println!("进入内层循环");
            break 'outer; // 直接跳出外层循环
        }

        println!("这行永远不会执行");
    }

    println!("跳出了外层循环");

    // 复杂的嵌套循环示例
    println!("\n查找第一个能被7整除的数字:");
    'search: for x in 1..=100 {
        for y in 1..=100 {
            if x * y % 7 == 0 && x * y > 20 {
                println!("找到: {} × {} = {}", x, y, x * y);
                break 'search;
            }
        }
    }

    // ========== match 模式匹配 ==========
    println!("\n========== match 模式匹配 ==========");

    // 7. 基本 match
    println!("\n7. 基本 match：");
    let number = 3;

    match number {
        1 => println!("一"),
        2 => println!("二"),
        3 => println!("三"),
        4 | 5 => println!("四或五"),  // 多个模式
        6..=10 => println!("六到十"), // 范围模式
        _ => println!("其他数字"),    // 默认分支
    }

    // 8. match 作为表达式
    println!("\n8. match 作为表达式：");
    let number = 7;
    let description = match number {
        1..=5 => "小数字",
        6..=10 => "中等数字",
        11..=100 => "大数字",
        _ => "非常大的数字",
    };
    println!("{} 是 {}", number, description);

    // 9. 匹配元组
    println!("\n9. 匹配元组：");
    let point = (3, 5);

    match point {
        (0, 0) => println!("原点"),
        (0, y) => println!("在Y轴上，y = {}", y),
        (x, 0) => println!("在X轴上，x = {}", x),
        (x, y) => println!("点坐标: ({}, {})", x, y),
    }

    // 10. 匹配 Option
    println!("\n10. 匹配 Option：");
    let some_number = Some(5);
    let no_number: Option<i32> = None;

    match some_number {
        Some(x) => println!("有值: {}", x),
        None => println!("没有值"),
    }

    match no_number {
        Some(x) => println!("有值: {}", x),
        None => println!("没有值"),
    }

    // ========== if let 和 while let ==========
    println!("\n========== if let 和 while let ==========");

    // 11. if let 语法糖
    println!("\n11. if let 语法糖：");
    let some_value = Some(3);

    // 使用 match
    match some_value {
        Some(3) => println!("匹配到三"),
        _ => (),
    }

    // 使用 if let（更简洁）
    if let Some(3) = some_value {
        println!("if let 匹配到三");
    }

    // 12. while let 循环
    println!("\n12. while let 循环：");
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);

    while let Some(top) = stack.pop() {
        println!("弹出: {}", top);
    }

    // ========== 实际应用示例 ==========
    println!("\n========== 实际应用示例 ==========");

    // 13. 综合示例：简单计算器
    println!("\n13. 简单计算器：");
    calculate_and_print(10, 3, '+');
    calculate_and_print(10, 3, '-');
    calculate_and_print(10, 3, '*');
    calculate_and_print(10, 3, '/');
    calculate_and_print(10, 0, '/');

    // 14. 数字分类
    println!("\n14. 数字分类：");
    let numbers = vec![-5, -1, 0, 1, 5, 10, 15];
    for num in numbers {
        classify_number(num);
    }

    println!("\n=== 流程控制学习完成 ===");
}

// 简单计算器函数
fn calculate_and_print(a: i32, b: i32, operation: char) {
    let result = match operation {
        '+' => Some(a + b),
        '-' => Some(a - b),
        '*' => Some(a * b),
        '/' => {
            if b != 0 {
                Some(a / b)
            } else {
                None
            }
        }
        _ => None,
    };

    match result {
        Some(value) => println!("{} {} {} = {}", a, operation, b, value),
        None => println!("无法计算 {} {} {}", a, operation, b),
    }
}

// 数字分类函数
fn classify_number(n: i32) {
    let classification = match n {
        n if n < 0 => "负数",
        0 => "零",
        1..=10 => "小正数",
        11..=100 => "中正数",
        _ => "大正数",
    };

    let parity = if n % 2 == 0 { "偶数" } else { "奇数" };

    println!("{} 是 {} 且是 {}", n, classification, parity);
}

/*
重要概念总结：

条件语句：
- if/else if/else：条件分支
- if 是表达式，可以返回值
- 条件必须是 bool 类型

循环：
- loop：无限循环，可以用 break 返回值
- while：条件循环
- for：迭代循环，通常用于遍历

循环控制：
- break：跳出循环
- continue：继续下一次迭代
- 循环标签：控制嵌套循环的跳转

模式匹配：
- match：强大的模式匹配，必须穷尽所有可能
- 可以匹配字面值、范围、元组等
- if let：match 的语法糖，用于单一模式
- while let：结合循环的模式匹配

关键特性：
1. 表达式导向：大多数控制结构都是表达式
2. 穷尽性：match 必须处理所有可能的情况
3. 性能：编译器优化，零成本抽象
4. 安全性：编译时检查，避免运行时错误

编译运行：
cargo run --bin control
*/
