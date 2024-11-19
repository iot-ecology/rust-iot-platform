use common_lib::sql_utils::{CrudOperations, FilterInfo, FilterOperation, PaginationParams};

use crate::biz::signal_delay_waring_param_biz::SignalDelayWaringParamBiz;
use crate::db::db_model::SignalDelayWaringParam;
use common_lib::config::Config;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;

#[post("/signal-delay-waring-param/create", format = "json", data = "<data>")]
pub async fn create_signal_delay_waring_param(
    data: Json<SignalDelayWaringParam>,
    signal_delay_waring_param_api: &rocket::State<SignalDelayWaringParamBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    if data.signal_delay_waring_id.is_none() || data.signal_delay_waring_id.unwrap() == 0 {
        return Custom(
            Status::BadRequest,
            Json(json!({
                "status": "error",
                "message": "脚本必选"
            })),
        );
    }

    // Validate name
    if data.name.is_none() || data.name.as_ref().unwrap().is_empty() {
        return Custom(
            Status::BadRequest,
            Json(json!({
                "code": 40000,
                "status": "error",
                "message": "名称不能为空"
            })),
        );
    }

    // Get signal name from database
    let mut param = data.into_inner();
    if let Some(signal_id) = param.signal_id {
        if let Ok(signal) = signal_delay_waring_param_api.get_signal_by_id(signal_id).await {
            param.signal_name = signal.name;
        }
    }

    // Create record in database
    match signal_delay_waring_param_api.create(param).await {
        Ok(u) => {
            signal_delay_waring_param_api.push_to_redis(&u).await.expect("TODO: panic message");

            Custom(
                Status::Ok,
                Json(json!({
                "code": 20000,

                    "status": "success",
                    "data": u
                })),
            )
        }
        Err(e) => Custom(
            Status::InternalServerError,
            Json(json!({
                "code": 40000,
                "status": "error",
                "message": e.to_string()
            })),
        ),
    }
}

#[post("/signal-delay-waring-param/update", format = "json", data = "<data>")]
pub async fn update_signal_delay_waring_param(
    data: Json<SignalDelayWaringParam>,
    signal_delay_waring_param_api: &rocket::State<SignalDelayWaringParamBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    // Check if ID is provided
    let id = match data.id {
        Some(id) => id,
        None => return Custom(
            Status::BadRequest,
            Json(json!({
                "code": 40000,
                "status": "error",
                "message": "ID is required"
            })),
        ),
    };

    // Get existing record
    let old_param = match signal_delay_waring_param_api.by_id(id).await {
        Ok(param) => param,
        Err(_) => return Custom(
            Status::NotFound,
            Json(json!({
                "code": 40000,
                "status": "error",
                "message": "SignalDelayWaringParam not found"
            })),
        ),
    };

    // Remove old value from Redis
    if let Err(e) = signal_delay_waring_param_api.remove_from_redis(&old_param).await {
        eprintln!("Failed to remove from Redis: {}", e);
    }

    // Create new parameter with updated values
    let mut new_param = old_param;
    new_param.device_uid = data.device_uid;
    new_param.protocol = data.protocol.clone();
    new_param.identification_code = data.identification_code.clone();
    new_param.name = data.name.clone();
    new_param.signal_name = data.signal_name.clone();
    new_param.signal_id = data.signal_id;

    // Update in database and Redis
    match signal_delay_waring_param_api.update(new_param.id.unwrap(), new_param).await {
        Ok(updated) => {
            // Push updated value to Redis
            if let Err(e) = signal_delay_waring_param_api.push_to_redis(&updated).await {
                eprintln!("Failed to push to Redis: {}", e);
            }

            Custom(
                Status::Ok,
                Json(json!({
                    "code": 20000,
                    "status": "success",
                    "data": updated
                })),
            )
        }
        Err(e) => Custom(
            Status::InternalServerError,
            Json(json!({
                "code": 40000,
                "status": "error",
                "message": e.to_string()
            })),
        ),
    }
}

#[get("/signal-delay-waring-param/page?<page>&<page_size>&<name>&<signal_delay_waring_id>")]
pub async fn page_signal_delay_waring_param(
    page: Option<i64>,
    page_size: Option<i64>,
    name: Option<String>,
    signal_delay_waring_id: Option<i64>,
    signal_delay_waring_param_api: &rocket::State<SignalDelayWaringParamBiz>,
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
    }, FilterInfo {
        field: "signal_delay_waring_id".to_string(),
        value: signal_delay_waring_id.unwrap_or_default().to_string(),
        operation: FilterOperation::AllLike,
        value2: None,
    }];

    let result = signal_delay_waring_param_api
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

#[post("/signal-delay-waring-param/delete/<id>")]
pub async fn delete_signal_delay_waring_param(
    id: i64,
    signal_delay_waring_param_api: &rocket::State<SignalDelayWaringParamBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let result = signal_delay_waring_param_api.delete(id).await;
    match result {
        Ok(o) => {
            let success_json = json!({
                "code": 20000,
                "message": "删除成功",
            });
            signal_delay_waring_param_api.remove_from_redis(&o).await;
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
