use crate::config::RedisConfig;
use redis::{AsyncCommands, Client, RedisError};
use tokio::sync::{Mutex, MutexGuard, OnceCell};

#[derive(Clone, Debug)]
pub struct RedisWrapper {
    client: Client,
}

impl RedisWrapper {
    pub fn new(config: RedisConfig) -> Result<Self, RedisError> {
        let url = format!(
            "redis://:{}@{}:{}/{}",
            config.password, config.host, config.port, config.db
        );
        let client = Client::open(url)?;
        Ok(Self { client })
    }
    async fn get_connection(&self) -> Result<redis::aio::MultiplexedConnection, RedisError> {
        let con = self.client.get_multiplexed_async_connection().await?;
        Ok(con)
    }

    // String 操作
    pub async fn set_string(&self, key: &str, value: &str) -> Result<(), RedisError> {
        let mut con = self.get_connection().await?;
        con.set::<&str, &str, ()>(key, value).await?;
        Ok(())
    }

    pub async fn set_string_with_expiry(
        &self,
        key: &str,
        value: &str,
        expiry_seconds: i64,
    ) -> Result<(), RedisError> {
        let mut con = self.get_connection().await?;
        // 使用 set 方法和过期时间选项
        let _: () = con.set(key, value).await?;
        let _: () = con.expire(key, expiry_seconds).await?; // 设置过期时间
        Ok(())
    }
    pub async fn get_string(&self, key: &str) -> Result<Option<String>, RedisError> {
        let mut con = self.get_connection().await?;
        let value: Option<String> = con.get(key).await?;
        Ok(value)
    }

    pub async fn delete_string(&self, key: &str) -> Result<(), RedisError> {
        let mut con = self.get_connection().await?;
        con.del::<&str, ()>(key).await?; // 显式指定类型
        Ok(())
    }

    // List 操作
    pub async fn push_list(&self, key: &str, value: &str) -> Result<(), RedisError> {
        let mut con = self.get_connection().await?;
        con.rpush::<&str, &str, ()>(key, value).await?; // 显式指定类型
        Ok(())
    }

    pub async fn pop_list(&self, key: &str) -> Result<Option<String>, RedisError> {
        let mut con = self.get_connection().await?;
        // 传递 `None` 表示只弹出一个元素
        let value: Option<String> = con.lpop(key, None).await?;
        Ok(value)
    }

    pub async fn get_list_all(&self, key: &str) -> Result<Vec<String>, RedisError> {
        let mut con = self.get_connection().await?;
        let members: Vec<String> = con.lrange(key, 0, -1).await?;
        Ok(members)
    }

    pub async fn delete_list(&self, key: &str) -> Result<(), RedisError> {
        let mut con = self.get_connection().await?;
        con.del::<&str, ()>(key).await?; // 显式指定类型
        Ok(())
    }

    // Zset 操作
    pub async fn add_zset(&self, key: &str, member: &str, score: f64) -> Result<(), RedisError> {
        let mut con = self.get_connection().await?;
        con.zadd::<&str, f64, &str, ()>(key, member, score).await?; // 显式指定类型
        Ok(())
    }

    pub async fn get_zset(&self, key: &str) -> Result<Vec<(String, f64)>, RedisError> {
        let mut con = self.get_connection().await?;
        let members: Vec<(String, f64)> = con.zrange_withscores(key, 0, -1).await?;
        Ok(members)
    }
    pub async fn get_zset_length(&self, key: &str) -> Result<u64, RedisError> {
        let mut con = self.get_connection().await?;
        let length: u64 = con.zcard(key).await?;
        Ok(length)
    }
    pub async fn delete_zset(&self, key: &str, member: &str) -> Result<(), RedisError> {
        let mut con = self.get_connection().await?;
        con.zrem::<&str, &str, ()>(key, member).await?; // 显式指定类型
        Ok(())
    }
    pub async fn delete_first_zset_member(&self, key: &str) -> Result<(), RedisError> {
        let mut con = self.get_connection().await?;

        // 获取 zset 中第一个元素
        if let Some((first_member, _score)) = con
            .zrange_withscores::<&str, Vec<(String, f64)>>(key, 0, 0)
            .await?
            .into_iter()
            .next()
        {
            // 删除第一个元素
            con.zrem(key, first_member).await?;
        }

        Ok(())
    }

    // Hash 操作
    pub async fn set_hash(&self, key: &str, field: &str, value: &str) -> Result<(), RedisError> {
        let mut con = self.get_connection().await?;
        con.hset::<&str, &str, &str, ()>(key, field, value).await?; // 显式指定类型
        Ok(())
    }

    pub async fn get_hash(&self, key: &str, field: &str) -> Result<Option<String>, RedisError> {
        let mut con = self.get_connection().await?;
        let value: Option<String> = con.hget(key, field).await?;
        Ok(value)
    }
    pub async fn get_hash_length(&self, key: &str) -> Result<Option<usize>, RedisError> {
        let mut con = self.get_connection().await?;
        // 获取哈希表中字段的长度
        let length = con.hlen(key).await?;

        Ok(Some(length))
    }

    pub async fn delete_hash_field(&self, key: &str, field: &str) -> Result<(), RedisError> {
        let mut con = self.get_connection().await?;
        con.hdel::<&str, &str, ()>(key, field).await?; // 显式指定类型
        Ok(())
    }

    pub async fn delete_hash(&self, key: &str) -> Result<(), RedisError> {
        let mut con = self.get_connection().await?;
        con.del::<&str, ()>(key).await?; // 显式指定类型
        Ok(())
    }
}

static REDIS_INSTANCE: OnceCell<Mutex<RedisWrapper>> = OnceCell::const_new();

pub async fn init_redis(config: RedisConfig) -> Result<(), Box<dyn std::error::Error>> {
    let redis = RedisWrapper::new(config)?;
    REDIS_INSTANCE
        .set(Mutex::new(redis))
        .map_err(|_| "Redis instance already initialized")?;
    Ok(())
}

pub async fn get_redis_instance(
) -> Result<MutexGuard<'static, RedisWrapper>, Box<dyn std::error::Error>> {
    let instance = REDIS_INSTANCE
        .get()
        .ok_or("Redis instance not initialized")?;
    let guard = instance.lock().await;
    Ok(guard)
}

#[cfg(test)]
mod tests {
    use super::*;
    use lazy_static::lazy_static;
    use std::sync::Mutex;
    use tokio;

    lazy_static! {
        static ref REDIS: Mutex<RedisWrapper> = Mutex::new({
            let config = RedisConfig {
                host: "127.0.0.1".to_string(),
                port: 6379,
                db: 10,
                password: "eYVX7EwVmmxKPCDmwMtyKVge8oLd2t81".to_string(),
            };
            RedisWrapper::new(config).unwrap()
        });
    }

    #[tokio::test]
    async fn test_set_and_get_string() {
        // let redis = REDIS.lock().unwrap();
        let config = RedisConfig {
            host: "127.0.0.1".to_string(),
            port: 6379,
            db: 2,
            password: "eYVX7EwVmmxKPCDmwMtyKVge8oLd2t81".to_string(),
        };

        init_redis(config).await.unwrap();
        let redis = get_redis_instance().await.unwrap();

        redis
            .set_string("my_key", "Hello, Red11111is!")
            .await
            .unwrap();
        // assert_eq!(redis.get_string("my_key").await.unwrap(), Some("Hello, Redis!".to_string()));
        // redis.delete_string("my_key").await.unwrap();
        // assert_eq!(redis.get_string("my_key").await.unwrap(), None);
    }

    #[tokio::test]
    async fn test_list_operations() {
        let redis = REDIS.lock().unwrap();

        redis.push_list("my_list", "item1").await.unwrap();
        redis.push_list("my_list", "item2").await.unwrap();
        assert_eq!(
            redis.pop_list("my_list").await.unwrap(),
            Some("item1".to_string())
        );
        redis.delete_list("my_list").await.unwrap();
        assert_eq!(redis.pop_list("my_list").await.unwrap(), None);
    }

    #[tokio::test]
    async fn test_zset_operations() {
        let redis = REDIS.lock().unwrap();

        redis.add_zset("my_zset", "member1", 1.0).await.unwrap();
        redis.add_zset("my_zset", "member2", 2.0).await.unwrap();
        let zset_members = redis.get_zset("my_zset").await.unwrap();
        assert_eq!(zset_members.len(), 2);
        redis.delete_first_zset_member("zset").await.unwrap();
        let zset_members = redis.get_zset("my_zset").await.unwrap();
        // assert_eq!(zset_members.len(), 1);
    }

    #[tokio::test]
    async fn test_hash_operations() {
        let redis = REDIS.lock().unwrap();

        redis.set_hash("my_hash", "field1", "value1").await.unwrap();
        assert_eq!(
            redis.get_hash("my_hash", "field1").await.unwrap(),
            Some("value1".to_string())
        );
        redis.delete_hash_field("my_hash", "field1").await.unwrap();
        assert_eq!(redis.get_hash("my_hash", "field1").await.unwrap(), None);
        redis.delete_hash("my_hash").await.unwrap();
    }
}
