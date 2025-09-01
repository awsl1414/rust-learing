// 16_async_programming.rs - 异步编程与并发
// 本文件展示 Rust 中的异步编程概念和并发编程技术

use std::time::Duration;
use tokio::time::sleep;

// 异步主函数
#[tokio::main]
async fn main() {
    println!("=== Rust 异步编程与并发 ===\n");
    
    // ========== 基础异步概念 ==========
    println!("========== 基础异步概念 ==========");
    
    // 1. 简单异步函数
    println!("\n1. 基础异步函数：");
    
    let result = simple_async_function().await;
    println!("异步函数结果: {}", result);
    
    // 2. 异步延迟
    println!("\n2. 异步延迟：");
    
    println!("开始计时...");
    let start = std::time::Instant::now();
    
    delay_async(1000).await;
    
    let elapsed = start.elapsed();
    println!("异步延迟完成，耗时: {:?}", elapsed);
    
    // ========== 并发执行 ==========
    println!("\n========== 并发执行 ==========");
    
    // 3. 同时执行多个异步任务
    println!("\n3. 并发执行多个任务：");
    
    let start = std::time::Instant::now();
    
    // 使用 tokio::join! 同时执行
    let (result1, result2, result3) = tokio::join!(
        async_task("任务A", 1000),
        async_task("任务B", 1500),
        async_task("任务C", 800)
    );
    
    let elapsed = start.elapsed();
    println!("所有任务完成:");
    println!("  {}", result1);
    println!("  {}", result2);
    println!("  {}", result3);
    println!("总耗时: {:?}", elapsed);
    
    // 4. 选择执行（竞态）
    println!("\n4. 竞态执行 - 第一个完成的获胜：");
    
    let start = std::time::Instant::now();
    
    let winner = tokio::select! {
        result = async_task("快速任务", 500) => {
            format!("获胜者: {}", result)
        }
        result = async_task("慢速任务", 2000) => {
            format!("获胜者: {}", result)
        }
        result = async_task("中等任务", 1000) => {
            format!("获胜者: {}", result)
        }
    };
    
    let elapsed = start.elapsed();
    println!("{}", winner);
    println!("竞态耗时: {:?}", elapsed);
    
    // ========== 异步流和通道 ==========
    println!("\n========== 异步流和通道 ==========");
    
    // 5. 异步通道
    println!("\n5. 异步通道通信：");
    
    let (sender, mut receiver) = tokio::sync::mpsc::channel::<String>(10);
    
    // 启动发送者任务
    let sender_handle = tokio::spawn(async move {
        for i in 1..=5 {
            let message = format!("消息 {}", i);
            if sender.send(message.clone()).await.is_ok() {
                println!("发送: {}", message);
                sleep(Duration::from_millis(300)).await;
            }
        }
        println!("发送者完成");
    });
    
    // 启动接收者任务
    let receiver_handle = tokio::spawn(async move {
        while let Some(message) = receiver.recv().await {
            println!("接收: {}", message);
            sleep(Duration::from_millis(100)).await;
        }
        println!("接收者完成");
    });
    
    // 等待两个任务完成
    let _ = tokio::join!(sender_handle, receiver_handle);
    
    // ========== 任务管理 ==========
    println!("\n========== 任务管理 ==========");
    
    // 6. 任务句柄
    println!("\n6. 任务句柄管理：");
    
    let mut handles = Vec::new();
    
    // 创建多个任务
    for i in 1..=3 {
        let handle = tokio::spawn(async move {
            let delay = i * 500;
            sleep(Duration::from_millis(delay)).await;
            format!("工作任务 {} 完成 (延迟 {}ms)", i, delay)
        });
        handles.push(handle);
    }
    
    // 等待所有任务完成
    for (i, handle) in handles.into_iter().enumerate() {
        match handle.await {
            Ok(result) => println!("任务 {} 结果: {}", i + 1, result),
            Err(e) => println!("任务 {} 错误: {}", i + 1, e),
        }
    }
    
    // ========== 超时控制 ==========
    println!("\n========== 超时控制 ==========");
    
    // 7. 超时处理
    println!("\n7. 超时控制：");
    
    // 正常完成的任务
    match tokio::time::timeout(
        Duration::from_millis(1500),
        async_task("正常任务", 1000)
    ).await {
        Ok(result) => println!("任务成功完成: {}", result),
        Err(_) => println!("任务超时"),
    }
    
    // 超时的任务
    match tokio::time::timeout(
        Duration::from_millis(500),
        async_task("慢速任务", 1000)
    ).await {
        Ok(result) => println!("任务成功完成: {}", result),
        Err(_) => println!("任务超时（预期的）"),
    }
    
    // ========== 异步互斥锁 ==========
    println!("\n========== 异步互斥锁 ==========");
    
    // 8. 共享状态
    println!("\n8. 异步互斥锁：");
    
    use std::sync::Arc;
    use tokio::sync::Mutex;
    
    let counter = Arc::new(Mutex::new(0));
    let mut handles = Vec::new();
    
    // 创建多个任务共享计数器
    for i in 1..=5 {
        let counter_clone = Arc::clone(&counter);
        let handle = tokio::spawn(async move {
            for _ in 0..3 {
                let mut num = counter_clone.lock().await;
                *num += 1;
                println!("任务 {} 增加计数器: {}", i, *num);
                drop(num); // 显式释放锁
                sleep(Duration::from_millis(100)).await;
            }
        });
        handles.push(handle);
    }
    
    // 等待所有任务完成
    for handle in handles {
        let _ = handle.await;
    }
    
    let final_count = *counter.lock().await;
    println!("最终计数器值: {}", final_count);
    
    // ========== 异步流处理 ==========
    println!("\n========== 异步流处理 ==========");
    
    // 9. 异步迭代器
    println!("\n9. 异步流：");
    
    use tokio_stream::{self as stream, StreamExt};
    
    // 创建数字流
    let number_stream = stream::iter(1..=5)
        .then(|n| async move {
            sleep(Duration::from_millis(200)).await;
            n * n // 计算平方
        });
    
    tokio::pin!(number_stream);
    
    println!("处理数字流:");
    while let Some(square) = number_stream.next().await {
        println!("平方值: {}", square);
    }
    
    // ========== 错误处理 ==========
    println!("\n========== 异步错误处理 ==========");
    
    // 10. 异步错误处理
    println!("\n10. 异步错误处理：");
    
    // 可能失败的异步函数
    let results = vec![
        fallible_async_task(1, false).await,
        fallible_async_task(2, true).await,
        fallible_async_task(3, false).await,
    ];
    
    for (i, result) in results.into_iter().enumerate() {
        match result {
            Ok(value) => println!("任务 {} 成功: {}", i + 1, value),
            Err(e) => println!("任务 {} 失败: {}", i + 1, e),
        }
    }
    
    // ========== 实际应用示例 ==========
    println!("\n========== 实际应用示例 ==========");
    
    // 11. 并发HTTP请求模拟
    println!("\n11. 模拟并发网络请求：");
    
    let urls = vec![
        "https://api.example.com/users",
        "https://api.example.com/posts",
        "https://api.example.com/comments",
    ];
    
    let start = std::time::Instant::now();
    
    let mut handles = Vec::new();
    for (i, url) in urls.into_iter().enumerate() {
        let handle = tokio::spawn(async move {
            simulate_http_request(url, 200 + i * 100).await
        });
        handles.push(handle);
    }
    
    let mut responses = Vec::new();
    for handle in handles {
        match handle.await {
            Ok(response) => responses.push(response),
            Err(e) => println!("请求失败: {}", e),
        }
    }
    
    let elapsed = start.elapsed();
    println!("所有HTTP请求完成:");
    for response in responses {
        println!("  {}", response);
    }
    println!("总耗时: {:?}", elapsed);
    
    // ========== 资源管理 ==========
    println!("\n========== 异步资源管理 ==========");
    
    // 12. 异步资源池
    println!("\n12. 连接池模拟：");
    
    let pool = Arc::new(Mutex::new(ConnectionPool::new(3)));
    let mut handles = Vec::new();
    
    for i in 1..=5 {
        let pool_clone = Arc::clone(&pool);
        let handle = tokio::spawn(async move {
            use_connection_from_pool(pool_clone, i).await
        });
        handles.push(handle);
    }
    
    for handle in handles {
        let _ = handle.await;
    }
    
    println!("\n=== 异步编程与并发学习完成 ===");
}

// ========== 异步函数定义 ==========

// 简单异步函数
async fn simple_async_function() -> String {
    sleep(Duration::from_millis(500)).await;
    "异步函数执行完成".to_string()
}

// 异步延迟函数
async fn delay_async(millis: u64) {
    println!("开始延迟 {} 毫秒", millis);
    sleep(Duration::from_millis(millis)).await;
    println!("延迟完成");
}

// 异步任务函数
async fn async_task(name: &str, delay_ms: u64) -> String {
    println!("  {} 开始执行", name);
    sleep(Duration::from_millis(delay_ms)).await;
    let result = format!("{} 完成 ({}ms)", name, delay_ms);
    println!("  {}", result);
    result
}

// 可能失败的异步任务
async fn fallible_async_task(id: u32, should_fail: bool) -> Result<String, String> {
    sleep(Duration::from_millis(300)).await;
    
    if should_fail {
        Err(format!("任务 {} 故意失败", id))
    } else {
        Ok(format!("任务 {} 成功完成", id))
    }
}

// 模拟HTTP请求
async fn simulate_http_request(url: &str, delay_ms: usize) -> String {
    println!("  发起请求: {}", url);
    sleep(Duration::from_millis(delay_ms as u64)).await;
    
    // 模拟响应
    let response = format!("响应来自 {} ({}ms)", url, delay_ms);
    println!("  {}", response);
    response
}

// ========== 连接池示例 ==========

struct ConnectionPool {
    connections: Vec<Connection>,
    available: usize,
}

#[derive(Clone)]
struct Connection {
    id: usize,
}

impl ConnectionPool {
    fn new(size: usize) -> Self {
        let connections = (1..=size)
            .map(|id| Connection { id })
            .collect();
        
        Self {
            connections,
            available: size,
        }
    }
    
    async fn get_connection(&mut self) -> Option<Connection> {
        if self.available > 0 {
            self.available -= 1;
            Some(self.connections[self.available].clone())
        } else {
            None
        }
    }
    
    async fn return_connection(&mut self, _conn: Connection) {
        if self.available < self.connections.len() {
            self.available += 1;
        }
    }
}

async fn use_connection_from_pool(
    pool: std::sync::Arc<tokio::sync::Mutex<ConnectionPool>>,
    user_id: usize,
) {
    println!("用户 {} 请求连接", user_id);
    
    // 尝试获取连接
    let connection = {
        let mut pool_guard = pool.lock().await;
        pool_guard.get_connection().await
    };
    
    match connection {
        Some(conn) => {
            println!("用户 {} 获得连接 {}", user_id, conn.id);
            
            // 模拟使用连接
            sleep(Duration::from_millis(500)).await;
            println!("用户 {} 使用连接 {} 完成工作", user_id, conn.id);
            
            // 归还连接
            let mut pool_guard = pool.lock().await;
            pool_guard.return_connection(conn).await;
            println!("用户 {} 归还连接", user_id);
        }
        None => {
            println!("用户 {} 无法获取连接，等待中...", user_id);
            sleep(Duration::from_millis(100)).await;
            
            // 递归重试（在实际应用中可能需要更智能的重试策略）
            Box::pin(use_connection_from_pool(pool, user_id)).await;
        }
    }
}

/* 
重要概念总结：

异步编程基础：
- async/await：异步函数定义和等待
- Future trait：异步计算的核心抽象
- 异步运行时：tokio 提供异步执行环境

并发执行：
- tokio::join!：同时执行多个异步任务
- tokio::select!：竞态执行，第一个完成获胜
- 并发 vs 并行：理解概念差异

异步通信：
- mpsc channel：多生产者单消费者通道
- 异步消息传递：跨任务通信
- 背压控制：通道容量管理

任务管理：
- tokio::spawn：创建异步任务
- JoinHandle：任务句柄和结果获取
- 任务取消：处理未完成的任务

超时控制：
- tokio::time::timeout：为异步操作设置超时
- 优雅降级：超时后的处理策略
- 资源清理：避免资源泄漏

同步原语：
- Arc<Mutex<T>>：异步共享状态
- RwLock：读写锁
- Semaphore：信号量控制并发数

异步流：
- Stream trait：异步迭代器
- 流处理：map, filter, collect
- 背压处理：流控制

错误处理：
- Result 在异步上下文中的使用
- 错误传播：? 操作符
- 异步错误恢复策略

实际应用：
- HTTP 客户端：并发请求
- 数据库连接池：资源管理
- WebSocket：实时通信
- 文件 I/O：异步读写

性能考虑：
- 避免阻塞：使用异步版本的操作
- 任务粒度：合理划分异步任务
- 内存使用：异步状态机的开销

最佳实践：
- 合理使用 tokio::spawn：避免过度创建任务
- 正确处理取消：使用 CancellationToken
- 资源清理：Drop trait 在异步中的使用
- 测试异步代码：使用 tokio::test

常见陷阱：
- 阻塞调用：在异步上下文中调用阻塞函数
- 生命周期：异步函数中的借用检查
- 死锁：异步锁的使用注意事项

编译运行：
cargo run --bin 16_async_programming

依赖要求：
tokio = { version = "1.0", features = ["full"] }
tokio-stream = "0.1"
*/
