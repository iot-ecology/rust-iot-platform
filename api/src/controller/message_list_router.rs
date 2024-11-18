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
    page: Option<u64>,
    page_size: Option<u64>,
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
