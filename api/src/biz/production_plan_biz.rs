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

use crate::biz::product_biz::ProductBiz;
use crate::biz::user_biz::UserBiz;
use crate::db::db_model::{ProductionPlan, Signal, WebSocketHandler};
use anyhow::{Context, Error, Result};
use common_lib::redis_pool_utils::RedisOp;
use common_lib::sql_utils::{CrudOperations, FilterInfo, PaginationParams, PaginationResult};
use sqlx::MySqlPool;

pub struct ProductionPlanBiz {
    pub redis: RedisOp,
    pub mysql: MySqlPool,
}
impl ProductionPlanBiz {
    pub fn new(redis: RedisOp, mysql: MySqlPool) -> Self {
        ProductionPlanBiz { redis, mysql }
    }
}

#[async_trait::async_trait]
impl CrudOperations<ProductionPlan> for ProductionPlanBiz {
    async fn create(&self, item: ProductionPlan) -> Result<ProductionPlan, Error> {
        let mut updates = vec![];

        if let Some(name) = item.name {
            updates.push(("name", name));
        }

        if let Some(start_date) = item.start_date {
            updates.push(("start_date", start_date.to_string()));
        }

        if let Some(end_date) = item.end_date {
            updates.push(("end_date", end_date.to_string()));
        }

        if let Some(description) = item.description {
            updates.push(("description", description));
        }

        if let Some(status) = item.status {
            updates.push(("status", status));
        }

        log::info!("Creating production plan with updates: {:?}", updates);

        let result = common_lib::sql_utils::insert::<ProductionPlan>(
            &self.mysql,
            "production_plans",
            updates,
        )
            .await;

        result
    }

    async fn update(&self, id: i64, item: ProductionPlan) -> Result<ProductionPlan, Error> {
        let mut updates = vec![];

        if let Some(name) = item.name {
            updates.push(("name", name));
        }

        if let Some(start_date) = item.start_date {
            updates.push(("start_date", start_date.to_string()));
        }

        if let Some(end_date) = item.end_date {
            updates.push(("end_date", end_date.to_string()));
        }

        if let Some(description) = item.description {
            updates.push(("description", description));
        }

        if let Some(status) = item.status {
            updates.push(("status", status));
        }

        log::info!("Updating production plan with ID {}: {:?}", id, updates);

        let result = common_lib::sql_utils::update_by_id::<ProductionPlan>(
            &self.mysql,
            "production_plans",
            id,
            updates,
        )
            .await;

        return match result {
            Ok(it) => Ok(it),
            Err(err) => Err(err),
        };
    }

    async fn delete(&self, id: i64) -> Result<ProductionPlan, Error> {
        log::info!("Deleting production plan with ID {}", id);

        common_lib::sql_utils::delete_by_id(&self.mysql, "production_plans", id).await
    }

    async fn page(
        &self,
        filters: Vec<FilterInfo>,
        pagination: PaginationParams,
    ) -> Result<PaginationResult<ProductionPlan>, Error> {
        log::info!(
            "Fetching page of production plans with filters: {:?} and pagination: {:?}",
            filters,
            pagination
        );

        let result = common_lib::sql_utils::paginate::<ProductionPlan>(
            &self.mysql,
            "production_plans",
            filters,
            pagination,
        )
            .await;

        result
    }

    async fn list(&self, filters: Vec<FilterInfo>) -> Result<Vec<ProductionPlan>, Error> {
        log::info!(
            "Fetching list of production plans with filters: {:?}",
            filters
        );
        let result =
            common_lib::sql_utils::list::<ProductionPlan>(&self.mysql, "production_plans", filters)
                .await;
        return result;
    }

    async fn by_id(&self, id: i64) -> Result<ProductionPlan, Error> {
        let result = common_lib::sql_utils::by_id_common::<ProductionPlan>(
            &self.mysql,
            "production_plans",
            id,
        )
            .await;
        result
    }
}
