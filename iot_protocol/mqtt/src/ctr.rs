use crate::mqtt_async_sample::{create_client, event_loop, shutdown_client};
use crate::service_instace::{pub_create_mqtt_client_op, sendRemoveMqttClient};
use common_lib::config::{Config, NodeInfo};
use common_lib::models::MqttConfig;
use common_lib::rabbit_utils::{get_rabbitmq_instance, RabbitMQ};
use common_lib::redis_pool_utils::RedisOp;
use log::{error, info};
use r2d2::Pool;
use r2d2_redis::redis::RedisError;
use r2d2_redis::RedisConnectionManager;
use rocket::fairing::AdHoc;
use rocket::http::{Method, Status};
use rocket::response::status;
use rocket::serde::json::Json;
use rocket::tokio::time::Duration;
use rocket::yansi::Paint;
use rocket::{get, post, State};
use rocket::{Request, Response};
use rumqttc::{AsyncClient, Client, ConnectionError, Event, Incoming, MqttOptions, QoS};
use serde_json::{json, to_string};
use std::collections::HashMap;
use std::error::Error;
use std::future::poll_fn;
use std::string::String;
use tokio::sync::MutexGuard;
use tokio::task;

#[get("/beat")]
pub fn HttpBeat(pool: &rocket::State<RedisOp>) -> rocket::response::status::Custom<&str> {
    return rocket::response::status::Custom(Status::Ok, "ok");
}

#[get("/bb?<client_id>")]
pub async fn HttpBeat2(
    pool: &State<RedisOp>,
    client_id: Option<String>,
) -> status::Custom<&'static str> {
    info!("client_id = {:?}", client_id);

    shutdown_client(client_id.unwrap().as_str());
    status::Custom(Status::Ok, "ok")
}

#[post("/create_mqtt", format = "json", data = "<mqtt_config>")]
pub async fn create_mqtt_client_http(
    redis_op: &State<RedisOp>,
    config: &State<Config>,
    mqtt_config: Json<MqttConfig>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    info!("mqtt_config = {:?}", mqtt_config);
    info!("config = {:?}", config);

    let key = format!("mqtt_create:{}", mqtt_config.client_id);
    let x = redis_op.acquire_lock(&key, &key, 100).unwrap();
    if x {
        if check_has_config(mqtt_config.client_id.clone(), redis_op) {
            info!("当前客户端已存在");

            let response = json!({
                "status": 400,
                "message": "已经存在客户端id"
            });
            return rocket::response::status::Custom(Status::Ok, Json(response));
        } else {
            let usz = create_mqtt_client(&mqtt_config, &redis_op, &config.node_info).await;

            if usz == -1 {
                let response = json!({
                    "status": 400,
                    "message": "达到最大客户端数量"
                });
                redis_op.release_lock(&key, &key).unwrap();
                return rocket::response::status::Custom(Status::Ok, Json(response));
            } else if usz == -2 {
                let response = json!({
                    "status": 400,
                    "message": "MQTT客户端配置异常"
                });
                redis_op.release_lock(&key, &key).unwrap();
                return rocket::response::status::Custom(Status::Ok, Json(response));
            } else {
                AddNoUseConfig(&mqtt_config, redis_op);
                BindNode(&mqtt_config, config.node_info.name.clone(), redis_op);
                let response = json!({
                    "status": 200,
                    "message": "创建成功"
                });
                redis_op.release_lock(&key, &key).unwrap();
                return rocket::response::status::Custom(Status::Ok, Json(response));
            }
        }
    } else {
        error!("上锁异常 ,{}", key);
        let response = json!({
            "status": 400,
            "message": "上锁异常"
        });
        rocket::response::status::Custom(Status::Ok, Json(response))
    }
}

pub fn AddNoUseConfig(mqtt_config: &MqttConfig, redis_op: &RedisOp) {
    redis_op
        .set_hash(
            "mqtt_config:no",
            mqtt_config.client_id.as_str(),
            serde_json::to_string(mqtt_config).unwrap().as_str(),
        )
        .expect("add no use config 异常");
}

pub fn GetNoUseConfigById(id: String, pool: &State<RedisOp>) -> Option<String> {
    pool.get_hash("mqtt_config:no", id.as_str())
        .expect("get no use config by id")
}
pub fn BindNode(mqtt_config: &MqttConfig, node_name: String, redis_op: &RedisOp) {
    let binding = serde_json::to_string(mqtt_config).unwrap();

    let x = binding.as_str();
    let key = format!("node_bind:{}", node_name);
    redis_op
        .add_set(key.as_str(), mqtt_config.client_id.as_str())
        .unwrap();

    RemoveNoUseConfig(mqtt_config, redis_op);
    AddUseConfig(mqtt_config, redis_op);
}

pub fn AddUseConfig(mqtt_config: &MqttConfig, redis_op: &RedisOp) {
    let binding = serde_json::to_string(mqtt_config).unwrap();
    let x = binding.as_str();
    redis_op
        .set_hash("mqtt_config:use", mqtt_config.client_id.as_str(), x)
        .expect("add use config 异常");
}
pub fn RemoveNoUseConfig(mqtt_config: &MqttConfig, redis_op: &RedisOp) {
    info!("mqtt_config = {:?}", mqtt_config);

    redis_op
        .delete_hash_field("mqtt_config:no", mqtt_config.client_id.as_str())
        .expect("TODO: panic message");
}
pub async fn create_mqtt_client(
    mqtt_config: &MqttConfig,
    redis_op: &RedisOp,
    node_info: &NodeInfo,
) -> i64 {
    let key = format!("node_bind:{}", node_info.name);
    info!("key = {:?}", key);

    let result = redis_op.get_zset_length(key.as_str()).unwrap_or(0) as i64;

    let mqtt_config = Arc::new(Mutex::new(mqtt_config.clone()));

    if node_info.size > result {
        let min = create_mqtt_client_min(mqtt_config).await;
        info!("min = {}", min);
        if min {
            return result + 1;
        } else {
            return -2;
        }
    } else {
        return -1;
    }
}
use crate::service_instace::PubCreateMqttClientOp;
use crate::{GetBindClientId, GetThisTypeService, GetUseConfig};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;

pub async fn create_mqtt_client_min(mqtt_config: Arc<Mutex<MqttConfig>>) -> bool {
    // 获取锁并使用锁中的数据
    let mqtt_config = mqtt_config.lock().await;

    // 克隆数据
    let client_id = mqtt_config.client_id.clone();
    let sub_topic = mqtt_config.sub_topic.clone();
    let username = mqtt_config.username.clone();
    let password = mqtt_config.password.clone();
    let broker = mqtt_config.broker.clone();
    let port = mqtt_config.port;

    match create_client(
        client_id.as_str(),
        sub_topic.as_str(),
        username.as_str(),
        password.as_str(),
        broker.as_str(),
        port as u16,
    )
    .await
    {
        Ok(_) => {
            return true;
        }
        Err(e) => {
            error!("create mqtt client error: {}", e);
            return false;
        }
    }
}

pub fn check_has_config(mqtt_client_id: String, redis_op: &RedisOp) -> bool {
    info!("mqtt_client_id = {}", mqtt_client_id);
    return match redis_op.hash_exists("mqtt_config:use", &mqtt_client_id) {
        Ok(e) => return e,
        Err(error) => {
            error!("redis error: {}", error);
            false
        }
    };
}

pub fn GetNoUseConfig(redis_op: &RedisOp) -> Vec<String> {
    return match redis_op.get_hash_all_value("mqtt_config:no") {
        Ok(x) => x,
        Err(error) => {
            error!("redis error: {}", error);
            vec![]
        }
    };
}
#[get("/node_list")]
pub fn NodeList(pool: &rocket::State<RedisOp>, config: &State<Config>) -> Json<serde_json::Value> {
    let service = GetThisTypeService(config.node_info.node_type.clone(), pool);
    Json(json!({
        "status": 200,
        "message": "创建成功",
        "data": service
    }))
}

#[get("/node_using_status")]
pub fn NodeUsingStatus(
    pool: &rocket::State<RedisOp>,
    config: &State<Config>,
) -> Result<Json<HashMap<String, serde_json::Value>>, status::Custom<&'static str>> {
    #[derive(Serialize, Deserialize, Debug)]
    pub struct NodeInfo {
        pub name: String,
        pub size: i64,
        pub client_ids: Vec<String>,
        pub client_infos: Vec<MqttConfig>,
        pub max_size: i64,
    }

    // 获取服务对象
    let service = GetThisTypeService(config.node_info.node_type.clone(), pool);

    let mut node_infos: Vec<NodeInfo> = Vec::new();

    for info in service {
        let node_name = info.name.clone();
        let size = match pool.get_zset_length(&format!("node_bind:{}", node_name)) {
            Ok(len) => len as i64,
            Err(_) => 0,
        };

        // 获取 ClientInfos 列表
        let mut mc: Vec<MqttConfig> = Vec::new();
        let client_ids = GetBindClientId(node_name.clone(), pool); // 传递引用避免所有权转移

        for el in &client_ids {
            match GetUseConfig(el.clone(), pool) {
                None => {}
                Some(v) => {
                    let config: MqttConfig = match serde_json::from_str(&v) {
                        Ok(config) => config,
                        Err(e) => {
                            eprintln!("HandlerOffNode JSON 解析错误: {:?}", e);
                            continue;
                        }
                    };
                    mc.push(config);
                }
            }
        }

        // 添加到 NodeInfo 列表
        node_infos.push(NodeInfo {
            name: node_name,
            size,
            max_size: info.size,
            client_ids: client_ids.clone(), // 克隆避免所有权问题
            client_infos: mc,
        });
    }

    let mut response = HashMap::new();
    response.insert("status".to_string(), json!(200));
    response.insert("message".to_string(), json!("成功"));
    response.insert("data".to_string(), json!(node_infos));

    Ok(Json(response))
}

#[get("/get_use_mqtt_config?<id>")]
pub async fn get_use_mqtt_config(
    id: Option<String>,
    pool: &rocket::State<RedisOp>,
) -> Result<
    Json<HashMap<&'static str, serde_json::Value>>,
    status::Custom<Json<HashMap<&'static str, serde_json::Value>>>,
> {
    info!("id = {:?}", id);

    let id = match id {
        Some(id) => id,
        None => {
            let mut response = HashMap::new();
            response.insert("status", json!(400));
            response.insert("message", json!("Missing 'id' query parameter"));
            response.insert("data", json!(null));
            return Err(status::Custom(Status::Ok, Json(response)));
        }
    };

    let option = GetUseConfig(id, pool);
    match option {
        None => {
            let mut response = HashMap::new();
            response.insert("status", json!(400));
            response.insert("message", json!("Config Nont Fount"));
            response.insert("data", json!(null));
            return Err(status::Custom(Status::Ok, Json(response)));
        }
        Some(config_json) => {
            let config: MqttConfig = match serde_json::from_str(&config_json) {
                Ok(config) => config,
                Err(e) => {
                    eprintln!("Error unmarshalling JSON: {:?}", e);
                    let mut response = HashMap::new();
                    response.insert("status", json!(500));
                    response.insert("message", json!("Failed to parse configuration"));
                    response.insert("data", json!(null));
                    return Err(status::Custom(Status::InternalServerError, Json(response)));
                }
            };

            // 构建响应
            let mut response = HashMap::new();
            response.insert("status", json!(200));
            response.insert("message", json!("Success"));
            response.insert("data", json!(config));

            Ok(Json(response))
        }
    }
}

#[get("/no_mqtt_config?<id>")]
pub fn GetNoUseMqttConfig(
    id: Option<String>,
    pool: &rocket::State<RedisOp>,
) -> Result<
    Json<HashMap<&'static str, serde_json::Value>>,
    status::Custom<Json<HashMap<&'static str, serde_json::Value>>>,
> {
    info!("id = {:?}", id);

    let id = match id {
        Some(id) => id,
        None => {
            let mut response = HashMap::new();
            response.insert("status", json!(400));
            response.insert("message", json!("Missing 'id' query parameter"));
            response.insert("data", json!(null));
            return Err(status::Custom(Status::Ok, Json(response)));
        }
    };

    let option = GetNoUseConfigById(id, pool);
    match option {
        None => {
            let mut response = HashMap::new();
            response.insert("status", json!(400));
            response.insert("message", json!("Config Nont Fount"));
            response.insert("data", json!(null));
            return Err(status::Custom(Status::Ok, Json(response)));
        }
        Some(config_json) => {
            let config: MqttConfig = match serde_json::from_str(&config_json) {
                Ok(config) => config,
                Err(e) => {
                    eprintln!("Error unmarshalling JSON: {:?}", e);
                    let mut response = HashMap::new();
                    response.insert("status", json!(500));
                    response.insert("message", json!("Failed to parse configuration"));
                    response.insert("data", json!(null));
                    return Err(status::Custom(Status::InternalServerError, Json(response)));
                }
            };

            // 构建响应
            let mut response = HashMap::new();
            response.insert("status", json!(200));
            response.insert("message", json!("Success"));
            response.insert("data", json!(config));

            Ok(Json(response))
        }
    }
}

#[get("/remove_mqtt_client?<client_id>")]
pub async fn RemoveMqttClient(
    pool: &rocket::State<RedisOp>,
    config: &State<Config>,

    client_id: Option<String>,
) -> Json<serde_json::Value> {
    StopMqttClient2(
        client_id.clone().unwrap(),
        pool,
        config.node_info.name.clone(),
    );
    shutdown_client(client_id.unwrap().as_str());

    Json(json!({ "status": 200, "message": "Success","data":"已停止" }))
}

fn StopMqttClient2(client_id: String, pool: &State<RedisOp>, node_name: String) {
    pool.delete_hash_field("mqtt_config:no", client_id.as_str());
    pool.delete_hash_field("mqtt_config:use", client_id.as_str());

    pool.delete_set_member(
        format!("node_bind:{}", node_name).as_str(),
        client_id.as_str(),
    );
}

// #[post("/public_create_mqtt", format = "json", data = "<mqtt_config>")]
// pub fn PubCreateMqttClientHttp(
//     pool: &State<RedisOp>,
//     config: &State<Config>,
//     mqtt_config: Json<MqttConfig>,
// ) -> Json<serde_json::Value> {
//     // 将 mqtt_config 转换为 JSON 字符串
//     let string = match serde_json::to_string(&mqtt_config.into_inner()) {
//         Ok(s) => s,
//         Err(_) => {
//             return Json(json!({ "status": 500, "message": "Failed to serialize MQTT config" }))
//         }
//     };
//
//     // 调用 PubCreateMqttClientOp 函数并根据返回结果处理
//     match PubCreateMqttClientOp(string, pool, config.node_info.node_type.clone()) {
//         1 => Json(json!({ "status": 200, "message": "创建成功" })),
//         _ => Json(json!({ "status": 400, "message": "创建失败" })),
//     }
// }

#[post("/public_create_mqtt", format = "json", data = "<mqtt_config>")]
pub async fn PubCreateMqttClientHttp(
    pool: &State<RedisOp>,
    config: &State<Config>,
    mqtt_config: Json<MqttConfig>,
) -> Json<serde_json::Value> {
    // 将 mqtt_config 转换为 JSON 字符串
    let string = match serde_json::to_string(&mqtt_config.into_inner()) {
        Ok(s) => s,
        Err(_) => {
            return Json(json!({ "status": 500, "message": "Failed to serialize MQTT config" }))
        }
    };

    // 调用 PubCreateMqttClientOp 函数并根据返回结果处理
    match pub_create_mqtt_client_op(string, pool, config.node_info.node_type.clone()).await {
        1 => Json(json!({ "status": 200, "message": "创建成功" })),
        _ => Json(json!({ "status": 400, "message": "创建失败" })),
    }
}

#[get("/public_remove_mqtt_client?<client_id>")]
pub fn PubRemoveMqttClient(
    pool: &rocket::State<RedisOp>,
    config: &State<Config>,

    client_id: Option<String>,
) -> Json<serde_json::Value> {
    let s = FindMqttClient(client_id.clone().unwrap(), pool);
    if s == String::new() {
        Json(json!({ "status": 200, "message": "节点未找到" }))
    } else {
        let o = getNodeInfo(s.to_string(), pool, config);
        match o {
            None => Json(json!({ "status": 200, "message": "节点未找到" })),
            Some(v) => {
                sendRemoveMqttClient(&v, client_id.clone().unwrap());
                Json(json!({ "status": 200, "message": "删除成功" }))
            }
        }
    }
}

fn getNodeInfo(
    node_name: String,
    pool: &State<RedisOp>,
    config: &State<Config>,
) -> Option<NodeInfo> {
    let k = format!("register:{}", config.node_info.node_type.clone());
    let option = pool.get_hash(k.as_str(), node_name.as_str()).unwrap();

    match option {
        None => {
            // 返回 None，表示没有找到对应的节点信息
            return None;
        }
        Some(v) => {
            // 尝试解析 JSON，如果失败，返回 None
            match serde_json::from_str(&v) {
                Ok(node_info) => Some(node_info),
                Err(e) => {
                    eprintln!("HandlerOffNode JSON 解析错误: {:?}", e);
                    None
                }
            }
        }
    }
}

fn FindMqttClient(client_id: String, pool: &State<RedisOp>) -> String {
    let vec = pool.keys("node_bind:*").unwrap();
    for x in vec {
        let vec1 = pool.get_set(x.as_str()).unwrap();
        for x2 in vec1 {
            if x2.as_str() == client_id.as_str() {
                return x2.replace("node_bind:", "");
            }
        }
    }
    String::new()
}
