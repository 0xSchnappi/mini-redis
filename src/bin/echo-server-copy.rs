/*
 * @Author: 0xSchnappi 952768182@qq.com
 * @Date: 2024-07-22 10:32:20
 * @LastEditors: 0xSchnappi 952768182@qq.com
 * @LastEditTime: 2024-07-22 11:23:38
 * @FilePath: /mini-redis/src/bin/echo-server-copy.rs
 * @Description: 回声壁
 *
 * Copyright (c) 2024 by github.com/0xSchnappi, All Rights Reserved.
 */
use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.1:6142").await?;

    // loop {
    //     let (mut socket, _) = listener.accept().await?;

    //     tokio::spawn(async move {
    //         let (mut rd, mut wr) = socket.split();

    //         if io::copy(&mut rd, &mut wr).await.is_err() {
    //             eprintln!("failed to copy");
    //         }
    //     });
    // }

    // 手动拷贝
    loop {
        let (mut socket, _) = listener.accept().await?;

        tokio::spawn(async move {
            let mut buf = vec![0; 1024];

            loop {
                match socket.read(&mut buf).await {
                    // 返回值`Ok(0)`说明对端已经关闭
                    Ok(0) => return,
                    Ok(n) => {
                        // 将数据拷贝回socket中
                        if socket.write_all(&buf[..n]).await.is_err() {
                            // 非预期错误，由于我们这里无需再做什么，因此直接停止处理
                            return;
                        }
                    }
                    Err(_) => {
                        // 非预期错误，由于我们无需再做什么，因此直接停止处理
                        return;
                    }
                }
            }
        });
    }
}
