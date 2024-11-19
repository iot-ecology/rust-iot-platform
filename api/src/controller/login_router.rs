use crate::biz::user_biz::UserBiz;
use crate::AuthToken;
use chrono::{Duration, Utc};
use common_lib::config::Config;
use jsonwebtoken::errors::Error;
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use log::info;
use rocket::{
    fairing::AdHoc,
    http::Status,
    post,
    request::{FromRequest, Outcome},
    serde::json::Json,
    State,
};
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::sync::Arc;

/// 登录参数
#[derive(Serialize, Deserialize, Debug)]
pub struct LoginParam {
    #[serde(rename = "user_name")]
    pub user_name: String,

    #[serde(rename = "password")]
    pub password: String,
}

#[post("/login", data = "<login_param>")]
pub async fn login(
    login_param: Json<LoginParam>,
    user_api: &rocket::State<UserBiz>,
    config: &rocket::State<Config>,
) -> Result<Json<LoginResponse>, Status> {
    if let Some(user) = user_api
        .find_user_with_pwd(login_param.user_name.clone(), login_param.password.clone())
        .await
    {
        let token = generate_token(
            user.id.unwrap(),
            user.username.clone().unwrap(),
            vec![1, 2, 3],
        )
            .map_err(|_| Status::InternalServerError)?;
        Ok(Json(LoginResponse {
            token,
            uid: user.id.unwrap(),
            username: user.username.clone().unwrap(),
        }))
    } else {
        Err(Status::Unauthorized)
    }
}

/// 生成 JWT Token
fn generate_token(
    uid: u64,
    user_name: String,
    role_ids: Vec<u64>,
) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("valid timestamp")
        .timestamp() as usize;

    let claims = MyClaims {
        uid,
        user_name: user_name.to_string(),
        role_ids,
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET.as_ref()),
    )
}

/// 配置文件或常量
const JWT_SECRET: &str = "a_secret_test123456";
/// 解析 JWT Token
fn parse_token(token: &str) -> Result<MyClaims, jsonwebtoken::errors::Error> {
    let decoded = decode::<MyClaims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET.as_ref()),
        &Validation::default(),
    )?;
    Ok(decoded.claims)
}

/// JWT Claims
#[derive(Debug, Serialize, Deserialize)]
struct MyClaims {
    uid: u64,
    user_name: String,
    role_ids: Vec<u64>,
    exp: usize,
}

/// 登录 API 的响应
#[derive(Serialize)]
struct LoginResponse {
    token: String,
    uid: u64,
    username: String,
}
use rocket::response::status::Custom;

#[post("/userinfo")]
pub async fn userinfo(
    auth_token: AuthToken,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let token_value = auth_token.0;
    format!("Authorization token is: {}", token_value);
    let result = parse_token(&token_value);
    match result {
        Ok(u) => {
            let error_json = json!({
                "code": 20000,
                "message": "操作成功",
                "data": u
            });
            return Custom(Status::Ok, Json(error_json));
        }
        Err(e) => {
            let error_json = json!({
                "code": 40000,
                "message": "操作失败",
                "data": "名称不能为空"
            });
            return Custom(Status::InternalServerError, Json(error_json));
        }
    }
}
