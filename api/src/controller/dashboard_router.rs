use crate::biz::dashboard_biz::DashboardBiz;
use crate::biz::mqtt_client_biz::MqttClientBiz;
use crate::db::db_model::Dashboard;
use common_lib::config::Config;
use common_lib::sql_utils::{CrudOperations, FilterInfo, FilterOperation, PaginationParams};
use log::error;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;

#[post("/dashboard/create", format = "json", data = "<data>")]
pub async fn create_dashboard(
    data: Json<Dashboard>,
    dashboard_api: &rocket::State<DashboardBiz>,
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
    if data.config.clone().is_none() {
        let error_json = json!({
            "code": 40000,
            "message": "操作失败",
            "data": "配置不能为空"
        });
        return Custom(Status::InternalServerError, Json(error_json));
    }

    match dashboard_api.find_by_name(data.name.clone()).await {
        Ok(u) => {
            if u.is_none() {
                match dashboard_api.create(data.into_inner()).await {
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

#[post("/dashboard/update", format = "json", data = "<data>")]
pub async fn update_dashboard(
    data: Json<Dashboard>,
    dashboard_api: &rocket::State<DashboardBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let x = dashboard_api.by_id(data.id.unwrap()).await;
    match x {
        Ok(mut old) => {
            old.name = data.name.clone();
            old.config = data.config.clone();
            let result = dashboard_api.update(old.id.unwrap(), old).await;
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

#[get("/dashboard/<id>")]
pub async fn by_id_dashboard(
    id: i64,
    dashboard_api: &rocket::State<DashboardBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let result = dashboard_api.delete(id).await;
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

#[get("/dashboard/page?<page>&<page_size>&<name>")]
pub async fn page_dashboard(
    page: Option<i64>,
    page_size: Option<i64>,
    name: Option<String>,
    dashboard_api: &rocket::State<DashboardBiz>,
    config: &rocket::State<Config>,
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

    let result = dashboard_api.page(
        filters,
        PaginationParams {
            page: page,
            size: page_size,
        },
    ).await;

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

#[post("/dashboard/delete/<id>")]
pub async fn delete_dashboard(
    id: i64,
    dashboard_api: &rocket::State<DashboardBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let result = dashboard_api.delete(id).await;
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
