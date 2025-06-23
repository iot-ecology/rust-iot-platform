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

use crate::biz::transmit::bind::influxdb_bind_biz::InfluxDbTransmitBindBiz;
use crate::biz::user_biz::UserBiz;
use crate::db::db_model::{MongoTransmitBind, Signal, WebSocketHandler};
use anyhow::{Context, Error, Result};
use common_lib::redis_pool_utils::RedisOp;
use common_lib::sql_utils;
use common_lib::sql_utils::{CrudOperations, FilterInfo, PaginationParams, PaginationResult};
use sqlx::MySqlPool;

pub struct MongoTransmitBindBiz {
    pub redis: RedisOp,
    pub mysql: MySqlPool,
}
impl MongoTransmitBindBiz {
    pub fn new(redis: RedisOp, mysql: MySqlPool) -> Self {
        MongoTransmitBindBiz { redis, mysql }
    }
}
#[async_trait::async_trait]
impl CrudOperations<MongoTransmitBind> for MongoTransmitBindBiz {
    async fn create(&self, item: MongoTransmitBind) -> Result<MongoTransmitBind, Error> {
        let mut updates = vec![];

        if let Some(device_uid) = item.device_uid {
            updates.push(("device_uid", device_uid.to_string()));
        }

        if let Some(protocol) = item.protocol {
            updates.push(("protocol", protocol.to_string()));
        }

        if let Some(identification_code) = item.identification_code {
            updates.push(("identification_code", identification_code.to_string()));
        }

        if let Some(mysql_transmit_id) = item.mysql_transmit_id {
            updates.push(("mysql_transmit_id", mysql_transmit_id.to_string()));
        }

        if let Some(collection) = item.collection {
            updates.push(("collection", collection.to_string()));
        }

        if let Some(database) = item.database {
            updates.push(("database", database.to_string()));
        }

        if let Some(script) = item.script {
            updates.push(("script", script.to_string()));
        }

        if let Some(enable) = item.enable {
            updates.push(("enable", enable.to_string()));
        }

        log::info!("Creating MongoTransmitBind with updates: {:?}", updates);

        let result =
            sql_utils::insert::<MongoTransmitBind>(&self.mysql, "mongo_transmit_binds", updates)
                .await;

        result
    }

    async fn update(&self, id: i64, item: MongoTransmitBind) -> Result<MongoTransmitBind, Error> {
        let mut updates = vec![];

        if let Some(device_uid) = item.device_uid {
            updates.push(("device_uid", device_uid.to_string()));
        }

        if let Some(protocol) = item.protocol {
            updates.push(("protocol", protocol.to_string()));
        }

        if let Some(identification_code) = item.identification_code {
            updates.push(("identification_code", identification_code.to_string()));
        }

        if let Some(mysql_transmit_id) = item.mysql_transmit_id {
            updates.push(("mysql_transmit_id", mysql_transmit_id.to_string()));
        }

        if let Some(collection) = item.collection {
            updates.push(("collection", collection.to_string()));
        }

        if let Some(database) = item.database {
            updates.push(("database", database.to_string()));
        }

        if let Some(script) = item.script {
            updates.push(("script", script.to_string()));
        }

        if let Some(enable) = item.enable {
            updates.push(("enable", enable.to_string()));
        }

        log::info!("Updating MongoTransmitBind with ID {}: {:?}", id, updates);

        let result = sql_utils::update_by_id::<MongoTransmitBind>(
            &self.mysql,
            "mongo_transmit_binds",
            id,
            updates,
        )
            .await;

        match result {
            Ok(it) => Ok(it),
            Err(err) => Err(err),
        }
    }

    async fn delete(&self, id: i64) -> Result<MongoTransmitBind, Error> {
        log::info!("Deleting MongoTransmitBind with ID {}", id);

        sql_utils::delete_by_id(&self.mysql, "mongo_transmit_binds", id).await
    }

    async fn page(
        &self,
        filters: Vec<FilterInfo>,
        pagination: PaginationParams,
    ) -> Result<PaginationResult<MongoTransmitBind>, Error> {
        log::info!(
            "Fetching page of MongoTransmitBinds with filters: {:?} and pagination: {:?}",
            filters,
            pagination
        );

        let result = sql_utils::paginate::<MongoTransmitBind>(
            &self.mysql,
            "mongo_transmit_binds",
            filters,
            pagination,
        )
            .await;

        result
    }

    async fn list(&self, filters: Vec<FilterInfo>) -> Result<Vec<MongoTransmitBind>, Error> {
        log::info!(
            "Fetching list of MongoTransmitBinds with filters: {:?}",
            filters
        );

        let result =
            sql_utils::list::<MongoTransmitBind>(&self.mysql, "mongo_transmit_binds", filters)
                .await;
        result
    }

    async fn by_id(&self, id: i64) -> Result<MongoTransmitBind, Error> {
        let result =
            sql_utils::by_id_common::<MongoTransmitBind>(&self.mysql, "mongo_transmit_binds", id)
                .await;
        result
    }
}
