use crate::biz::transmit::bind::clickhouse_bind_biz::ClickhouseTransmitBindBiz;
use crate::biz::user_biz::UserBiz;
use crate::db::db_model::{InfluxDbTransmitBind, Signal, WebSocketHandler};
use anyhow::{Context, Error, Result};
use common_lib::redis_pool_utils::RedisOp;
use common_lib::sql_utils;
use common_lib::sql_utils::{CrudOperations, FilterInfo, PaginationParams, PaginationResult};
use sqlx::MySqlPool;

pub struct InfluxDbTransmitBindBiz {
    pub redis: RedisOp,
    pub mysql: MySqlPool,
}
impl InfluxDbTransmitBindBiz {
    pub fn new(redis: RedisOp, mysql: MySqlPool) -> Self {
        InfluxDbTransmitBindBiz { redis, mysql }
    }
}
#[async_trait::async_trait]
impl CrudOperations<InfluxDbTransmitBind> for InfluxDbTransmitBindBiz {
    async fn create(&self, item: InfluxDbTransmitBind) -> Result<InfluxDbTransmitBind, Error> {
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

        if let Some(influxdb_transmit_id) = item.influxdb_transmit_id {
            updates.push(("influxdb_transmit_id", influxdb_transmit_id.to_string()));
        }

        if let Some(bucket) = item.bucket {
            updates.push(("bucket", bucket.to_string()));
        }

        if let Some(org) = item.org {
            updates.push(("org", org.to_string()));
        }

        if let Some(measurement) = item.measurement {
            updates.push(("measurement", measurement.to_string()));
        }

        if let Some(script) = item.script {
            updates.push(("script", script.to_string()));
        }

        if let Some(enable) = item.enable {
            updates.push(("enable", enable.to_string()));
        }

        log::info!("Creating InfluxDbTransmitBind with updates: {:?}", updates);

        let result = sql_utils::insert::<InfluxDbTransmitBind>(
            &self.mysql,
            "influxdb_transmit_binds",
            updates,
        )
            .await;

        result
    }

    async fn update(
        &self,
        id: i64,
        item: InfluxDbTransmitBind,
    ) -> Result<InfluxDbTransmitBind, Error> {
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

        if let Some(influxdb_transmit_id) = item.influxdb_transmit_id {
            updates.push(("influxdb_transmit_id", influxdb_transmit_id.to_string()));
        }

        if let Some(bucket) = item.bucket {
            updates.push(("bucket", bucket.to_string()));
        }

        if let Some(org) = item.org {
            updates.push(("org", org.to_string()));
        }

        if let Some(measurement) = item.measurement {
            updates.push(("measurement", measurement.to_string()));
        }

        if let Some(script) = item.script {
            updates.push(("script", script.to_string()));
        }

        if let Some(enable) = item.enable {
            updates.push(("enable", enable.to_string()));
        }

        log::info!(
            "Updating InfluxDbTransmitBind with ID {}: {:?}",
            id,
            updates
        );

        let result = sql_utils::update_by_id::<InfluxDbTransmitBind>(
            &self.mysql,
            "influxdb_transmit_binds",
            id,
            updates,
        )
            .await;

        match result {
            Ok(it) => Ok(it),
            Err(err) => Err(err),
        }
    }

    async fn delete(&self, id: i64) -> Result<InfluxDbTransmitBind, Error> {
        log::info!("Deleting InfluxDbTransmitBind with ID {}", id);

        sql_utils::delete_by_id(&self.mysql, "influxdb_transmit_binds", id).await
    }

    async fn page(
        &self,
        filters: Vec<FilterInfo>,
        pagination: PaginationParams,
    ) -> Result<PaginationResult<InfluxDbTransmitBind>, Error> {
        log::info!(
            "Fetching page of InfluxDbTransmitBinds with filters: {:?} and pagination: {:?}",
            filters,
            pagination
        );

        let result = sql_utils::paginate::<InfluxDbTransmitBind>(
            &self.mysql,
            "influxdb_transmit_binds",
            filters,
            pagination,
        )
            .await;

        result
    }

    async fn list(&self, filters: Vec<FilterInfo>) -> Result<Vec<InfluxDbTransmitBind>, Error> {
        log::info!(
            "Fetching list of InfluxDbTransmitBinds with filters: {:?}",
            filters
        );

        let result = sql_utils::list::<InfluxDbTransmitBind>(
            &self.mysql,
            "influxdb_transmit_binds",
            filters,
        )
            .await;
        result
    }

    async fn by_id(&self, id: i64) -> Result<InfluxDbTransmitBind, Error> {
        let result = sql_utils::by_id_common::<InfluxDbTransmitBind>(
            &self.mysql,
            "influxdb_transmit_binds",
            id,
        )
            .await;
        result
    }
}
