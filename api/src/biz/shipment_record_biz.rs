use crate::biz::role_biz::RoleBiz;
use crate::biz::user_biz::UserBiz;
use crate::db::db_model::{ShipmentRecord, Signal, WebSocketHandler};
use anyhow::{Context, Error, Result};
use common_lib::redis_pool_utils::RedisOp;
use common_lib::sql_utils::{CrudOperations, FilterInfo, PaginationParams, PaginationResult};
use sqlx::MySqlPool;

pub struct ShipmentRecordBiz {
    pub redis: RedisOp,
    pub mysql: MySqlPool,
}
impl ShipmentRecordBiz {
    pub fn new(redis: RedisOp, mysql: MySqlPool) -> Self {
        ShipmentRecordBiz { redis, mysql }
    }
}

#[async_trait::async_trait]
impl CrudOperations<ShipmentRecord> for ShipmentRecordBiz {
    async fn create(&self, item: ShipmentRecord) -> Result<ShipmentRecord, Error> {
        let mut updates = vec![];

        if let Some(shipment_date) = item.shipment_date {
            updates.push(("shipment_date", shipment_date.to_string()));
        }

        if let Some(technician) = item.technician {
            updates.push(("technician", technician));
        }

        if let Some(customer_name) = item.customer_name {
            updates.push(("customer_name", customer_name));
        }

        if let Some(customer_phone) = item.customer_phone {
            updates.push(("customer_phone", customer_phone));
        }

        if let Some(customer_address) = item.customer_address {
            updates.push(("customer_address", customer_address));
        }

        if let Some(tracking_number) = item.tracking_number {
            updates.push(("tracking_number", tracking_number));
        }

        if let Some(status) = item.status {
            updates.push(("status", status));
        }

        if let Some(description) = item.description {
            updates.push(("description", description));
        }

        log::info!("Creating shipment record with updates: {:?}", updates);

        let result = common_lib::sql_utils::insert::<ShipmentRecord>(
            &self.mysql,
            "shipment_records",
            updates,
        )
            .await;

        result
    }

    async fn update(&self, id: i64, item: ShipmentRecord) -> Result<ShipmentRecord, Error> {
        let mut updates = vec![];

        if let Some(shipment_date) = item.shipment_date {
            updates.push(("shipment_date", shipment_date.to_string()));
        }

        if let Some(technician) = item.technician {
            updates.push(("technician", technician));
        }

        if let Some(customer_name) = item.customer_name {
            updates.push(("customer_name", customer_name));
        }

        if let Some(customer_phone) = item.customer_phone {
            updates.push(("customer_phone", customer_phone));
        }

        if let Some(customer_address) = item.customer_address {
            updates.push(("customer_address", customer_address));
        }

        if let Some(tracking_number) = item.tracking_number {
            updates.push(("tracking_number", tracking_number));
        }

        if let Some(status) = item.status {
            updates.push(("status", status));
        }

        if let Some(description) = item.description {
            updates.push(("description", description));
        }

        log::info!("Updating shipment record with ID {}: {:?}", id, updates);

        let result = common_lib::sql_utils::update_by_id::<ShipmentRecord>(
            &self.mysql,
            "shipment_records",
            id,
            updates,
        )
            .await;

        return match result {
            Ok(it) => Ok(it),
            Err(err) => Err(err),
        };
    }

    async fn delete(&self, id: i64) -> Result<ShipmentRecord, Error> {
        log::info!("Deleting shipment record with ID {}", id);

        common_lib::sql_utils::delete_by_id(&self.mysql, "shipment_records", id).await
    }

    async fn page(
        &self,
        filters: Vec<FilterInfo>,
        pagination: PaginationParams,
    ) -> Result<PaginationResult<ShipmentRecord>, Error> {
        log::info!(
            "Fetching page of shipment records with filters: {:?} and pagination: {:?}",
            filters,
            pagination
        );

        let result = common_lib::sql_utils::paginate::<ShipmentRecord>(
            &self.mysql,
            "shipment_records",
            filters,
            pagination,
        )
            .await;

        result
    }

    async fn list(&self, filters: Vec<FilterInfo>) -> Result<Vec<ShipmentRecord>, Error> {
        log::info!(
            "Fetching list of shipment records with filters: {:?}",
            filters
        );
        let result =
            common_lib::sql_utils::list::<ShipmentRecord>(&self.mysql, "shipment_records", filters)
                .await;
        return result;
    }

    async fn by_id(&self, id: i64) -> Result<ShipmentRecord, Error> {
        let result = common_lib::sql_utils::by_id_common::<ShipmentRecord>(
            &self.mysql,
            "shipment_records",
            id,
        )
            .await;
        result
    }
}
