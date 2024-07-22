/*
 * @Author: 0xSchnappi 952768182@qq.com
 * @Date: 2024-07-22 10:55:21
 * @LastEditors: 0xSchnappi 952768182@qq.com
 * @LastEditTime: 2024-07-22 11:28:37
 * @FilePath: /mini-redis/src/bin/echo-client-copy.rs
 * @Description: 回声客户端
 *
 * Copyright (c) 2024 by github.com/0xSchnappi, All Rights Reserved.
 */
use tokio::io::{self, AsyncBufReadExt, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;

#[tokio::main]
async fn main() -> io::Result<()> {
    let socket = TcpStream::connect("127.0.0.1:6142").await?;
    let (mut rd, mut wr) = io::split(socket);

    // 创建异步任务，在后台写入数据
    tokio::spawn(async move {
        wr.write_all(b"hello\r\n").await?;
        wr.write_all(b"world\r\n").await?;

        Ok::<_, io::Error>(())
    });

    let mut buf = vec![0; 128];

    loop {
        let n = rd.read(&mut buf).await?;

        if n == 0 {
            break;
        }

        println!("GOT = {:?}", &buf[..n]);
    }

    Ok(())
}
