// 05_ownership_and_borrowing.rs - 所有权与借用
// 本文件展示 Rust 最核心的概念：所有权系统和借用机制

fn main() {
    println!("=== Rust 所有权与借用 ===\n");
    
    // ========== 所有权基础 ==========
    println!("========== 所有权基础 ==========");
    
    // 1. 基本所有权规则
    println!("\n1. 基本所有权：");
    
    // 栈上数据的复制（Copy trait）
    let x = 5;
    let y = x; // x 的值被复制给 y
    println!("x = {}, y = {}", x, y); // x 仍然有效
    
    // 堆上数据的移动（Move）
    let s1 = String::from("hello");
    let s2 = s1; // s1 的所有权移动给 s2
    println!("s2 = {}", s2);
    // println!("s1 = {}", s1); // 编译错误！s1 已经失效
    
    // 2. 克隆堆数据
    println!("\n2. 克隆堆数据：");
    let s1 = String::from("hello");
    let s2 = s1.clone(); // 深拷贝
    println!("s1 = {}, s2 = {}", s1, s2); // 两者都有效
    
    // ========== 函数与所有权 ==========
    println!("\n========== 函数与所有权 ==========");
    
    // 3. 函数参数的所有权转移
    println!("\n3. 函数参数的所有权转移：");
    let s = String::from("hello");
    takes_ownership(s); // s 的所有权转移给函数
    // println!("{}", s); // 编译错误！s 已经失效
    
    let x = 5;
    makes_copy(x); // x 被复制到函数中
    println!("x 仍然有效: {}", x); // x 仍然有效，因为 i32 实现了 Copy
    
    // 4. 函数返回值的所有权
    println!("\n4. 函数返回值的所有权：");
    let s1 = gives_ownership(); // 函数返回值的所有权转移给 s1
    println!("s1 = {}", s1);
    
    let s2 = String::from("hello");
    let s3 = takes_and_gives_back(s2); // s2 的所有权转移给函数，然后返回给 s3
    println!("s3 = {}", s3);
    // println!("s2 = {}", s2); // 编译错误！s2 已经失效
    
    // ========== 引用与借用 ==========
    println!("\n========== 引用与借用 ==========");
    
    // 5. 不可变引用（借用）
    println!("\n5. 不可变引用：");
    let s1 = String::from("hello");
    let len = calculate_length(&s1); // 借用 s1，不转移所有权
    println!("字符串 '{}' 的长度是 {}", s1, len); // s1 仍然有效
    
    // 6. 可变引用
    println!("\n6. 可变引用：");
    let mut s = String::from("hello");
    change(&mut s); // 可变借用
    println!("修改后的字符串: {}", s);
    
    // 7. 借用规则演示
    println!("\n7. 借用规则：");
    
    // 规则1：可以有多个不可变引用
    let s1 = String::from("hello");
    let r1 = &s1;
    let r2 = &s1;
    println!("r1: {}, r2: {}", r1, r2);
    
    // 规则2：可变引用与不可变引用不能同时存在
    let mut s = String::from("hello");
    {
        let r1 = &mut s;
        println!("可变引用: {}", r1);
    } // r1 离开作用域
    
    let r2 = &s; // 现在可以创建不可变引用
    println!("不可变引用: {}", r2);
    
    // 8. 引用的作用域
    println!("\n8. 引用的作用域：");
    let mut s = String::from("hello");
    
    let r1 = &s; // 不可变引用
    let r2 = &s; // 不可变引用
    println!("{} and {}", r1, r2);
    // r1 和 r2 在这里不再使用
    
    let r3 = &mut s; // 可变引用，因为不可变引用已经不再使用
    println!("{}", r3);
    
    // ========== 悬垂引用 ==========
    println!("\n========== 悬垂引用 ==========");
    
    // 9. 避免悬垂引用
    println!("\n9. 悬垂引用的防止：");
    // let reference_to_nothing = dangle(); // 编译错误！
    let no_dangle_result = no_dangle();
    println!("安全的函数返回: {}", no_dangle_result);
    
    // ========== 实际应用示例 ==========
    println!("\n========== 实际应用示例 ==========");
    
    // 10. 字符串操作示例
    println!("\n10. 字符串操作示例：");
    let mut text = String::from("hello world");
    
    // 查找第一个单词
    let word = first_word(&text);
    println!("第一个单词: '{}'", word);
    
    // 添加文本
    append_text(&mut text, "!");
    println!("修改后: '{}'", text);
    
    // 获取单词数量
    let count = word_count(&text);
    println!("单词数量: {}", count);
    
    // 11. 数据结构借用示例
    println!("\n11. 数据结构借用：");
    let numbers = vec![1, 2, 3, 4, 5];
    
    // 不可变借用数组
    let sum = calculate_sum(&numbers);
    println!("数组 {:?} 的和是: {}", numbers, sum);
    
    // 可变借用数组
    let mut numbers = vec![1, 2, 3, 4, 5];
    double_values(&mut numbers);
    println!("翻倍后的数组: {:?}", numbers);
    
    // 12. 复杂借用场景
    println!("\n12. 复杂借用场景：");
    complex_borrowing_example();
    
    println!("\n=== 所有权与借用学习完成 ===");
}

// ========== 所有权转移函数 ==========

fn takes_ownership(some_string: String) {
    println!("函数接收所有权: {}", some_string);
} // some_string 在这里被销毁

fn makes_copy(some_integer: i32) {
    println!("函数接收拷贝: {}", some_integer);
} // some_integer 在这里离开作用域，但没有特殊处理

fn gives_ownership() -> String {
    let some_string = String::from("hello");
    some_string // 返回 some_string，所有权转移给调用者
}

fn takes_and_gives_back(a_string: String) -> String {
    a_string // 返回 a_string，所有权转移给调用者
}

// ========== 借用函数 ==========

fn calculate_length(s: &String) -> usize {
    s.len() // 借用，不获取所有权
} // s 离开作用域，但没有所有权，所以不会销毁

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}

// 这个函数会导致悬垂引用，无法编译
// fn dangle() -> &String {
//     let s = String::from("hello");
//     &s // 返回对 s 的引用，但 s 即将被销毁
// }

fn no_dangle() -> String {
    let s = String::from("hello");
    s // 直接返回 s，转移所有权
}

// ========== 实用函数示例 ==========

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..] // 如果没有空格，返回整个字符串
}

fn append_text(s: &mut String, text: &str) {
    s.push_str(text);
}

fn word_count(s: &str) -> usize {
    if s.is_empty() {
        return 0;
    }
    
    s.split_whitespace().count()
}

fn calculate_sum(numbers: &[i32]) -> i32 {
    let mut sum = 0;
    for &number in numbers {
        sum += number;
    }
    sum
}

fn double_values(numbers: &mut Vec<i32>) {
    for number in numbers.iter_mut() {
        *number *= 2;
    }
}

fn complex_borrowing_example() {
    let mut data = vec![1, 2, 3, 4, 5];
    
    println!("原始数据: {:?}", data);
    
    // 创建不可变引用
    let sum = {
        let data_ref = &data;
        data_ref.iter().sum::<i32>()
    };
    
    println!("数据和: {}", sum);
    
    // 创建可变引用
    {
        let data_ref = &mut data;
        data_ref.push(6);
        data_ref.push(7);
    }
    
    println!("修改后的数据: {:?}", data);
    
    // 再次使用不可变引用
    let length = data.len();
    println!("数据长度: {}", length);
}

/* 
重要概念总结：

所有权规则：
1. Rust 中每个值都有一个所有者
2. 值在任意时刻只能有一个所有者
3. 当所有者离开作用域时，值被自动销毁

移动 vs 复制：
- 栈上数据（实现了 Copy trait）：复制
- 堆上数据（如 String）：移动所有权
- clone() 方法可以深拷贝堆数据

借用规则：
1. 在任意时刻，要么只能有一个可变引用，要么只能有多个不可变引用
2. 引用必须总是有效的（防止悬垂引用）

关键优势：
- 内存安全：编译时防止内存泄漏和悬垂指针
- 零成本抽象：没有运行时开销
- 并发安全：借用检查器防止数据竞争

常见模式：
- 传递引用而不是所有权来避免不必要的移动
- 使用可变引用进行就地修改
- 合理安排引用的生命周期

编译运行：
cargo run --bin 05_ownership_and_borrowing
*/
