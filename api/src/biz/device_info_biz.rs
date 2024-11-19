use crate::biz::device_group_biz::DeviceGroupBiz;
use crate::biz::user_biz::UserBiz;
use crate::db::db_model::{DeviceInfo, Signal};
use anyhow::{Context, Error, Result};
use common_lib::redis_pool_utils::RedisOp;
use common_lib::sql_utils::{CrudOperations, FilterInfo, PaginationParams, PaginationResult};
use serde_json;
use sqlx::MySqlPool;

#[derive(Debug)]
pub struct DeviceInfoBiz {
    pub redis: RedisOp,
    pub mysql: MySqlPool,
}
impl DeviceInfoBiz {
    pub async fn find_by_sn(&self, sn: Option<String>) -> Result<Option<DeviceInfo>, Error> {
        if sn.is_none() {
            return Ok(None);
        }

        let sql = "select * from device_infos where sn = ?";

        let record = sqlx::query_as::<_, DeviceInfo>(sql)
            .bind(sn.clone().unwrap())
            .fetch_optional(&self.mysql)
            .await
            .with_context(|| {
                format!(
                    "Failed to fetch updated record from table '{}' with username {:?}",
                    "device_infos",
                    sn.clone()
                )
            });

        return match record {
            Ok(u) => Ok(u),
            Err(ee) => Err(ee),
        };
    }

    pub fn new(redis: RedisOp, mysql: MySqlPool) -> Self {
        DeviceInfoBiz { redis, mysql }
    }

    pub async fn set_redis(&self, device_info: &DeviceInfo) -> Result<(), Error> {
        let json_data = serde_json::to_string(device_info)
            .map_err(|e| anyhow::anyhow!("Failed to serialize device info: {}", e))?;

        // 保存到 struct:device_info hash
        if let Some(id) = device_info.id {
            self.redis.set_hash(
                "struct:device_info",
                id.to_string().as_str(),
                json_data.as_str(),
            ).map_err(|e| anyhow::anyhow!("Failed to set hash for device id {}: {}", id, e))?;
        }

        // 保存到 share:device_info:{protocol}:{device_uid}:{identification_code}
        if let (Some(protocol), Some(device_uid), Some(identification_code)) =
            (device_info.protocol.as_ref(), device_info.device_uid, device_info.identification_code.as_ref()) {
            let key = format!(
                "share:device_info:{}:{}:{}",
                protocol,
                device_uid,
                identification_code
            );
            self.redis.set_string(key.as_str(), json_data.as_str())
                .map_err(|e| anyhow::anyhow!("Failed to set string for key {}: {}", key, e))?;
        }

        Ok(())
    }


    pub async fn bind_mqtt() {}

    pub async fn query_mqtt(&self, device_info_id: u64) {}
    pub async fn query_tcp(&self, device_info_id: u64) {}
    pub async fn query_http(&self, device_info_id: u64) {}
    pub async fn query_ws(&self, device_info_id: u64) {}
    pub async fn query_coap(&self, device_info_id: u64) {}
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
