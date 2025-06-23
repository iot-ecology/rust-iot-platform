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
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    if let Some(user) = user_api
        .find_user_with_pwd(login_param.user_name.clone(), login_param.password.clone())
        .await
    {
        let token = generate_token(
            user.id.unwrap(),
            user.username.clone().unwrap(),
            vec![1, 2, 3],
        ).unwrap();

        let error_json = json!({
                "code": 20000,
                "message": "操作成功",
                "data": LoginResponse {
            token,
            uid: user.id.unwrap(),
            username: user.username.clone().unwrap(),
        }
            });
        Custom(Status::Ok, Json(error_json))

    } else {
        let error_json = json!({
                "code": 40000,
                "message": "登录失败"
            });
        Custom(Status::InternalServerError, Json(error_json))

    }
}

/// 生成 JWT Token
fn generate_token(
    uid: i64,
    user_name: String,
    role_ids: Vec<i64>,
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
    uid: i64,
    user_name: String,
    role_ids: Vec<i64>,
    exp: usize,
}

/// 登录 API 的响应
#[derive(Serialize)]
struct LoginResponse {
    token: String,
    uid: i64,
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
