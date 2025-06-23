/*
Copyright [2025] [Zen HuiFer]

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use crate::ctr::GetNoUseConfig;
use crate::{processHeartbeats, GetThisTypeService};
use common_lib::config::NodeInfo;
use common_lib::redis_pool_utils::RedisOp;
use log::warn;
use log::{debug, error, info};
use r2d2_redis::redis::RedisError;
use reqwest::blocking::Client;
use serde_json::to_string;
use std::thread;
use time::Instant;
use tokio::time::{self, Duration};
use tracing_subscriber::fmt::format;

pub fn CBeat(f: &NodeInfo, redis_op: &RedisOp) {
    let mut last_tick = Instant::now();
    let tick_duration = Duration::from_secs(1);

    loop {
        // 检查是否已到达下一个执行时间
        if last_tick.elapsed() >= tick_duration {
            debug!("cbeat  task");

            let result = redis_op.acquire_lock("c_beat", "c_beat", 100);
            match result {
                Ok(x) => {
                    if (x) {
                        let vec = GetThisTypeService(f.clone().node_type, redis_op);
                        processHeartbeats(vec, redis_op);

                        redis_op.release_lock("c_beat", "c_beat").unwrap();
                    }
                }
                Err(err) => {
                    error!("获取锁失败: {}", err);
                }
            }

            last_tick = Instant::now();
        }

        std::thread::sleep(Duration::from_millis(1));
    }
}

pub fn noHandlerConfig(f: &NodeInfo, redis_op: &RedisOp) {
    let mut last_tick = Instant::now();
    let tick_duration = Duration::from_secs(1);

    loop {
        // 检查是否已到达下一个执行时间
        if last_tick.elapsed() >= tick_duration {
            debug!("cbeat  task");

            let result =
                redis_op.acquire_lock("no_handler_config_lock", "no_handler_config_lock", 100);
            match result {
                Ok(x) => {
                    if (x) {
                        let vec: Vec<String> = GetNoUseConfig(redis_op);
                        for x in vec {
                            if PubCreateMqttClientOp(x, redis_op, f.node_type.clone()) == -1 {
                                continue;
                            }
                        }
                        std::thread::sleep(Duration::from_millis(100));

                        redis_op
                            .release_lock("no_handler_config_lock", "no_handler_config_lock")
                            .unwrap();
                    }
                }
                Err(err) => {
                    error!("获取锁失败: {}", err);
                }
            }

            last_tick = Instant::now();
        }

        std::thread::sleep(Duration::from_millis(1));
    }
}

pub fn PubCreateMqttClientOp(config: String, redis_op: &RedisOp, node_type: String) -> i32 {
    let option = get_size_lose("".to_string(), redis_op, node_type);

    info!("option  = {:?}", option);

    match option {
        Some(x) => {
            if SendCreateMqttMessage(&x, &config) {
                return 1;
            } else {
                return -1;
            }
        }
        None => -1,
    }
}
pub fn SendCreateMqttMessage(node: &NodeInfo, param: &str) -> bool {
    info!(
        "Sending create MQTT client request, node info: {:?}, params: {}",
        node, param
    );

    let url = format!("http://{}:{}/create_mqtt", node.host, node.port);
    let client = reqwest::blocking::Client::new(); // 使用 blocking client

    let resp = client
        .post(&url)
        .header("Content-Type", "application/json")
        .body(param.to_string())
        .send();

    match resp {
        Ok(response) => {
            let status = response.status();
            let body = response.text().unwrap_or_else(|_| String::from(""));

            info!("Response Status: {:?}, Body: {}", status, body);

            if body.as_str() == "ok" {
                return true;
            } else {
                return false;
            }
        }
        Err(err) => {
            error!("Error sending request: {}", err);
            false
        }
    }
}

pub async fn pub_create_mqtt_client_op(
    config: String,
    redis_op: &RedisOp,
    node_type: String,
) -> i32 {
    let option = get_size_lose("".to_string(), redis_op, node_type);

    info!("option = {:?}", option);

    match option {
        Some(x) => {
            if send_create_mqtt_message(&x, &config).await {
                1
            } else {
                -1
            }
        }
        None => -1,
    }
}

pub async fn send_create_mqtt_message(node: &NodeInfo, param: &str) -> bool {
    info!(
        "Sending create MQTT client request, node info: {:?}, params: {}",
        node, param
    );

    let url = format!("http://{}:{}/create_mqtt", node.host, node.port);
    let client = reqwest::Client::new(); // 使用异步客户端

    // 使用 await 等待异步操作
    let resp = client
        .post(&url)
        .header("Content-Type", "application/json")
        .body(param.to_string())
        .send()
        .await; // 在这里加上 await

    match resp {
        Ok(response) => {
            let status = response.status();
            let body = response.text().await.unwrap_or_else(|_| String::from(""));

            info!("Response Status: {:?}, Body: {}", status, body);

            if body == "ok" {
                true
            } else {
                false
            }
        }
        Err(err) => {
            error!("Error sending request: {}", err);
            false
        }
    }
}
pub async fn send_remove_mqtt_client(node: &NodeInfo, id: String) -> bool {
    let url = format!(
        "http://{}:{}/remove_mqtt_client?id={}",
        node.host, node.port, id
    );
    let client = reqwest::Client::new();
    // Send POST request
    let resp = client
        .get(&url)
        .header("Content-Type", "application/json")
        .send()
        .await;

    match resp {
        Ok(response) => {
            let status = response.status();
            let body = response.text().await.unwrap_or_else(|_| String::from(""));

            info!("Response Status: {:?}, Body: {}", status, body);

            if body.as_str() == "ok" {
                return true;
            } else {
                return false;
            }
        }
        Err(err) => {
            error!("Error sending request: {}", err);
            false
        }
    }
}

pub fn get_size_lose(
    pass_node_name: String,
    redis_op: &RedisOp,
    node_type: String,
) -> Option<NodeInfo> {
    let vec = GetThisTypeService(node_type, redis_op);

    if vec.len() == 0 {
        return None;
    }
    let mut min_size = -1;
    let mut min_node_info: Option<NodeInfo> = Option::None;
    for v in vec {
        if v.name.as_str() == pass_node_name.as_str() {
            continue;
        }

        let i = redis_op
            .get_set_length(format!("node_bind:{}", v.name).as_str())
            .unwrap_or(0);
        if i < v.size {
            if min_node_info.is_none() || v.size < min_size {
                min_size = v.size;
                min_node_info = Option::Some(v);
            }
        } else {
            continue;
        }
    }

    min_node_info
}

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
