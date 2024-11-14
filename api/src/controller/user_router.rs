use crate::biz::user_biz::UserBiz;
use crate::db::db_model::User;
use common_lib::config::Config;
use common_lib::redis_pool_utils::RedisOp;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, State};
use serde_json::json;
use sqlx::{Error, MySqlPool};

#[get("/user/index")]
pub async fn user_index(
    user_biz: &rocket::State<UserBiz>,
    config: &State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    // 返回 Custom<Json<Value>>
    match user_biz.get_user_by_id(1).await {
        Ok(user) => {
            // 将 User 转换为 serde_json::Value
            let response_json = json!({
                "status": "success",
                "data": user
            });

            // 返回 Custom 状态和 JSON 内容
            Custom(rocket::http::Status::Ok, Json(response_json))
        }
        Err(e) => {
            // 错误时返回带有错误信息的 JSON
            let error_json = json!({
                "status": "error",
                "message": e.to_string()
            });

            Custom(rocket::http::Status::InternalServerError, Json(error_json))
        }
    }
}
