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

    // TODO: 添加具体的练习题
    println!("✅ 基础练习完成");
}

fn ownership_exercises() {
    println!("🔹 所有权系统练习");
    println!("涵盖所有权、借用、生命周期等核心概念");

    // TODO: 添加具体的练习题
    println!("✅ 所有权练习完成");
}

fn generics_exercises() {
    println!("🔹 泛型和Trait练习");
    println!("涵盖泛型、Trait、生命周期等高级特性");

    // TODO: 添加具体的练习题
    println!("✅ 泛型练习完成");
}

fn async_exercises() {
    println!("🔹 异步编程练习");
    println!("涵盖async/await、并发等异步编程概念");

    // TODO: 添加具体的练习题
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
