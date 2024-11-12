use crate::config::MySQLConfig;
use log::{error, info};
use r2d2_mysql::{
    mysql::{prelude::*, Opts, OptsBuilder},
    r2d2, MySqlConnectionManager,
};
use std::{sync::Arc, thread};
use urlencoding::encode;
#[derive(Debug, Clone)]
pub struct MysqlOp {
    pub pool: r2d2::Pool<MySqlConnectionManager>,
}

impl MysqlOp {
    pub fn new(config: MySQLConfig) -> Self {
        let pool = create_db_pool(&config);
        Self { pool }
    }
}

fn create_db_pool(config: &MySQLConfig) -> r2d2::Pool<MySqlConnectionManager> {
    let encoded_username = encode(&config.username);
    let encoded_password = encode(&config.password);

    // 构建数据库连接字符串
    let url = format!(
        "mysql://{}:{}@{}:{}/{}",
        encoded_username,
        encoded_password,
        config.host.as_deref().unwrap_or("localhost"),
        config.port,
        config.dbname
    );

    let opts = Opts::from_url(&url).expect("Invalid database URL");
    let builder = OptsBuilder::from_opts(opts);
    let manager = MySqlConnectionManager::new(builder);

    info!("Creating MySQL connection pool...");
    let pool = r2d2::Pool::builder()
        .max_size(4)
        .build(manager)
        .expect("Failed to create pool");
    info!("Connection pool created successfully.");

    pool
}

pub fn run_db_task(pool: r2d2::Pool<MySqlConnectionManager>) {
    match pool.get() {
        Ok(mut conn) => {
            info!("Successfully got a connection from the pool.");
            match conn.query::<String, &str>("SELECT version()") {
                Ok(rows) => {
                    info!("Query executed successfully. Rows: {:?}", rows);
                }
                Err(err) => {
                    error!("Error executing query: {:?}", err);
                }
            }
        }
        Err(err) => {
            error!("Error getting connection from pool: {:?}", err);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::mpsc;

    #[test]
    fn test_db_pool_connection() {
        env_logger::Builder::from_default_env()
            .filter_level(log::LevelFilter::Info)
            .init();

        let pool = create_db_pool(&MySQLConfig {
            username: "root".to_string(),
            password: "root123@".to_string(),
            host: Some("localhost".to_string()),
            port: 3306,
            dbname: "iot".to_string(),
        });

        let (tx, rx) = mpsc::channel();
        let mut tasks = vec![];

        for _ in 0..3 {
            let pool = pool.clone();
            let tx = tx.clone();
            let th = thread::spawn(move || {
                run_db_task(pool);
                tx.send(()).expect("Failed to send completion signal");
            });
            tasks.push(th);
        }

        for _ in 0..3 {
            rx.recv().expect("Failed to receive completion signal");
        }

        for th in tasks {
            th.join().expect("Thread panicked");
        }
    }
}
