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

use crate::biz::notice::dingding_biz::DingDingBiz;
use crate::biz::user_biz::UserBiz;
use crate::db::db_model::{FeiShu, Signal, WebSocketHandler};
use anyhow::{Context, Error, Result};
use common_lib::redis_pool_utils::RedisOp;
use common_lib::sql_utils;
use common_lib::sql_utils::{CrudOperations, FilterInfo, PaginationParams, PaginationResult};
use sqlx::MySqlPool;

pub struct FeiShuBiz {
    pub redis: RedisOp,
    pub mysql: MySqlPool,
}
impl FeiShuBiz {
    pub fn new(redis: RedisOp, mysql: MySqlPool) -> Self {
        FeiShuBiz { redis, mysql }
    }
}
#[async_trait::async_trait]
impl CrudOperations<FeiShu> for FeiShuBiz {
    async fn create(&self, item: FeiShu) -> Result<FeiShu, Error> {
        let mut updates = vec![];

        if let Some(name) = item.name {
            updates.push(("name", name));
        }

        if let Some(access_token) = item.access_token {
            updates.push(("access_token", access_token));
        }

        if let Some(secret) = item.secret {
            updates.push(("secret", secret));
        }

        if let Some(content) = item.content {
            updates.push(("content", content));
        }

        log::info!("Creating FeiShu with updates: {:?}", updates);

        let result = sql_utils::insert::<FeiShu>(&self.mysql, "feishus", updates).await;

        result
    }

    async fn update(&self, id: i64, item: FeiShu) -> Result<FeiShu, Error> {
        let mut updates = vec![];

        if let Some(name) = item.name {
            updates.push(("name", name));
        }

        if let Some(access_token) = item.access_token {
            updates.push(("access_token", access_token));
        }

        if let Some(secret) = item.secret {
            updates.push(("secret", secret));
        }

        if let Some(content) = item.content {
            updates.push(("content", content));
        }

        log::info!("Updating FeiShu with ID {}: {:?}", id, updates);

        let result = sql_utils::update_by_id::<FeiShu>(&self.mysql, "feishus", id, updates).await;

        match result {
            Ok(it) => Ok(it),
            Err(err) => Err(err),
        }
    }

    async fn delete(&self, id: i64) -> Result<FeiShu, Error> {
        log::info!("Deleting FeiShu with ID {}", id);

        sql_utils::delete_by_id(&self.mysql, "feishus", id).await
    }

    async fn page(
        &self,
        filters: Vec<FilterInfo>,
        pagination: PaginationParams,
    ) -> Result<PaginationResult<FeiShu>, Error> {
        log::info!(
            "Fetching page of FeiShus with filters: {:?} and pagination: {:?}",
            filters,
            pagination
        );

        let result =
            sql_utils::paginate::<FeiShu>(&self.mysql, "feishus", filters, pagination).await;

        result
    }

    async fn list(&self, filters: Vec<FilterInfo>) -> Result<Vec<FeiShu>, Error> {
        log::info!("Fetching list of FeiShus with filters: {:?}", filters);

        let result = sql_utils::list::<FeiShu>(&self.mysql, "feishus", filters).await;
        result
    }

    async fn by_id(&self, id: i64) -> Result<FeiShu, Error> {
        let result = sql_utils::by_id_common::<FeiShu>(&self.mysql, "feishus", id).await;
        result
    }
}
