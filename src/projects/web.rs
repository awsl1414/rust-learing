// projects/web.rs - Web API 项目
// 简单的 HTTP 服务器示例

use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};

fn main() {
    println!("🚀 启动简单的 Web 服务器");
    println!("========================");

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    println!("服务器运行在 http://127.0.0.1:7878");

    for stream in listener.incoming() {
        let stream = stream.unwrap();
        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let response = "HTTP/1.1 200 OK\r\n\r\nHello from Rust!";

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
