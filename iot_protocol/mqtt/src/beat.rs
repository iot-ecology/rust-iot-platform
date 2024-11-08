use crate::HandlerOffNode;
use common_lib::redis_pool_utils::RedisOp;
use r2d2_redis::redis::Commands;
use redis::AsyncCommands;
use redis::{PubSubCommands, RedisError, RedisResult};
use std::thread;
use std::time::Duration;
use tokio::runtime::Runtime;

pub fn ListenerBeat(redis_op: &RedisOp, redis_db: u8) {
    let mut con = redis_op.get_connection();
    con.set::<&str, &str, ()>("notify-keyspace-events", "Ex")
        .unwrap();
    let op = redis_op.clone();
    thread::spawn(move || {
        // 使用 PubSub 订阅过期事件频道
        let mut pubsub_con = con.as_pubsub();
        pubsub_con
            .subscribe(format!("__keyevent@{}__:expired", redis_db))
            .unwrap(); // 数据库0的过期事件频道

        loop {
            // 等待消息
            let msg = pubsub_con.get_message().unwrap();
            let payload: String = msg.get_payload().unwrap();

            // 检查消息是否以 "beat" 开头
            if payload.starts_with("beat") {
                // 分割字符串获取最后一个元素
                let parts: Vec<&str> = payload.split(':').collect();
                if let Some(last_element) = parts.last() {
                    // 调用 HandlerOffNode 处理过期的 "beat" 键
                    HandlerOffNode(last_element.to_string(), &op);
                }
            }

            // 处理过期事件
            println!("Key expired: {}", payload);
        }
    });
}
