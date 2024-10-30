use common_lib::redis_handler::{get_redis_instance, RedisWrapper};
use rocket::async_trait;
use rocket::http::{Header, Status};
use rocket::post;
use rocket::request::{FromRequest, Outcome, Request};
use rocket::response::status;
use rocket::serde::{json::Json, Deserialize};
use rocket::tokio::sync::Mutex;
use serde_json::from_str;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::MutexGuard;
use uuid::Uuid;

/// 认证信息结构体，包含用户名和密码
#[derive(Debug, Deserialize)]
pub struct Auth {
    pub username: String,
    pub password: String,
}

/// 包含认证信息和设备 ID 的结构体
#[derive(Debug)]
pub struct AuthAndDevice {
    pub username: String,
    pub password: String,
    pub device_id: Option<String>,
}

/// 共享状态管理
pub struct AppState {
    pub device_map: Mutex<HashMap<String, Auth>>,
    pub max_connections: usize,
    pub current_connections: Mutex<usize>,
}

/// 解析 Basic 认证信息
fn parse_basic_auth(auth: &str) -> Option<(String, String)> {
    if let Some(encoded) = auth.strip_prefix("Basic ") {
        if let Ok(decoded) = base64::decode(encoded) {
            if let Ok(credentials) = String::from_utf8(decoded) {
                let mut split = credentials.splitn(2, ':');
                if let (Some(username), Some(password)) = (split.next(), split.next()) {
                    return Some((username.to_string(), password.to_string()));
                }
            }
        }
    }
    None
}

/// 实现从请求中提取 `AuthAndDevice` 的逻辑
#[async_trait]
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

#[post("/auth", format = "application/json")]
pub async fn auth_api(
    auth_and_device: AuthAndDevice,
) -> Result<Json<serde_json::Value>, status::Custom<Json<&'static str>>> {
    let device_id = match &auth_and_device.device_id {
        Some(id) => id,
        None => {
            return Err(status::Custom(
                Status::Unauthorized,
                Json(&"请求头 Device-ID 丢失"),
            ))
        }
    };

    let redis = get_redis_instance().await.unwrap();

    // 在 Redis 中查找设备映射
    let x = find_device_mapping_up(redis, device_id).await.unwrap();

    if auth_and_device.username != x.0 || auth_and_device.password != x.1 {
        return Err(status::Custom(Status::Unauthorized, Json(&"账号密码错误")));
    }

    let uid = Uuid::new_v4();
    let storage_id = format!("{}@{}", device_id, uid);

    // 创建 JSON 响应对象
    let response = serde_json::json!({
        "message": "认证通过",
        "uid": storage_id,
    });

    // 返回 JSON 格式的响应
    Ok(Json(response))
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
    if let Some(string) = wrapper.get_hash("auth:ws", device_id).await.unwrap() {
        let auth: common_lib::models::Auth = from_str(&string).expect("Failed to deserialize Auth");
        Some((auth.username, auth.password))
    } else {
        None
    }
}
