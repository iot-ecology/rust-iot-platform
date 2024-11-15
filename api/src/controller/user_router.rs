use crate::biz::user_biz::UserBiz;
use common_lib::config::Config;
use rocket::http::Status;
use rocket::post;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, State};
use serde_json::json;

#[get("/user/index")]
pub async fn user_index(
    user_biz: &rocket::State<UserBiz>,
    config: &State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    // 返回 Custom<Json<Value>>
    match user_biz.get_user_by_id(13).await {
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

#[post("/User/create")]
pub async fn create_user(
    user_api: &rocket::State<UserBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/User/update")]
pub async fn update_user(
    user_api: &rocket::State<UserBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/User/page")]
pub async fn page_user(
    user_api: &rocket::State<UserBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/User/delete/<id>")]
pub async fn delete_user(
    id: u64,
    user_api: &rocket::State<UserBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/User/<id>")]
pub async fn by_id_user(
    id: u64,
    user_api: &rocket::State<UserBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/User/list")]
pub async fn list_user(
    user_api: &rocket::State<UserBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/User/BindRole")]
pub async fn bind_role(
    user_api: &rocket::State<UserBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/User/BindDept")]
pub async fn bind_dept(
    user_api: &rocket::State<UserBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/User/QueryBindRole")]
pub async fn query_bind_role(
    user_api: &rocket::State<UserBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/User/QueryBindDept")]
pub async fn query_bind_dept(
    user_api: &rocket::State<UserBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/User/BindDeviceInfo")]
pub async fn bind_device_info(
    user_api: &rocket::State<UserBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/User/QueryBindDeviceInfo")]
pub async fn query_bind_device_info(
    user_api: &rocket::State<UserBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}
