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

mod beat;
mod ctr;
mod mqtt_async_sample;
mod service_instace;

use crate::beat::ListenerBeat;
use crate::ctr::GetNoUseMqttConfig;
use crate::ctr::NodeList;
use crate::ctr::NodeUsingStatus;
use crate::ctr::PubCreateMqttClientHttp;
use crate::ctr::PubRemoveMqttClient;
use crate::ctr::RemoveMqttClient;
use crate::ctr::{create_mqtt_client_http, AddNoUseConfig};
use crate::ctr::{get_use_mqtt_config, HttpBeat};
use crate::service_instace::{noHandlerConfig, register_task, CBeat};
use common_lib::config::{get_config, read_config, read_config_tb, MqConfig, NodeInfo};
use common_lib::models::MqttConfig;
use common_lib::rabbit_utils::{init_rabbitmq_with_config, RabbitMQFairing};
use common_lib::redis_handler::init_redis;
use common_lib::redis_pool_utils::{create_redis_pool_from_config, RedisOp};
use log::{debug, error};
use r2d2_redis::redis::RedisError;
use reqwest::blocking::Client;
use rocket::{
    fairing::{Fairing, Info, Kind},
    Build, Rocket, State,
};
use rocket::{launch, routes};
use serde_json::from_str;
use std::sync::Arc;
use std::thread;
use tokio::runtime::Runtime;
use tracing_subscriber::fmt::format;

#[launch]
fn rocket() -> _ {
    // 初始化日志
    common_lib::init_logger();
    let rt = Runtime::new().unwrap();

    // 读取配置
    let config1 = read_config_tb("app-local.yml");

    // 创建 Redis 连接池
    let pool = create_redis_pool_from_config(&config1.redis_config);
    let redis_op = RedisOp { pool };
    let redis_op_for_task = redis_op.clone();

    let node_info = Arc::new(config1.node_info.clone());
    let node_info_for_task = Arc::clone(&node_info);
    // 启动 register_task 的线程
    ListenerBeat(&redis_op_for_task, config1.redis_config.db);
    thread::spawn(move || {
        register_task(&node_info_for_task, &redis_op_for_task);
    });

    let node_info2 = Arc::new(config1.node_info.clone());
    let node_info_for_task2 = Arc::clone(&node_info2);
    let redis_op_for_task2 = redis_op.clone();

    thread::spawn(move || {
        CBeat(&node_info_for_task2, &redis_op_for_task2);
    });

    let node_info3 = Arc::new(config1.node_info.clone());
    let node_info_for_task3 = Arc::clone(&node_info3);
    let redis_op_for_task3 = redis_op.clone();

    beforeStart(&redis_op, &config1.clone());

    thread::spawn(move || noHandlerConfig(&node_info_for_task3, &redis_op_for_task3));

    let node_info_for_rocket = config1.node_info.clone();

    // 构建 Rocket 实例
    rocket::build()
        .attach(RabbitMQFairing {
            config: config1.mq_config.clone(),
        })
        .manage(redis_op)
        .manage(config1.clone())
        .configure(rocket::Config {
            port: node_info_for_rocket.port,
            log_level: rocket::config::LogLevel::Off,
            ..Default::default()
        })
        .mount(
            "/",
            routes![
                HttpBeat,
                create_mqtt_client_http,
                NodeList,
                NodeUsingStatus,
                get_use_mqtt_config,
                GetNoUseMqttConfig,
                RemoveMqttClient,
                PubCreateMqttClientHttp,
                PubRemoveMqttClient,
            ],
        )
}

fn beforeStart(redis_op: &RedisOp, config: &common_lib::config::Config) {
    HandlerOffNode(config.node_info.name.clone(), redis_op);
}

pub fn HandlerOffNode(node_name: String, redis_op: &RedisOp) {
    let vec = GetBindClientId(node_name.clone(), redis_op);
    for x in vec {
        if let Some(cf) = GetUseConfig(x, redis_op) {
            if let Ok(mqtt_config) = from_str::<MqttConfig>(cf.as_str()) {
                RemoveBindNode(node_name.clone(), mqtt_config.client_id, redis_op);
            } else {
                eprintln!("解析 MQTT 配置失败");
            }
        }
    }
    CheckMqttConfigIsUsingAndMove(node_name.clone(), redis_op);
    let key = format!("node_bind:{}", node_name.clone());
    redis_op.delete(key.as_str()).unwrap();
}

fn CheckMqttConfigIsUsingAndMove(node_name: String, redis_op: &RedisOp) {
    let using = GetAllMqttConfigUsing(redis_op);
    let bind = GetBindClientId(node_name.clone(), redis_op);

    for x in &bind {
        CheckUsing(using.clone(), x, redis_op);
    }

    for config in using {
        if !string_in_slice(bind.clone(), &config.client_id) {
            RemoveUseConfig(config.client_id.clone(), redis_op);
            AddNoUseConfig(&config, redis_op);
        }
    }
}
fn string_in_slice(slice: Vec<String>, str: &str) -> bool {
    slice.iter().any(|v| v == str)
}

fn CheckUsing(v: Vec<MqttConfig>, mqtt_client_id: &String, redis_op: &RedisOp) {
    for x in v {
        if x.client_id.as_str() == mqtt_client_id.as_str() {
            RemoveUseConfig(x.client_id.clone(), redis_op);
            AddNoUseConfig(&x, redis_op);
        }
    }
}
fn GetAllMqttConfigUsing(redis_op: &RedisOp) -> Vec<MqttConfig> {
    let mut configs = Vec::new();

    if let Ok(vec) = redis_op.get_hash_all_value("mqtt_config:use") {
        for cf in vec {
            if let Ok(mqtt_config) = from_str::<MqttConfig>(&cf) {
                configs.push(mqtt_config);
            }
        }
    }

    configs
}
pub fn RemoveBindNode(node_name: String, mqtt_client_id: String, redis_op: &RedisOp) {
    redis_op.delete_set_member(
        format!("node_bind:{}", node_name).as_str(),
        mqtt_client_id.as_str(),
    );

    let option = GetUseConfig(mqtt_client_id, redis_op);
    match option {
        None => {}
        Some(str) => {
            let mqtt_config: MqttConfig = from_str::<MqttConfig>(str.as_str()).unwrap();
            RemoveUseConfig(mqtt_config.client_id.clone(), redis_op);
            AddNoUseConfig(&mqtt_config, redis_op);
        }
    }
}
pub fn GetUseConfig(client_id: String, redis_op: &RedisOp) -> Option<String> {
    let result = redis_op
        .get_hash("mqtt_config:use", client_id.as_str())
        .unwrap();
    result
}

pub fn RemoveUseConfig(client_id: String, redis_op: &RedisOp) {
    redis_op
        .delete_hash_field("mqtt_config:use", client_id.as_str())
        .unwrap();
}

pub fn GetBindClientId(nodeName: String, redis_op: &RedisOp) -> Vec<String> {
    let key = format!("node_bind:{}", nodeName);
    let result = redis_op.get_set(key.as_str());
    match result {
        Ok(res) => {
            return res;
        }
        Err(e) => {
            error!("{}", e);
            return vec![];
        }
    }
}

pub fn GetThisTypeService(node_type: String, redis_op: &RedisOp) -> Vec<NodeInfo> {
    // 从 Redis 中获取哈希表的所有值
    let result = redis_op.get_hash_all_value(&format!("register:{}", node_type));

    // 结果容器
    let mut nodes = Vec::new();

    match result {
        Ok(values) => {
            for value in values {
                match from_str::<NodeInfo>(&value) {
                    Ok(node_info) => {
                        nodes.push(node_info);
                    }
                    Err(parse_error) => {
                        error!("Failed to parse NodeInfo: {}", parse_error);
                    }
                }
            }
        }
        Err(error) => {
            error!("Failed to retrieve nodes from Redis: {}", error);
            return vec![];
        }
    }

    nodes
}

pub fn processHeartbeats(service: Vec<NodeInfo>, redis_op: &RedisOp) {
    for x in service {
        if !send_beat(&x, "beat") {
            redis_op
                .delete_hash_field(
                    format!("register:{}", x.node_type).as_str(),
                    x.name.as_str(),
                )
                .unwrap();
            HandlerOffNode(x.node_type, redis_op);
        }
    }
}

pub fn send_beat(node: &NodeInfo, param: &str) -> bool {
    debug!(
        "Sending heartbeat request, node info: {:?}, params: {}",
        node, param
    );

    let url = format!("http://{}:{}/beat", node.host, node.port);
    let client = Client::new();

    // Create GET request with JSON content-type
    let resp = client
        .get(&url)
        .header("Content-Type", "application/json")
        .body(param.to_string())
        .send();

    match resp {
        Ok(response) => {
            let status = response.status();
            debug!("Response Status: {:?}", status);
            status == reqwest::StatusCode::OK
        }
        Err(err) => {
            error!("Error sending request: {}", err);
            false
        }
    }
}
