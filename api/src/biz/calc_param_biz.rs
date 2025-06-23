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

use crate::biz::transmit::mysql_transmit_biz::MysqlTransmitBiz;
use crate::biz::user_biz::UserBiz;
use crate::db::db_model::{CalcParam, Signal};
use anyhow::{Context, Error, Result};
use common_lib::redis_pool_utils::RedisOp;
use common_lib::sql_utils;
use common_lib::sql_utils::{CrudOperations, FilterInfo, PaginationParams, PaginationResult};
use sqlx::MySqlPool;

pub struct CalcParamBiz {
    pub redis: RedisOp,
    pub mysql: MySqlPool,
}
impl CalcParamBiz {
    pub fn new(redis: RedisOp, mysql: MySqlPool) -> Self {
        CalcParamBiz { redis, mysql }
    }
}

#[async_trait::async_trait]
impl CrudOperations<CalcParam> for CalcParamBiz {
    async fn create(&self, item: CalcParam) -> Result<CalcParam, Error> {
        let mut updates = vec![];

        if let Some(protocol) = item.protocol {
            updates.push(("protocol", protocol.to_string()));
        }

        if let Some(identification_code) = item.identification_code {
            updates.push(("identification_code", identification_code.to_string()));
        }

        if let Some(device_uid) = item.device_uid {
            updates.push(("device_uid", device_uid.to_string()));
        }

        if let Some(name) = item.name {
            updates.push(("name", name.to_string()));
        }

        if let Some(signal_name) = item.signal_name {
            updates.push(("signal_name", signal_name.to_string()));
        }

        if let Some(signal_id) = item.signal_id {
            updates.push(("signal_id", signal_id.to_string()));
        }

        if let Some(reduce) = item.reduce {
            updates.push(("reduce", reduce.to_string()));
        }

        if let Some(calc_rule_id) = item.calc_rule_id {
            updates.push(("calc_rule_id", calc_rule_id.to_string()));
        }

        log::info!("Creating CalcParam with updates: {:?}", updates);

        let result = sql_utils::insert::<CalcParam>(&self.mysql, "calc_params", updates).await;

        result
    }

    async fn update(&self, id: i64, item: CalcParam) -> Result<CalcParam, Error> {
        let mut updates = vec![];

        if let Some(protocol) = item.protocol {
            updates.push(("protocol", protocol.to_string()));
        }

        if let Some(identification_code) = item.identification_code {
            updates.push(("identification_code", identification_code.to_string()));
        }

        if let Some(device_uid) = item.device_uid {
            updates.push(("device_uid", device_uid.to_string()));
        }

        if let Some(name) = item.name {
            updates.push(("name", name.to_string()));
        }

        if let Some(signal_name) = item.signal_name {
            updates.push(("signal_name", signal_name.to_string()));
        }

        if let Some(signal_id) = item.signal_id {
            updates.push(("signal_id", signal_id.to_string()));
        }

        if let Some(reduce) = item.reduce {
            updates.push(("reduce", reduce.to_string()));
        }

        if let Some(calc_rule_id) = item.calc_rule_id {
            updates.push(("calc_rule_id", calc_rule_id.to_string()));
        }

        log::info!("Updating CalcParam with ID {}: {:?}", id, updates);

        let result =
            sql_utils::update_by_id::<CalcParam>(&self.mysql, "calc_params", id, updates).await;

        match result {
            Ok(it) => Ok(it),
            Err(err) => Err(err),
        }
    }

    async fn delete(&self, id: i64) -> Result<CalcParam, Error> {
        log::info!("Deleting CalcParam with ID {}", id);

        sql_utils::delete_by_id(&self.mysql, "calc_params", id).await
    }

    async fn page(
        &self,
        filters: Vec<FilterInfo>,
        pagination: PaginationParams,
    ) -> Result<PaginationResult<CalcParam>, Error> {
        log::info!(
            "Fetching page of CalcParams with filters: {:?} and pagination: {:?}",
            filters,
            pagination
        );

        let result =
            sql_utils::paginate::<CalcParam>(&self.mysql, "calc_params", filters, pagination).await;

        result
    }

    async fn list(&self, filters: Vec<FilterInfo>) -> Result<Vec<CalcParam>, Error> {
        log::info!("Fetching list of CalcParams with filters: {:?}", filters);

        let result = sql_utils::list::<CalcParam>(&self.mysql, "calc_params", filters).await;
        result
    }

    async fn by_id(&self, id: i64) -> Result<CalcParam, Error> {
        let result = sql_utils::by_id_common::<CalcParam>(&self.mysql, "calc_params", id).await;
        result
    }
}
