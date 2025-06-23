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

use crate::biz::transmit::bind::mongo_bind_biz::MongoTransmitBindBiz;
use crate::biz::user_biz::UserBiz;
use crate::db::db_model::{MysqlTransmitBind, Signal, WebSocketHandler};
use anyhow::{Context, Error, Result};
use common_lib::redis_pool_utils::RedisOp;
use common_lib::sql_utils;
use common_lib::sql_utils::{CrudOperations, FilterInfo, PaginationParams, PaginationResult};
use sqlx::MySqlPool;

pub struct MysqlTransmitBindBiz {
    pub redis: RedisOp,
    pub mysql: MySqlPool,
}
impl MysqlTransmitBindBiz {
    pub fn new(redis: RedisOp, mysql: MySqlPool) -> Self {
        MysqlTransmitBindBiz { redis, mysql }
    }
}

#[async_trait::async_trait]
impl CrudOperations<MysqlTransmitBind> for MysqlTransmitBindBiz {
    async fn create(&self, item: MysqlTransmitBind) -> Result<MysqlTransmitBind, Error> {
        let mut updates = vec![];

        if let Some(protocol) = item.protocol {
            updates.push(("protocol", protocol));
        }

        if let Some(device_uid) = item.device_uid {
            updates.push(("device_uid", device_uid));
        }

        if let Some(identification_code) = item.identification_code {
            updates.push(("identification_code", identification_code));
        }

        if let Some(mysql_transmit_id) = item.mysql_transmit_id {
            updates.push(("mysql_transmit_id", mysql_transmit_id.to_string()));
        }

        if let Some(table_name) = item.table_name {
            updates.push(("table_name", table_name));
        }

        if let Some(script) = item.script {
            updates.push(("script", script));
        }

        if let Some(enable) = item.enable {
            updates.push(("enable", enable.to_string()));
        }

        log::info!("Creating MysqlTransmitBind with updates: {:?}", updates);

        let result =
            sql_utils::insert::<MysqlTransmitBind>(&self.mysql, "mysql_transmit_binds", updates)
                .await;

        result
    }

    async fn update(&self, id: i64, item: MysqlTransmitBind) -> Result<MysqlTransmitBind, Error> {
        let mut updates = vec![];

        if let Some(protocol) = item.protocol {
            updates.push(("protocol", protocol));
        }

        if let Some(device_uid) = item.device_uid {
            updates.push(("device_uid", device_uid));
        }

        if let Some(identification_code) = item.identification_code {
            updates.push(("identification_code", identification_code));
        }

        if let Some(mysql_transmit_id) = item.mysql_transmit_id {
            updates.push(("mysql_transmit_id", mysql_transmit_id.to_string()));
        }

        if let Some(table_name) = item.table_name {
            updates.push(("table_name", table_name));
        }

        if let Some(script) = item.script {
            updates.push(("script", script));
        }

        if let Some(enable) = item.enable {
            updates.push(("enable", enable.to_string()));
        }

        log::info!("Updating MysqlTransmitBind with ID {}: {:?}", id, updates);

        let result = sql_utils::update_by_id::<MysqlTransmitBind>(
            &self.mysql,
            "mysql_transmit_binds",
            id,
            updates,
        )
            .await;

        match result {
            Ok(it) => Ok(it),
            Err(err) => Err(err),
        }
    }

    async fn delete(&self, id: i64) -> Result<MysqlTransmitBind, Error> {
        log::info!("Deleting MysqlTransmitBind with ID {}", id);

        sql_utils::delete_by_id(&self.mysql, "mysql_transmit_binds", id).await
    }

    async fn page(
        &self,
        filters: Vec<FilterInfo>,
        pagination: PaginationParams,
    ) -> Result<PaginationResult<MysqlTransmitBind>, Error> {
        log::info!(
            "Fetching page of MysqlTransmitBinds with filters: {:?} and pagination: {:?}",
            filters,
            pagination
        );

        let result = sql_utils::paginate::<MysqlTransmitBind>(
            &self.mysql,
            "mysql_transmit_binds",
            filters,
            pagination,
        )
            .await;

        result
    }

    async fn list(&self, filters: Vec<FilterInfo>) -> Result<Vec<MysqlTransmitBind>, Error> {
        log::info!(
            "Fetching list of MysqlTransmitBinds with filters: {:?}",
            filters
        );

        let result =
            sql_utils::list::<MysqlTransmitBind>(&self.mysql, "mysql_transmit_binds", filters)
                .await;
        result
    }

    async fn by_id(&self, id: i64) -> Result<MysqlTransmitBind, Error> {
        let result =
            sql_utils::by_id_common::<MysqlTransmitBind>(&self.mysql, "mysql_transmit_binds", id)
                .await;
        result
    }
}
