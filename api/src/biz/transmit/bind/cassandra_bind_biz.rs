use crate::biz::notice::feishu_biz::FeiShuBiz;
use crate::biz::user_biz::UserBiz;
use crate::db::db_model::{CassandraTransmitBind, Signal, WebSocketHandler};
use anyhow::{Context, Error, Result};
use common_lib::redis_pool_utils::RedisOp;
use common_lib::sql_utils;
use common_lib::sql_utils::{CrudOperations, FilterInfo, PaginationParams, PaginationResult};
use sqlx::MySqlPool;

pub struct CassandraTransmitBindBiz {
    pub redis: RedisOp,
    pub mysql: MySqlPool,
}
impl CassandraTransmitBindBiz {
    pub fn new(redis: RedisOp, mysql: MySqlPool) -> Self {
        CassandraTransmitBindBiz { redis, mysql }
    }
}
#[async_trait::async_trait]
impl CrudOperations<CassandraTransmitBind> for CassandraTransmitBindBiz {
    async fn create(&self, item: CassandraTransmitBind) -> Result<CassandraTransmitBind, Error> {
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

        if let Some(cassandra_transmit_id) = item.cassandra_transmit_id {
            updates.push(("cassandra_transmit_id", cassandra_transmit_id.to_string()));
        }

        if let Some(database) = item.database {
            updates.push(("database", database.to_string()));
        }

        if let Some(table_name) = item.table_name {
            updates.push(("table_name", table_name.to_string()));
        }

        if let Some(script) = item.script {
            updates.push(("script", script.to_string()));
        }

        if let Some(enable) = item.enable {
            updates.push(("enable", enable.to_string()));
        }

        log::info!("Creating CassandraTransmitBind with updates: {:?}", updates);

        let result = sql_utils::insert::<CassandraTransmitBind>(
            &self.mysql,
            "cassandra_transmit_binds",
            updates,
        )
            .await;

        result
    }

    async fn update(
        &self,
        id: i64,
        item: CassandraTransmitBind,
    ) -> Result<CassandraTransmitBind, Error> {
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

        if let Some(cassandra_transmit_id) = item.cassandra_transmit_id {
            updates.push(("cassandra_transmit_id", cassandra_transmit_id.to_string()));
        }

        if let Some(database) = item.database {
            updates.push(("database", database.to_string()));
        }

        if let Some(table_name) = item.table_name {
            updates.push(("table_name", table_name.to_string()));
        }

        if let Some(script) = item.script {
            updates.push(("script", script.to_string()));
        }

        if let Some(enable) = item.enable {
            updates.push(("enable", enable.to_string()));
        }

        log::info!(
            "Updating CassandraTransmitBind with ID {}: {:?}",
            id,
            updates
        );

        let result = sql_utils::update_by_id::<CassandraTransmitBind>(
            &self.mysql,
            "cassandra_transmit_binds",
            id,
            updates,
        )
            .await;

        match result {
            Ok(it) => Ok(it),
            Err(err) => Err(err),
        }
    }

    async fn delete(&self, id: i64) -> Result<CassandraTransmitBind, Error> {
        log::info!("Deleting CassandraTransmitBind with ID {}", id);

        sql_utils::delete_by_id(&self.mysql, "cassandra_transmit_binds", id).await
    }

    async fn page(
        &self,
        filters: Vec<FilterInfo>,
        pagination: PaginationParams,
    ) -> Result<PaginationResult<CassandraTransmitBind>, Error> {
        log::info!(
            "Fetching page of CassandraTransmitBinds with filters: {:?} and pagination: {:?}",
            filters,
            pagination
        );

        let result = sql_utils::paginate::<CassandraTransmitBind>(
            &self.mysql,
            "cassandra_transmit_binds",
            filters,
            pagination,
        )
            .await;

        result
    }

    async fn list(&self, filters: Vec<FilterInfo>) -> Result<Vec<CassandraTransmitBind>, Error> {
        log::info!(
            "Fetching list of CassandraTransmitBinds with filters: {:?}",
            filters
        );

        let result = sql_utils::list::<CassandraTransmitBind>(
            &self.mysql,
            "cassandra_transmit_binds",
            filters,
        )
            .await;
        result
    }

    async fn by_id(&self, id: i64) -> Result<CassandraTransmitBind, Error> {
        let result = sql_utils::by_id_common::<CassandraTransmitBind>(
            &self.mysql,
            "cassandra_transmit_binds",
            id,
        )
            .await;
        result
    }
}
