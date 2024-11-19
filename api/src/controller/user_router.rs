use crate::biz::user_biz::UserBiz;
use crate::db::db_model::User;
use common_lib::config::Config;
use common_lib::sql_utils::{CrudOperations, FilterInfo, FilterOperation, PaginationParams};
use log::error;
use log::info;
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

#[post("/User/create", format = "json", data = "<data>")]
pub async fn create_user(
    data: Json<User>,
    user_api: &rocket::State<UserBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    info!("user = {:?}", data);

    if data.username.clone().is_none() {
        let error_json = json!({
            "code": 40000,
            "message": "操作失败",
            "data": "名称不能为空"
        });
        return Custom(Status::InternalServerError, Json(error_json));
    }

    match user_api.find_by_username(data.username.clone()).await {
        Ok(u) => {
            if u.is_none() {
                match user_api.create(data.into_inner()).await {
                    Ok(u) => {
                        let success_json = json!({
                            "code": 20000,
                            "message": "创建成功",
                            "data": u
                        });
                        Custom(Status::Ok, Json(success_json))
                    }
                    Err(_) => {
                        let error_json = json!({
                            "code": 40000,
                            "message": "创建失败"
                        });
                        Custom(Status::InternalServerError, Json(error_json))
                    }
                }
            } else {
                let error_json = json!({
                    "code": 40000,
                    "message": "用户已存在"
                });
                Custom(Status::InternalServerError, Json(error_json))
            }
        }
        Err(e) => {
            error!("error =  {:?}", e);

            let error_json = json!({
                "code": 40000,
                "message": "查询失败"
            });
            Custom(Status::InternalServerError, Json(error_json))
        }
    }
}

#[post("/User/update", format = "json", data = "<data>")]
pub async fn update_user(
    data: Json<User>,
    user_api: &rocket::State<UserBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let x = user_api.by_id(data.id.unwrap()).await;

    match x {
        Ok(mut u) => {
            u.password = data.password.clone();
            u.email = data.email.clone();
            let result = user_api.update(u.id.unwrap(), u).await;
            match result {
                Ok(u2) => {
                    let success_json = json!({
                        "code": 20000,
                        "message": "更新成功",
                        "data": u2
                    });
                    Custom(Status::Ok, Json(success_json))
                }
                Err(e) => {
                    error!("error =  {:?}", e);

                    let error_json = json!({
                        "code": 40000,
                        "message": "查询失败"
                    });
                    Custom(Status::InternalServerError, Json(error_json))
                }
            }
        }
        Err(e) => {
            error!("error =  {:?}", e);

            let error_json = json!({
                "code": 40000,
                "message": "查询失败"
            });
            Custom(Status::InternalServerError, Json(error_json))
        }
    }
}

#[get("/User/page?<username>&<page>&<page_size>")]
pub async fn page_user(
    user_api: &rocket::State<UserBiz>,
    config: &rocket::State<Config>,
    username: Option<String>,
    page: Option<i64>,
    page_size: Option<i64>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let page = page.unwrap_or(1);
    let page_size = page_size.unwrap_or(10);

    if page == 0 || page_size == 0 {
        let error_json = json!({
                           "code": 40000,

            "message": "Invalid page or page_size parameters"
        });
        return Custom(Status::Ok, Json(error_json));
    };

    let filters = vec![FilterInfo {
        field: "username".to_string(),
        value: username.unwrap_or_else(String::new),
        operation: FilterOperation::AllLike,
        value2: None,
    }];

    let result = user_api
        .page(
            filters,
            PaginationParams {
                page: page,
                size: page_size,
            },
        )
        .await;

    match result {
        Ok(uu) => {
            let success_json = json!({
                "code": 20000,
                "message": "查询成功",
                "data": uu
            });
            Custom(Status::Ok, Json(success_json))
        }
        Err(e) => {
            let error_json = json!({
                               "code": 40000,

                    "message": "查询失败"

            });
            return Custom(Status::Ok, Json(error_json));
        }
    }
}

#[post("/User/delete/<id>")]
pub async fn delete_user(
    id: i64,
    user_api: &rocket::State<UserBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let result = user_api.delete(id).await;
    match result {
        Ok(o) => {
            let success_json = json!({
                "code": 20000,
                "message": "删除成功",
            });
            Custom(Status::Ok, Json(success_json))
        }
        Err(e) => {
            let success_json = json!({
                "code": 40000,
                "message": "删除失败",
            });
            Custom(Status::Ok, Json(success_json))
        }
    }
}

#[get("/User/<id>")]
pub async fn by_id_user(
    id: i64,
    user_api: &rocket::State<UserBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let result = user_api.by_id(id).await;
    match result {
        Ok(u) => {
            let success_json = json!({
                        "code": 20000,
                        "message": "查询成功",
            "data":u
                    });
            Custom(Status::Ok, Json(success_json))
        }
        Err(e) => {
            let success_json = json!({
                "code": 40000,
                "message": "查询失败",
            });
            Custom(Status::Ok, Json(success_json))
        }
    }
}

#[get("/User/list")]
pub async fn list_user(
    user_api: &rocket::State<UserBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let result = user_api.list(Vec::new()).await;
    match result {
        Ok(u) => {
            let success_json = json!({
                        "code": 20000,
                        "message": "查询成功",
            "data":u
                    });
            Custom(Status::Ok, Json(success_json))
        }
        Err(e) => {
            let success_json = json!({
                "code": 40000,
                "message": "查询失败",
            });
            Custom(Status::Ok, Json(success_json))
        }
    }
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
