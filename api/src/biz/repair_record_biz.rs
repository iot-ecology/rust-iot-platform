/*
Copyright [2025] [Zen HuiFer]

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use crate::biz::production_plan_biz::ProductionPlanBiz;
use crate::biz::user_biz::UserBiz;
use crate::db::db_model::{RepairRecord, Signal, WebSocketHandler};
use anyhow::{Context, Error, Result};
use common_lib::redis_pool_utils::RedisOp;
use common_lib::sql_utils::{CrudOperations, FilterInfo, PaginationParams, PaginationResult};
use sqlx::MySqlPool;

pub struct RepairRecordBiz {
    pub redis: RedisOp,
    pub mysql: MySqlPool,
}
impl RepairRecordBiz {
    pub fn new(redis: RedisOp, mysql: MySqlPool) -> Self {
        RepairRecordBiz { redis, mysql }
    }
}

#[async_trait::async_trait]
impl CrudOperations<RepairRecord> for RepairRecordBiz {
    async fn create(&self, item: RepairRecord) -> Result<RepairRecord, Error> {
        let mut updates = vec![];

        if let Some(device_group_group_id) = item.device_group_group_id {
            updates.push(("device_group_group_id", device_group_group_id.to_string()));
        }

        if let Some(device_info_id) = item.device_info_id {
            updates.push(("device_info_id", device_info_id.to_string()));
        }

        if let Some(repair_date) = item.repair_date {
            updates.push(("repair_date", repair_date.to_string()));
        }

        if let Some(technician) = item.technician {
            updates.push(("technician", technician));
        }

        if let Some(cost) = item.cost {
            updates.push(("cost", cost.to_string()));
        }

        if let Some(description) = item.description {
            updates.push(("description", description));
        }

        log::info!("Creating repair record with updates: {:?}", updates);

        let result =
            common_lib::sql_utils::insert::<RepairRecord>(&self.mysql, "repair_records", updates)
                .await;

        result
    }

    async fn update(&self, id: i64, item: RepairRecord) -> Result<RepairRecord, Error> {
        let mut updates = vec![];

        if let Some(device_group_group_id) = item.device_group_group_id {
            updates.push(("device_group_group_id", device_group_group_id.to_string()));
        }

        if let Some(device_info_id) = item.device_info_id {
            updates.push(("device_info_id", device_info_id.to_string()));
        }

        if let Some(repair_date) = item.repair_date {
            updates.push(("repair_date", repair_date.to_string()));
        }

        if let Some(technician) = item.technician {
            updates.push(("technician", technician));
        }

        if let Some(cost) = item.cost {
            updates.push(("cost", cost.to_string()));
        }

        if let Some(description) = item.description {
            updates.push(("description", description));
        }

        log::info!("Updating repair record with ID {}: {:?}", id, updates);

        let result = common_lib::sql_utils::update_by_id::<RepairRecord>(
            &self.mysql,
            "repair_records",
            id,
            updates,
        )
            .await;

        return match result {
            Ok(it) => Ok(it),
            Err(err) => Err(err),
        };
    }

    async fn delete(&self, id: i64) -> Result<RepairRecord, Error> {
        log::info!("Deleting repair record with ID {}", id);

        common_lib::sql_utils::delete_by_id(&self.mysql, "repair_records", id).await
    }

    async fn page(
        &self,
        filters: Vec<FilterInfo>,
        pagination: PaginationParams,
    ) -> Result<PaginationResult<RepairRecord>, Error> {
        log::info!(
            "Fetching page of repair records with filters: {:?} and pagination: {:?}",
            filters,
            pagination
        );

        let result = common_lib::sql_utils::paginate::<RepairRecord>(
            &self.mysql,
            "repair_records",
            filters,
            pagination,
        )
            .await;

        result
    }

    async fn list(&self, filters: Vec<FilterInfo>) -> Result<Vec<RepairRecord>, Error> {
        log::info!(
            "Fetching list of repair records with filters: {:?}",
            filters
        );
        let result =
            common_lib::sql_utils::list::<RepairRecord>(&self.mysql, "repair_records", filters)
                .await;
        return result;
    }

    async fn by_id(&self, id: i64) -> Result<RepairRecord, Error> {
        let result =
            common_lib::sql_utils::by_id_common::<RepairRecord>(&self.mysql, "repair_records", id)
                .await;
        result
    }
}
