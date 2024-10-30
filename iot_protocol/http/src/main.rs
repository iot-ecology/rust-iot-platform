#[macro_use]
extern crate rocket;

use common_lib::models::Auth;
use common_lib::protocol_config::read_config;
use common_lib::redis_handler::RedisWrapper;
use log::info;
use rocket::http::{Header, Status};
use rocket::request::{FromRequest, Outcome, Request};
use rocket::serde::{json::Json, Deserialize, Serialize};
use serde_json::{from_str, json};
use std::sync::Mutex;

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}
fn init_logger() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
}
#[get("/ping")]
fn ping() -> (Status, Json<&'static str>) {
    (Status::Ok, Json("pong"))
}

// 定义传入参数结构
#[derive(Debug, Deserialize)]
struct Param {
    data: String,
}

// 定义消息结构
#[derive(Debug, Serialize)]
struct HttpMessage {
    uid: String,
    message: String,
}

async fn find_device_mapping_up(wrapper: RedisWrapper, device_id: &str) -> (String, String) {
    if let Some(string) = wrapper.get_hash("auth:http", device_id).await.unwrap() {
        let auth: Auth = from_str(&string).expect("Failed to deserialize Auth");
        (auth.username, auth.password)
    } else {
        (String::new(), String::new())
    }
}

// BasicAuth 解析实现
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

// 自定义请求授权和设备 ID 结构
struct AuthAndDevice {
    username: String,
    password: String,
    device_id: Option<String>,
}

// 从请求头中提取授权和 device_id
#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthAndDevice {
    type Error = &'static str;

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

// 处理消息的主要路由
#[post("/handler", format = "json", data = "<param>")]
async fn handler_message(
    auth: AuthAndDevice,
    param: Json<Param>,
) -> Result<Json<serde_json::Value>, Status> {
    let device_id = match &auth.device_id {
        Some(id) => id,
        None => return Err(Status::BadRequest), // 如果 device_id 为空，返回错误
    };

    // 查找与 device_id 关联的用户名和密码
    let (username, password) = match find_device_mapping_up(device_id) {
        Some(up) => up,
        None => return Err(Status::Unauthorized),
    };

    // 验证用户名和密码
    if auth.username != username || auth.password != password {
        return Err(Status::Unauthorized);
    }

    // 创建 HttpMessage 并转换为 JSON
    let mqtt_msg = HttpMessage {
        uid: device_id.clone(),
        message: param.data.clone(),
    };
    let json_data = serde_json::to_string("&mqtt_msg").map_err(|_| Status::InternalServerError)?;

    // 模拟将消息推送到队列
    println!("Pushing message to queue: {}", json_data);

    Ok(Json(serde_json::json!({ "message": "成功获取数据" })))
}

#[launch]
fn rocket() -> _ {
    init_logger();
    let result = read_config("app-local.yml").unwrap();
    let redis_wrapper = RedisWrapper::new(result.redis_config).unwrap();

    rocket::build()
        .configure(rocket::Config {
            port: result.node_info.port,
            ..Default::default()
        })
        .mount("/", routes![index, ping, handler_message])
}
