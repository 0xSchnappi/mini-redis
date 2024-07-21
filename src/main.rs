/*
 * @Author: 0xSchnappi 952768182@qq.com
 * @Date: 2024-07-21 16:58:33
 * @LastEditors: 0xSchnappi 952768182@qq.com
 * @LastEditTime: 2024-07-21 17:19:50
 * @FilePath: /mini-redis/src/main.rs
 * @Description: redis服务端
 *
 * Copyright (c) 2024 by github.com/0xSchnappi, All Rights Reserved.
 */
use mini_redis::{Connection, Frame};
use tokio::net::{TcpListener, TcpStream};

#[tokio::main]
async fn main() {
    // 监听指定地址，等待TCP连接进来
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();

    loop {
        // 第二个被忽略的项中包含有新连接的`IP`和端口信息
        let (socket, _) = listener.accept().await.unwrap();
        process(socket).await;
    }
}

async fn process(socket: TcpStream) {
    // `Connection` 对于 redis 的读写进行了抽象封装，因此我们读到的是一个一个数据帧frame(数据帧 = redis命令 + 数据)，而不是字节流
    // `Connection` 是在mini-redis中定义
    let mut connection = Connection::new(socket);

    if let Some(frame) = connection.read_frame().await.unwrap() {
        println!("GOT: {:?}", frame);

        // 回复一个错误
        let response = Frame::Error("Unimplemented".to_string());
        connection.write_frame(&response).await.unwrap();
    }
}
