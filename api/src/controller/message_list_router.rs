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

use common_lib::sql_utils::{CrudOperations, FilterInfo, FilterOperation, PaginationParams};

use crate::biz::message_list_biz::MessageListBiz;
use common_lib::config::Config;
use rocket::http::Status;
use rocket::response::status::Custom;
use rocket::serde::json::Json;
use rocket::{get, post};
use serde_json::json;
#[get("/MessageList/page?<page>&<page_size>")]
pub async fn page_message_list(
    page: Option<i64>,
    page_size: Option<i64>,
    message_list_api: &rocket::State<MessageListBiz>,
    config: &rocket::State<Config>,
) -> rocket::response::status::Custom<Json<serde_json::Value>> {
    // 实际的业务逻辑处理，比如获取分页数据
    let error_json = json!({
        "status": "error",
        "message": "Failed to retrieve message list page"
    });

    // 假设成功获取了分页数据
    let success_json = json!({
        "status": "success",
        "data": {
            "messages": [],  // 这里填充实际的消息列表数据
            "total": 0       // 总条数
        }
    });

    // 返回成功的 JSON 响应
    Custom(Status::Ok, Json(success_json))
}
