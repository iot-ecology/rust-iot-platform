use crate::biz::signal_biz::SignalBiz;
use crate::biz::user_biz::UserBiz;
use crate::db::db_model::{SignalDelayWaring, SimCard, WebSocketHandler, SignalWaringConfig};
use anyhow::{Context as _, Error, Result};
use common_lib::redis_pool_utils::RedisOp;
use common_lib::sql_utils::{CrudOperations, FilterInfo, PaginationParams, PaginationResult};
use sqlx::MySqlPool;
use serde_json;
use mongodb::{Client, Collection};
use common_lib::config::MongoConfig;
use common_lib::mongo_utils::MongoDBManager;


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

    pub async fn init_mongo_collection(&self, warning_config: &SignalWaringConfig) -> Result<(), Error> {
        let waring_collection = self.mongo_config.waring_collection
            .as_ref()
            .ok_or_else(|| Error::msg("Warning collection name not configured"))?;
            
        let collection_name = common_lib::ut::calc_collection_name(
            waring_collection,
            warning_config.id.ok_or_else(|| Error::msg("Warning config id not found"))?
        );
        
        if let Err(e) = self.mongo.create_collection(&collection_name).await {
            return Err(Error::msg(format!("Failed to create MongoDB collection: {}", e)));
        }
        
        Ok(())
    }

    pub fn set_signal_waring_cache(
        &self, 
        signal_id: u64,
        warning_config: &SignalWaringConfig
    ) -> Result<(), Error> {
        let key = format!("waring:{}", signal_id);
        let serialized_config = serde_json::to_string(&warning_config)
            .context("Failed to serialize warning config")?;
        self.redis.push_list(&key, &serialized_config)
            .context("Failed to push warning config to Redis")
    }

    pub fn remove_signal_waring_cache(
        &self,
        signal_id: u64,
        warning_config: &SignalWaringConfig
    ) -> Result<(), Error> {
        let key = format!("waring:{}", signal_id);
        let serialized_config = serde_json::to_string(&warning_config)
            .context("Failed to serialize warning config")?;
        self.redis.remove_from_list(&key, 0, &serialized_config)
            .context("Failed to remove warning config from Redis")
    }

    pub fn get_signal_waring_cache(
        &self,
        signal_id: u64
    ) -> Result<Vec<SignalWaringConfig>, Error> {
        let key = format!("waring:{}", signal_id);
        let configs = self.redis.get_list_all(&key)
            .context("Failed to get warning configs from Redis")?;
        
        let mut result = Vec::new();
        for config_str in configs {
            match serde_json::from_str(&config_str) {
                Ok(config) => result.push(config),
                Err(e) => {
                    log::error!("Failed to deserialize warning config: {}", e);
                    continue;
                }
            }
        }
        Ok(result)
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
        )
        .await;

        result
    }

    async fn update(&self, id: u64, item: SignalDelayWaring) -> Result<SignalDelayWaring, Error> {
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
        )
        .await;

        return match result {
            Ok(it) => Ok(it),
            Err(err) => Err(err),
        };
    }

    async fn delete(&self, id: u64) -> Result<SignalDelayWaring, Error> {
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
        )
        .await;

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
        )
        .await;
        return result;
    }

    async fn by_id(&self, id: u64) -> Result<SignalDelayWaring, Error> {
        let result = common_lib::sql_utils::by_id_common::<SignalDelayWaring>(
            &self.mysql,
            "signal_delay_warings",
            id,
        )
        .await;
        result
    }
}
