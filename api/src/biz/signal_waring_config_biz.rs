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

use crate::db::db_model::SignalWaringConfig;
use anyhow::{Error, Result};
use common_lib::config::MongoConfig;
use common_lib::mongo_utils::MongoDBManager;
use common_lib::redis_pool_utils::RedisOp;
use common_lib::sql_utils::{CrudOperations, FilterInfo, PaginationParams, PaginationResult};
use serde_json;
use sqlx::MySqlPool;

pub struct SignalWaringConfigBiz {
    pub redis: RedisOp,
    pub mysql: MySqlPool,
    pub mongo: &'static MongoDBManager,
    pub mongo_config: MongoConfig,
}

impl SignalWaringConfigBiz {
    pub fn new(redis: RedisOp, mysql: MySqlPool, mongo: &'static MongoDBManager, mongo_config: MongoConfig) -> Self {
        SignalWaringConfigBiz { redis, mysql, mongo, mongo_config }
    }


    pub async fn set_signal_waring_cache(&self, signal_id: i64, config: &SignalWaringConfig) -> Result<(), Error> {
        let config_json = serde_json::to_string(config)
            .map_err(|e| Error::msg(format!("Failed to serialize config: {}", e)))?;

        let key = format!("waring:{}", signal_id);
        self.redis.push_list(&key, config_json.as_str())
            .map_err(|e| Error::msg(format!("Failed to push to Redis list: {}", e)))?;

        Ok(())
    }

    pub async fn remove_signal_waring_cache(&self, signal_id: i64, config: &SignalWaringConfig) -> Result<(), Error> {
        let config_json = serde_json::to_string(config)
            .map_err(|e| Error::msg(format!("Failed to serialize config: {}", e)))?;

        let key = format!("waring:{}", signal_id);
        self.redis.remove_from_list(&key, 0, config_json.as_str())
            .map_err(|e| Error::msg(format!("Failed to remove from Redis list: {}", e)))?;

        Ok(())
    }

    pub async fn init_mongo_collection(&self, warning_config: &SignalWaringConfig) -> Result<(), Error> {
        let waring_collection = self.mongo_config.waring_collection
            .as_ref()
            .ok_or_else(|| Error::msg("Warning collection name not configured"))?;

        let collection_name = common_lib::ut::calc_collection_name(
            waring_collection,
            warning_config.id.ok_or_else(|| Error::msg("Warning config id not found"))?,
        );

        if let Err(e) = self.mongo.create_collection(&collection_name).await {
            return Err(Error::msg(format!("Failed to create MongoDB collection: {}", e)));
        }

        Ok(())
    }
}

#[async_trait::async_trait]
impl CrudOperations<SignalWaringConfig> for SignalWaringConfigBiz {
    async fn create(&self, item: SignalWaringConfig) -> Result<SignalWaringConfig, Error> {
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

        if let Some(name) = item.in_or_out {
            updates.push(("name", name.to_string()));
        }

        if let Some(alias) = item.signal_id {
            updates.push(("alias", alias.to_string()));
        }

        if let Some(signal_type) = item.min {
            updates.push(("signal_type", signal_type.to_string()));
        }

        if let Some(unit) = item.max {
            updates.push(("unit", unit.to_string()));
        }
        let result = common_lib::sql_utils::insert::<SignalWaringConfig>(&self.mysql, "signal_waring_configs", updates).await;
        return result;
    }

    async fn update(&self, id: i64, item: SignalWaringConfig) -> Result<SignalWaringConfig, Error> {
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

        if let Some(name) = item.in_or_out {
            updates.push(("name", name.to_string()));
        }

        if let Some(alias) = item.signal_id {
            updates.push(("alias", alias.to_string()));
        }

        if let Some(signal_type) = item.min {
            updates.push(("signal_type", signal_type.to_string()));
        }

        if let Some(unit) = item.max {
            updates.push(("unit", unit.to_string()));
        }

        let result = common_lib::sql_utils::update_by_id::<SignalWaringConfig>(&self.mysql, "signal_waring_configs", id, updates).await;
        return result;
    }

    async fn delete(&self, id: i64) -> Result<SignalWaringConfig, Error> {
        let result = common_lib::sql_utils::delete_by_id::<SignalWaringConfig>(&self.mysql, "signal_waring_configs", id).await;
        return result;
    }

    async fn by_id(&self, id: i64) -> Result<SignalWaringConfig, Error> {
        let result = common_lib::sql_utils::by_id_common::<SignalWaringConfig>(&self.mysql, "signal_waring_configs", id).await;
        return result;
    }

    async fn list(&self, filters: Vec<FilterInfo>) -> Result<Vec<SignalWaringConfig>, Error> {
        let result = common_lib::sql_utils::list::<SignalWaringConfig>(&self.mysql, "signal_waring_configs", filters).await;
        return result;
    }

    async fn page(
        &self,
        filters: Vec<FilterInfo>,
        pagination: PaginationParams,
    ) -> Result<PaginationResult<SignalWaringConfig>, Error> {
        let result = common_lib::sql_utils::paginate::<SignalWaringConfig>(
            &self.mysql,
            "signal_waring_configs",
            filters,
            pagination,
        )
            .await;
        return result;
    }
}
