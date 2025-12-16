// 01_variables.rs - 变量和可变性
// 本文件展示 Rust 中变量的声明、可变性和常量的使用

fn main() {
    println!("=== Rust 变量和可变性 ===\n");

    // 1. 不可变变量（默认）
    println!("1. 不可变变量：");
    let x = 5;
    println!("x 的值是: {}", x);
    // x = 6; // 这行会导致编译错误，因为 x 是不可变的

    // 2. 可变变量
    println!("\n2. 可变变量：");
    let mut y = 5;
    println!("y 的初始值: {}", y);
    y = 6; // 可以修改可变变量的值
    println!("y 的新值: {}", y);

    // 3. 变量遮蔽（Shadowing）
    println!("\n3. 变量遮蔽：");
    let z = 5;
    println!("第一个 z: {}", z);

    let z = z + 1; // 遮蔽前一个 z，创建新变量
    println!("遮蔽后的 z: {}", z);

    let z = z * 2; // 再次遮蔽
    println!("再次遮蔽的 z: {}", z);

    // 遮蔽还可以改变变量类型
    let spaces = "   "; // 字符串类型
    let spaces = spaces.len(); // 数字类型
    println!("spaces 的长度: {}", spaces);

    // 4. 常量
    println!("\n4. 常量：");
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("三小时的秒数: {}", THREE_HOURS_IN_SECONDS);

    // 5. 作用域示例
    println!("\n5. 作用域示例：");
    let outer_var = "外部变量";
    println!("外部作用域: {}", outer_var);

    {
        let inner_var = "内部变量";
        println!("内部作用域: {}", inner_var);
        println!("内部作用域也能访问: {}", outer_var);

        // 在内部作用域中遮蔽外部变量
        let outer_var = "被遮蔽的外部变量";
        println!("遮蔽后的外部变量: {}", outer_var);
    }

    // inner_var 在这里不可访问
    println!("回到外部作用域: {}", outer_var); // 恢复原值

    // 6. 未使用变量的处理
    println!("\n6. 未使用变量：");
    let _unused_var = "以下划线开头的变量不会产生未使用警告";

    println!("\n=== 变量和可变性学习完成 ===");
}

/*
重要概念总结：

1. 默认不可变性：Rust 中变量默认是不可变的，这有助于防止意外修改
2. 显式可变性：使用 mut 关键字声明可变变量
3. 变量遮蔽：可以用相同名称声明新变量，甚至改变类型
4. 常量：使用 const 关键字，必须标注类型，只能设置为常量表达式
5. 作用域：变量在其声明的作用域内有效

编译运行：
cargo run --bin variables
*/
