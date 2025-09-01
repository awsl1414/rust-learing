// 07_structs_and_methods.rs - 结构体和方法
// 本文件展示 Rust 中结构体的定义、实例化和方法的使用

fn main() {
    println!("=== Rust 结构体和方法 ===\n");
    
    // ========== 结构体基础 ==========
    println!("========== 结构体基础 ==========");
    
    // 1. 基本结构体定义和使用
    println!("\n1. 基本结构体：");
    
    let user1 = User {
        email: String::from("user@example.com"),
        username: String::from("alice"),
        active: true,
        sign_in_count: 1,
    };
    
    println!("用户信息:");
    println!("  邮箱: {}", user1.email);
    println!("  用户名: {}", user1.username);
    println!("  活跃状态: {}", user1.active);
    println!("  登录次数: {}", user1.sign_in_count);
    
    // 2. 可变结构体
    println!("\n2. 可变结构体：");
    let mut user2 = User {
        email: String::from("bob@example.com"),
        username: String::from("bob"),
        active: true,
        sign_in_count: 1,
    };
    
    user2.email = String::from("bob_new@example.com");
    user2.sign_in_count += 1;
    
    println!("修改后的用户信息:");
    println!("  新邮箱: {}", user2.email);
    println!("  登录次数: {}", user2.sign_in_count);
    
    // 3. 结构体构造函数
    println!("\n3. 结构体构造函数：");
    let user3 = build_user(String::from("charlie@example.com"), String::from("charlie"));
    println!("通过函数创建的用户: {}", user3.username);
    
    // 4. 结构体更新语法
    println!("\n4. 结构体更新语法：");
    let user4 = User {
        email: String::from("david@example.com"),
        username: String::from("david"),
        ..user1 // 使用 user1 的其他字段值
    };
    
    println!("使用更新语法的用户:");
    println!("  邮箱: {}", user4.email);
    println!("  活跃状态: {}", user4.active);
    
    // ========== 元组结构体 ==========
    println!("\n========== 元组结构体 ==========");
    
    // 5. 元组结构体
    println!("\n5. 元组结构体：");
    let black = Color(0, 0, 0);
    let white = Color(255, 255, 255);
    let origin = Point(0, 0, 0);
    
    println!("黑色: RGB({}, {}, {})", black.0, black.1, black.2);
    println!("白色: RGB({}, {}, {})", white.0, white.1, white.2);
    println!("原点: ({}, {}, {})", origin.0, origin.1, origin.2);
    
    // ========== 单元结构体 ==========
    println!("\n========== 单元结构体 ==========");
    
    // 6. 单元结构体
    println!("\n6. 单元结构体：");
    let subject = AlwaysEqual;
    println!("单元结构体实例创建成功");
    
    // ========== 结构体方法 ==========
    println!("\n========== 结构体方法 ==========");
    
    // 7. 定义和使用方法
    println!("\n7. 结构体方法：");
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };
    
    println!("矩形信息:");
    println!("  宽度: {}", rect1.width);
    println!("  高度: {}", rect1.height);
    println!("  面积: {}", rect1.area());
    println!("  周长: {}", rect1.perimeter());
    
    // 方法链调用
    let rect2 = Rectangle::square(25);
    println!("正方形面积: {}", rect2.area());
    
    // 8. 方法参数
    println!("\n8. 方法参数：");
    let rect3 = Rectangle {
        width: 20,
        height: 40,
    };
    
    println!("rect1 能容纳 rect3 吗? {}", rect1.can_hold(&rect3));
    println!("rect3 能容纳 rect1 吗? {}", rect3.can_hold(&rect1));
    
    // ========== 关联函数 ==========
    println!("\n========== 关联函数 ==========");
    
    // 9. 关联函数（静态方法）
    println!("\n9. 关联函数：");
    let square = Rectangle::square(10);
    println!("正方形: {}×{}", square.width, square.height);
    
    let rect_from_area = Rectangle::from_area(100);
    println!("从面积创建的矩形: {}×{}", rect_from_area.width, rect_from_area.height);
    
    // ========== 复杂结构体示例 ==========
    println!("\n========== 复杂结构体示例 ==========");
    
    // 10. 包含其他结构体的结构体
    println!("\n10. 复杂结构体：");
    let person = Person {
        name: String::from("张三"),
        age: 30,
        address: Address {
            street: String::from("中山路123号"),
            city: String::from("北京"),
            zip_code: String::from("100000"),
        },
    };
    
    person.display_info();
    
    // 11. 结构体中的向量
    println!("\n11. 包含集合的结构体：");
    let mut library = Library {
        name: String::from("中央图书馆"),
        books: Vec::new(),
    };
    
    library.add_book(Book {
        title: String::from("Rust程序设计语言"),
        author: String::from("Steve Klabnik"),
        isbn: String::from("978-7-121-31166-1"),
    });
    
    library.add_book(Book {
        title: String::from("深入理解计算机系统"),
        author: String::from("Randal E. Bryant"),
        isbn: String::from("978-7-111-54493-7"),
    });
    
    library.display_books();
    
    // 12. 结构体的调试输出
    println!("\n12. 结构体调试输出：");
    let debug_rect = DebugRectangle {
        width: 30,
        height: 50,
    };
    
    println!("普通输出: {}", debug_rect);
    println!("调试输出: {:?}", debug_rect);
    println!("美化调试输出: {:#?}", debug_rect);
    
    println!("\n=== 结构体和方法学习完成 ===");
}

// ========== 基本结构体定义 ==========

#[derive(Debug)]
struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

// 元组结构体
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// 单元结构体
struct AlwaysEqual;

// ========== 带方法的结构体 ==========

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

// 为 Rectangle 实现方法
impl Rectangle {
    // 方法：计算面积
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    // 方法：计算周长
    fn perimeter(&self) -> u32 {
        2 * (self.width + self.height)
    }
    
    // 方法：判断是否能容纳另一个矩形
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
    
    // 关联函数：创建正方形
    fn square(size: u32) -> Rectangle {
        Rectangle {
            width: size,
            height: size,
        }
    }
    
    // 关联函数：从面积创建正方形
    fn from_area(area: u32) -> Rectangle {
        let side = (area as f64).sqrt() as u32;
        Rectangle::square(side)
    }
    
    // 方法：修改尺寸
    fn resize(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
    
    // 方法：判断是否为正方形
    fn is_square(&self) -> bool {
        self.width == self.height
    }
}

// ========== 复杂结构体 ==========

#[derive(Debug)]
struct Address {
    street: String,
    city: String,
    zip_code: String,
}

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
    address: Address,
}

impl Person {
    // 构造函数
    fn new(name: String, age: u32, address: Address) -> Person {
        Person { name, age, address }
    }
    
    // 显示信息的方法
    fn display_info(&self) {
        println!("个人信息:");
        println!("  姓名: {}", self.name);
        println!("  年龄: {}", self.age);
        println!("  地址: {}, {}, {}", 
                 self.address.street, 
                 self.address.city, 
                 self.address.zip_code);
    }
    
    // 年龄增长
    fn have_birthday(&mut self) {
        self.age += 1;
    }
    
    // 搬家
    fn move_to(&mut self, new_address: Address) {
        self.address = new_address;
    }
}

// ========== 图书馆示例 ==========

#[derive(Debug)]
struct Book {
    title: String,
    author: String,
    isbn: String,
}

#[derive(Debug)]
struct Library {
    name: String,
    books: Vec<Book>,
}

impl Library {
    // 新建图书馆
    fn new(name: String) -> Library {
        Library {
            name,
            books: Vec::new(),
        }
    }
    
    // 添加书籍
    fn add_book(&mut self, book: Book) {
        self.books.push(book);
    }
    
    // 显示所有书籍
    fn display_books(&self) {
        println!("{}的藏书:", self.name);
        for (index, book) in self.books.iter().enumerate() {
            println!("  {}. 《{}》 - {}", index + 1, book.title, book.author);
        }
    }
    
    // 查找书籍
    fn find_book(&self, title: &str) -> Option<&Book> {
        self.books.iter().find(|book| book.title == title)
    }
    
    // 书籍数量
    fn book_count(&self) -> usize {
        self.books.len()
    }
}

// ========== 实现Display trait的结构体 ==========

use std::fmt;

#[derive(Debug)]
struct DebugRectangle {
    width: u32,
    height: u32,
}

impl fmt::Display for DebugRectangle {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "矩形[宽: {}, 高: {}]", self.width, self.height)
    }
}

// ========== 辅助函数 ==========

// 构造用户的函数
fn build_user(email: String, username: String) -> User {
    User {
        email,     // 字段初始化简写
        username,  // 字段初始化简写
        active: true,
        sign_in_count: 1,
    }
}

/* 
重要概念总结：

结构体类型：
1. 经典结构体：具名字段，如 struct User { name: String }
2. 元组结构体：匿名字段，如 struct Color(i32, i32, i32)
3. 单元结构体：无字段，如 struct AlwaysEqual

结构体特性：
- 所有字段必须有值才能创建实例
- 结构体实例的可变性是整体性的
- 可以使用结构体更新语法复用字段值

方法系统：
- impl 块为结构体定义方法
- &self：不可变借用，用于读取
- &mut self：可变借用，用于修改
- self：获取所有权，消费实例

关联函数：
- 不以 self 为第一参数的函数
- 使用 :: 语法调用，如 Rectangle::square(5)
- 通常用作构造函数

派生宏：
- #[derive(Debug)]：自动实现调试输出
- #[derive(Clone)]：自动实现克隆
- #[derive(PartialEq)]：自动实现相等比较

最佳实践：
- 使用结构体组织相关数据
- 为结构体实现有意义的方法
- 合理使用借用避免不必要的所有权转移
- 使用关联函数作为构造器

编译运行：
cargo run --bin 07_structs_and_methods
*/
