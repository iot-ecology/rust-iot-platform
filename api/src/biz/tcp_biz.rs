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

use crate::biz::sim_card_biz::SimCardBiz;
use crate::db::db_model::{TcpHandler, WebSocketHandler};
use anyhow::{Context, Error, Result};
use common_lib::redis_pool_utils::RedisOp;
use common_lib::sql_utils::{
    by_id_common, CrudOperations, FilterInfo, PaginationParams, PaginationResult,
};
use log::error;
use r2d2;
use rocket::serde::json;
use sqlx::MySqlPool;

pub struct TcpHandlerBiz {
    pub redis: RedisOp,
    pub mysql: MySqlPool,
}
impl TcpHandlerBiz {
    pub fn new(redis: RedisOp, mysql: MySqlPool) -> Self {
        TcpHandlerBiz { redis, mysql }
    }

    pub async fn find_by_device_info_id(&self, device_info_id: i64) -> Result<Option<TcpHandler>, Error> {
        let sql = "select * from tcp_handlers where 1=1 and device_info_id = ?";

        let record = sqlx::query_as::<_, TcpHandler>(sql).bind(device_info_id.to_string())

            .fetch_optional(&self.mysql).await.with_context(|| {
            format!(
                "Failed to fetch updated record from table '{}' with username {:?}",
                "tcp_handlers",
                device_info_id
            )
        });

        match record {
            Ok(u) => Ok(u),
            Err(ee) => Err(ee),
        }
    }

    pub fn setRedis(&self, ws: &TcpHandler) {
        self.redis.set_hash(
            "struct:tcp",
            ws.device_info_id.unwrap().to_string().as_str(),
            <std::option::Option<std::string::String> as Clone>::clone(&ws.script)
                .unwrap()
                .as_str(),
        ).expect("TODO: panic message");
    }
    pub fn deleteRedis(&self, ws: &TcpHandler) {
        self.redis.delete_hash_field(
            "struct:tcp",
            ws.device_info_id.unwrap().to_string().as_str(),
        ).expect("TODO: panic message");
    }

    pub fn set_auth(&self, ws: &TcpHandler) {
        let result = json::to_string(ws);
        match result {
            Ok(o) => {
                let x = self
                    .redis
                    .set_hash(
                        "auth:tcp",
                        ws.device_info_id.unwrap().to_string().as_str(),
                        o.as_str(),
                    )
                    .unwrap();
            }
            Err(e) => {
                error!("Error: {}", e);
            }
        }
    }
}

#[async_trait::async_trait]
impl CrudOperations<TcpHandler> for TcpHandlerBiz {
    async fn create(&self, item: TcpHandler) -> Result<TcpHandler, Error> {
        let mut updates = vec![];

        if let Some(username) = item.username {
            updates.push(("username", username));
        } else {
            return Err(Error::msg("Username is required for tcp handler creation"));
        }

        if let Some(password) = item.password {
            updates.push(("password", password));
        } else {
            return Err(Error::msg("Password is required for tcp handler creation"));
        }

        if let Some(name) = item.name {
            updates.push(("name", name));
        } else {
            return Err(Error::msg("Name is required for tcp handler creation"));
        }

        if let Some(script) = item.script {
            updates.push(("script", script));
        } else {
            return Err(Error::msg("Script is required for tcp handler creation"));
        }

        log::info!("Creating tcp handler with updates: {:?}", updates);

        let result =
            common_lib::sql_utils::insert::<TcpHandler>(&self.mysql, "tcp_handlers", updates).await;

        result
    }

    async fn update(&self, id: i64, item: TcpHandler) -> Result<TcpHandler, Error> {
        let mut updates = vec![];

        if let Some(username) = item.username {
            updates.push(("username", username));
        } else {
            return Err(Error::msg("Username is required for tcp handler update"));
        }

        if let Some(password) = item.password {
            updates.push(("password", password));
        } else {
            return Err(Error::msg("Password is required for tcp handler update"));
        }

        if let Some(name) = item.name {
            updates.push(("name", name));
        } else {
            return Err(Error::msg("Name is required for tcp handler update"));
        }

        if let Some(script) = item.script {
            updates.push(("script", script));
        } else {
            return Err(Error::msg("Script is required for tcp handler update"));
        }

        log::info!("Updating tcp handler with ID {}: {:?}", id, updates);

        let result = common_lib::sql_utils::update_by_id::<TcpHandler>(
            &self.mysql,
            "tcp_handlers",
            id,
            updates,
        )
            .await;
        return match result {
            Ok(it) => Ok(it),
            Err(err) => Err(err),
        };
    }

    async fn delete(&self, id: i64) -> Result<TcpHandler, Error> {
        log::info!("Deleting tcp handler with ID {}", id);

        common_lib::sql_utils::delete_by_id(&self.mysql, "tcp_handlers", id).await
    }

    async fn page(
        &self,
        filters: Vec<FilterInfo>,
        pagination: PaginationParams,
    ) -> Result<PaginationResult<TcpHandler>, Error> {
        log::info!(
            "Fetching page of tcp handlers with filters: {:?} and pagination: {:?}",
            filters,
            pagination
        );

        let result = common_lib::sql_utils::paginate::<TcpHandler>(
            &self.mysql,
            "tcp_handlers",
            filters,
            pagination,
        )
            .await;

        result
    }

    async fn list(&self, filters: Vec<FilterInfo>) -> Result<Vec<TcpHandler>, Error> {
        log::info!("Fetching list of tcp handlers with filters: {:?}", filters);
        let result =
            common_lib::sql_utils::list::<TcpHandler>(&self.mysql, "tcp_handlers", filters).await;
        return result;
    }

    async fn by_id(&self, id: i64) -> Result<TcpHandler, Error> {
        let result =
            common_lib::sql_utils::by_id_common::<TcpHandler>(&self.mysql, "tcp_handlers", id)
                .await;
        result
    }
}
