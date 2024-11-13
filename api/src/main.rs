use common_lib::config::{get_config, read_config, read_config_tb, MqConfig, MySQLConfig};
use common_lib::mysql_utils::{gen_mysql_url, MysqlOp};
use common_lib::rabbit_utils::{init_rabbitmq_with_config, RabbitMQFairing};
use common_lib::redis_handler::init_redis;
use common_lib::redis_pool_utils::{create_redis_pool_from_config, RedisOp};
use rocket::fairing::{Info, Kind};
use rocket::{launch, routes, Build, Rocket};
use sqlx::MySqlPool;
use tokio::runtime::Runtime;
mod controller;
mod db;

#[launch]
fn rocket() -> _ {
    common_lib::init_logger(); // 初始化日志记录

    let config1 = read_config_tb("app-local.yml");
    let pool = create_redis_pool_from_config(&config1.redis_config);

    let redis_op = RedisOp { pool };

    // 构建并启动 Rocket 应用
    rocket::build()
        .attach(RabbitMQFairing {
            config: config1.mq_config.clone(),
        })
        .attach(MySqlPoolFairing {
            config: config1.mysql_config.clone().unwrap(),
        })
        .manage(redis_op)
        .manage(config1.clone())
        .configure(rocket::Config {
            port: config1.node_info.port,
            log_level: rocket::config::LogLevel::Off,
            ..Default::default()
        })
        .mount("/", routes![crate::controller::demo_api::index]) // 挂载路由
}
pub struct MySqlPoolFairing {
    pub config: MySQLConfig,
}
#[rocket::async_trait]
impl rocket::fairing::Fairing for MySqlPoolFairing {
    fn info(&self) -> Info {
        Info {
            name: "Mysql Initializer",
            kind: Kind::Ignite,
        }
    }
    async fn on_ignite(&self, rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
        let string = gen_mysql_url(&self.config);

        let pool = MySqlPool::connect(&string).await.unwrap();
        Ok(rocket.manage(pool))
    }
}
