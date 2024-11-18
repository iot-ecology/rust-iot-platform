use crate::biz::transmit::mongo_transmit_biz::MongoTransmitBiz;
use crate::biz::user_biz::UserBiz;
use crate::db::db_model::{MysqlTransmit, Signal, WebSocketHandler};
use anyhow::{Context, Error, Result};
use common_lib::redis_pool_utils::RedisOp;
use common_lib::sql_utils;
use common_lib::sql_utils::{CrudOperations, FilterInfo, PaginationParams, PaginationResult};
use sqlx::MySqlPool;

pub struct MysqlTransmitBiz {
    pub redis: RedisOp,
    pub mysql: MySqlPool,
}
impl MysqlTransmitBiz {
    pub fn new(redis: RedisOp, mysql: MySqlPool) -> Self {
        MysqlTransmitBiz { redis, mysql }
    }
}

#[async_trait::async_trait]
impl CrudOperations<MysqlTransmit> for MysqlTransmitBiz {
    async fn create(&self, item: MysqlTransmit) -> Result<MysqlTransmit, Error> {
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

        if let Some(database) = item.database {
            updates.push(("database", database.to_string()));
        }

        log::info!("Creating MysqlTransmit with updates: {:?}", updates);

        let result =
            sql_utils::insert::<MysqlTransmit>(&self.mysql, "mysql_transmits", updates).await;

        result
    }

    async fn update(&self, id: u64, item: MysqlTransmit) -> Result<MysqlTransmit, Error> {
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

        if let Some(database) = item.database {
            updates.push(("database", database.to_string()));
        }

        log::info!("Updating MysqlTransmit with ID {}: {:?}", id, updates);

        let result =
            sql_utils::update_by_id::<MysqlTransmit>(&self.mysql, "mysql_transmits", id, updates)
                .await;

        match result {
            Ok(it) => Ok(it),
            Err(err) => Err(err),
        }
    }

    async fn delete(&self, id: u64) -> Result<(), Error> {
        log::info!("Deleting MysqlTransmit with ID {}", id);

        sql_utils::delete_by_id(&self.mysql, "mysql_transmits", id).await
    }

    async fn page(
        &self,
        filters: Vec<FilterInfo>,
        pagination: PaginationParams,
    ) -> Result<PaginationResult<MysqlTransmit>, Error> {
        log::info!(
            "Fetching page of MysqlTransmits with filters: {:?} and pagination: {:?}",
            filters,
            pagination
        );

        let result = sql_utils::paginate::<MysqlTransmit>(
            &self.mysql,
            "mysql_transmits",
            filters,
            pagination,
        )
        .await;

        result
    }

    async fn list(&self, filters: Vec<FilterInfo>) -> Result<Vec<MysqlTransmit>, Error> {
        log::info!(
            "Fetching list of MysqlTransmits with filters: {:?}",
            filters
        );

        let result =
            sql_utils::list::<MysqlTransmit>(&self.mysql, "mysql_transmits", filters).await;
        result
    }

    async fn by_id(&self, id: u64) -> Result<MysqlTransmit, Error> {
        let result =
            sql_utils::by_id_common::<MysqlTransmit>(&self.mysql, "mysql_transmits", id).await;
        result
    }
}
