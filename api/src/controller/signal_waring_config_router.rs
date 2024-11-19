use common_lib::sql_utils::{CrudOperations, FilterInfo, FilterOperation, PaginationParams};
use log::error;

use crate::biz::signal_waring_config_biz::SignalWaringConfigBiz;
use crate::db::db_model::{Signal, SignalDelayWaring, SignalWaringConfig};
use common_lib::config::Config;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;

#[post("/signal-waring-config/create", format = "json", data = "<data>")]
pub async fn create_signal_waring_config(
    data: Json<SignalWaringConfig>,
    signal_waring_config_api: &rocket::State<SignalWaringConfigBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    // Validate signal ID
    if data.signal_id.unwrap_or(0) < 1 {
        let error_json = json!({
            "code": 40000,
            "message": "信号ID必填"
        });
        return Custom(Status::BadRequest, Json(error_json));
    }

    // Create record in database
    match signal_waring_config_api.create(data.into_inner()).await {
        Ok(config) => {
            // Set cache
            if let Err(e) = signal_waring_config_api.set_signal_waring_cache(
                config.signal_id.unwrap(),
                &config,
            ).await {
                error!("Failed to set signal warning cache: {}", e);
                let error_json = json!({
                    "code": 40000,
                    "message": format!("Failed to set cache: {}", e)
                });
                return Custom(Status::InternalServerError, Json(error_json));
            }

            // Initialize MongoDB collection
            if let Err(e) = signal_waring_config_api.init_mongo_collection(&config).await {
                error!("Failed to initialize MongoDB collection: {}", e);
                let error_json = json!({
                    "code": 40000,
                    "message": format!("Failed to initialize MongoDB: {}", e)
                });
                return Custom(Status::InternalServerError, Json(error_json));
            }

            let success_json = json!({
                "code": 20000,
                "message": "创建成功",
                "data": config
            });
            Custom(Status::Ok, Json(success_json))
        }
        Err(e) => {
            error!("Failed to create signal warning config: {}", e);
            let error_json = json!({
                "code": 40000,
                "message": format!("创建失败: {}", e)
            });
            Custom(Status::InternalServerError, Json(error_json))
        }
    }
}

#[post("/signal-waring-config/delete/<id>")]
pub async fn delete_signal_waring_config(
    id: i64,
    signal_waring_config_api: &rocket::State<SignalWaringConfigBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/signal-waring-config/query-row")]
pub async fn query_waring_list(
    signal_waring_config_api: &rocket::State<SignalWaringConfigBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let error_json = json!({
        "status": "error",
        "message": ""
    });
    Custom(Status::InternalServerError, Json(error_json))
}

#[post("/signal-waring-config/update", format = "json", data = "<data>")]
pub async fn update_signal_waring_config(
    data: Json<SignalWaringConfig>,
    signal_waring_config_api: &rocket::State<SignalWaringConfigBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let req = data.into_inner();

    // Get the old config
    let old_config = match signal_waring_config_api.by_id(req.id.unwrap_or(0)).await {
        Ok(config) => config,
        Err(_) => {
            let error_json = json!({
                "code": 40000,
                "message": "SignalWaringConfig not found"
            });
            return Custom(Status::NotFound, Json(error_json));
        }
    };

    // Remove old config from cache
    if let Err(e) = signal_waring_config_api.remove_signal_waring_cache(
        old_config.signal_id.unwrap(),
        &old_config,
    ).await {
        error!("Failed to remove old config from cache: {}", e);
        let error_json = json!({
            "code": 40000,
            "message": format!("Failed to remove old config from cache: {}", e)
        });
        return Custom(Status::InternalServerError, Json(error_json));
    }

    // Create new config with updated values
    let mut new_config = old_config;
    new_config.min = req.min;
    new_config.max = req.max;
    new_config.in_or_out = req.in_or_out;

    // Update in database
    match signal_waring_config_api.update(new_config.id.unwrap(), new_config.clone()).await {
        Ok(updated_config) => {
            // Set new config in cache
            if let Err(e) = signal_waring_config_api.set_signal_waring_cache(
                updated_config.signal_id.unwrap(),
                &updated_config,
            ).await {
                error!("Failed to set new config in cache: {}", e);
                let error_json = json!({
                    "code": 40000,
                    "message": format!("Failed to set new config in cache: {}", e)
                });
                return Custom(Status::InternalServerError, Json(error_json));
            }

            let success_json = json!({
                "code": 20000,
                "message": "更新成功",
                "data": updated_config
            });
            Custom(Status::Ok, Json(success_json))
        }
        Err(e) => {
            error!("Failed to update signal warning config: {}", e);
            let error_json = json!({
                "code": 40000,
                "message": format!("更新失败: {}", e)
            });
            Custom(Status::InternalServerError, Json(error_json))
        }
    }
}

#[get("/signal-waring-config/page?<page>&<page_size>&<signal_id>&<device_uid>&<identification_code>&<protocol>"
)]
pub async fn page_signal_waring_config(
    page: Option<i64>,
    page_size: Option<i64>,
    signal_id: Option<String>,
    device_uid: Option<String>,
    identification_code: Option<String>,
    protocol: Option<String>,
    signal_waring_config_api: &rocket::State<SignalWaringConfigBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let page = page.unwrap_or(1);
    let page_size = page_size.unwrap_or(10);

    if page == 0 || page_size == 0 {
        let error_json = json!({
            "code": 40000,
            "message": "Invalid page or page_size parameters"
        });
        return Custom(Status::BadRequest, Json(error_json));
    }

    // Parse signal_id if provided
    let signal_id = if let Some(sid) = signal_id {
        match sid.parse::<i32>() {
            Ok(id) => Some(id),
            Err(_) => {
                let error_json = json!({
                    "code": 40000,
                    "message": "无效的signal_id"
                });
                return Custom(Status::BadRequest, Json(error_json));
            }
        }
    } else {
        None
    };

    let protocol = protocol.unwrap_or_else(|| "mqtt".to_string());

    let mut filters = Vec::new();

    if let Some(sid) = signal_id {
        filters.push(FilterInfo {
            field: "signal_id".to_string(),
            value: sid.to_string(),
            operation: FilterOperation::Equal,
            value2: None,
        });
    }

    if let Some(uid) = device_uid {
        filters.push(FilterInfo {
            field: "device_uid".to_string(),
            value: uid,
            operation: FilterOperation::Equal,
            value2: None,
        });
    }

    if let Some(code) = identification_code {
        filters.push(FilterInfo {
            field: "identification_code".to_string(),
            value: code,
            operation: FilterOperation::Equal,
            value2: None,
        });
    }

    filters.push(FilterInfo {
        field: "protocol".to_string(),
        value: protocol,
        operation: FilterOperation::Equal,
        value2: None,
    });

    let result = signal_waring_config_api
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
        Err(e) => {
            error!("Failed to fetch page: {}", e);
            let error_json = json!({
                "code": 40000,
                "message": "查询异常"
            });
            Custom(Status::InternalServerError, Json(error_json))
        }
    }
}
