// 09_collections.rs - 常见集合类型及常用操作
// 本文件展示 Rust 中三种主要集合类型：Vector、String、HashMap

fn main() {
    println!("=== Rust 常见集合类型 ===\n");

    // ========== Vector 向量 ==========
    println!("========== Vector 向量 ==========");

    // 1. 创建 Vector
    println!("\n1. 创建 Vector：");

    // 创建空向量
    let mut v1: Vec<i32> = Vec::new();
    println!("空向量: {:?}", v1);

    // 使用宏创建向量
    let v2 = vec![1, 2, 3, 4, 5];
    println!("使用宏创建: {:?}", v2);

    // 添加元素
    v1.push(1);
    v1.push(2);
    v1.push(3);
    println!("添加元素后: {:?}", v1);

    // 2. 访问 Vector 元素
    println!("\n2. 访问 Vector 元素：");
    let v = vec![1, 2, 3, 4, 5];

    // 使用索引访问（可能panic）
    let third = v[2];
    println!("第三个元素: {}", third);

    // 使用 get 方法安全访问
    match v.get(2) {
        Some(third) => println!("get方法获取第三个元素: {}", third),
        None => println!("没有第三个元素"),
    }

    match v.get(100) {
        Some(element) => println!("第101个元素: {}", element),
        None => println!("没有第101个元素"),
    }

    // 3. 遍历 Vector
    println!("\n3. 遍历 Vector：");
    let v = vec![100, 32, 57];

    // 不可变遍历
    print!("不可变遍历: ");
    for i in &v {
        print!("{} ", i);
    }
    println!();

    // 可变遍历
    let mut v = vec![100, 32, 57];
    print!("修改前: {:?}", v);
    for i in &mut v {
        *i += 50;
    }
    println!(" 修改后: {:?}", v);

    // 4. Vector 的高级操作
    println!("\n4. Vector 高级操作：");
    let mut numbers = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10];

    println!("原始向量: {:?}", numbers);
    println!("长度: {}", numbers.len());
    println!("容量: {}", numbers.capacity());

    // 删除元素
    let removed = numbers.remove(0);
    println!("删除第一个元素 {}: {:?}", removed, numbers);

    // 弹出最后一个元素
    if let Some(last) = numbers.pop() {
        println!("弹出最后一个元素 {}: {:?}", last, numbers);
    }

    // 插入元素
    numbers.insert(0, 100);
    println!("在开头插入100: {:?}", numbers);

    // 5. Vector 与枚举结合
    println!("\n5. Vector 与枚举：");
    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];

    for cell in &row {
        match cell {
            SpreadsheetCell::Int(i) => println!("整数: {}", i),
            SpreadsheetCell::Text(s) => println!("文本: {}", s),
            SpreadsheetCell::Float(f) => println!("浮点数: {}", f),
        }
    }

    // ========== String 字符串 ==========
    println!("\n========== String 字符串 ==========");

    // 6. 创建字符串
    println!("\n6. 创建字符串：");

    let mut s1 = String::new();
    println!("空字符串: '{}'", s1);

    let s2 = "初始内容".to_string();
    println!("to_string: '{}'", s2);

    let s3 = String::from("初始内容");
    println!("String::from: '{}'", s3);

    // 7. 更新字符串
    println!("\n7. 更新字符串：");

    // 追加字符串切片
    s1.push_str("Hello");
    println!("push_str后: '{}'", s1);

    // 追加单个字符
    s1.push(' ');
    s1.push('!');
    println!("push字符后: '{}'", s1);

    // 8. 字符串连接
    println!("\n8. 字符串连接：");

    let s1 = String::from("Hello, ");
    let s2 = String::from("world!");

    // 使用 + 运算符
    let s3 = s1 + &s2; // s1 被移动，不能再使用
    println!("使用+连接: '{}'", s3);

    // 使用 format! 宏
    let s1 = String::from("tic");
    let s2 = String::from("tac");
    let s3 = String::from("toe");

    let s = format!("{}-{}-{}", s1, s2, s3);
    println!("使用format!: '{}'", s);

    // 9. 字符串索引和切片
    println!("\n9. 字符串索引和切片：");

    let hello = "Здравствуйте";
    println!("俄语字符串: '{}'", hello);
    println!("字节长度: {}", hello.len());

    // 字符串切片（注意UTF-8边界）
    let s = &hello[0..4]; // 注意：这可能panic如果不在字符边界
    println!("前4个字节: '{}'", s);

    // 安全的字符遍历
    print!("逐字符遍历: ");
    for c in hello.chars() {
        print!("{} ", c);
    }
    println!();

    print!("逐字节遍历: ");
    for b in hello.bytes() {
        print!("{} ", b);
    }
    println!();

    // 10. 字符串方法
    println!("\n10. 字符串方法：");

    let text = "  Hello, Rust World!  ";
    println!("原文本: '{}'", text);
    println!("去空白: '{}'", text.trim());
    println!("转大写: '{}'", text.to_uppercase());
    println!("转小写: '{}'", text.to_lowercase());
    println!("包含'Rust': {}", text.contains("Rust"));
    println!("以'Hello'开始: {}", text.trim().starts_with("Hello"));
    println!("以'!'结束: {}", text.trim().ends_with("!"));

    // 字符串分割
    let data = "apple,banana,orange";
    let fruits: Vec<&str> = data.split(',').collect();
    println!("分割结果: {:?}", fruits);

    // 字符串替换
    let text = "I like apples";
    let new_text = text.replace("apples", "oranges");
    println!("替换结果: '{}'", new_text);

    // ========== HashMap 哈希映射 ==========
    println!("\n========== HashMap 哈希映射 ==========");

    // 11. 创建 HashMap
    println!("\n11. 创建 HashMap：");

    use std::collections::HashMap;

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    println!("分数表: {:?}", scores);

    // 从向量创建
    let teams = vec![String::from("Blue"), String::from("Yellow")];
    let initial_scores = vec![10, 50];

    let scores: HashMap<_, _> = teams.into_iter().zip(initial_scores.into_iter()).collect();
    println!("从向量创建: {:?}", scores);

    // 12. 访问 HashMap 值
    println!("\n12. 访问 HashMap 值：");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    let team_name = String::from("Blue");
    if let Some(score) = scores.get(&team_name) {
        println!("{}队的分数: {}", team_name, score);
    }

    // 遍历哈希映射
    println!("所有队伍和分数:");
    for (key, value) in &scores {
        println!("  {}: {}", key, value);
    }

    // 13. 更新 HashMap
    println!("\n13. 更新 HashMap：");

    let mut scores = HashMap::new();

    // 覆盖值
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Blue"), 25);
    println!("覆盖后: {:?}", scores);

    // 只在键没有值时插入
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);
    println!("条件插入后: {:?}", scores);

    // 根据旧值更新值
    let text = "hello world wonderful world";
    let mut map = HashMap::new();

    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("单词计数: {:?}", map);

    // ========== 集合综合应用 ==========
    println!("\n========== 集合综合应用 ==========");

    // 14. 学生成绩管理系统
    println!("\n14. 学生成绩管理系统：");

    let mut grade_book = GradeBook::new();

    grade_book.add_student("张三".to_string());
    grade_book.add_student("李四".to_string());
    grade_book.add_student("王五".to_string());

    grade_book.add_grade("张三", 85);
    grade_book.add_grade("张三", 92);
    grade_book.add_grade("李四", 78);
    grade_book.add_grade("李四", 88);
    grade_book.add_grade("王五", 95);

    grade_book.display_grades();
    grade_book.display_statistics();

    // 15. 文本分析
    println!("\n15. 文本分析：");
    let text = "Hello world! This is a sample text for analysis. Hello Rust!";
    analyze_text(text);

    println!("\n=== 集合类型学习完成 ===");
}

// ========== 辅助类型和结构 ==========

#[derive(Debug)]
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

// 学生成绩管理系统
struct GradeBook {
    students: Vec<String>,
    grades: std::collections::HashMap<String, Vec<i32>>,
}

impl GradeBook {
    fn new() -> GradeBook {
        GradeBook {
            students: Vec::new(),
            grades: std::collections::HashMap::new(),
        }
    }

    fn add_student(&mut self, name: String) {
        if !self.students.contains(&name) {
            self.students.push(name.clone());
            self.grades.insert(name, Vec::new());
        }
    }

    fn add_grade(&mut self, student: &str, grade: i32) {
        if let Some(grades) = self.grades.get_mut(student) {
            grades.push(grade);
        } else {
            println!("学生 {} 不存在", student);
        }
    }

    fn display_grades(&self) {
        println!("学生成绩:");
        for student in &self.students {
            if let Some(grades) = self.grades.get(student) {
                println!("  {}: {:?}", student, grades);
            }
        }
    }

    fn display_statistics(&self) {
        println!("成绩统计:");
        for student in &self.students {
            if let Some(grades) = self.grades.get(student) {
                if !grades.is_empty() {
                    let sum: i32 = grades.iter().sum();
                    let average = sum as f64 / grades.len() as f64;
                    let max = grades.iter().max().unwrap();
                    let min = grades.iter().min().unwrap();

                    println!("  {}:", student);
                    println!("    平均分: {:.2}", average);
                    println!("    最高分: {}", max);
                    println!("    最低分: {}", min);
                }
            }
        }
    }
}

// 文本分析函数
fn analyze_text(text: &str) {
    use std::collections::HashMap;

    println!("分析文本: '{}'", text);

    // 统计单词
    let mut word_count = HashMap::new();
    for word in text.split_whitespace() {
        let word = word
            .to_lowercase()
            .chars()
            .filter(|c| c.is_alphabetic())
            .collect::<String>();

        if !word.is_empty() {
            *word_count.entry(word).or_insert(0) += 1;
        }
    }

    println!("单词统计:");
    let mut words: Vec<_> = word_count.iter().collect();
    words.sort_by(|a, b| b.1.cmp(a.1)); // 按出现次数排序

    for (word, count) in words {
        println!("  '{}': {} 次", word, count);
    }

    // 统计字符
    let mut char_count = HashMap::new();
    for ch in text.chars() {
        if ch.is_alphabetic() {
            *char_count.entry(ch.to_lowercase().to_string()).or_insert(0) += 1;
        }
    }

    println!("字符统计 (前5个):");
    let mut chars: Vec<_> = char_count.iter().collect();
    chars.sort_by(|a, b| b.1.cmp(a.1));

    for (ch, count) in chars.iter().take(5) {
        println!("  '{}': {} 次", ch, count);
    }

    // 基本统计
    let word_count = text.split_whitespace().count();
    let char_count = text.chars().count();
    let sentence_count =
        text.split('.').count().saturating_sub(1) + text.split('!').count().saturating_sub(1);

    println!("基本统计:");
    println!("  单词数: {}", word_count);
    println!("  字符数: {}", char_count);
    println!("  句子数: {}", sentence_count);
}

/*
重要概念总结：

Vector (Vec<T>)：
- 可变长度的数组，存储在堆上
- 使用 Vec::new() 或 vec![] 宏创建
- push/pop 添加/删除元素
- 支持索引访问和迭代

String：
- UTF-8 编码的可变字符串
- String::new(), String::from(), to_string() 创建
- push_str/push 添加内容
- 不支持直接索引，需要使用切片

HashMap<K, V>：
- 键值对映射，基于哈希表
- insert/get 插入/获取值
- entry API 提供高级操作
- 迭代时顺序不确定

共同特点：
- 都存储在堆上，支持动态增长
- 实现了常见的迭代器模式
- 提供丰富的方法进行操作

集合选择指南：
- 需要有序列表：Vec<T>
- 需要字符串操作：String
- 需要键值映射：HashMap<K, V>
- 需要唯一值集合：HashSet<T>

性能考虑：
- Vec 按索引访问 O(1)，插入/删除 O(n)
- HashMap 查找/插入 O(1) 平均
- String 连接操作可能涉及内存分配

编译运行：
cargo run --bin collections
*/
