use crate::biz::transmit::cassandra_transmit_biz::CassandraTransmitBiz;
use crate::biz::user_biz::UserBiz;
use crate::db::db_model::{ClickhouseTransmit, Signal, WebSocketHandler};
use anyhow::{Context, Error, Result};
use common_lib::redis_pool_utils::RedisOp;
use common_lib::sql_utils;
use common_lib::sql_utils::{CrudOperations, FilterInfo, PaginationParams, PaginationResult};
use sqlx::MySqlPool;

pub struct ClickhouseTransmitBiz {
    pub redis: RedisOp,
    pub mysql: MySqlPool,
}
impl ClickhouseTransmitBiz {
    pub fn new(redis: RedisOp, mysql: MySqlPool) -> Self {
        ClickhouseTransmitBiz { redis, mysql }
    }
}
#[async_trait::async_trait]
impl CrudOperations<ClickhouseTransmit> for ClickhouseTransmitBiz {
    async fn create(&self, item: ClickhouseTransmit) -> Result<ClickhouseTransmit, Error> {
        let mut updates = vec![];

        if let Some(name) = item.name {
            updates.push(("name", name.to_string()));
        }

        if let Some(host) = item.host {
            updates.push(("host", host.to_string()));
        }

        if let Some(port) = item.port {
            updates.push(("port", port.to_string()));
        }

        if let Some(username) = item.username {
            updates.push(("username", username.to_string()));
        }

        if let Some(password) = item.password {
            updates.push(("password", password.to_string()));
        }

        log::info!("Creating ClickhouseTransmit with updates: {:?}", updates);

        let result =
            sql_utils::insert::<ClickhouseTransmit>(&self.mysql, "clickhouse_transmits", updates)
                .await;

        result
    }

    async fn update(&self, id: i64, item: ClickhouseTransmit) -> Result<ClickhouseTransmit, Error> {
        let mut updates = vec![];

        if let Some(name) = item.name {
            updates.push(("name", name.to_string()));
        }

        if let Some(host) = item.host {
            updates.push(("host", host.to_string()));
        }

        if let Some(port) = item.port {
            updates.push(("port", port.to_string()));
        }

        if let Some(username) = item.username {
            updates.push(("username", username.to_string()));
        }

        if let Some(password) = item.password {
            updates.push(("password", password.to_string()));
        }

        log::info!("Updating ClickhouseTransmit with ID {}: {:?}", id, updates);

        let result = sql_utils::update_by_id::<ClickhouseTransmit>(
            &self.mysql,
            "clickhouse_transmits",
            id,
            updates,
        )
            .await;

        match result {
            Ok(it) => Ok(it),
            Err(err) => Err(err),
        }
    }

    async fn delete(&self, id: i64) -> Result<ClickhouseTransmit, Error> {
        log::info!("Deleting ClickhouseTransmit with ID {}", id);

        sql_utils::delete_by_id(&self.mysql, "clickhouse_transmits", id).await
    }

    async fn page(
        &self,
        filters: Vec<FilterInfo>,
        pagination: PaginationParams,
    ) -> Result<PaginationResult<ClickhouseTransmit>, Error> {
        log::info!(
            "Fetching page of ClickhouseTransmits with filters: {:?} and pagination: {:?}",
            filters,
            pagination
        );

        let result = sql_utils::paginate::<ClickhouseTransmit>(
            &self.mysql,
            "clickhouse_transmits",
            filters,
            pagination,
        )
            .await;

        result
    }

    async fn list(&self, filters: Vec<FilterInfo>) -> Result<Vec<ClickhouseTransmit>, Error> {
        log::info!(
            "Fetching list of ClickhouseTransmits with filters: {:?}",
            filters
        );

        let result =
            sql_utils::list::<ClickhouseTransmit>(&self.mysql, "clickhouse_transmits", filters)
                .await;
        result
    }

    async fn by_id(&self, id: i64) -> Result<ClickhouseTransmit, Error> {
        let result =
            sql_utils::by_id_common::<ClickhouseTransmit>(&self.mysql, "clickhouse_transmits", id)
                .await;
        result
    }
}
