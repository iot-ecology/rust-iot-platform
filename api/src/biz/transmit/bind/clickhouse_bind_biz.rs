use crate::biz::user_biz::UserBiz;
use crate::db::db_model::{ClickhouseTransmitBind, Signal, WebSocketHandler};
use anyhow::{Context, Error, Result};
use common_lib::redis_pool_utils::RedisOp;
use common_lib::sql_utils;
use common_lib::sql_utils::{CrudOperations, Filter, PaginationParams, PaginationResult};
use sqlx::MySqlPool;
pub struct ClickhouseTransmitBindBiz {
    pub redis: RedisOp,
    pub mysql: MySqlPool,
}

#[async_trait::async_trait]
impl CrudOperations<ClickhouseTransmitBind> for ClickhouseTransmitBindBiz {
    async fn create(&self, item: ClickhouseTransmitBind) -> Result<ClickhouseTransmitBind, Error> {
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

        if let Some(clickhouse_transmit_id) = item.clickhouse_transmit_id {
            updates.push(("clickhouse_transmit_id", clickhouse_transmit_id.to_string()));
        }

        if let Some(database) = item.database {
            updates.push(("database", database.to_string()));
        }

        if let Some(script) = item.script {
            updates.push(("script", script.to_string()));
        }

        if let Some(table_name) = item.table_name {
            updates.push(("table_name", table_name.to_string()));
        }

        if let Some(enable) = item.enable {
            updates.push(("enable", enable.to_string()));
        }

        log::info!(
            "Creating ClickhouseTransmitBind with updates: {:?}",
            updates
        );

        let result = sql_utils::insert::<ClickhouseTransmitBind>(
            &self.mysql,
            "clickhouse_transmit_binds",
            updates,
        )
        .await;

        result
    }

    async fn update(
        &self,
        id: u64,
        item: ClickhouseTransmitBind,
    ) -> Result<ClickhouseTransmitBind, Error> {
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

        if let Some(clickhouse_transmit_id) = item.clickhouse_transmit_id {
            updates.push(("clickhouse_transmit_id", clickhouse_transmit_id.to_string()));
        }

        if let Some(database) = item.database {
            updates.push(("database", database.to_string()));
        }

        if let Some(script) = item.script {
            updates.push(("script", script.to_string()));
        }

        if let Some(table_name) = item.table_name {
            updates.push(("table_name", table_name.to_string()));
        }

        if let Some(enable) = item.enable {
            updates.push(("enable", enable.to_string()));
        }

        log::info!(
            "Updating ClickhouseTransmitBind with ID {}: {:?}",
            id,
            updates
        );

        let result = sql_utils::update_by_id::<ClickhouseTransmitBind>(
            &self.mysql,
            "clickhouse_transmit_binds",
            id,
            updates,
        )
        .await;

        match result {
            Ok(it) => Ok(it),
            Err(err) => Err(err),
        }
    }

    async fn delete(&self, id: u64) -> Result<(), Error> {
        log::info!("Deleting ClickhouseTransmitBind with ID {}", id);

        sql_utils::delete_by_id(&self.mysql, "clickhouse_transmit_binds", id).await
    }

    async fn page(
        &self,
        filters: Vec<Filter>,
        pagination: PaginationParams,
    ) -> Result<PaginationResult<ClickhouseTransmitBind>, Error> {
        log::info!(
            "Fetching page of ClickhouseTransmitBinds with filters: {:?} and pagination: {:?}",
            filters,
            pagination
        );

        let result = sql_utils::paginate::<ClickhouseTransmitBind>(
            &self.mysql,
            "clickhouse_transmit_binds",
            filters,
            pagination,
        )
        .await;

        result
    }

    async fn list(&self, filters: Vec<Filter>) -> Result<Vec<ClickhouseTransmitBind>, Error> {
        log::info!(
            "Fetching list of ClickhouseTransmitBinds with filters: {:?}",
            filters
        );

        let result = sql_utils::list::<ClickhouseTransmitBind>(
            &self.mysql,
            "clickhouse_transmit_binds",
            filters,
        )
        .await;
        result
    }

    async fn by_id(&self, id: u64) -> Result<ClickhouseTransmitBind, Error> {
        let result = sql_utils::by_id_common::<ClickhouseTransmitBind>(
            &self.mysql,
            "clickhouse_transmit_binds",
            id,
        )
        .await;
        result
    }
}
