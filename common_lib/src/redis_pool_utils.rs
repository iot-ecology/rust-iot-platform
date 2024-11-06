use crate::config::RedisConfig;
use r2d2::Pool;

use r2d2_redis::{r2d2, redis, RedisConnectionManager};
pub fn create_redis_pool_from_config(config: &RedisConfig) -> r2d2::Pool<RedisConnectionManager> {
    let url = format!(
        "redis://:{}@{}:{}/{}",
        config.password, config.host, config.port, config.db
    );
    let manager = RedisConnectionManager::new(url).unwrap();
    r2d2::Pool::builder().build(manager).unwrap()
}

#[cfg(test)]
mod tests {
    use crate::config::RedisConfig;
    use crate::redis_pool_utils::create_redis_pool_from_config;
    use r2d2_redis::redis::Commands;
    use r2d2_redis::RedisConnectionManager;
    use std::ops::Deref;
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

                let n: i64 = conn.incr("counter", 1).unwrap();
                println!("Counter increased to {}", n);
            }));
        }

        for h in handles {
            h.join().unwrap();
        }
    }
}
