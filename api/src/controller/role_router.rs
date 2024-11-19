use crate::biz::role_biz::RoleBiz;
use crate::db::db_model::Role;
use common_lib::config::Config;
use common_lib::sql_utils::{CrudOperations, FilterInfo, FilterOperation, PaginationParams};
use log::error;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;

#[post("/Role/create", format = "json", data = "<data>")]
pub async fn create_role(
    data: Json<Role>,
    role_api: &rocket::State<RoleBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    if data.name.clone().is_none() {
        let error_json = json!({
            "code": 40000,
            "message": "操作失败",
            "data": "名称不能为空"
        });
        return Custom(Status::InternalServerError, Json(error_json));
    }

    match role_api.find_by_name(data.name.clone()).await {
        Ok(u) => {
            if u.is_none() {
                match role_api.create(data.into_inner()).await {
                    Ok(u) => {
                        let success_json = json!({
                            "code": 20000,
                            "message": "创建成功",
                            "data": u
                        });
                        Custom(Status::Ok, Json(success_json))
                    }
                    Err(e) => {
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
                    "message": "角色已存在"
                });
                Custom(Status::InternalServerError, Json(error_json))
            }
        }
        Err(e) => {
            let error_json = json!({
                "code": 40000,
                "message": "查询失败"
            });
            Custom(Status::InternalServerError, Json(error_json))
        }
    }
}

#[post("/Role/update", format = "json", data = "<data>")]
pub async fn update_role(
    data: Json<Role>,
    role_api: &rocket::State<RoleBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let x = role_api.by_id(data.id.unwrap()).await;
    match x {
        Ok(mut old) => {
            old.description = data.description.clone();
            let result = role_api.update(old.id.unwrap(), old).await;
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

#[get("/Role/page?<name>&<page>&<page_size>")]
pub async fn page_role(
    role_api: &rocket::State<RoleBiz>,
    config: &rocket::State<Config>,
    name: Option<String>,
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
        field: "name".to_string(),
        value: name.unwrap_or_else(String::new),
        operation: FilterOperation::AllLike,
        value2: None,
    }];

    let result = role_api
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

#[post("/Role/delete/<id>")]
pub async fn delete_role(
    id: i64,
    role_api: &rocket::State<RoleBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let result = role_api.delete(id).await;
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

#[get("/Role/<id>")]
pub async fn by_id_role(
    id: i64,
    role_api: &rocket::State<RoleBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let result = role_api.by_id(id).await;
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

#[get("/Role/list")]
pub async fn list_role(
    role_api: &rocket::State<RoleBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let result = role_api.list(Vec::new()).await;
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
