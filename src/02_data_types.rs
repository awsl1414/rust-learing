// 02_data_types.rs - 数据类型（标量与复合）
// 本文件展示 Rust 中的基本数据类型和复合数据类型

fn main() {
    println!("=== Rust 数据类型 ===\n");
    
    // ========== 标量类型 ==========
    println!("========== 标量类型 ==========");
    
    // 1. 整数类型
    println!("\n1. 整数类型：");
    let a: i8 = 127;           // 8位有符号整数 (-128 到 127)
    let b: u8 = 255;           // 8位无符号整数 (0 到 255)
    let c: i32 = -42;          // 32位有符号整数（默认整数类型）
    let d: u32 = 42;           // 32位无符号整数
    let e: i64 = -9223372036854775808; // 64位有符号整数
    let f: usize = 100;        // 指针大小的无符号整数
    
    println!("i8: {}, u8: {}, i32: {}, u32: {}, i64: {}, usize: {}", a, b, c, d, e, f);
    
    // 整数字面值的不同写法
    let decimal = 98_222;      // 十进制
    let hex = 0xff;            // 十六进制
    let octal = 0o77;          // 八进制
    let binary = 0b1111_0000;  // 二进制
    let byte = b'A';           // 字节（仅限 u8）
    
    println!("不同进制: decimal={}, hex={}, octal={}, binary={}, byte={}", 
             decimal, hex, octal, binary, byte);
    
    // 2. 浮点类型
    println!("\n2. 浮点类型：");
    let x = 2.0;      // f64（默认浮点类型）
    let y: f32 = 3.0; // f32
    
    println!("f64: {}, f32: {}", x, y);
    
    // 浮点数运算
    let sum = 5.0 + 10.0;
    let difference = 95.5 - 4.3;
    let product = 4.0 * 30.0;
    let quotient = 56.7 / 32.2;
    let remainder = 43.0 % 5.0;
    
    println!("加法: {}, 减法: {}, 乘法: {}, 除法: {}, 取余: {}", 
             sum, difference, product, quotient, remainder);
    
    // 3. 布尔类型
    println!("\n3. 布尔类型：");
    let t = true;
    let f: bool = false;
    
    println!("t: {}, f: {}", t, f);
    
    // 布尔运算
    println!("逻辑与: {}", t && f);
    println!("逻辑或: {}", t || f);
    println!("逻辑非: {}", !t);
    
    // 4. 字符类型
    println!("\n4. 字符类型：");
    let c = 'z';
    let z = 'ℤ';
    let heart_eyed_cat = '😻';
    let chinese_char = '中';
    
    println!("字符: '{}', '{}', '{}', '{}'", c, z, heart_eyed_cat, chinese_char);
    
    // ========== 复合类型 ==========
    println!("\n========== 复合类型 ==========");
    
    // 5. 元组类型
    println!("\n5. 元组类型：");
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    
    // 解构元组
    let (x, y, z) = tup;
    println!("解构元组: x={}, y={}, z={}", x, y, z);
    
    // 通过索引访问元组元素
    println!("通过索引访问: {}, {}, {}", tup.0, tup.1, tup.2);
    
    // 单元类型（空元组）
    let unit = ();
    println!("单元类型: {:?}", unit);
    
    // 6. 数组类型
    println!("\n6. 数组类型：");
    
    // 固定长度数组
    let arr1 = [1, 2, 3, 4, 5];
    let arr2: [i32; 5] = [1, 2, 3, 4, 5]; // 显式类型标注
    let arr3 = [3; 5]; // 创建包含 5 个 3 的数组
    
    println!("arr1: {:?}", arr1);
    println!("arr2: {:?}", arr2);
    println!("arr3: {:?}", arr3);
    
    // 访问数组元素
    println!("第一个元素: {}", arr1[0]);
    println!("最后一个元素: {}", arr1[arr1.len() - 1]);
    
    // 数组长度
    println!("数组长度: {}", arr1.len());
    
    // 7. 字符串类型
    println!("\n7. 字符串类型：");
    
    // 字符串字面值（&str）
    let string_literal = "Hello, Rust!";
    println!("字符串字面值: {}", string_literal);
    
    // String 类型
    let mut string_object = String::from("Hello");
    string_object.push_str(", World!");
    println!("String 对象: {}", string_object);
    
    // 8. 类型转换
    println!("\n8. 类型转换：");
    
    let integer = 42;
    let float = integer as f64;
    println!("整数转浮点: {} -> {}", integer, float);
    
    let large_int: i64 = 1000;
    let small_int = large_int as i32;
    println!("大整数转小整数: {} -> {}", large_int, small_int);
    
    // 字符串解析
    let string_number = "42";
    let parsed_number: i32 = string_number.parse().expect("不是有效数字");
    println!("字符串解析为数字: '{}' -> {}", string_number, parsed_number);
    
    // 9. 类型推断示例
    println!("\n9. 类型推断：");
    let inferred = 42; // Rust 会推断为 i32
    let explicitly_typed: u64 = 42; // 显式类型标注
    
    println!("推断类型 (i32): {}", inferred);
    println!("显式类型 (u64): {}", explicitly_typed);
    
    println!("\n=== 数据类型学习完成 ===");
}

/* 
重要概念总结：

标量类型：
- 整数：i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
- 浮点：f32, f64
- 布尔：bool
- 字符：char（4字节 Unicode）

复合类型：
- 元组：固定大小，可包含不同类型
- 数组：固定大小，相同类型元素

关键特性：
1. 类型安全：编译时检查类型
2. 类型推断：编译器可以推断大多数类型
3. 显式转换：使用 as 关键字进行类型转换
4. 溢出检查：debug 模式下检查整数溢出

编译运行：
cargo run --bin 02_data_types
*/
