use common_lib::sql_utils::{CrudOperations, FilterInfo, FilterOperation, PaginationParams};

use crate::biz::signal_delay_waring_biz::SignalDelayWaringBiz;
use crate::db::db_model::{SignalDelayWaring, SignalDelayWaringParam};
use common_lib::config::Config;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;

#[post("/signal-delay-waring/create", format = "json", data = "<data>")]
pub async fn create_signal_delay_waring(
    data: Json<SignalDelayWaring>,
    signal_delay_waring_api: &rocket::State<SignalDelayWaringBiz>,
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

    match signal_delay_waring_api.create(data.into_inner()).await {
        Ok(u) => {
            let success_json = json!({
                "code": 20000,
                "message": "创建成功",
                "data": u
            });
            signal_delay_waring_api.set_redis(&u).await;
            signal_delay_waring_api.init_mongo_collection(&u).await;
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
}

#[post("/signal-delay-waring/update", format = "json", data = "<data>")]
pub async fn update_signal_delay_waring(
    data: Json<SignalDelayWaring>,
    signal_delay_waring_api: &rocket::State<SignalDelayWaringBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    // First check if the record exists
    let x = signal_delay_waring_api.by_id(data.id.unwrap()).await;

    match x {
        Ok(mut old) => {
            // Update the fields
            old.name = data.name.clone();
            old.script = data.script.clone();

            // Attempt to update the record
            let result = signal_delay_waring_api.update(old.id.unwrap(), old).await;
            match result {
                Ok(updated) => {
                    let success_json = json!({
                        "code": 20000,
                        "message": "更新成功",
                        "data": updated
                    });

                    // Update Redis cache
                    signal_delay_waring_api.set_redis(&updated);

                    // Update MongoDB collection
                    if let Err(e) = signal_delay_waring_api
                        .init_mongo_collection(&updated)
                        .await
                    {
                        log::error!("Failed to initialize MongoDB collection: {:?}", e);
                    }

                    Custom(Status::Ok, Json(success_json))
                }
                Err(e) => {
                    log::error!("Failed to update signal delay warning: {:?}", e);
                    let error_json = json!({
                        "code": 40000,
                        "message": "更新失败",
                        "data": e.to_string()
                    });
                    Custom(Status::InternalServerError, Json(error_json))
                }
            }
        }
        Err(e) => {
            log::error!("Failed to find signal delay warning: {:?}", e);
            let error_json = json!({
                "code": 40000,
                "message": "记录不存在",
                "data": e.to_string()
            });
            Custom(Status::NotFound, Json(error_json))
        }
    }
}

#[post("/signal-delay-waring/delete/<id>")]
pub async fn delete_signal_delay_waring(
    id: u64,
    signal_delay_waring_api: &rocket::State<SignalDelayWaringBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let result = signal_delay_waring_api.delete(id).await;
    match result {
        Ok(o) => {
            let success_json = json!({
                "code": 20000,
                "message": "删除成功",
            });
            signal_delay_waring_api.remove_redis(id).await;

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

#[post("/signal-delay-waring/Mock/<id>")]
pub async fn mock_signal_delay_waring(
    id: u64,
    signal_delay_waring_api: &rocket::State<SignalDelayWaringBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/signal-delay-waring/GenParam/<id>")]
pub async fn gen_param_signal_delay_waring(
    id: u64,
    signal_delay_waring_api: &rocket::State<SignalDelayWaringBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/signal-delay-waring/query-row")]
pub async fn query_waring_list(
    signal_delay_waring_api: &rocket::State<SignalDelayWaringBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[get("/signal-delay-waring/list")]
pub async fn list_signal_delay_waring(
    signal_delay_waring_api: &rocket::State<SignalDelayWaringBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let result = signal_delay_waring_api.list(Vec::new()).await;
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

#[get("/signal-delay-waring/byId/<id>")]
pub async fn by_id_signal_delay_waring(
    id: u64,
    signal_delay_waring_api: &rocket::State<SignalDelayWaringBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let result = signal_delay_waring_api.by_id(id).await;
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

#[get("/signal-delay-waring/page?<page>&<page_size>&<name>")]
pub async fn page_signal_delay_waring(
    page: Option<u64>,
    page_size: Option<u64>,
    name: Option<String>,
    signal_delay_waring_api: &rocket::State<SignalDelayWaringBiz>,
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
    }

    let filters = vec![FilterInfo {
        field: "name".to_string(),
        value: name.unwrap_or_else(String::new),
        operation: FilterOperation::AllLike,
        value2: None,
    }];

    let result = signal_delay_waring_api
        .page(
            filters,
            PaginationParams {
                page,
                size: page_size,
            },
        )
        .await;

    match result {
        Ok(data) => {
            let success_json = json!({
                "code": 20000,
                "message": "查询成功",
                "data": data
            });
            Custom(Status::Ok, Json(success_json))
        }
        Err(_) => {
            let error_json = json!({
                "code": 40000,
                "message": "查询失败"
            });
            Custom(Status::Ok, Json(error_json))
        }
    }
}
