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

use crate::biz::ws_biz::WebSocketHandlerBiz;
use crate::db::db_model::WebSocketHandler;
use common_lib::config::Config;
use common_lib::influxdb_utils::InfluxDBManager;
use common_lib::sql_utils::{CrudOperations, FilterInfo, FilterOperation, PaginationParams};
use common_lib::ut::calc_bucket_name;
use log::{error, info};
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;

#[post("/WebsocketHandler/create", format = "json", data = "<data>")]
pub async fn create_websocket_handler(
    data: Json<WebSocketHandler>,
    ws_handler_api: &rocket::State<WebSocketHandlerBiz>,
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

    match ws_handler_api.create(data.into_inner()).await {
        Ok(u) => {
            let success_json = json!({
                "code": 20000,
                "message": "创建成功",
                "data": u
            });

            ws_handler_api.setRedis(&u);
            ws_handler_api.set_auth(&u);

            let string = calc_bucket_name(
                config
                    .influx_config
                    .clone()
                    .unwrap()
                    .bucket
                    .unwrap()
                    .as_str(),
                "WebSocket",
                u.device_info_id.unwrap(),
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

#[post("/WebsocketHandler/update", format = "json", data = "<data>")]
pub async fn update_websocket_handler(
    data: Json<WebSocketHandler>,
    ws_handler_api: &rocket::State<WebSocketHandlerBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let x = ws_handler_api.by_id(data.id.unwrap()).await;

    match x {
        Ok(mut u) => {
            u.password = data.password.clone();
            u.username = data.username.clone();
            u.name = data.name.clone();
            u.script = data.script.clone();
            let result = ws_handler_api.update(u.id.unwrap(), u).await;
            match result {
                Ok(u2) => {
                    let success_json = json!({
                        "code": 20000,
                        "message": "更新成功",
                        "data": u2
                    });

                    ws_handler_api.setRedis(&u2);
                    ws_handler_api.set_auth(&u2);
                    let string = calc_bucket_name(
                        config
                            .influx_config
                            .clone()
                            .unwrap()
                            .bucket
                            .unwrap()
                            .as_str(),
                        "WebSocket",
                        u2.device_info_id.unwrap(),
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

#[get("/WebsocketHandler/<id>")]
pub async fn by_id_websocket_handler(
    id: i64,
    ws_handler_api: &rocket::State<WebSocketHandlerBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let result = ws_handler_api.by_id(id).await;
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

#[get("/WebsocketHandler/page?<page>&<page_size>&<name>")]
pub async fn page_websocket_handler(
    ws_handler_api: &rocket::State<WebSocketHandlerBiz>,
    config: &rocket::State<Config>,
    page: Option<i64>,
    name: Option<String>,
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

    let result = ws_handler_api
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

#[post("/WebsocketHandler/delete/<id>")]
pub async fn delete_websocket_handler(
    id: i64,
    ws_handler_api: &rocket::State<WebSocketHandlerBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    let result = ws_handler_api.delete(id).await;
    match result {
        Ok(o) => {
            let success_json = json!({
                "code": 20000,
                "message": "删除成功",
            });
            ws_handler_api.deleteRedis(&o);
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
