use crate::biz::transmit::bind::mysql_bind_biz::MysqlTransmitBindBiz;
use crate::biz::user_biz::UserBiz;
use crate::db::db_model::{CassandraTransmit, Signal, WebSocketHandler};
use anyhow::{Context, Error, Result};
use common_lib::redis_pool_utils::RedisOp;
use common_lib::sql_utils;
use common_lib::sql_utils::{CrudOperations, FilterInfo, PaginationParams, PaginationResult};
use sqlx::MySqlPool;

pub struct CassandraTransmitBiz {
    pub redis: RedisOp,
    pub mysql: MySqlPool,
}
impl CassandraTransmitBiz {
    pub fn new(redis: RedisOp, mysql: MySqlPool) -> Self {
        CassandraTransmitBiz { redis, mysql }
    }
}

#[async_trait::async_trait]
impl CrudOperations<CassandraTransmit> for CassandraTransmitBiz {
    async fn create(&self, item: CassandraTransmit) -> Result<CassandraTransmit, Error> {
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

        log::info!("Creating CassandraTransmit with updates: {:?}", updates);

        let result =
            sql_utils::insert::<CassandraTransmit>(&self.mysql, "cassandra_transmits", updates)
                .await;

        result
    }

    async fn update(&self, id: u64, item: CassandraTransmit) -> Result<CassandraTransmit, Error> {
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

        log::info!("Updating CassandraTransmit with ID {}: {:?}", id, updates);

        let result = sql_utils::update_by_id::<CassandraTransmit>(
            &self.mysql,
            "cassandra_transmits",
            id,
            updates,
        )
        .await;

        match result {
            Ok(it) => Ok(it),
            Err(err) => Err(err),
        }
    }

    async fn delete(&self, id: u64) -> Result<CassandraTransmit, Error> {
        log::info!("Deleting CassandraTransmit with ID {}", id);

        sql_utils::delete_by_id(&self.mysql, "cassandra_transmits", id).await
    }

    async fn page(
        &self,
        filters: Vec<FilterInfo>,
        pagination: PaginationParams,
    ) -> Result<PaginationResult<CassandraTransmit>, Error> {
        log::info!(
            "Fetching page of CassandraTransmits with filters: {:?} and pagination: {:?}",
            filters,
            pagination
        );

        let result = sql_utils::paginate::<CassandraTransmit>(
            &self.mysql,
            "cassandra_transmits",
            filters,
            pagination,
        )
        .await;

        result
    }

    async fn list(&self, filters: Vec<FilterInfo>) -> Result<Vec<CassandraTransmit>, Error> {
        log::info!(
            "Fetching list of CassandraTransmits with filters: {:?}",
            filters
        );

        let result =
            sql_utils::list::<CassandraTransmit>(&self.mysql, "cassandra_transmits", filters).await;
        result
    }

    async fn by_id(&self, id: u64) -> Result<CassandraTransmit, Error> {
        let result =
            sql_utils::by_id_common::<CassandraTransmit>(&self.mysql, "cassandra_transmits", id)
                .await;
        result
    }
}
