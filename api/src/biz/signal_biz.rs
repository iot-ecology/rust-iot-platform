use crate::biz::shipment_record_biz::ShipmentRecordBiz;
use crate::biz::user_biz::UserBiz;
use crate::db::db_model::{Signal, SimCard, WebSocketHandler};
use anyhow::{Context, Error, Result};
use common_lib::redis_pool_utils::RedisOp;
use common_lib::sql_utils::{CrudOperations, FilterInfo, PaginationParams, PaginationResult};
use sqlx::MySqlPool;

pub struct SignalBiz {
    pub redis: RedisOp,
    pub mysql: MySqlPool,
}

impl SignalBiz {
    pub fn new(redis: RedisOp, mysql: MySqlPool) -> Self {
        SignalBiz { redis, mysql }
    }

    pub async fn remove_old_cache(&self, threshold: i64, signal_id: i64, device_uid: i64, code: &str) -> Result<(), Error> {
        let redis_key = format!("signal_delay_warning:{}:{}:{}", device_uid, code, signal_id);

        // Get current count of elements
        let count = self.redis.get_zset_length(&redis_key)
            .map_err(|e| anyhow::anyhow!("Failed to get sorted set size: {}", e))?;

        if count <= threshold {
            log::info!("No need to remove elements from {}", redis_key);
            return Ok(());
        }

        // Remove extra elements (oldest ones) one by one
        let to_remove = count - threshold;
        self.redis.zremrangebyrank(&redis_key, 0, to_remove as i64 - 1);
        log::info!("Removed {} extra elements from sorted set '{}'", to_remove, redis_key);
        Ok(())
    }

    pub async fn set_signal_cache(&self, signal: &Signal) -> Result<(), Error> {
        let json_data = serde_json::to_string(signal)
            .map_err(|e| anyhow::anyhow!("Failed to serialize signal: {}", e))?;

        let key = format!(
            "signal:{}:{}",
            signal.device_uid.unwrap_or_default(),
            signal.identification_code.as_deref().unwrap_or_default()
        );

        self.redis
            .push_list(key.as_str(), json_data.as_str())
            .map_err(|e| anyhow::anyhow!("Failed to push signal to Redis list: {}", e))?;

        Ok(())
    }

    pub async fn remove_signal_cache(&self, signal: &Signal) -> Result<(), Error> {
        let json_data = serde_json::to_string(signal)
            .map_err(|e| anyhow::anyhow!("Failed to serialize signal: {}", e))?;

        let key = format!(
            "signal:{}:{}",
            signal.device_uid.unwrap_or_default(),
            signal.identification_code.as_deref().unwrap_or_default()
        );

        self.redis
            .remove_from_list(key.as_str(), 0, json_data.as_str())
            .map_err(|e| anyhow::anyhow!("Failed to remove signal from Redis list: {}", e))?;

        Ok(())
    }
}

#[async_trait::async_trait]
impl CrudOperations<Signal> for SignalBiz {
    async fn create(&self, item: Signal) -> Result<Signal, Error> {
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

        if let Some(alias) = item.alias {
            updates.push(("alias", alias));
        }

        if let Some(signal_type) = item.signal_type {
            updates.push(("signal_type", signal_type));
        }

        if let Some(unit) = item.unit {
            updates.push(("unit", unit));
        }

        if let Some(cache_size) = item.cache_size {
            updates.push(("cache_size", cache_size.to_string()));
        }

        log::info!("Creating signal with updates: {:?}", updates);

        let result = common_lib::sql_utils::insert::<Signal>(&self.mysql, "signals", updates).await;

        result
    }

    async fn update(&self, id: i64, item: Signal) -> Result<Signal, Error> {
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

        if let Some(alias) = item.alias {
            updates.push(("alias", alias));
        }

        if let Some(signal_type) = item.signal_type {
            updates.push(("signal_type", signal_type));
        }

        if let Some(unit) = item.unit {
            updates.push(("unit", unit));
        }

        if let Some(cache_size) = item.cache_size {
            updates.push(("cache_size", cache_size.to_string()));
        }

        log::info!("Updating signal with ID {}: {:?}", id, updates);

        let result =
            common_lib::sql_utils::update_by_id::<Signal>(&self.mysql, "signals", id, updates)
                .await;

        return match result {
            Ok(it) => Ok(it),
            Err(err) => Err(err),
        };
    }

    async fn delete(&self, id: i64) -> Result<Signal, Error> {
        log::info!("Deleting signal with ID {}", id);

        common_lib::sql_utils::delete_by_id(&self.mysql, "signals", id).await
    }

    async fn page(
        &self,
        filters: Vec<FilterInfo>,
        pagination: PaginationParams,
    ) -> Result<PaginationResult<Signal>, Error> {
        log::info!(
            "Fetching page of signals with filters: {:?} and pagination: {:?}",
            filters,
            pagination
        );

        let result =
            common_lib::sql_utils::paginate::<Signal>(&self.mysql, "signals", filters, pagination)
                .await;

        result
    }

    async fn list(&self, filters: Vec<FilterInfo>) -> Result<Vec<Signal>, Error> {
        log::info!("Fetching list of signals with filters: {:?}", filters);
        let result = common_lib::sql_utils::list::<Signal>(&self.mysql, "signals", filters).await;
        return result;
    }

    async fn by_id(&self, id: i64) -> Result<Signal, Error> {
        let result =
            common_lib::sql_utils::by_id_common::<Signal>(&self.mysql, "signals", id).await;
        result
    }
}
