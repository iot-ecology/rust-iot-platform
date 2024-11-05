extern crate coap;
mod mainc;

use coap::Server;
use coap_lite::{CoapRequest, RequestType as Method};
use common_lib::models::{Auth, CoapMessage};
use common_lib::protocol_config::read_config;
use common_lib::rabbit_utils::init_rabbitmq_with_config;
use common_lib::redis_handler::{get_redis_instance, init_redis, RedisWrapper};
use log::info;
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::net::SocketAddr;
use tokio::runtime::Runtime;
use tokio::sync::MutexGuard;

fn init_logger() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
}

#[tokio::main]
async fn main() {
    init_logger();
    let result = read_config("app-local.yml").unwrap();
    init_rabbitmq_with_config(result.mq_config).await.unwrap();
    init_redis(result.redis_config).await.unwrap();
    let addr = "127.0.0.1:5683";

    let mut server = Server::new_udp(addr).unwrap();
    info!("Server up on {}", addr);

    let node_name = result.node_info.name.clone();

    server
        .run(move |mut request: Box<CoapRequest<SocketAddr>>| {
            let path = request.get_path().to_string();
            let name = node_name.clone();

            async move {
                match request.get_method() {
                    &Method::Get => handle_get_request(&path, &mut request, &name).await,
                    &Method::Post => handle_post_request(&path, &request).await,
                    _ => info!("Request to unknown path or unsupported method"),
                }

                request
            }
        })
        .await
        .unwrap();
}
/// 认证信息和设备结构体
#[derive(Serialize, Deserialize, Debug)]
struct AuthAndDevice {
    username: String,
    password: String,
    device_id: Option<String>,
}

async fn handle_get_request(path: &str, request: &mut CoapRequest<SocketAddr>, name: &str) {
    let payload = String::from_utf8(request.message.payload.clone()).unwrap();
    let remote_address = request.source.unwrap().to_string();
    let redis = get_redis_instance().await.unwrap();
    let string1 = remote_address.replace(":", "@");

    let option = get_uid(name, string1.clone())
        .await
        .unwrap_or(String::new());

    info!("uid = {}", option);

    if path == "auth" {
        info!("auth");
        let auth_device: Result<AuthAndDevice, _> = from_str(&payload);
        match auth_device {
            Ok(auth_data) => {
                let device_id = auth_data.device_id.unwrap();
                let id = device_id.as_str();

                let x = find_device_mapping_up(redis, id.clone())
                    .await
                    .unwrap_or((String::new(), String::new()));
                if x.0 == auth_data.username && x.1 == auth_data.password {
                    store_uid(name, id.clone(), string1).await;

                    if let Some(ref mut message) = request.response {
                        message.message.payload = "认证通过".into();
                    }
                } else {
                    if let Some(ref mut message) = request.response {
                        message.message.payload = "认证失败".into();
                    }
                }

                info!("auth 处理结束")
            }
            Err(e) => {
                info!("Failed to parse payload as AuthAndDevice: {}", e);
            }
        }
    } else if path == "data" {
        info!("datadata");

        info!("source ={}", string1);

        if let Some(ref mut message) = request.response {
            message.message.payload = "数据处理完成".into();
        }
        info!("data 处理结束")
    } else {
        info!("Unknown GET request path")
    }
}
async fn handle_post_request(path: &str, request: &CoapRequest<SocketAddr>) {
    let payload = String::from_utf8(request.message.payload.clone()).unwrap();
    let redis = get_redis_instance().await.unwrap();
    match path {
        "auth" => info!("POST request to /auth with payload: {}", payload),
        "data" => info!("POST request to /data with payload: {}", payload),
        _ => info!("Unknown POST request path"),
    }
}

/// 在 Redis 中查找设备映射
///
/// # Arguments
///
/// * `wrapper` - Redis 包装器的可变锁。
/// * `device_id` - 设备 ID。
///
/// # Returns
///
/// 如果找到，返回包含用户名和密码的元组；否则返回 None。
async fn find_device_mapping_up(
    wrapper: MutexGuard<'static, RedisWrapper>,
    device_id: &str,
) -> Option<(String, String)> {
    if let Some(string) = wrapper.get_hash("auth:coap", device_id).await.unwrap() {
        let auth: Auth = from_str(&string).expect("Failed to deserialize Auth");
        Some((auth.username, auth.password))
    } else {
        None
    }
}

async fn get_uid(name: &str, remote_address: String) -> Option<String> {
    let redis = get_redis_instance().await.unwrap();
    let key = format!("coap_uid_f:{}", name);
    redis.get_hash(&key, remote_address.as_str()).await.unwrap()
}

async fn store_uid(name: &str, device_id: &str, remote_address: String) {
    let k1 = format!("coap_uid:{}", name);
    let k2 = format!("coap_uid_f:{}", name);
    let redis = get_redis_instance().await.unwrap();
    redis
        .set_hash(&k1, device_id, &remote_address)
        .await
        .expect("Failed to set hash");
    redis
        .set_hash(&k2, &remote_address, device_id)
        .await
        .expect("Failed to set hash");
}
