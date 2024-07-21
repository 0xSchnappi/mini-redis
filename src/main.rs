/*
 * @Author: 0xSchnappi 952768182@qq.com
 * @Date: 2024-07-21 16:58:33
 * @LastEditors: 0xSchnappi 952768182@qq.com
 * @LastEditTime: 2024-07-21 20:47:37
 * @FilePath: /mini-redis/src/main.rs
 * @Description: redis服务端
 *
 * Copyright (c) 2024 by github.com/0xSchnappi, All Rights Reserved.
 */
use mini_redis::{Connection, Frame};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio::net::{TcpListener, TcpStream};

use bytes::Bytes;
type Db = Arc<Mutex<HashMap<String, Bytes>>>;

#[tokio::main]
async fn main() {
    // 监听指定地址，等待TCP连接进来
    let listener = TcpListener::bind("127.0.0.1:6379").await.unwrap();
    println!("Listening");

    let db = Arc::new(Mutex::new(HashMap::new()));

    loop {
        // 第二个被忽略的项中包含有新连接的`IP`和端口信息
        let (socket, _) = listener.accept().await.unwrap();

        // 将handle 克隆一份
        let db = db.clone();

        // 为每一条连接都生成一个新任务
        // `socket`的所有权将被移动到新的任务中，并在那里进行处理
        tokio::spawn(async move {
            process(socket, db).await;
        });
    }

    // let handle = tokio::spawn(async { 10086 });
    // let out = handle.await.unwrap();
    // println!("GOT {}", out);
}

async fn process(socket: TcpStream, db: Db) {
    use mini_redis::Command::{self, Get, Set};

    // `Connection` 对于 redis 的读写进行了抽象封装，因此我们读到的是一个一个数据帧frame(数据帧 = redis命令 + 数据)，而不是字节流
    // `Connection` 是在mini-redis中定义
    // `mini-redis` 提供的遍历函数，使用返回的`connection` 可以用于从socket中读取并解析为数据帧
    let mut connection = Connection::new(socket);

    // 使用`read_frame`方法从连接获取一个数据帧： 一条redis命令 + 相应的数据
    while let Some(frame) = connection.read_frame().await.unwrap() {
        let response = match Command::from_frame(frame).unwrap() {
            Set(cmd) => {
                // 值被存储为 `Vec<u8>` 的形式
                let mut db = db.lock().unwrap();
                db.insert(cmd.key().to_string(), cmd.value().clone());
                Frame::Simple("OK".to_string())
            }
            Get(cmd) => {
                let db = db.lock().unwrap();
                if let Some(value) = db.get(cmd.key()) {
                    // `Frame::Bulk` 期待数据的类型是 `Bytes`，
                    // 此时，你只要知道`&Vec<u8>`可以使用`into()`方法转换成`Bytes`类型
                    Frame::Bulk(value.clone())
                } else {
                    Frame::Null
                }
            }
            cmd => panic!("Unimplemented {:?}", cmd),
        };

        // 将请求响应返回给客户端
        connection.write_frame(&response).await.unwrap();
    }
    // if let Some(frame) = connection.read_frame().await.unwrap() {
    //     println!("GOT: {:?}", frame);

    //     // 回复一个错误
    //     let response = Frame::Error("Unimplemented".to_string());
    //     connection.write_frame(&response).await.unwrap();
    // }
}
