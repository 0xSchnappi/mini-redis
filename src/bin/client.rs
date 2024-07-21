/*
 * @Author: 0xSchnappi 952768182@qq.com
 * @Date: 2024-07-21 20:51:05
 * @LastEditors: 0xSchnappi 952768182@qq.com
 * @LastEditTime: 2024-07-21 22:15:17
 * @FilePath: /mini-redis/src/bin/client.rs
 * @Description:
 *
 * Copyright (c) 2024 by github.com/0xSchnappi, All Rights Reserved.
 */
use tokio::sync::mpsc;

use bytes::Bytes;
use mini_redis::client;
use tokio::sync::oneshot;

type Responder<T> = oneshot::Sender<mini_redis::Result<T>>;

#[derive(Debug)]
enum Command {
    Get {
        key: String,
        resp: Responder<Option<Bytes>>,
    },
    Set {
        key: String,
        val: Bytes,
        resp: Responder<()>,
    },
}

#[tokio::main]
async fn main() {
    // 创建一个新通道，缓冲队列长度是 32
    let (tx, mut rx) = mpsc::channel(32);
    let tx2 = tx.clone();

    // 将消息通道的接收者rx的所有权转移到管理任务中
    let manager = tokio::spawn(async move {
        // 建立到redis服务器的连接
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        // 开始接收消息
        while let Some(cmd) = rx.recv().await {
            use Command::*;
            match cmd {
                Get { key, resp } => {
                    let res = client.get(&key).await;

                    // 忽略错误
                    let _ = resp.send(res);
                }
                Set { key, val, resp } => {
                    let res = client.set(&key, val).await;

                    // 忽略错误
                    let _ = resp.send(res);
                }
            }
        }
    });

    let t1 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Get {
            key: "hello".to_string(),
            resp: resp_tx,
        };

        // 发送GET请求
        tx.send(cmd).await.unwrap();

        // 等待回复
        let res = resp_rx.await;
        println!("GOT = {:?}", res);
    });

    let t2 = tokio::spawn(async move {
        let (resp_tx, resp_rx) = oneshot::channel();
        let cmd = Command::Set {
            key: "foo".to_string(),
            val: "bar".into(),
            resp: resp_tx,
        };

        // 发送 SET 请求
        tx2.send(cmd).await.unwrap();

        // 等待回复
        let res = resp_rx.await;
        println!("GOT = {:?}", res);
    });

    t1.await.unwrap();
    t2.await.unwrap();
    manager.await.unwrap();
}
