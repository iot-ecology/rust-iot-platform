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

use crate::biz::calc_rule_biz::CalcRuleBiz;
use crate::biz::user_biz::UserBiz;
use crate::db::db_model::{CoapHandler, Signal, User};
use crate::rocket::serde::json;
use anyhow::{Context, Error, Result};
use common_lib::redis_pool_utils::RedisOp;
use common_lib::sql_utils;
use common_lib::sql_utils::{CrudOperations, FilterInfo, PaginationParams, PaginationResult};
use log::error;
use sqlx::MySqlPool;

pub struct CoapHandlerBiz {
    pub redis: RedisOp,
    pub mysql: MySqlPool,
}
impl CoapHandlerBiz {
    pub fn setRedis(&self, ws: &CoapHandler) {
        self.redis.set_hash(
            "struct:Coap",
            ws.device_info_id.unwrap().to_string().as_str(),
            <Option<String> as Clone>::clone(&ws.script).unwrap().as_str(),
        ).expect("TODO: panic message");
    }
    pub fn deleteRedis(&self, ws: &CoapHandler) {
        self.redis.delete_hash_field(
            "struct:Coap",
            ws.device_info_id.unwrap().to_string().as_str(),
        ).expect("TODO: panic message");
    }

    pub async fn find_by_device_info_id(&self, device_info_id: i64) -> Result<Option<CoapHandler>, Error> {
        let sql = "select * from coap_handlers where 1=1 and device_info_id = ?";

        let record = sqlx::query_as::<_, CoapHandler>(sql).bind(device_info_id.to_string())

            .fetch_optional(&self.mysql).await.with_context(|| {
            format!(
                "Failed to fetch updated record from table '{}' with username {:?}",
                "coap_handlers",
                device_info_id
            )
        });

        match record {
            Ok(u) => Ok(u),
            Err(ee) => Err(ee),
        }
    }

    pub fn set_auth(&self, ws: &CoapHandler) {
        let result = json::to_string(ws);
        match result {
            Ok(o) => {
                let x = self.redis.set_hash(
                    "auth:coap",
                    ws.device_info_id.unwrap().to_string().as_str(),
                    o.as_str(),
                ).unwrap();
            }
            Err(e) => {
                error!("Error: {}", e);
            }
        }
    }
    pub fn new(redis: RedisOp, mysql: MySqlPool) -> Self {
        CoapHandlerBiz { redis, mysql }
    }
}

#[async_trait::async_trait]
impl CrudOperations<CoapHandler> for CoapHandlerBiz {
    async fn create(&self, item: CoapHandler) -> Result<CoapHandler, Error> {
        let mut updates = vec![];

        if let Some(device_info_id) = item.device_info_id {
            updates.push(("device_info_id", device_info_id.to_string()));
        }

        if let Some(name) = item.name {
            updates.push(("name", name.to_string()));
        }

        if let Some(username) = item.username {
            updates.push(("username", username.to_string()));
        }

        if let Some(password) = item.password {
            updates.push(("password", password.to_string()));
        }

        if let Some(script) = item.script {
            updates.push(("script", script.to_string()));
        }

        log::info!("Creating CoapHandler with updates: {:?}", updates);

        let result = sql_utils::insert::<CoapHandler>(&self.mysql, "coap_handlers", updates).await;

        result
    }

    async fn update(&self, id: i64, item: CoapHandler) -> Result<CoapHandler, Error> {
        let mut updates = vec![];

        if let Some(device_info_id) = item.device_info_id {
            updates.push(("device_info_id", device_info_id.to_string()));
        }

        if let Some(name) = item.name {
            updates.push(("name", name.to_string()));
        }

        if let Some(username) = item.username {
            updates.push(("username", username.to_string()));
        }

        if let Some(password) = item.password {
            updates.push(("password", password.to_string()));
        }

        if let Some(script) = item.script {
            updates.push(("script", script.to_string()));
        }

        log::info!("Updating CoapHandler with ID {}: {:?}", id, updates);

        let result = sql_utils::update_by_id::<CoapHandler>(&self.mysql, "coap_handlers", id, updates).await;

        match result {
            Ok(it) => Ok(it),
            Err(err) => Err(err),
        }
    }

    async fn delete(&self, id: i64) -> Result<CoapHandler, Error> {
        log::info!("Deleting CoapHandler with ID {}", id);

        sql_utils::delete_by_id(&self.mysql, "coap_handlers", id).await
    }

    async fn page(
        &self,
        filters: Vec<FilterInfo>,
        pagination: PaginationParams,
    ) -> Result<PaginationResult<CoapHandler>, Error> {
        log::info!(
            "Fetching page of CoapHandlers with filters: {:?} and pagination: {:?}",
            filters,
            pagination
        );

        let result = sql_utils::paginate::<CoapHandler>(&self.mysql, "coap_handlers", filters, pagination).await;

        result
    }

    async fn list(&self, filters: Vec<FilterInfo>) -> Result<Vec<CoapHandler>, Error> {
        log::info!("Fetching list of CoapHandlers with filters: {:?}", filters);

        let result = sql_utils::list::<CoapHandler>(&self.mysql, "coap_handlers", filters).await;
        result
    }

    async fn by_id(&self, id: i64) -> Result<CoapHandler, Error> {
        let result = sql_utils::by_id_common::<CoapHandler>(&self.mysql, "coap_handlers", id).await;
        result
    }
}
