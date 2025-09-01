// 03_functions_and_scope.rs - 函数与作用域
// 本文件展示 Rust 中函数的定义、参数、返回值和作用域规则

fn main() {
    println!("=== Rust 函数与作用域 ===\n");
    
    // 1. 基本函数调用
    println!("1. 基本函数调用：");
    hello_world();
    
    // 2. 带参数的函数
    println!("\n2. 带参数的函数：");
    greet("Alice");
    greet_with_age("Bob", 25);
    
    // 3. 有返回值的函数
    println!("\n3. 有返回值的函数：");
    let sum = add(5, 3);
    println!("5 + 3 = {}", sum);
    
    let product = multiply(4, 7);
    println!("4 × 7 = {}", product);
    
    // 4. 表达式与语句
    println!("\n4. 表达式与语句：");
    let y = {
        let x = 3;
        x + 1  // 表达式，没有分号
    };
    println!("表达式块的值: {}", y);
    
    // 5. 多返回值（使用元组）
    println!("\n5. 多返回值：");
    let (quotient, remainder) = divide_with_remainder(17, 5);
    println!("17 ÷ 5 = {} 余 {}", quotient, remainder);
    
    // 6. 函数作为参数
    println!("\n6. 函数作为参数：");
    let result1 = apply_operation(10, 5, add);
    let result2 = apply_operation(10, 5, multiply);
    println!("应用加法: {}", result1);
    println!("应用乘法: {}", result2);
    
    // 7. 闭包（匿名函数）
    println!("\n7. 闭包：");
    let square = |x| x * x;
    println!("5 的平方: {}", square(5));
    
    let add_ten = |x| x + 10;
    println!("15 + 10 = {}", add_ten(15));
    
    // 捕获环境变量的闭包
    let multiplier = 3;
    let multiply_by_three = |x| x * multiplier;
    println!("7 × 3 = {}", multiply_by_three(7));
    
    // 8. 作用域示例
    println!("\n8. 作用域示例：");
    scope_demonstration();
    
    // 9. 早期返回
    println!("\n9. 早期返回：");
    println!("检查正数: {}", check_positive(5));
    println!("检查正数: {}", check_positive(-3));
    println!("检查正数: {}", check_positive(0));
    
    // 10. 递归函数
    println!("\n10. 递归函数：");
    println!("5! = {}", factorial(5));
    println!("斐波那契数列第10项: {}", fibonacci(10));
    
    println!("\n=== 函数与作用域学习完成 ===");
}

// 1. 最简单的函数
fn hello_world() {
    println!("Hello, World!");
}

// 2. 带参数的函数
fn greet(name: &str) {
    println!("你好, {}!", name);
}

// 带多个参数的函数
fn greet_with_age(name: &str, age: u32) {
    println!("你好, {}! 你 {} 岁了。", name, age);
}

// 3. 有返回值的函数
fn add(a: i32, b: i32) -> i32 {
    a + b  // 表达式，作为返回值
}

// 使用 return 关键字的函数
fn multiply(a: i32, b: i32) -> i32 {
    return a * b;  // 显式返回
}

// 4. 返回元组的函数
fn divide_with_remainder(dividend: i32, divisor: i32) -> (i32, i32) {
    (dividend / divisor, dividend % divisor)
}

// 5. 函数指针作为参数
fn apply_operation(a: i32, b: i32, operation: fn(i32, i32) -> i32) -> i32 {
    operation(a, b)
}

// 6. 作用域演示函数
fn scope_demonstration() {
    let outer_variable = "外部变量";
    println!("函数开始: {}", outer_variable);
    
    {
        let inner_variable = "内部变量";
        println!("内部作用域: {}", inner_variable);
        println!("内部作用域也能访问: {}", outer_variable);
        
        // 遮蔽外部变量
        let outer_variable = "遮蔽的外部变量";
        println!("遮蔽后: {}", outer_variable);
    } // inner_variable 在这里被销毁
    
    println!("回到外部作用域: {}", outer_variable);
    
    // 条件作用域
    let condition = true;
    if condition {
        let conditional_var = "条件变量";
        println!("条件作用域内: {}", conditional_var);
    }
    
    // conditional_var 在这里不可访问
}

// 7. 早期返回示例
fn check_positive(number: i32) -> &'static str {
    if number > 0 {
        return "正数";
    }
    
    if number < 0 {
        return "负数";
    }
    
    "零"
}

// 8. 递归函数示例
fn factorial(n: u32) -> u32 {
    if n <= 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn fibonacci(n: u32) -> u32 {
    match n {
        0 => 0,
        1 => 1,
        _ => fibonacci(n - 1) + fibonacci(n - 2),
    }
}

// 9. 不同返回类型的函数
fn return_nothing() {
    println!("这个函数不返回任何值");
    // 隐式返回 ()
}

fn return_unit() -> () {
    println!("这个函数显式返回单元类型");
    ()
}

// 10. 发散函数（永不返回）
fn diverging_function() -> ! {
    panic!("这个函数永远不会正常返回");
}

/* 
重要概念总结：

函数定义：
- 使用 fn 关键字
- 参数必须指定类型
- 返回类型在 -> 后指定
- 函数体是表达式或语句块

参数和返回值：
- 参数按值传递（除非使用引用）
- 最后一个表达式作为返回值（无分号）
- 可以使用 return 关键字早期返回
- 可以返回元组实现多返回值

作用域规则：
- 变量在声明的作用域内有效
- 内部作用域可以访问外部变量
- 变量遮蔽：内部可以声明同名变量
- 作用域结束时，变量被销毁

特殊函数类型：
- 闭包：可以捕获环境变量的匿名函数
- 函数指针：可以将函数作为参数传递
- 发散函数：返回 ! 类型，永不返回

编译运行：
cargo run --bin 03_functions_and_scope
*/
