use crate::config::RedisConfig;
use log::{error, info};
use r2d2::PooledConnection;

use r2d2_redis::redis::{Commands, RedisResult};
use r2d2_redis::{r2d2, redis, RedisConnectionManager};
use redis::RedisError;

pub fn create_redis_pool_from_config(config: &RedisConfig) -> r2d2::Pool<RedisConnectionManager> {
    let url = format!(
        "redis://:{}@{}:{}/{}",
        config.password, config.host, config.port, config.db
    );
    let manager = RedisConnectionManager::new(url).unwrap();
    r2d2::Pool::builder().build(manager).unwrap()
}

#[derive(Debug, Clone)]
pub struct RedisOp {
    pub pool: r2d2::Pool<RedisConnectionManager>,
}
impl RedisOp {
    pub fn delete(&self, key: &str) -> Result<(), RedisError> {
        let mut con = self.get_connection();
        con.del::<&str, ()>(key)?;
        Ok(())
    }

    pub fn get_connection(&self) -> PooledConnection<RedisConnectionManager> {
        let result = self.pool.get();
        let connection = result.unwrap();

        return connection;
    }

    pub fn acquire_lock(
        &self,
        lock_key: &str,
        lock_value: &str,
        ttl: u64,
    ) -> Result<bool, redis::RedisError> {
        let mut con = self.get_connection();

        let result1 = con.set_nx(lock_key, lock_value);
        let result: RedisResult<i32> = result1;
        match result {
            Ok(yes) => {
                con.expire::<&str, usize>(lock_key, ttl as usize)
                    .expect("设置过期时间异常");
                return Ok(true);
            }
            Err(e) => {
                error!("分布式锁上锁失败{}", e);

                return Ok(false);
            }
        }
    }

    pub fn release_lock(
        &self,
        lock_key: &str,
        lock_value: &str,
    ) -> Result<bool, redis::RedisError> {
        if let Some(result1) = self.get_string(lock_key)? {
            let x = result1.as_str();
            info!("x = {}   lock_value = {}", x, lock_value);

            if x == lock_value {
                self.delete_string(lock_key)?;
                return Ok(true);
            }
        }
        Ok(false)
    }

    // String 操作
    pub fn set_string(&self, key: &str, value: &str) -> Result<(), RedisError> {
        let mut con = self.get_connection();
        let result = con.set::<&str, &str, ()>(key, value);
        result
    }

    pub fn set_string_with_expiry(
        &self,
        key: &str,
        value: &str,
        expiry_seconds: i64,
    ) -> Result<(), RedisError> {
        let mut con = self.get_connection();
        // 使用 set 方法和过期时间选项
        con.set::<&str, &str, ()>(key, value).expect("设置数据异常");
        con.expire::<&str, usize>(key, expiry_seconds as usize)
            .expect("设置过期时间异常");
        Ok(())
    }
    pub fn get_string(&self, key: &str) -> Result<Option<String>, RedisError> {
        let mut con = self.get_connection();
        match con.get(key) {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    pub fn delete_string(&self, key: &str) -> Result<(), RedisError> {
        let mut con = self.get_connection();
        match con.del::<&str, ()>(key) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    // List 操作
    pub fn push_list(&self, key: &str, value: &str) -> Result<(), RedisError> {
        let mut con = self.get_connection();
        match con.rpush::<&str, &str, ()>(key, value) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// 从列表中弹出值
    pub fn pop_list(&self, key: &str) -> Result<Option<String>, RedisError> {
        let mut con = self.get_connection();
        match con.lpop(key) {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }

    /// 获取整个列表
    pub fn get_list_all(&self, key: &str) -> Result<Vec<String>, RedisError> {
        let mut con = self.get_connection();
        match con.lrange(key, 0, -1) {
            Ok(members) => Ok(members),
            Err(e) => Err(e),
        }
    }

    /// 删除列表
    pub fn delete_list(&self, key: &str) -> Result<(), RedisError> {
        let mut con = self.get_connection();
        match con.del::<&str, ()>(key) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// 添加 Zset 成员
    pub fn add_zset(&self, key: &str, member: &str, score: f64) -> Result<(), RedisError> {
        let mut con = self.get_connection();
        match con.zadd::<&str, f64, &str, ()>(key, member, score) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// 获取 Zset 成员及其分数
    pub fn get_zset(&self, key: &str) -> Result<Vec<(String, f64)>, RedisError> {
        let mut con = self.get_connection();
        match con.zrange_withscores(key, 0, -1) {
            Ok(members) => Ok(members),
            Err(e) => Err(e),
        }
    }

    /// 获取 Zset 长度
    pub fn get_zset_length(&self, key: &str) -> Result<u64, RedisError> {
        let mut con = self.get_connection();
        match con.zcard(key) {
            Ok(length) => Ok(length),
            Err(e) => Err(e),
        }
    }

    /// 删除 Zset 中的指定成员
    pub fn delete_zset(&self, key: &str, member: &str) -> Result<(), RedisError> {
        let mut con = self.get_connection();
        match con.zrem::<&str, &str, ()>(key, member) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// 删除 Zset 中的第一个成员
    pub fn delete_first_zset_member(&self, key: &str) -> Result<(), RedisError> {
        let mut con = self.get_connection();
        if let Some((first_member, _score)) =
            match con.zrange_withscores::<&str, Vec<(String, f64)>>(key, 0, 0) {
                Ok(result) => result.into_iter().next(),
                Err(e) => return Err(e),
            }
        {
            match con.zrem::<&str, String, ()>(key, first_member) {
                Ok(_) => Ok(()),
                Err(e) => Err(e),
            }
        } else {
            Ok(())
        }
    }

    /// 设置哈希字段
    pub fn set_hash(&self, key: &str, field: &str, value: &str) -> Result<(), RedisError> {
        let mut con = self.get_connection();
        match con.hset::<&str, &str, &str, ()>(key, field, value) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// 获取哈希字段的值
    pub fn get_hash(&self, key: &str, field: &str) -> Result<Option<String>, RedisError> {
        let mut con = self.get_connection();
        match con.hget(key, field) {
            Ok(value) => Ok(value),
            Err(e) => Err(e),
        }
    }
    pub fn get_hash_all_value(&self, key: &str) -> Result<Vec<String>, RedisError> {
        let mut con = self.get_connection();

        con.hvals(key)
    }

    pub fn hash_exists(&self, key: &str, field: &str) -> Result<bool, RedisError> {
        let mut con = self.get_connection();
        con.hexists(key, field)
    }
    /// 获取哈希字段的长度
    pub fn get_hash_length(&self, key: &str) -> Result<Option<usize>, RedisError> {
        let mut con = self.get_connection();
        match con.hlen(key) {
            Ok(length) => Ok(Some(length)),
            Err(e) => Err(e),
        }
    }

    /// 删除哈希中的字段
    pub fn delete_hash_field(&self, key: &str, field: &str) -> Result<(), RedisError> {
        let mut con = self.get_connection();
        match con.hdel::<&str, &str, ()>(key, field) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    /// 删除整个哈希
    pub fn delete_hash(&self, key: &str) -> Result<(), RedisError> {
        let mut con = self.get_connection();
        match con.del::<&str, ()>(key) {
            Ok(_) => Ok(()),
            Err(e) => Err(e),
        }
    }

    pub fn add_set(&self, key: &str, value: &str) -> Result<(), RedisError> {
        let mut con = self.get_connection();
        con.sadd::<&str, &str, ()>(key, value)
    }

    pub fn get_set(&self, key: &str) -> Result<Vec<String>, RedisError> {
        let mut con = self.get_connection();
        con.smembers(key)
    }

    pub fn delete_set_member(&self, key: &str, v: &str) -> Result<(), RedisError> {
        let mut con = self.get_connection();
        con.srem(key, v)
    }
}

#[cfg(test)]
mod tests {
    use crate::config::RedisConfig;
    use crate::init_logger;
    use crate::redis_pool_utils::{create_redis_pool_from_config, RedisOp};
    use log::info;
    use r2d2_redis::redis::Commands;
    use std::env;
    use std::thread;

    #[test]
    fn test_pool() {
        let config = RedisConfig {
            host: "127.0.0.1".to_string(),
            port: 6379,
            db: 10,
            password: "eYVX7EwVmmxKPCDmwMtyKVge8oLd2t81".to_string(),
        };

        let pool = create_redis_pool_from_config(&config);

        let mut handles = vec![];

        for _i in 0..10i32 {
            let pool = pool.clone();
            handles.push(thread::spawn(move || {
                let mut conn = pool.get().unwrap();
                conn.rpush::<&str, &str, ()>("1", "value").unwrap();
                conn.set::<&str, &str, ()>("1", "value").unwrap();
                let n: i64 = conn.incr("counter", 1).unwrap();
                println!("Counter increased to {}", n);
            }));
        }

        for h in handles {
            h.join().unwrap();
        }
    }

    // 用于创建 RedisOp 实例
    fn setup_redis_op() -> RedisOp {
        let config = RedisConfig {
            host: "127.0.0.1".to_string(),
            port: 6379,
            db: 10,
            password: "eYVX7EwVmmxKPCDmwMtyKVge8oLd2t81".to_string(),
        };

        let pool = create_redis_pool_from_config(&config);

        RedisOp { pool }
    }

    // 测试 set_string 方法
    #[test]
    fn test_set_string() {
        let redis_op = setup_redis_op();
        let result = redis_op.set_string("test_key", "test_value");

        assert!(result.is_ok());
    }

    // 测试 get_string 方法
    #[test]
    fn test_get_string() {
        let redis_op = setup_redis_op();

        // 先设置一个字符串
        redis_op.set_string("test_key", "test_value").unwrap();

        // 再读取该字符串
        let result = redis_op.get_string("test_key");

        assert_eq!(result.unwrap(), Some("test_value".to_string()));
    }

    // 测试 set_string_with_expiry 方法
    #[test]
    fn test_set_string_with_expiry() {
        let redis_op = setup_redis_op();
        let result = redis_op.set_string_with_expiry("test_key", "test_value", 1000);

        assert!(result.is_ok());
    }

    // 测试 delete_string 方法
    #[test]
    fn test_delete_string() {
        let redis_op = setup_redis_op();

        // 先设置一个字符串
        redis_op.set_string("test_key", "test_value").unwrap();

        // 再删除该字符串
        let result = redis_op.delete_string("test_key");

        assert!(result.is_ok());

        // 确认删除后无法获取该字符串
        let get_result = redis_op.get_string("test_key");
        assert_eq!(get_result.unwrap(), None);
    }

    // 测试 push_list 方法
    #[test]
    fn test_push_list() {
        let redis_op = setup_redis_op();
        let result = redis_op.push_list("test_list", "value_1");

        assert!(result.is_ok());
    }

    // 测试 get_list_all 方法
    #[test]
    fn test_get_list_all() {
        let redis_op = setup_redis_op();
        redis_op.push_list("test_list", "value_1").unwrap();
        redis_op.push_list("test_list", "value_2").unwrap();

        let result = redis_op.get_list_all("test_list");

        assert_eq!(
            result.unwrap(),
            vec!["value_1".to_string(), "value_2".to_string()]
        );
    }

    // 测试 pop_list 方法
    #[test]
    fn test_pop_list() {
        let redis_op = setup_redis_op();
        redis_op.push_list("test_list", "value_1").unwrap();

        let result = redis_op.pop_list("test_list");

        assert_eq!(result.unwrap(), Some("value_1".to_string()));
    }

    // 测试 add_zset 方法
    #[test]
    fn test_add_zset() {
        let redis_op = setup_redis_op();
        let result = redis_op.add_zset("test_zset", "member_1", 1.0);

        assert!(result.is_ok());
    }

    // 测试 get_zset 方法
    #[test]
    fn test_get_zset() {
        let redis_op = setup_redis_op();
        redis_op.add_zset("test_zset", "member_1", 1.0).unwrap();

        let result = redis_op.get_zset("test_zset");

        assert_eq!(result.unwrap(), vec![("member_1".to_string(), 1.0)]);
    }

    // 测试 get_zset_length 方法
    #[test]
    fn test_get_zset_length() {
        let redis_op = setup_redis_op();
        redis_op.add_zset("test_zset", "member_1", 1.0).unwrap();

        let length = redis_op.get_zset_length("test_zset");

        assert_eq!(length.unwrap(), 1);
    }

    // 测试 delete_zset 方法
    #[test]
    fn test_delete_zset() {
        let redis_op = setup_redis_op();
        redis_op.add_zset("test_zset", "member_1", 1.0).unwrap();

        let result = redis_op.delete_zset("test_zset", "member_1");

        assert!(result.is_ok());

        let length = redis_op.get_zset_length("test_zset");

        assert_eq!(length.unwrap(), 0);
    }

    // 测试 delete_first_zset_member 方法
    #[test]
    fn test_delete_first_zset_member() {
        let redis_op = setup_redis_op();
        redis_op.add_zset("test_zset", "member_1", 1.0).unwrap();
        redis_op.add_zset("test_zset", "member_2", 2.0).unwrap();

        let result = redis_op.delete_first_zset_member("test_zset");

        assert!(result.is_ok());
    }

    // 测试 set_hash 方法
    #[test]
    fn test_set_hash() {
        let redis_op = setup_redis_op();
        let result = redis_op.set_hash("test_hash", "field_1", "value_1");

        assert!(result.is_ok());
    }

    // 测试 get_hash 方法
    #[test]
    fn test_get_hash() {
        let redis_op = setup_redis_op();
        redis_op
            .set_hash("test_hash", "field_1", "value_1")
            .unwrap();

        let result = redis_op.get_hash("test_hash", "field_1");

        assert_eq!(result.unwrap(), Some("value_1".to_string()));
    }

    // 测试 delete_hash_field 方法
    #[test]
    fn test_delete_hash_field() {
        let redis_op = setup_redis_op();
        redis_op
            .set_hash("test_hash", "field_1", "value_1")
            .unwrap();
        redis_op
            .set_hash("test_hash", "field_2", "value_1")
            .unwrap();

        let result = redis_op.delete_hash_field("test_hash", "field_1");

        assert!(result.is_ok());

        let get_result = redis_op.get_hash("test_hash", "field_1");
        assert_eq!(get_result.unwrap(), None);
    }

    // 测试 delete_hash 方法
    #[test]
    fn test_delete_hash() {
        let redis_op = setup_redis_op();
        redis_op
            .set_hash("test_hash", "field_1", "value_1")
            .unwrap();

        let result = redis_op.delete_hash("test_hash");

        assert!(result.is_ok());
    }

    #[test]
    fn test_acquire_lock() {
        env::set_var("RUST_LOG", "info");
        env_logger::init();
        let redis_op = setup_redis_op();
        let x = redis_op.acquire_lock("asfaf", "asf", 10000).unwrap();
        info!("{}", x);

        let result = redis_op.release_lock("asfaf", "asf");
        info!("{:?}", result.unwrap());
    }
    #[test]
    fn test_hash_exists() {
        env::set_var("RUST_LOG", "info");
        env_logger::init();
        let redis_op = setup_redis_op();
        let x = redis_op.hash_exists("asfaf", "asf").unwrap();
        info!("{}", x);
    }
    #[test]
    fn test_get_set() {
        env::set_var("RUST_LOG", "info");
        env_logger::init();
        let redis_op = setup_redis_op();
        let x = redis_op.get_set("get_set").unwrap();
        info!("{:?}", x);
    }
    #[test]
    fn test_remove() {
        env::set_var("RUST_LOG", "info");
        env_logger::init();
        let redis_op = setup_redis_op();
        redis_op.delete_set_member("get_set", "1").unwrap()
    }
    #[test]
    fn test_get_hash_all() {
        env::set_var("RUST_LOG", "info");
        env_logger::init();
        let redis_op = setup_redis_op();
        let result = redis_op.get_hash_all("mqtt_config:use");
        info!("{:?}", result.unwrap());
    }
}
