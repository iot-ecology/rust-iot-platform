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

use crate::biz::signal_delay_waring_biz::SignalDelayWaringBiz;
use crate::biz::user_biz::UserBiz;
use crate::db::db_model::{Signal, SignalDelayWaringParam, SimCard, WebSocketHandler};
use anyhow::{Context, Error, Result};
use common_lib::redis_pool_utils::RedisOp;
use common_lib::sql_utils::{CrudOperations, FilterInfo, PaginationParams, PaginationResult};
use sqlx::MySqlPool;

pub struct SignalDelayWaringParamBiz {
    pub redis: RedisOp,
    pub mysql: MySqlPool,
}

impl SignalDelayWaringParamBiz {
    pub fn new(redis: RedisOp, mysql: MySqlPool) -> Self {
        SignalDelayWaringParamBiz { redis, mysql }
    }

    pub async fn get_signal_by_id(&self, signal_id: i64) -> Result<Signal, Error> {
        let signal = sqlx::query_as::<_, Signal>(
            "SELECT * FROM signals WHERE id = ? AND deleted_at IS NULL"
        )
            .bind(signal_id)
            .fetch_one(&self.mysql)
            .await
            .context("Failed to get signal")?;

        Ok(signal)
    }

    pub async fn push_to_redis(&self, v: &SignalDelayWaringParam) -> Result<(), Error> {
        let result = serde_json::to_string(v).unwrap();
        self.redis.push_list("delay_param", &result)
            .context("Failed to push to Redis")?;
        Ok(())
    }

    pub async fn remove_from_redis(&self, v: &SignalDelayWaringParam) -> Result<(), Error> {
        let json_str = serde_json::to_string(v).unwrap();
        let count = self.redis.remove_from_list("delay_param", 0, &json_str)
            .context("Failed to remove from Redis")?;
        Ok(())
    }
}

#[async_trait::async_trait]
impl CrudOperations<SignalDelayWaringParam> for SignalDelayWaringParamBiz {
    async fn create(&self, item: SignalDelayWaringParam) -> Result<SignalDelayWaringParam, Error> {
        let mut updates = vec![];

        if let Some(protocol) = item.protocol {
            updates.push(("protocol", protocol));
        }

        if let Some(identification_code) = item.identification_code {
            updates.push(("identification_code", identification_code));
        }

        if let Some(device_uid) = item.device_uid {
            updates.push(("device_uid", device_uid.to_string()));
        }

        if let Some(name) = item.name {
            updates.push(("name", name));
        }

        if let Some(signal_name) = item.signal_name {
            updates.push(("signal_name", signal_name));
        }

        if let Some(signal_id) = item.signal_id {
            updates.push(("signal_id", signal_id.to_string()));
        }

        if let Some(signal_delay_waring_id) = item.signal_delay_waring_id {
            updates.push(("signal_delay_waring_id", signal_delay_waring_id.to_string()));
        }

        log::info!(
            "Creating signal delay warning param with updates: {:?}",
            updates
        );

        let result = common_lib::sql_utils::insert::<SignalDelayWaringParam>(
            &self.mysql,
            "signal_delay_waring_params",
            updates,
        )
            .await;

        result
    }

    async fn update(
        &self,
        id: i64,
        item: SignalDelayWaringParam,
    ) -> Result<SignalDelayWaringParam, Error> {
        let mut updates = vec![];

        if let Some(protocol) = item.protocol {
            updates.push(("protocol", protocol));
        }

        if let Some(identification_code) = item.identification_code {
            updates.push(("identification_code", identification_code));
        }

        if let Some(device_uid) = item.device_uid {
            updates.push(("device_uid", device_uid.to_string()));
        }

        if let Some(name) = item.name {
            updates.push(("name", name));
        }

        if let Some(signal_name) = item.signal_name {
            updates.push(("signal_name", signal_name));
        }

        if let Some(signal_id) = item.signal_id {
            updates.push(("signal_id", signal_id.to_string()));
        }

        if let Some(signal_delay_waring_id) = item.signal_delay_waring_id {
            updates.push(("signal_delay_waring_id", signal_delay_waring_id.to_string()));
        }

        log::info!(
            "Updating signal delay warning param with ID {}: {:?}",
            id,
            updates
        );

        let result = common_lib::sql_utils::update_by_id::<SignalDelayWaringParam>(
            &self.mysql,
            "signal_delay_waring_params",
            id,
            updates,
        )
            .await;

        return match result {
            Ok(it) => Ok(it),
            Err(err) => Err(err),
        };
    }

    async fn delete(&self, id: i64) -> Result<SignalDelayWaringParam, Error> {
        log::info!("Deleting signal delay warning param with ID {}", id);

        common_lib::sql_utils::delete_by_id(&self.mysql, "signal_delay_waring_params", id).await
    }

    async fn page(
        &self,
        filters: Vec<FilterInfo>,
        pagination: PaginationParams,
    ) -> Result<PaginationResult<SignalDelayWaringParam>, Error> {
        log::info!(
            "Fetching page of signal delay warning params with filters: {:?} and pagination: {:?}",
            filters,
            pagination
        );

        let result = common_lib::sql_utils::paginate::<SignalDelayWaringParam>(
            &self.mysql,
            "signal_delay_waring_params",
            filters,
            pagination,
        )
            .await;

        result
    }

    async fn list(&self, filters: Vec<FilterInfo>) -> Result<Vec<SignalDelayWaringParam>, Error> {
        log::info!(
            "Fetching list of signal delay warning params with filters: {:?}",
            filters
        );
        let result = common_lib::sql_utils::list::<SignalDelayWaringParam>(
            &self.mysql,
            "signal_delay_waring_params",
            filters,
        )
            .await;
        return result;
    }

    async fn by_id(&self, id: i64) -> Result<SignalDelayWaringParam, Error> {
        let result = common_lib::sql_utils::by_id_common::<SignalDelayWaringParam>(
            &self.mysql,
            "signal_delay_waring_params",
            id,
        )
            .await;
        result
    }
}
