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

use crate::biz::signal_delay_waring_param_biz::SignalDelayWaringParamBiz;
use crate::biz::user_biz::UserBiz;
use crate::db::db_model::{SimCard, WebSocketHandler};
use anyhow::{Context, Error, Result};
use common_lib::redis_pool_utils::RedisOp;
use common_lib::sql_utils::{CrudOperations, FilterInfo, PaginationParams, PaginationResult};
use sqlx::MySqlPool;

pub struct SimCardBiz {
    pub redis: RedisOp,
    pub mysql: MySqlPool,
}
impl SimCardBiz {
    pub fn new(redis: RedisOp, mysql: MySqlPool) -> Self {
        SimCardBiz { redis, mysql }
    }
}

#[async_trait::async_trait]
impl CrudOperations<SimCard> for SimCardBiz {
    async fn create(&self, item: SimCard) -> Result<SimCard, Error> {
        let mut updates = vec![];

        updates.push(("access_number", item.access_number));
        updates.push(("iccid", item.iccid));
        updates.push(("imsi", item.imsi));
        updates.push(("operator", item.operator));

        if let Some(expiration) = item.expiration {
            updates.push(("expiration", expiration.to_string()));
        }

        log::info!("Creating sim card with updates: {:?}", updates);

        let result =
            common_lib::sql_utils::insert::<SimCard>(&self.mysql, "sim_cards", updates).await;

        result
    }

    async fn update(&self, id: i64, item: SimCard) -> Result<SimCard, Error> {
        let mut updates = vec![];

        updates.push(("access_number", item.access_number));
        updates.push(("iccid", item.iccid));
        updates.push(("imsi", item.imsi));
        updates.push(("operator", item.operator));

        if let Some(expiration) = item.expiration {
            updates.push(("expiration", expiration.to_string()));
        }

        log::info!("Updating sim card with ID {}: {:?}", id, updates);

        let result =
            common_lib::sql_utils::update_by_id::<SimCard>(&self.mysql, "sim_cards", id, updates)
                .await;
        return match result {
            Ok(it) => Ok(it),
            Err(err) => Err(err),
        };
    }

    async fn delete(&self, id: i64) -> Result<SimCard, Error> {
        log::info!("Deleting sim card with ID {}", id);

        common_lib::sql_utils::delete_by_id(&self.mysql, "sim_cards", id).await
    }

    async fn page(
        &self,
        filters: Vec<FilterInfo>,
        pagination: PaginationParams,
    ) -> Result<PaginationResult<SimCard>, Error> {
        log::info!(
            "Fetching page of sim cards with filters: {:?} and pagination: {:?}",
            filters,
            pagination
        );

        let result = common_lib::sql_utils::paginate::<SimCard>(
            &self.mysql,
            "sim_cards",
            filters,
            pagination,
        )
            .await;

        result
    }

    async fn list(&self, filters: Vec<FilterInfo>) -> Result<Vec<SimCard>, Error> {
        log::info!("Fetching list of sim cards with filters: {:?}", filters);
        let result =
            common_lib::sql_utils::list::<SimCard>(&self.mysql, "sim_cards", filters).await;
        return result;
    }

    async fn by_id(&self, id: i64) -> Result<SimCard, Error> {
        let result =
            common_lib::sql_utils::by_id_common::<SimCard>(&self.mysql, "sim_cards", id).await;
        result
    }
}
