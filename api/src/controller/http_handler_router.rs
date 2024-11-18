use common_lib::sql_utils::{CrudOperations, FilterInfo, FilterOperation, PaginationParams};
use log::error;

use crate::biz::http_biz::HttpHandlerBiz;

use crate::db::db_model::{DeviceInfo, HttpHandler};
use common_lib::config::Config;
use common_lib::influxdb_utils::InfluxDBManager;
use common_lib::ut::calc_bucket_name;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;

#[post("/HttpHandler/create", format = "json", data = "<data>")]
pub async fn create_http_handler(
    data: Json<HttpHandler>,
    http_handler_api: &rocket::State<HttpHandlerBiz>,
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

    match http_handler_api.create(data.into_inner()).await {
        Ok(u) => {
            let success_json = json!({
                "code": 20000,
                "message": "创建成功",
                "data": u
            });

            http_handler_api.setRedis(&u);
            http_handler_api.set_auth(&u);

            let string = calc_bucket_name(
                config
                    .influx_config
                    .clone()
                    .unwrap()
                    .bucket
                    .unwrap()
                    .as_str(),
                "HTTP",
                u.device_info_id.unwrap() ,
            );
            let db_manager = InfluxDBManager::new(
                &config.influx_config.clone().unwrap().host.unwrap(),
                config.influx_config.clone().unwrap().port.unwrap(),
                &config.influx_config.clone().unwrap().org.unwrap(),
                &config.influx_config.clone().unwrap().token.unwrap(),
            );

            let _ = db_manager.create_bucket(string).await;
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

#[post("/HttpHandler/update", format = "json", data = "<data>")]
pub async fn update_http_handler(
    data: Json<HttpHandler>,
    http_handler_api: &rocket::State<HttpHandlerBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let x = http_handler_api.by_id(data.id.unwrap()).await;

    match x {
        Ok(mut u) => {
            u.password = data.password.clone();
            u.username = data.username.clone();
            u.name = data.name.clone();
            u.script = data.script.clone();
            let result = http_handler_api.update(u.id.unwrap(), u).await;
            match result {
                Ok(u2) => {
                    let success_json = json!({
                        "code": 20000,
                        "message": "更新成功",
                        "data": u2
                    });

                    http_handler_api.setRedis(&u2);
                    http_handler_api.set_auth(&u2);
                    let string = calc_bucket_name(
                        config
                            .influx_config
                            .clone()
                            .unwrap()
                            .bucket
                            .unwrap()
                            .as_str(),
                        "HTTP",
                        u2.device_info_id.unwrap() ,
                    );

                    let db_manager = InfluxDBManager::new(
                        &config.influx_config.clone().unwrap().host.unwrap(),
                        config.influx_config.clone().unwrap().port.unwrap(),
                        &config.influx_config.clone().unwrap().org.unwrap(),
                        &config.influx_config.clone().unwrap().token.unwrap(),
                    );

                    let _ = db_manager.create_bucket(string).await;
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

#[get("/HttpHandler/<id>")]
pub async fn by_id_http_handler(
    id: u64,
    http_handler_api: &rocket::State<HttpHandlerBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let result = http_handler_api.by_id(id).await;
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

#[get("/HttpHandler/page?<page>&<page_size>&<name>")]
pub async fn page_http_handler(
    page: Option<u64>,
    name: Option<String>,
    page_size: Option<u64>,
    http_handler_api: &rocket::State<HttpHandlerBiz>,
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

    let result = http_handler_api
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

#[post("/HttpHandler/delete/<id>")]
pub async fn delete_http_handler(
    id: u64,
    http_handler_api: &rocket::State<HttpHandlerBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let result = http_handler_api.delete(id).await;
    match result {
        Ok(o) => {
            let success_json = json!({
                "code": 20000,
                "message": "删除成功",
            });
            http_handler_api.deleteRedis(&o);
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
