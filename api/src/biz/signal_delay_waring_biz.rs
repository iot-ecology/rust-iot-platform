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

use crate::biz::signal_biz::SignalBiz;
use crate::biz::user_biz::UserBiz;
use crate::db::db_model::{SignalDelayWaring, SignalWaringConfig, SimCard, WebSocketHandler};
use anyhow::{Context as _, Error, Result};
use common_lib::config::MongoConfig;
use common_lib::mongo_utils::MongoDBManager;
use common_lib::redis_pool_utils::RedisOp;
use common_lib::sql_utils::{CrudOperations, FilterInfo, PaginationParams, PaginationResult};
use mongodb::{Client, Collection};
use serde_json;
use sqlx::MySqlPool;


pub struct SignalDelayWaringBiz {
    pub redis: RedisOp,
    pub mysql: MySqlPool,
    pub mongo: &'static MongoDBManager,
    pub mongo_config: MongoConfig,
}

impl SignalDelayWaringBiz {
    pub fn new(redis: RedisOp, mysql: MySqlPool, mongo: &'static MongoDBManager, mongo_config: MongoConfig) -> Self {
        SignalDelayWaringBiz { redis, mysql, mongo, mongo_config }
    }

    pub async fn init_mongo_collection(&self, warning_config: &SignalDelayWaring) -> Result<(), Error> {
        let waring_collection = self.mongo_config.script_waring_collection.as_ref().ok_or_else(|| Error::msg("script waring collection name not configured"))?;

        let collection_name = common_lib::ut::calc_collection_name(
            waring_collection,
            warning_config.id.ok_or_else(|| Error::msg("script waring config id not found"))?,
        );

        if let Err(e) = self.mongo.create_collection(&collection_name).await {
            return Err(Error::msg(format!("Failed to create MongoDB collection: {}", e)));
        }

        Ok(())
    }

    pub async fn set_redis(&self, warning_config: &SignalDelayWaring) {
        let result = serde_json::to_string(warning_config).unwrap();

        self.redis.set_hash("signal_delay_config", warning_config.id.unwrap().to_string().as_str(), &result).unwrap();
    }

    pub async fn remove_redis(&self, id: i64) -> Result<(), Error> {
        match self.redis.delete_hash_field("signal_delay_config", id.to_string().as_str()) {
            Ok(_) => Ok(()),
            Err(e) => Err(Error::msg(format!("Failed to remove from Redis: {}", e)))
        }
    }
}

#[async_trait::async_trait]
impl CrudOperations<SignalDelayWaring> for SignalDelayWaringBiz {
    async fn create(&self, item: SignalDelayWaring) -> Result<SignalDelayWaring, Error> {
        let mut updates = vec![];

        if let Some(name) = item.name {
            updates.push(("name", name));
        }

        if let Some(script) = item.script {
            updates.push(("script", script));
        }

        log::info!("Creating signal delay warning with updates: {:?}", updates);

        let result = common_lib::sql_utils::insert::<SignalDelayWaring>(
            &self.mysql,
            "signal_delay_warings",
            updates,
        ).await;

        result
    }

    async fn update(&self, id: i64, item: SignalDelayWaring) -> Result<SignalDelayWaring, Error> {
        let mut updates = vec![];

        if let Some(name) = item.name {
            updates.push(("name", name));
        }

        if let Some(script) = item.script {
            updates.push(("script", script));
        }

        log::info!(
            "Updating signal delay warning with ID {}: {:?}",
            id,
            updates
        );

        let result = common_lib::sql_utils::update_by_id::<SignalDelayWaring>(
            &self.mysql,
            "signal_delay_warings",
            id,
            updates,
        ).await;

        return match result {
            Ok(it) => Ok(it),
            Err(err) => Err(err),
        };
    }

    async fn delete(&self, id: i64) -> Result<SignalDelayWaring, Error> {
        log::info!("Deleting signal delay warning with ID {}", id);

        common_lib::sql_utils::delete_by_id(&self.mysql, "signal_delay_warings", id).await
    }

    async fn page(
        &self,
        filters: Vec<FilterInfo>,
        pagination: PaginationParams,
    ) -> Result<PaginationResult<SignalDelayWaring>, Error> {
        log::info!(
            "Fetching page of signal delay warnings with filters: {:?} and pagination: {:?}",
            filters,
            pagination
        );

        let result = common_lib::sql_utils::paginate::<SignalDelayWaring>(
            &self.mysql,
            "signal_delay_warings",
            filters,
            pagination,
        ).await;

        result
    }

    async fn list(&self, filters: Vec<FilterInfo>) -> Result<Vec<SignalDelayWaring>, Error> {
        log::info!(
            "Fetching list of signal delay warnings with filters: {:?}",
            filters
        );
        let result = common_lib::sql_utils::list::<SignalDelayWaring>(
            &self.mysql,
            "signal_delay_warings",
            filters,
        ).await;
        return result;
    }

    async fn by_id(&self, id: i64) -> Result<SignalDelayWaring, Error> {
        let result = common_lib::sql_utils::by_id_common::<SignalDelayWaring>(
            &self.mysql,
            "signal_delay_warings",
            id,
        ).await;
        result
    }
}
