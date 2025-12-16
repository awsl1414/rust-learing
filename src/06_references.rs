// 06_references.rs - 引用与切片
// 本文件深入展示 Rust 中引用的高级用法和切片类型

fn main() {
    println!("=== Rust 引用与切片 ===\n");

    // ========== 引用的深入理解 ==========
    println!("========== 引用的深入理解 ==========");

    // 1. 引用的基本概念
    println!("\n1. 引用的基本概念：");
    let x = 5;
    let y = &x; // y 是 x 的引用

    println!("x = {}", x);
    println!("y = {}", y);
    println!("*y = {}", *y); // 解引用获取值

    // 引用的引用
    let z = &y; // z 是 y 的引用（y 本身是引用）
    println!("z = {}", z);
    println!("*z = {}", *z);
    println!("**z = {}", **z); // 双重解引用

    // 2. 引用的比较
    println!("\n2. 引用的比较：");
    let a = 5;
    let b = 5;
    let ref_a = &a;
    let ref_b = &b;

    println!("a == b: {}", a == b);
    println!("ref_a == ref_b: {}", ref_a == ref_b); // 比较值，不是地址
    println!(
        "ref_a as *const i32 == ref_b as *const i32: {}",
        ref_a as *const i32 == ref_b as *const i32
    ); // 比较地址

    // ========== 字符串切片 ==========
    println!("\n========== 字符串切片 ==========");

    // 3. 字符串切片基础
    println!("\n3. 字符串切片基础：");
    let s = String::from("hello world");

    // 创建字符串切片
    let hello = &s[0..5]; // 或 &s[..5]
    let world = &s[6..11]; // 或 &s[6..]
    let whole = &s[..]; // 整个字符串的切片

    println!("原字符串: '{}'", s);
    println!("hello: '{}'", hello);
    println!("world: '{}'", world);
    println!("whole: '{}'", whole);

    // 4. 字符串字面值就是切片
    println!("\n4. 字符串字面值：");
    let s = "Hello, world!"; // s 的类型是 &str
    println!("字符串字面值: '{}'", s);

    // 5. 字符串切片作为参数
    println!("\n5. 字符串切片作为参数：");
    let my_string = String::from("hello world");
    let my_string_literal = "hello world";

    // 使用 &str 类型的参数可以同时接受 String 和 &str
    let word1 = first_word_slice(&my_string[..]);
    let word2 = first_word_slice(&my_string);
    let word3 = first_word_slice(my_string_literal);

    println!("第一个单词1: '{}'", word1);
    println!("第一个单词2: '{}'", word2);
    println!("第一个单词3: '{}'", word3);

    // ========== 数组切片 ==========
    println!("\n========== 数组切片 ==========");

    // 6. 数组切片基础
    println!("\n6. 数组切片基础：");
    let arr = [1, 2, 3, 4, 5];

    let slice1 = &arr[1..4]; // [2, 3, 4]
    let slice2 = &arr[..3]; // [1, 2, 3]
    let slice3 = &arr[2..]; // [3, 4, 5]
    let slice4 = &arr[..]; // [1, 2, 3, 4, 5]

    println!("原数组: {:?}", arr);
    println!("slice1 [1..4]: {:?}", slice1);
    println!("slice2 [..3]: {:?}", slice2);
    println!("slice3 [2..]: {:?}", slice3);
    println!("slice4 [..]: {:?}", slice4);

    // 7. 切片的操作
    println!("\n7. 切片的操作：");
    let numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];
    let slice = &numbers[2..8];

    println!("原向量: {:?}", numbers);
    println!("切片: {:?}", slice);
    println!("切片长度: {}", slice.len());
    println!("切片第一个元素: {}", slice[0]);
    println!("切片最后一个元素: {}", slice[slice.len() - 1]);

    // 迭代切片
    print!("迭代切片: ");
    for item in slice {
        print!("{} ", item);
    }
    println!();

    // ========== 可变切片 ==========
    println!("\n========== 可变切片 ==========");

    // 8. 可变切片
    println!("\n8. 可变切片：");
    let mut arr = [1, 2, 3, 4, 5];
    println!("修改前: {:?}", arr);

    {
        let slice = &mut arr[1..4];
        slice[0] = 10; // 修改切片中的元素
        slice[1] = 20;
        slice[2] = 30;
        println!("修改切片: {:?}", slice);
    }

    println!("修改后: {:?}", arr);

    // 9. 切片参数的函数
    println!("\n9. 切片参数的函数：");
    let mut numbers = vec![5, 2, 8, 1, 9, 3];
    println!("排序前: {:?}", numbers);

    // 传递切片给函数进行修改
    bubble_sort(&mut numbers);
    println!("排序后: {:?}", numbers);

    let sum = calculate_sum_slice(&numbers[1..4]);
    println!("中间三个数的和: {}", sum);

    // ========== 高级切片操作 ==========
    println!("\n========== 高级切片操作 ==========");

    // 10. 切片的split方法
    println!("\n10. 切片的split方法：");
    let text = "hello,world,rust,programming";
    let words: Vec<&str> = text.split(',').collect();
    println!("分割字符串: {:?}", words);

    // 11. 切片的窗口操作
    println!("\n11. 切片的窗口操作：");
    let data = [1, 2, 3, 4, 5, 6];

    // 滑动窗口
    println!("大小为3的滑动窗口:");
    for window in data.windows(3) {
        println!("  {:?}", window);
    }

    // 分块
    println!("大小为2的分块:");
    for chunk in data.chunks(2) {
        println!("  {:?}", chunk);
    }

    // ========== 字符串操作高级示例 ==========
    println!("\n========== 字符串操作高级示例 ==========");

    // 12. 复杂字符串处理
    println!("\n12. 复杂字符串处理：");
    let text = "  Hello, World!  ";

    // 去除空白
    let trimmed = text.trim();
    println!("原文本: '{}'", text);
    println!("去空白: '{}'", trimmed);

    // 查找子字符串
    if let Some(pos) = text.find("World") {
        println!("'World' 在位置: {}", pos);
        let world_slice = &text[pos..pos + 5];
        println!("提取的切片: '{}'", world_slice);
    }

    // 13. 字符串替换和处理
    println!("\n13. 字符串替换和处理：");
    let sentence = "The quick brown fox jumps over the lazy dog";

    // 单词处理
    let words: Vec<&str> = sentence.split_whitespace().collect();
    println!("单词列表: {:?}", words);

    // 查找长单词
    let long_words: Vec<&str> = words.into_iter().filter(|word| word.len() > 4).collect();
    println!("长单词 (>4字符): {:?}", long_words);

    // 14. UTF-8 字符串处理
    println!("\n14. UTF-8 字符串处理：");
    let chinese = "你好，世界！";
    println!("中文字符串: '{}'", chinese);
    println!("字节长度: {}", chinese.len());
    println!("字符数量: {}", chinese.chars().count());

    // 按字符迭代
    print!("逐字符: ");
    for c in chinese.chars() {
        print!("'{}' ", c);
    }
    println!();

    // 字符索引（注意：不能直接用数字索引UTF-8字符串）
    let chars: Vec<char> = chinese.chars().collect();
    println!("第二个字符: '{}'", chars[1]);

    println!("\n=== 引用与切片学习完成 ===");
}

// ========== 辅助函数 ==========

// 返回字符串中第一个单词的切片
fn first_word_slice(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}

// 冒泡排序（操作可变切片）
fn bubble_sort(arr: &mut [i32]) {
    let len = arr.len();
    for i in 0..len {
        for j in 0..len - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
            }
        }
    }
}

// 计算切片的和
fn calculate_sum_slice(slice: &[i32]) -> i32 {
    slice.iter().sum()
}

// 查找切片中的最大值
fn find_max(slice: &[i32]) -> Option<&i32> {
    slice.iter().max()
}

// 安全的字符串切片函数
fn safe_slice(s: &str, start: usize, end: usize) -> Option<&str> {
    if start <= end && end <= s.len() {
        Some(&s[start..end])
    } else {
        None
    }
}

// 字符串单词统计
fn count_words(text: &str) -> usize {
    if text.trim().is_empty() {
        0
    } else {
        text.split_whitespace().count()
    }
}

// 检查字符串是否为回文
fn is_palindrome(s: &str) -> bool {
    let s = s.chars().collect::<Vec<_>>();
    let len = s.len();

    for i in 0..len / 2 {
        if s[i] != s[len - 1 - i] {
            return false;
        }
    }

    true
}

/*
重要概念总结：

引用类型：
- 不可变引用 &T：允许读取但不能修改
- 可变引用 &mut T：允许读取和修改
- 引用不拥有数据，只是借用

切片类型：
- 字符串切片 &str：指向字符串一部分的引用
- 数组切片 &[T]：指向数组一部分的引用
- 切片包含指针和长度信息

字符串类型对比：
- String：拥有所有权的可变字符串
- &str：字符串切片，通常用于函数参数
- 字符串字面值是 &str 类型

切片操作：
- 索引语法：&s[start..end]
- 省略语法：&s[..end], &s[start..], &s[..]
- 方法：split, windows, chunks, trim 等

安全性特性：
- 编译时检查边界
- 防止悬垂引用
- UTF-8 字符串安全处理

最佳实践：
- 函数参数优先使用 &str 而不是 String
- 使用切片而不是索引来操作序列
- 利用迭代器而不是手动索引

编译运行：
cargo run --bin references
*/
