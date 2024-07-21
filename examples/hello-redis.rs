/*
 * @Author: 0xSchnappi 952768182@qq.com
 * @Date: 2024-07-19 10:57:54
 * @LastEditors: 0xSchnappi 952768182@qq.com
 * @LastEditTime: 2024-07-21 17:19:29
 * @FilePath: /mini-redis/examples/hello-redis.rs
 * @Description:
 *
 * Copyright (c) 2024 by github.com/0xSchnappi, All Rights Reserved.
 */

use rand::Rng;
use std::{thread::sleep, time::Duration};

use mini_redis::{client, Result};

#[tokio::main]
async fn main() -> Result<()> {
    // 建立与mini-redis服务器的连接
    let mut client = client::connect("127.0.0.1:6379").await?;

    // 设置key: "hello" 和 value: "world"
    client.set("hello", "world".into()).await?;

    // 获取“key=hello”的值
    let result = client.get("hello").await?;

    print!("从服务器端获取的查询结果={:?}", result);

    Ok(())
}
