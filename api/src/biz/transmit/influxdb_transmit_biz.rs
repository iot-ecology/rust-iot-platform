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

use crate::biz::transmit::clickhouse_transmit_biz::ClickhouseTransmitBiz;
use crate::biz::user_biz::UserBiz;
use crate::db::db_model::{InfluxDbTransmit, Signal, WebSocketHandler};
use anyhow::{Context, Error, Result};
use common_lib::redis_pool_utils::RedisOp;
use common_lib::sql_utils::{CrudOperations, FilterInfo, PaginationParams, PaginationResult};
use sqlx::MySqlPool;

pub struct InfluxDbTransmitBiz {
    pub redis: RedisOp,
    pub mysql: MySqlPool,
}
impl InfluxDbTransmitBiz {
    pub fn new(redis: RedisOp, mysql: MySqlPool) -> Self {
        InfluxDbTransmitBiz { redis, mysql }
    }
}

#[async_trait::async_trait]
impl CrudOperations<InfluxDbTransmit> for InfluxDbTransmitBiz {
    async fn create(&self, item: InfluxDbTransmit) -> Result<InfluxDbTransmit, Error> {
        let mut updates = vec![];

        if let Some(name) = item.name {
            updates.push(("name", name));
        }

        if let Some(host) = item.host {
            updates.push(("host", host));
        }

        if let Some(port) = item.port {
            updates.push(("port", port.to_string()));
        }

        if let Some(token) = item.token {
            updates.push(("token", token));
        }

        log::info!("Creating InfluxDbTransmit with updates: {:?}", updates);

        let result = common_lib::sql_utils::insert::<InfluxDbTransmit>(
            &self.mysql,
            "influx_db_transmits",
            updates,
        )
            .await;

        result
    }

    async fn update(&self, id: i64, item: InfluxDbTransmit) -> Result<InfluxDbTransmit, Error> {
        let mut updates = vec![];

        if let Some(name) = item.name {
            updates.push(("name", name));
        }

        if let Some(host) = item.host {
            updates.push(("host", host));
        }

        if let Some(port) = item.port {
            updates.push(("port", port.to_string()));
        }

        if let Some(token) = item.token {
            updates.push(("token", token));
        }

        log::info!("Updating InfluxDbTransmit with ID {}: {:?}", id, updates);

        let result = common_lib::sql_utils::update_by_id::<InfluxDbTransmit>(
            &self.mysql,
            "influx_db_transmits",
            id,
            updates,
        )
            .await;

        return match result {
            Ok(it) => Ok(it),
            Err(err) => Err(err),
        };
    }

    async fn delete(&self, id: i64) -> Result<InfluxDbTransmit, Error> {
        log::info!("Deleting InfluxDbTransmit with ID {}", id);

        common_lib::sql_utils::delete_by_id(&self.mysql, "influx_db_transmits", id).await
    }

    async fn page(
        &self,
        filters: Vec<FilterInfo>,
        pagination: PaginationParams,
    ) -> Result<PaginationResult<InfluxDbTransmit>, Error> {
        log::info!(
            "Fetching page of InfluxDbTransmits with filters: {:?} and pagination: {:?}",
            filters,
            pagination
        );

        let result = common_lib::sql_utils::paginate::<InfluxDbTransmit>(
            &self.mysql,
            "influx_db_transmits",
            filters,
            pagination,
        )
            .await;

        result
    }

    async fn list(&self, filters: Vec<FilterInfo>) -> Result<Vec<InfluxDbTransmit>, Error> {
        log::info!(
            "Fetching list of InfluxDbTransmits with filters: {:?}",
            filters
        );
        let result = common_lib::sql_utils::list::<InfluxDbTransmit>(
            &self.mysql,
            "influx_db_transmits",
            filters,
        )
            .await;
        return result;
    }

    async fn by_id(&self, id: i64) -> Result<InfluxDbTransmit, Error> {
        let result = common_lib::sql_utils::by_id_common::<InfluxDbTransmit>(
            &self.mysql,
            "influx_db_transmits",
            id,
        )
            .await;
        result
    }
}
