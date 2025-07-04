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

use crate::biz::coap_biz::CoapHandlerBiz;
use crate::biz::user_biz::UserBiz;
use crate::db::db_model::{Dashboard, Signal};
use anyhow::{Context, Error, Result};
use common_lib::redis_pool_utils::RedisOp;
use common_lib::sql_utils;
use common_lib::sql_utils::{CrudOperations, FilterInfo, PaginationParams, PaginationResult};
use sqlx::MySqlPool;

pub struct DashboardBiz {
    pub redis: RedisOp,
    pub mysql: MySqlPool,
}
impl DashboardBiz {
    pub async fn find_by_name(&self, username: Option<String>) -> Result<Option<Dashboard>, Error> {
        if username.is_none() {
            return Ok(None);
        }

        let sql = "select * from dashboards where name = ?";

        let record = sqlx::query_as::<_, Dashboard>(sql)
            .bind(username.clone().unwrap())
            .fetch_optional(&self.mysql)
            .await
            .with_context(|| {
                format!(
                    "Failed to fetch updated record from table '{}' with username {:?}",
                    "users",
                    username.clone()
                )
            });

        return match record {
            Ok(u) => Ok(u),
            Err(ee) => Err(ee),
        };
    }

    pub fn new(redis: RedisOp, mysql: MySqlPool) -> Self {
        DashboardBiz { redis, mysql }
    }
}

#[async_trait::async_trait]
impl CrudOperations<Dashboard> for DashboardBiz {
    async fn create(&self, item: Dashboard) -> Result<Dashboard, Error> {
        let mut updates = vec![];

        if let Some(name) = item.name {
            updates.push(("name", name.to_string()));
        }

        if let Some(config) = item.config {
            updates.push(("config", config.to_string()));
        }

        log::info!("Creating dashboard with updates: {:?}", updates);

        let result = sql_utils::insert::<Dashboard>(&self.mysql, "dashboards", updates).await;

        result
    }

    async fn update(&self, id: i64, item: Dashboard) -> Result<Dashboard, Error> {
        let mut updates = vec![];

        if let Some(name) = item.name {
            updates.push(("name", name.to_string()));
        }

        if let Some(config) = item.config {
            updates.push(("config", config.to_string()));
        }

        log::info!("Updating dashboard with ID {}: {:?}", id, updates);

        let result =
            sql_utils::update_by_id::<Dashboard>(&self.mysql, "dashboards", id, updates).await;

        match result {
            Ok(it) => Ok(it),
            Err(err) => Err(err),
        }
    }

    async fn delete(&self, id: i64) -> Result<Dashboard, Error> {
        log::info!("Deleting dashboard with ID {}", id);

        sql_utils::delete_by_id(&self.mysql, "dashboards", id).await
    }

    async fn page(
        &self,
        filters: Vec<FilterInfo>,
        pagination: PaginationParams,
    ) -> Result<PaginationResult<Dashboard>, Error> {
        log::info!(
            "Fetching page of dashboards with filters: {:?} and pagination: {:?}",
            filters,
            pagination
        );

        let result =
            sql_utils::paginate::<Dashboard>(&self.mysql, "dashboards", filters, pagination).await;

        result
    }

    async fn list(&self, filters: Vec<FilterInfo>) -> Result<Vec<Dashboard>, Error> {
        log::info!("Fetching list of dashboards with filters: {:?}", filters);

        let result = sql_utils::list::<Dashboard>(&self.mysql, "dashboards", filters).await;
        result
    }

    async fn by_id(&self, id: i64) -> Result<Dashboard, Error> {
        let result = sql_utils::by_id_common::<Dashboard>(&self.mysql, "dashboards", id).await;
        result
    }
}
