use crate::biz::calc_param_biz::CalcParamBiz;
use crate::biz::user_biz::UserBiz;
use crate::db::db_model::{CalcRule, Signal};
use anyhow::{Context, Error, Result};
use common_lib::redis_pool_utils::RedisOp;
use common_lib::sql_utils;
use common_lib::sql_utils::{CrudOperations, FilterInfo, PaginationParams, PaginationResult};
use sqlx::MySqlPool;

pub struct CalcRuleBiz {
    pub redis: RedisOp,
    pub mysql: MySqlPool,
}
impl CalcRuleBiz {
    pub fn new(redis: RedisOp, mysql: MySqlPool) -> Self {
        CalcRuleBiz { redis, mysql }
    }
}

#[async_trait::async_trait]
impl CrudOperations<CalcRule> for CalcRuleBiz {
    async fn create(&self, item: CalcRule) -> Result<CalcRule, Error> {
        let mut updates = vec![];

        if let Some(name) = item.name {
            updates.push(("name", name.to_string()));
        }

        if let Some(cron) = item.cron {
            updates.push(("cron", cron.to_string()));
        }

        if let Some(script) = item.script {
            updates.push(("script", script.to_string()));
        }

        if let Some(offset) = item.offset {
            updates.push(("offset", offset.to_string()));
        }

        if let Some(start) = item.start {
            updates.push(("start", start.to_string()));
        }

        if let Some(mock_value) = item.mock_value {
            updates.push(("mock_value", mock_value.to_string()));
        }

        log::info!("Creating CalcRule with updates: {:?}", updates);

        let result = sql_utils::insert::<CalcRule>(&self.mysql, "calc_rules", updates).await;

        result
    }

    async fn update(&self, id: u64, item: CalcRule) -> Result<CalcRule, Error> {
        let mut updates = vec![];

        if let Some(name) = item.name {
            updates.push(("name", name.to_string()));
        }

        if let Some(cron) = item.cron {
            updates.push(("cron", cron.to_string()));
        }

        if let Some(script) = item.script {
            updates.push(("script", script.to_string()));
        }

        if let Some(offset) = item.offset {
            updates.push(("offset", offset.to_string()));
        }

        if let Some(start) = item.start {
            updates.push(("start", start.to_string()));
        }

        if let Some(mock_value) = item.mock_value {
            updates.push(("mock_value", mock_value.to_string()));
        }

        log::info!("Updating CalcRule with ID {}: {:?}", id, updates);

        let result =
            sql_utils::update_by_id::<CalcRule>(&self.mysql, "calc_rules", id, updates).await;

        match result {
            Ok(it) => Ok(it),
            Err(err) => Err(err),
        }
    }

    async fn delete(&self, id: u64) -> Result<CalcRule, Error> {
        log::info!("Deleting CalcRule with ID {}", id);

        sql_utils::delete_by_id(&self.mysql, "calc_rules", id).await
    }

    async fn page(
        &self,
        filters: Vec<FilterInfo>,
        pagination: PaginationParams,
    ) -> Result<PaginationResult<CalcRule>, Error> {
        log::info!(
            "Fetching page of CalcRules with filters: {:?} and pagination: {:?}",
            filters,
            pagination
        );

        let result =
            sql_utils::paginate::<CalcRule>(&self.mysql, "calc_rules", filters, pagination).await;

        result
    }

    async fn list(&self, filters: Vec<FilterInfo>) -> Result<Vec<CalcRule>, Error> {
        log::info!("Fetching list of CalcRules with filters: {:?}", filters);

        let result = sql_utils::list::<CalcRule>(&self.mysql, "calc_rules", filters).await;
        result
    }

    async fn by_id(&self, id: u64) -> Result<CalcRule, Error> {
        let result = sql_utils::by_id_common::<CalcRule>(&self.mysql, "calc_rules", id).await;
        result
    }
}
