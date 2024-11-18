use crate::biz::device_group_biz::DeviceGroupBiz;
use crate::biz::user_biz::UserBiz;
use crate::db::db_model::{DeviceInfo, Signal};
use anyhow::{Context, Error, Result};
use common_lib::redis_pool_utils::RedisOp;
use common_lib::sql_utils::{CrudOperations, FilterInfo, PaginationParams, PaginationResult};
use sqlx::MySqlPool;

#[derive(Debug)]
pub struct DeviceInfoBiz {
    pub redis: RedisOp,
    pub mysql: MySqlPool,
}
impl DeviceInfoBiz {
    pub fn new(redis: RedisOp, mysql: MySqlPool) -> Self {
        DeviceInfoBiz { redis, mysql }
    }
}

#[async_trait::async_trait]
impl CrudOperations<DeviceInfo> for DeviceInfoBiz {
    async fn create(&self, item: DeviceInfo) -> Result<DeviceInfo, Error> {
        let mut updates = vec![];

        if let Some(product_id) = item.product_id {
            updates.push(("product_id", product_id.to_string()));
        }

        if let Some(sn) = item.sn {
            updates.push(("sn", sn));
        }

        if let Some(manufacturing_date) = item.manufacturing_date {
            updates.push(("manufacturing_date", manufacturing_date.to_string()));
        }

        if let Some(procurement_date) = item.procurement_date {
            updates.push(("procurement_date", procurement_date.to_string()));
        }

        if let Some(source) = item.source {
            updates.push(("source", source.to_string()));
        }

        if let Some(warranty_expiry) = item.warranty_expiry {
            updates.push(("warranty_expiry", warranty_expiry.to_string()));
        }

        if let Some(push_interval) = item.push_interval {
            updates.push(("push_interval", push_interval.to_string()));
        }

        if let Some(error_rate) = item.error_rate {
            updates.push(("error_rate", error_rate.to_string()));
        }

        if let Some(protocol) = item.protocol {
            updates.push(("protocol", protocol));
        }

        if let Some(identification_code) = item.identification_code {
            updates.push(("identification_code", identification_code));
        }

        if let Some(device_uid) = item.device_uid {
            updates.push(("device_uid", device_uid.to_string()));
        }

        log::info!("Creating DeviceInfo with updates: {:?}", updates);

        let result =
            common_lib::sql_utils::insert::<DeviceInfo>(&self.mysql, "device_infos", updates).await;

        result
    }

    async fn update(&self, id: u64, item: DeviceInfo) -> Result<DeviceInfo, Error> {
        let mut updates = vec![];

        if let Some(product_id) = item.product_id {
            updates.push(("product_id", product_id.to_string()));
        }

        if let Some(sn) = item.sn {
            updates.push(("sn", sn));
        }

        if let Some(manufacturing_date) = item.manufacturing_date {
            updates.push(("manufacturing_date", manufacturing_date.to_string()));
        }

        if let Some(procurement_date) = item.procurement_date {
            updates.push(("procurement_date", procurement_date.to_string()));
        }

        if let Some(source) = item.source {
            updates.push(("source", source.to_string()));
        }

        if let Some(warranty_expiry) = item.warranty_expiry {
            updates.push(("warranty_expiry", warranty_expiry.to_string()));
        }

        if let Some(push_interval) = item.push_interval {
            updates.push(("push_interval", push_interval.to_string()));
        }

        if let Some(error_rate) = item.error_rate {
            updates.push(("error_rate", error_rate.to_string()));
        }

        if let Some(protocol) = item.protocol {
            updates.push(("protocol", protocol));
        }

        if let Some(identification_code) = item.identification_code {
            updates.push(("identification_code", identification_code));
        }

        if let Some(device_uid) = item.device_uid {
            updates.push(("device_uid", device_uid.to_string()));
        }

        log::info!("Updating DeviceInfo with ID {}: {:?}", id, updates);

        let result = common_lib::sql_utils::update_by_id::<DeviceInfo>(
            &self.mysql,
            "device_infos",
            id,
            updates,
        )
        .await;

        match result {
            Ok(it) => Ok(it),
            Err(err) => Err(err),
        }
    }

    async fn delete(&self, id: u64) -> Result<DeviceInfo, Error> {
        log::info!("Deleting DeviceInfo with ID {}", id);
        common_lib::sql_utils::delete_by_id(&self.mysql, "device_infos", id).await
    }

    async fn page(
        &self,
        filters: Vec<FilterInfo>,
        pagination: PaginationParams,
    ) -> Result<PaginationResult<DeviceInfo>, Error> {
        log::info!(
            "Fetching page of DeviceInfos with filters: {:?} and pagination: {:?}",
            filters,
            pagination
        );

        let result = common_lib::sql_utils::paginate::<DeviceInfo>(
            &self.mysql,
            "device_infos",
            filters,
            pagination,
        )
        .await;

        result
    }

    async fn list(&self, filters: Vec<FilterInfo>) -> Result<Vec<DeviceInfo>, Error> {
        log::info!("Fetching list of DeviceInfos with filters: {:?}", filters);
        let result =
            common_lib::sql_utils::list::<DeviceInfo>(&self.mysql, "device_infos", filters).await;
        result
    }

    async fn by_id(&self, id: u64) -> Result<DeviceInfo, Error> {
        let result =
            common_lib::sql_utils::by_id_common::<DeviceInfo>(&self.mysql, "device_infos", id)
                .await;
        result
    }
}
