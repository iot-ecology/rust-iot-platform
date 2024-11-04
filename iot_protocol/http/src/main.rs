#[macro_use]
extern crate rocket;

use common_lib::models::{Auth, HttpMessage};
use common_lib::protocol_config::{get_config, read_config};
use common_lib::rabbit_utils::{get_rabbitmq_instance, init_rabbitmq_with_config};
use common_lib::redis_handler::{get_redis_instance, init_redis, RedisWrapper};
use log::info;
use rocket::http::{Header, Status};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::{json::Json, Deserialize, Serialize};
use serde_json::{from_str, json};
use std::sync::Mutex;
use tokio::runtime::Runtime;
use tokio::sync::MutexGuard;

/// 定义根路由，返回一个简单的欢迎消息
/// 创建根路由
///
/// # Returns
///
/// 返回一个静态字符串 "Hello, world!"。
#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

/// 初始化日志记录器
fn init_logger() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
}

/// 定义一个测试路由，返回 pong
/// 创建测试路由
///
/// # Returns
///
/// 返回状态为 200 和 JSON 格式的 "pong" 字符串。
#[get("/ping")]
fn ping() -> (Status, Json<&'static str>) {
    (Status::Ok, Json("pong"))
}

/// 定义请求参数结构体
/// 定义请求参数的结构体
///
/// # Fields
///
/// * `data` - 请求体中的数据。
#[derive(Debug, Deserialize)]
struct Param {
    data: String,
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
    if let Some(string) = wrapper.get_hash("auth:http", device_id).await.unwrap() {
        let auth: Auth = from_str(&string).expect("Failed to deserialize Auth");
        Some((auth.username, auth.password))
    } else {
        None
    }
}

/// 解析基本身份验证信息
///
/// # Arguments
///
/// * `auth_header` - 包含基本身份验证信息的头部字符串。
///
/// # Returns
///
/// 如果成功，返回用户名和密码的元组；否则返回 None。
fn parse_basic_auth(auth_header: &str) -> Option<(String, String)> {
    if let Some(encoded) = auth_header.strip_prefix("Basic ") {
        let decoded = base64::decode(encoded).ok()?;
        let decoded_str = String::from_utf8(decoded).ok()?;
        let parts: Vec<&str> = decoded_str.split(':').collect();
        if parts.len() == 2 {
            return Some((parts[0].to_string(), parts[1].to_string()));
        }
    }
    None
}

/// 认证信息和设备结构体
struct AuthAndDevice {
    username: String,
    password: String,
    device_id: Option<String>,
}

/// 从请求中提取认证信息和设备 ID
#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthAndDevice {
    type Error = &'static str;

    /// 从请求中提取认证信息和设备 ID
    ///
    /// # Arguments
    ///
    /// * `request` - 请求对象。
    ///
    /// # Returns
    ///
    /// 如果成功，返回 AuthAndDevice 结构体；否则返回错误信息。
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let auth_header = request.headers().get_one("Authorization");
        let device_id = request.headers().get_one("device_id").map(String::from);

        if let Some(auth) = auth_header {
            if let Some((username, password)) = parse_basic_auth(auth) {
                return Outcome::Success(AuthAndDevice {
                    username,
                    password,
                    device_id,
                });
            }
        }

        Outcome::Error((
            Status::Unauthorized,
            "Authorization header missing or malformed",
        ))
    }
}

/// 处理消息的路由
/// 处理消息请求
///
/// # Arguments
///
/// * `auth` - 提取的认证信息。
/// * `param` - 请求参数，包含数据。
///
/// # Returns
///
/// 如果成功，返回 JSON 格式的响应；否则返回状态错误。
#[post("/handler", format = "json", data = "<param>")]
async fn handler_message(
    auth: AuthAndDevice,
    param: Json<Param>,
) -> Result<Json<serde_json::Value>, Status> {
    let device_id = match &auth.device_id {
        Some(id) => id,
        None => return Err(Status::BadRequest), // 如果没有 device_id 返回错误
    };

    let redis = get_redis_instance().await.unwrap();

    // 在 Redis 中查找设备映射
    let x = find_device_mapping_up(redis, device_id).await.unwrap();

    // 检查用户名和密码是否匹配
    if auth.username != x.0 || auth.password != x.1 {
        return Err(Status::Unauthorized);
    }

    // 创建消息并转换为 JSON 字符串
    let mqtt_msg = HttpMessage {
        uid: device_id.clone(),
        message: param.data.clone(),
    };
    let json_data = mqtt_msg.to_json_string();
    debug!("Pushing message to queue: {}", json_data);

    // 获取 RabbitMQ 实例并发布消息
    let rabbit = get_rabbitmq_instance().await.unwrap();

    rabbit
        .publish("", "pre_http_handler", json_data.as_str())
        .await
        .expect("publish message failed");

    Ok(Json(serde_json::json!({ "message": "成功获取数据" }))) // 返回成功消息
}

/// 启动 Rocket 应用
/// 启动应用并配置服务
///
/// # Returns
///
/// 返回 Rocket 应用实例。
#[launch]
fn rocket() -> _ {
    init_logger(); // 初始化日志记录

    // 创建异步运行时
    let rt = Runtime::new().unwrap();

    // 读取配置并初始化 Redis 和 RabbitMQ
    let config = rt.block_on(read_config("app-local.yml")).unwrap(); // 读取配置
    let guard1 = rt.block_on(get_config()).unwrap(); // 获取配置
    rt.block_on(init_redis(guard1.redis_config.clone()))
        .unwrap(); // 初始化 Redis
    rt.block_on(init_rabbitmq_with_config(guard1.mq_config.clone()))
        .unwrap(); // 初始化 RabbitMQ

    // 构建并启动 Rocket 应用
    rocket::build()
        .configure(rocket::Config {
            port: guard1.node_info.port,
            ..Default::default()
        })
        .mount("/", routes![index, ping, handler_message]) // 挂载路由
}
