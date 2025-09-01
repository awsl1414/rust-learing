// 14_lifetimes.rs - 生命周期
// 本文件展示 Rust 中生命周期的概念、语法和高级用法

fn main() {
    println!("=== Rust 生命周期 ===\n");
    
    // ========== 生命周期基础 ==========
    println!("========== 生命周期基础 ==========");
    
    // 1. 引用的生命周期
    println!("\n1. 引用的生命周期：");
    
    let string1 = String::from("abcd");
    let string2 = "xyz";
    
    let result = longest(string1.as_str(), string2);
    println!("最长的字符串是: '{}'", result);
    
    // 2. 生命周期与作用域
    println!("\n2. 生命周期与作用域：");
    
    let string1 = String::from("long string is long");
    let result;
    {
        let string2 = String::from("xyz");
        result = longest(string1.as_str(), string2.as_str());
        println!("在内部作用域中，最长的字符串是: '{}'", result);
    }
    // result 在这里仍然有效，因为它指向 string1
    // println!("外部作用域: '{}'", result); // 这行会编译错误
    
    // 3. 生命周期注解的必要性
    println!("\n3. 生命周期注解示例：");
    
    let s1 = "Hello";
    let s2 = "World";
    
    let longer = longest(s1, s2);
    println!("更长的字符串: '{}'", longer);
    
    let first = first_word("Hello World");
    println!("第一个单词: '{}'", first);
    
    // ========== 结构体中的生命周期 ==========
    println!("\n========== 结构体中的生命周期 ==========");
    
    // 4. 生命周期与结构体
    println!("\n4. 结构体生命周期：");
    
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    
    let excerpt = ImportantExcerpt {
        part: first_sentence,
    };
    
    excerpt.announce_and_return_part("注意");
    
    // 5. 生命周期省略规则
    println!("\n5. 生命周期省略：");
    
    let text = "Hello, world!";
    let first = first_word_elided(text);
    println!("第一个单词（省略生命周期）: '{}'", first);
    
    // ========== 方法中的生命周期 ==========
    println!("\n========== 方法中的生命周期 ==========");
    
    // 6. 方法定义中的生命周期
    println!("\n6. 方法中的生命周期：");
    
    let announcement = "Ladies and gentlemen";
    let part = "Today is a beautiful day";
    
    let excerpt = ImportantExcerpt { part };
    let result = excerpt.announce_and_return_part(announcement);
    println!("返回的内容: '{}'", result);
    
    // ========== 静态生命周期 ==========
    println!("\n========== 静态生命周期 ==========");
    
    // 7. 'static 生命周期
    println!("\n7. 'static 生命周期：");
    
    let s: &'static str = "我有静态生命周期";
    println!("静态字符串: '{}'", s);
    
    // 8. 生命周期与泛型、trait bound
    println!("\n8. 生命周期与泛型：");
    
    let string1 = String::from("abcd");
    let string2 = "xyz";
    
    let result = longest_with_an_announcement(
        string1.as_str(),
        string2,
        "今天是公告日！",
    );
    println!("结果: '{}'", result);
    
    // ========== 复杂生命周期场景 ==========
    println!("\n========== 复杂生命周期场景 ==========");
    
    // 9. 多个生命周期参数
    println!("\n9. 多个生命周期参数：");
    
    let x = "hello";
    let y = "world";
    let z = "rust";
    
    let result = longest_of_three(x, y, z);
    println!("三个字符串中最长的: '{}'", result);
    
    // 10. 生命周期子类型
    println!("\n10. 生命周期关系：");
    
    let string1 = String::from("long string is long");
    {
        let string2 = String::from("xyz");
        let result = longest_explicit(string1.as_str(), string2.as_str());
        println!("显式生命周期结果: '{}'", result);
    }
    
    // ========== 实际应用示例 ==========
    println!("\n========== 实际应用示例 ==========");
    
    // 11. 字符串解析器
    println!("\n11. 字符串解析器：");
    
    let input = "name=Alice,age=30,city=Beijing";
    let mut parser = Parser::new(input);
    
    while let Some(pair) = parser.next_key_value() {
        println!("解析结果: {} = {}", pair.key, pair.value);
    }
    
    // 12. 缓存结构
    println!("\n12. 引用缓存：");
    
    let data = vec![
        "apple".to_string(),
        "banana".to_string(),
        "cherry".to_string(),
    ];
    
    let cache = StringCache::new(&data);
    
    if let Some(item) = cache.get(1) {
        println!("缓存命中: '{}'", item);
    }
    
    cache.print_all();
    
    // 13. 迭代器与生命周期
    println!("\n13. 迭代器生命周期：");
    
    let numbers = vec![1, 2, 3, 4, 5];
    let mut iter = NumberIterator::new(&numbers);
    
    while let Some(num) = iter.next() {
        println!("迭代器返回: {}", num);
    }
    
    // 14. 函数返回引用
    println!("\n14. 返回引用的函数：");
    
    let text = "Hello, Rust world!";
    let words = extract_words(text);
    println!("提取的单词: {:?}", words);
    
    let content = "apple,banana,cherry";
    let items = split_by_comma(content);
    println!("分割结果: {:?}", items);
    
    println!("\n=== 生命周期学习完成 ===");
}

// ========== 基本生命周期函数 ==========

// 需要显式生命周期注解的函数
fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// 只有一个引用参数，可以省略生命周期
fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();
    
    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }
    
    &s[..]
}

// 显式标注省略的生命周期
fn first_word_elided(s: &str) -> &str {
    first_word(s)
}

// ========== 结构体生命周期 ==========

struct ImportantExcerpt<'a> {
    part: &'a str,
}

impl<'a> ImportantExcerpt<'a> {
    // 方法中的生命周期省略
    fn level(&self) -> i32 {
        3
    }
    
    // 返回引用的方法
    fn announce_and_return_part(&self, announcement: &str) -> &str {
        println!("注意！{}", announcement);
        self.part
    }
}

// ========== 复杂生命周期 ==========

// 多个生命周期参数
fn longest_of_three<'a>(x: &'a str, y: &'a str, z: &'a str) -> &'a str {
    let mut longest = x;
    if y.len() > longest.len() {
        longest = y;
    }
    if z.len() > longest.len() {
        longest = z;
    }
    longest
}

// 显式生命周期关系
fn longest_explicit<'a, 'b>(x: &'a str, y: &'b str) -> &'a str 
where 
    'b: 'a  // 'b 至少要和 'a 一样长
{
    if x.len() > y.len() {
        x
    } else {
        // 这里我们只能返回 x，因为返回类型是 &'a str
        x
    }
}

// 生命周期与泛型和 trait bound
fn longest_with_an_announcement<'a, T>(
    x: &'a str,
    y: &'a str,
    ann: T,
) -> &'a str 
where
    T: std::fmt::Display,
{
    println!("公告！{}", ann);
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// ========== 实际应用结构 ==========

// 键值对解析器
struct Parser<'a> {
    input: &'a str,
    pos: usize,
}

struct KeyValue<'a> {
    key: &'a str,
    value: &'a str,
}

impl<'a> Parser<'a> {
    fn new(input: &'a str) -> Self {
        Parser { input, pos: 0 }
    }
    
    fn next_key_value(&mut self) -> Option<KeyValue<'a>> {
        if self.pos >= self.input.len() {
            return None;
        }
        
        let remaining = &self.input[self.pos..];
        
        // 查找下一个逗号或字符串结尾
        let end_pos = remaining.find(',').unwrap_or(remaining.len());
        let pair = &remaining[..end_pos];
        
        // 分割键值对
        if let Some(eq_pos) = pair.find('=') {
            let key = &pair[..eq_pos];
            let value = &pair[eq_pos + 1..];
            
            self.pos += end_pos + 1; // +1 for the comma
            
            Some(KeyValue { key, value })
        } else {
            None
        }
    }
}

// 字符串缓存
struct StringCache<'a> {
    data: &'a [String],
}

impl<'a> StringCache<'a> {
    fn new(data: &'a [String]) -> Self {
        StringCache { data }
    }
    
    fn get(&self, index: usize) -> Option<&str> {
        self.data.get(index).map(|s| s.as_str())
    }
    
    fn len(&self) -> usize {
        self.data.len()
    }
    
    fn print_all(&self) {
        for (i, item) in self.data.iter().enumerate() {
            println!("缓存[{}]: '{}'", i, item);
        }
    }
}

// 数字迭代器
struct NumberIterator<'a> {
    data: &'a [i32],
    index: usize,
}

impl<'a> NumberIterator<'a> {
    fn new(data: &'a [i32]) -> Self {
        NumberIterator { data, index: 0 }
    }
    
    fn next(&mut self) -> Option<&'a i32> {
        if self.index < self.data.len() {
            let result = &self.data[self.index];
            self.index += 1;
            Some(result)
        } else {
            None
        }
    }
}

// ========== 返回引用的函数 ==========

// 提取单词（返回原字符串的切片）
fn extract_words(text: &str) -> Vec<&str> {
    text.split_whitespace().collect()
}

// 按逗号分割
fn split_by_comma(text: &str) -> Vec<&str> {
    text.split(',').collect()
}

// 查找最长行
fn find_longest_line<'a>(lines: &[&'a str]) -> Option<&'a str> {
    lines.iter().max_by_key(|line| line.len()).copied()
}

// 获取文件扩展名
fn get_extension(filename: &str) -> Option<&str> {
    filename.rfind('.').map(|i| &filename[i + 1..])
}

// ========== 高级生命周期模式 ==========

// 生命周期与闭包
fn create_closure<'a>(s: &'a str) -> impl Fn() -> &'a str {
    move || s
}

// 条件返回不同生命周期的引用
fn conditional_return<'a, 'b>(
    condition: bool, 
    x: &'a str, 
    y: &'b str
) -> &'a str 
where 
    'b: 'a
{
    if condition {
        x
    } else {
        y  // 这里需要 'b: 'a 约束
    }
}

/* 
重要概念总结：

生命周期基础：
- 确保引用在其指向的数据有效期内有效
- 防止悬垂引用和内存安全问题
- 编译时检查，无运行时开销

生命周期语法：
- 'a, 'b, 'static 等生命周期参数
- &'a T 表示引用的生命周期
- 泛型语法：<'a, T>

生命周期省略规则：
1. 每个引用参数获得自己的生命周期
2. 如果只有一个输入生命周期，应用到所有输出
3. 如果有 &self 或 &mut self，其生命周期应用到所有输出

结构体生命周期：
- 结构体包含引用时需要生命周期参数
- 所有引用字段必须在结构体销毁前有效
- impl 块需要相同的生命周期参数

'static 生命周期：
- 引用在整个程序运行期间都有效
- 字符串字面值具有 'static 生命周期
- 不要轻易使用 'static

生命周期约束：
- where 'b: 'a 表示 'b 至少和 'a 一样长
- 生命周期子类型关系
- 与 trait bound 结合使用

最佳实践：
- 尽量依赖生命周期省略
- 使用有意义的生命周期参数名
- 避免过度复杂的生命周期关系
- 优先考虑拥有所有权的设计

编译运行：
cargo run --bin 14_lifetimes
*/
