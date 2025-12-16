# 📚 Rust 学习项目索引

## 🚀 快速导航

### 📖 学习顺序（推荐）
```bash
# 1. 环境配置与Hello World
cargo run --bin hello

# 2. 基础语法
cargo run --bin variables
cargo run --bin types
cargo run --bin functions
cargo run --bin control

# 3. 核心概念
cargo run --bin ownership
cargo run --bin references
cargo run --bin structs
cargo run --bin enums
cargo run --bin collections

# 4. 高级特性
cargo run --bin errors
cargo run --bin generics
cargo run --bin lifetimes
cargo run --bin modules
cargo run --bin async
cargo run --bin std_lib

# 5. 实战项目
cargo run --bin project_cli -- help
cargo run --bin project_web

# 6. 练习系统
cargo run --bin exercises -- all
```

## 📋 文件清单与功能

| 文件 | 编号 | 主题 | 关键概念 |
|------|------|------|----------|
| `00_hello.rs` | 00 | 环境配置 | println!, dbg!, 环境检查 |
| `01_variables.rs` | 01 | 变量与可变性 | let, mut, 常量, 作用域 |
| `02_types.rs` | 02 | 数据类型 | 标量类型, 复合类型, 字符串 |
| `03_functions.rs` | 03 | 函数 | 函数定义, 参数, 返回值 |
| `04_control.rs` | 04 | 控制流 | if/else, loop, while, for, match |
| `05_ownership.rs` | 05 | 所有权系统 | 所有权规则, 移动, 复制 |
| `06_references.rs` | 06 | 引用与切片 | 借用, 切片, 生命周期基础 |
| `07_structs.rs` | 07 | 结构体 | 定义, 方法, 关联函数 |
| `08_enums.rs` | 08 | 枚举 | 枚举定义, 模式匹配, Option/Result |
| `09_collections.rs` | 09 | 集合类型 | Vector, HashMap, 迭代器 |
| `10_errors.rs` | 10 | 错误处理 | Result, Option, panic!, 错误传播 |
| `11_generics.rs` | 11 | 泛型与Traits | 泛型函数, Trait定义, 约束 |
| `12_lifetimes.rs` | 12 | 生命周期 | 生命周期注解, 结构体生命周期 |
| `13_modules.rs` | 13 | 模块系统 | 模块, 包管理, 可见性 |
| `14_async.rs` | 14 | 异步编程 | async/await, Future, 并发 |
| `15_std.rs` | 15 | 标准库 | 常用模块, 工具函数, 宏 |

## 🛠️ 项目文件

| 文件 | 类型 | 描述 | 运行命令 |
|------|------|------|----------|
| `projects/cli.rs` | 命令行工具 | 文件处理CLI应用 | `cargo run --bin project_cli -- help` |
| `projects/web.rs` | Web服务器 | 简单HTTP服务器 | `cargo run --bin project_web` |

## 📝 练习系统

| 练习类型 | 命令 | 描述 |
|----------|------|------|
| 基础练习 | `cargo run --bin exercises -- basic` | 变量、类型、函数、控制流 |
| 所有权练习 | `cargo run --bin exercises -- ownership` | 所有权、借用系统 |
| 泛型练习 | `cargo run --bin exercises -- generics` | 泛型、Traits、生命周期 |
| 异步练习 | `cargo run --bin exercises -- async` | async/await、并发 |
| 全部练习 | `cargo run --bin exercises -- all` | 运行所有练习 |

## 📚 文档资源

| 文档 | 路径 | 内容 |
|------|------|------|
| **学习指南** | `docs/LEARNING_GUIDE.md` | 详细的学习路线和方法 |
| **项目总结** | `docs/PROJECT_SUMMARY.md` | 项目优化总结 |
| **Claude指南** | `CLAUDE.md` | Claude使用指南 |
| **项目介绍** | `README.md` | 项目概述和特色 |

## 🎯 学习目标检查

### ✅ 基础阶段完成后应该掌握：
- [ ] Rust 基本语法和数据类型
- [ ] 变量、函数和控制流的使用
- [ ] 理解所有权的基本概念

### ✅ 核心概念阶段完成后应该掌握：
- [ ] 深入理解所有权和借用系统
- [ ] 结构体和枚举的使用
- [ ] 模式匹配的技巧

### ✅ 高级特性阶段完成后应该掌握：
- [ ] 错误处理的最佳实践
- [ ] 泛型编程和Trait系统
- [ ] 生命周期的正确使用
- [ ] 异步编程基础

### ✅ 实战项目完成后应该具备：
- [ ] 独立开发Rust项目的能力
- [ ] 解决实际问题的编程技巧
- [ ] 代码组织和模块化设计能力

## 🔗 外部资源

- [Rust官方文档](https://doc.rust-lang.org/)
- [Rust by Example](https://doc.rust-lang.org/rust-by-example/)
- [Rust标准库](https://doc.rust-lang.org/std/)
- [Rust Playground](https://play.rust-lang.org/)

---

**开始学习：`cargo run --bin hello`** 🦀