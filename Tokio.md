<!--
 * @Author: 0xSchnappi 952768182@qq.com
 * @Date: 2024-07-21 21:03:41
 * @LastEditors: 0xSchnappi 952768182@qq.com
 * @LastEditTime: 2024-07-21 21:08:08
 * @FilePath: /mini-redis/Tokio.md
 * @Description: Tokio学习笔记
 * 
 * Copyright (c) 2024 by github.com/0xSchnappi, All Rights Reserved. 
-->
# Tokio

## Tokio 的消息通道(channel)

  - `mpsc` 多生产者，单消费者模式
  - `oneshot` 单生产者，单消费者，一次只能发送一个消息
  - `broadcast` 多生产者，多消费者，其中每一条发送的消息都可以被所有接收者收到，因此是广播
  - `watch` 单生产者，多消费者，只保存一条最新的消息，因此接收者只能看到最近的一条消息，例如，这种模式适用于配置文件变化的监听