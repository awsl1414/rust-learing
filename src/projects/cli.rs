// projects/cli.rs - 迷你项目：文件处理命令行工具
// 这是一个综合性的实战项目，展示如何使用 Rust 构建实用的命令行工具
// 本项目将使用多个已学过的概念：所有权、错误处理、结构体、枚举等

use std::env;
use std::fs;
use std::path::Path;
use std::process;

// 命令枚举 - 定义所有支持的命令
#[derive(Debug, PartialEq)]
enum Command {
    Help,
    Count { file: String },
    Info { file: String },
    Find { file: String, pattern: String },
    List { directory: String },
    Create { file: String, content: String },
}

// 文件信息结构体
#[derive(Debug)]
struct FileInfo {
    name: String,
    size: u64,
    is_file: bool,
    line_count: Option<usize>,
}

// 结果类型别名，简化错误处理
type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

fn main() {
    println!("🦀 Rust 文件处理工具 v1.0");
    println!("================================");

    // 解析命令行参数
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        show_help();
        return;
    }

    match parse_command(&args) {
        Ok(command) => {
            if let Err(e) = execute_command(command) {
                eprintln!("❌ 错误: {}", e);
                process::exit(1);
            }
        }
        Err(e) => {
            eprintln!("❌ 参数解析错误: {}", e);
            show_help();
            process::exit(1);
        }
    }
}

// 解析命令行参数
fn parse_command(args: &[String]) -> Result<Command> {
    match args[1].as_str() {
        "help" | "--help" | "-h" => Ok(Command::Help),

        "count" => {
            if args.len() != 3 {
                return Err("count 命令需要一个文件参数".into());
            }
            Ok(Command::Count {
                file: args[2].clone(),
            })
        }

        "info" => {
            if args.len() != 3 {
                return Err("info 命令需要一个文件参数".into());
            }
            Ok(Command::Info {
                file: args[2].clone(),
            })
        }

        "find" => {
            if args.len() != 4 {
                return Err("find 命令需要文件和搜索模式参数".into());
            }
            Ok(Command::Find {
                file: args[2].clone(),
                pattern: args[3].clone(),
            })
        }

        "list" => {
            let dir = if args.len() == 3 {
                args[2].clone()
            } else {
                ".".to_string() // 默认当前目录
            };
            Ok(Command::List { directory: dir })
        }

        "create" => {
            if args.len() < 4 {
                return Err("create 命令需要文件和内容参数".into());
            }
            let content = args[3..].join(" ");
            Ok(Command::Create {
                file: args[2].clone(),
                content,
            })
        }

        _ => Err(format!("未知命令: {}", args[1]).into()),
    }
}

// 执行命令
fn execute_command(command: Command) -> Result<()> {
    match command {
        Command::Help => {
            show_help();
            Ok(())
        }

        Command::Count { file } => {
            count_lines(&file)?;
            Ok(())
        }

        Command::Info { file } => {
            show_file_info(&file)?;
            Ok(())
        }

        Command::Find { file, pattern } => {
            find_pattern(&file, &pattern)?;
            Ok(())
        }

        Command::List { directory } => {
            list_directory(&directory)?;
            Ok(())
        }

        Command::Create { file, content } => {
            create_file(&file, &content)?;
            Ok(())
        }
    }
}

// 显示帮助信息
fn show_help() {
    println!("📖 使用方法:");
    println!("  cargo run --bin project_cli -- <命令> [参数...]");
    println!();
    println!("🔧 可用命令:");
    println!("  help                              - 显示此帮助信息");
    println!("  count <文件>                      - 统计文件行数");
    println!("  info <文件>                       - 显示文件详细信息");
    println!("  find <文件> <模式>                - 在文件中查找模式");
    println!("  list [目录]                       - 列出目录内容");
    println!("  create <文件> <内容>...           - 创建新文件");
    println!();
    println!("💡 示例:");
    println!("  cargo run --bin project_cli -- info Cargo.toml");
    println!("  cargo run --bin project_cli -- find Cargo.toml \"edition\"");
    println!("  cargo run --bin project_cli -- create hello.txt \"Hello, Rust!\"");
}

// 统计文件行数
fn count_lines(filename: &str) -> Result<()> {
    println!("📊 统计文件行数: {}", filename);

    let content = fs::read_to_string(filename)?;
    let line_count = content.lines().count();
    let char_count = content.chars().count();
    let word_count = content.split_whitespace().count();

    println!("   行数: {}", line_count);
    println!("   字符数: {}", char_count);
    println!("   单词数: {}", word_count);

    Ok(())
}

// 显示文件信息
fn show_file_info(filename: &str) -> Result<()> {
    println!("📋 文件信息: {}", filename);

    let path = Path::new(filename);
    let metadata = fs::metadata(path)?;

    let line_count = if path.is_file()
        && path
            .extension()
            .map_or(false, |ext| ext == "txt" || ext == "rs")
    {
        let content = fs::read_to_string(filename)?;
        Some(content.lines().count())
    } else {
        None
    };

    let file_info = FileInfo {
        name: path
            .file_name()
            .and_then(|name| name.to_str())
            .unwrap_or(filename)
            .to_string(),
        size: metadata.len(),
        is_file: path.is_file(),
        line_count,
    };

    println!("   名称: {}", file_info.name);
    println!("   大小: {} 字节", file_info.size);
    println!(
        "   类型: {}",
        if file_info.is_file {
            "文件"
        } else {
            "目录"
        }
    );

    if let Some(lines) = file_info.line_count {
        println!("   行数: {}", lines);
    }

    if let Some(extension) = path.extension() {
        println!("   扩展名: {}", extension.to_string_lossy());
    }

    Ok(())
}

// 在文件中查找模式
fn find_pattern(filename: &str, pattern: &str) -> Result<()> {
    println!("🔍 在文件 {} 中查找 '{}'", filename, pattern);

    let content = fs::read_to_string(filename)?;
    let lines: Vec<&str> = content.lines().collect();
    let mut matches = Vec::new();

    for (line_num, line) in lines.iter().enumerate() {
        if line.contains(pattern) {
            matches.push((line_num + 1, line));
        }
    }

    if matches.is_empty() {
        println!("   没有找到匹配的内容");
    } else {
        println!("   找到 {} 处匹配:", matches.len());
        for (line_num, line) in matches {
            println!("   行 {}: {}", line_num, line.trim());
        }
    }

    Ok(())
}

// 列出目录内容
fn list_directory(directory: &str) -> Result<()> {
    println!("📁 目录内容: {}", directory);

    let path = Path::new(directory);

    if !path.exists() {
        return Err(format!("目录不存在: {}", directory).into());
    }

    if !path.is_dir() {
        return Err(format!("{} 不是一个目录", directory).into());
    }

    let entries = fs::read_dir(path)?;
    let mut files = Vec::new();
    let mut dirs = Vec::new();

    for entry in entries {
        let entry = entry?;
        let metadata = entry.metadata()?;

        if metadata.is_dir() {
            dirs.push(entry.file_name().to_string_lossy().to_string());
        } else {
            files.push(entry.file_name().to_string_lossy().to_string());
        }
    }

    dirs.sort();
    files.sort();

    println!("📂 目录 ({} 个):", dirs.len());
    for dir in dirs {
        println!("   📁 {}/", dir);
    }

    println!("📄 文件 ({} 个):", files.len());
    for file in files {
        println!("   📄 {}", file);
    }

    Ok(())
}

// 创建文件
fn create_file(filename: &str, content: &str) -> Result<()> {
    println!("✏️ 创建文件: {}", filename);

    // 检查文件是否已存在
    if Path::new(filename).exists() {
        println!("⚠️ 警告: 文件已存在，将被覆盖");
    }

    fs::write(filename, content)?;
    println!("✅ 文件创建成功");

    // 验证文件
    let created_content = fs::read_to_string(filename)?;
    if created_content == content {
        println!("✅ 文件内容验证成功");
    } else {
        println!("❌ 文件内容验证失败");
    }

    Ok(())
}

/*
🎯 项目说明：

这个命令行工具项目综合了多个 Rust 概念：

1. **结构体和枚举**：
   - Command 枚举定义所有支持的命令
   - FileInfo 结构体存储文件信息

2. **错误处理**：
   - 使用 Result<T> 进行错误传播
   - 自定义错误类型和错误消息

3. **所有权和借用**：
   - 正确处理字符串的所有权
   - 使用引用避免不必要的克隆

4. **模式匹配**：
   - match 表达式处理不同命令
   - if let 处理可选值

5. **标准库使用**：
   - fs 模块进行文件操作
   - env 模块处理命令行参数
   - path 模块处理文件路径

6. **模块化设计**：
   - 函数职责单一
   - 清晰的代码组织

🚀 扩展建议：
1. 添加更多命令（复制、移动、删除等）
2. 支持正则表达式搜索
3. 添加文件权限管理
4. 实现配置文件支持
5. 添加颜色输出
6. 支持批量操作

💡 学习要点：
- 如何设计命令行程序架构
- 实际项目中的错误处理模式
- Rust 标准库的实际应用
- 代码组织和模块化

🔗 相关概念：
- 枚举和模式匹配 (08_enums.rs)
- 错误处理 (10_errors.rs)
- 结构体和方法 (07_structs.rs)
- 所有权和借用 (05_ownership.rs)
- 模块系统 (13_modules.rs)
*/
