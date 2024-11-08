use common_lib::config::NodeInfo;
use common_lib::redis_pool_utils::RedisOp;
use log::{debug, info};
use serde_json::to_string;
use std::thread;
use time::Instant;
use tokio::time::{self, Duration};

pub fn register_task(f: &NodeInfo, redis_op: &RedisOp) {
    let mut last_tick = Instant::now();
    let tick_duration = Duration::from_secs(1);

    loop {
        // 检查是否已到达下一个执行时间
        if last_tick.elapsed() >= tick_duration {
            debug!("beat task");
            register(f, redis_op);
            last_tick = Instant::now();
        }

        std::thread::sleep(Duration::from_millis(1));
    }
}
pub fn register(f: &NodeInfo, redis_op: &RedisOp) {
    redis_op
        .set_string_with_expiry(&format!("beat:{}:{}", f.node_type, f.name), &f.name, 3)
        .unwrap();

    let json_data_str = to_string(f).expect("序列化失败");

    redis_op
        .set_hash(
            &format!("register:{}", f.node_type),
            &f.name,
            &json_data_str,
        )
        .unwrap();
}
