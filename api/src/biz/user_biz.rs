use crate::db::db_model::User;
use common_lib::redis_pool_utils::RedisOp;
use common_lib::sql_utils::by_id;
use r2d2;
use sqlx::{Error, MySqlPool};

pub struct UserBiz {
    pub redis: RedisOp,
    pub mysql: MySqlPool,
}

impl UserBiz {
    pub fn new(redis: RedisOp, mysql: MySqlPool) -> Self {
        UserBiz { redis, mysql }
    }

    pub async fn get_user_by_id(&self, user_id: u64) -> Result<User, Error> {
        let user = self.query_mysql_for_user(user_id).await;
        user
    }

    pub async fn check_user_cache(&self, user_id: u64) -> Result<Option<String>, String> {
        Ok(Some("aaa".to_string()))
    }

    async fn query_mysql_for_user(&self, user_id: u64) -> Result<User, Error> {
        let result = by_id::<User>(&self.mysql, "users", user_id.to_string()).await;

        result
    }
}
