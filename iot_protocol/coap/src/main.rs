use coap::Server;
use coap_lite::{CoapRequest, RequestType as Method};
use common_lib::config::{get_config, read_config};
use common_lib::models::{Auth, CoapMessage};
use common_lib::rabbit_utils::{get_rabbitmq_instance, init_rabbitmq_with_config};
use common_lib::redis_handler::{get_redis_instance, init_redis, RedisWrapper};
use log::{debug, info};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::net::SocketAddr;
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;
use tokio::sync::MutexGuard;

fn init_logger() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
}

fn main() {
    let addr = "127.0.0.1:5683";
    let runtime = Runtime::new().unwrap();

    init_logger();
    let rt = Runtime::new().unwrap(); // 创建异步运行时

    let rt = Runtime::new().unwrap();

    // 读取配置并初始化 Redis 和 RabbitMQ
    let config = rt.block_on(read_config("app-local.yml")).unwrap(); // 读取配置

    let guard1 = rt.block_on(get_config()).unwrap(); // 获取配置
    let node_info_name = guard1.node_info.name.clone(); // 克隆 node_info.name

    // 初始化 Redis 和 RabbitMQ
    rt.block_on(init_redis(guard1.redis_config.clone()))
        .unwrap();
    rt.block_on(init_rabbitmq_with_config(guard1.mq_config.clone()))
        .unwrap();

    runtime.block_on(async move {
        let mut server = Server::new_udp(addr).unwrap();
        println!("Server up on {}", addr);

        server
            .run(move |request: Box<CoapRequest<SocketAddr>>| {
                let request = Arc::new(Mutex::new(request));
                let node_info_name = node_info_name.clone();

                async move {
                    let mut request = request.lock().await; // 获取锁
                    let method = request.get_method();

                    let payload = String::from_utf8(request.message.payload.clone()).unwrap();

                    let remote_address = request.source.unwrap().to_string();

                    let rel_adds = remote_address.replace(":", "@");

                    match method {
                        Method::Get => {
                            println!("Received GET request for path: {}", request.get_path());
                            match request.get_path().as_str() {
                                "auth" => {
                                    let x = handle_auth(payload, node_info_name, rel_adds).await;

                                    if let Some(ref mut message) = request.response {
                                        message.message.payload = x.to_string().into();
                                    }
                                }
                                "data" => {
                                    let x = handle_data(payload, node_info_name, rel_adds).await;
                                    if let Some(ref mut message) = request.response {
                                        message.message.payload = x.to_string().into();
                                    }
                                }
                                "/other" => handle_other(request.clone()).await,
                                _ => {
                                    if let Some(ref mut message) = request.response {
                                        message.message.payload = b"404 Not Found".to_vec();
                                    }
                                }
                            }
                        }
                        Method::Post => {
                            let payload = String::from_utf8_lossy(&request.message.payload);
                            println!("Received POST request with payload: {}", payload);
                            if let Some(ref mut message) = request.response {
                                message.message.payload = b"POST response".to_vec();
                            }
                        }
                        Method::Put => {
                            let payload = String::from_utf8_lossy(&request.message.payload);
                            println!("Received PUT request with payload: {}", payload);
                            if let Some(ref mut message) = request.response {
                                message.message.payload = b"PUT response".to_vec();
                            }
                        }
                        _ => {
                            println!("Received request with unsupported method");
                            if let Some(ref mut message) = request.response {
                                message.message.payload = b"Method Not Allowed".to_vec();
                            }
                        }
                    };

                    request.clone() // 返回请求
                }
            })
            .await
            .unwrap();
    });
}
#[derive(Serialize, Deserialize, Debug)]
struct AuthAndDevice {
    username: String,
    password: String,
    device_id: Option<String>,
}
// 处理 /auth 路径的函数
async fn handle_auth(payload: String, name: String, rel_adds: String) -> &'static str {
    // 处理认证逻辑
    println!("Handling authentication logic.");

    let redis = get_redis_instance().await.unwrap();

    info!("auth");
    let auth_device: Result<AuthAndDevice, _> = from_str(&payload);
    return match auth_device {
        Ok(auth_data) => {
            let device_id = auth_data.device_id.unwrap();
            let id = device_id.as_str();

            let x = find_device_mapping_up(redis, id.clone())
                .await
                .unwrap_or((String::new(), String::new()));
            if x.0 == auth_data.username && x.1 == auth_data.password {
                store_uid(name.as_str(), id.clone(), rel_adds).await;

                "认证通过"
            } else {
                "认证失败"
            }
        }
        Err(e) => {
            info!("Failed to parse payload as AuthAndDevice: {}", e);
            "认证失败"
        }
    };
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

// 处理 /data 路径的函数
async fn handle_data(payload: String, name: String, rel_adds: String) -> &'static str {
    let option = get_uid(name.as_str(), rel_adds).await;
    if option.is_none() {
        return "处理失败";
    } else {
        let msg = CoapMessage {
            uid: option.unwrap(),
            message: payload,
        };
        let json_data = msg.to_json_string();
        debug!("Pushing message to queue: {}", json_data);

        // 获取 RabbitMQ 实例并发布消息
        let rabbit = get_rabbitmq_instance().await.unwrap();

        let guard = rabbit.lock().await;
        guard
            .publish("", "pre_coap_handler", json_data.as_str())
            .await
            .expect("publish message failed");

        return "处理成功";
    }
}
async fn get_uid(name: &str, remote_address: String) -> Option<String> {
    let redis = get_redis_instance().await.unwrap();
    let key = format!("coap_uid_f:{}", name);
    redis.get_hash(&key, remote_address.as_str()).await.unwrap()
}
// 处理 /other 路径的函数
async fn handle_other(mut request: Box<CoapRequest<SocketAddr>>) {
    // 处理其他逻辑
    println!("Handling other logic.");
    // 返回响应
    if let Some(ref mut message) = request.response {
        message.message.payload = b"Other response".to_vec();
    }
}
