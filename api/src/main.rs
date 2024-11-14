use crate::biz::user_biz::UserBiz;
use crate::controller::user_router::user_index;
use common_lib::config::{read_config_tb, MySQLConfig, RedisConfig};
use common_lib::mysql_utils::gen_mysql_url;
use common_lib::rabbit_utils::RabbitMQFairing;
use common_lib::redis_pool_utils::{create_redis_pool_from_config, RedisOp};
use rocket::fairing::{Info, Kind};
use rocket::{launch, routes, Build, Rocket};
use sqlx::MySqlPool;

mod biz;
mod controller;
mod db;

#[launch]
fn rocket() -> _ {
    common_lib::init_logger(); // 初始化日志记录

    let config1 = read_config_tb("app-local.yml");

    // 构建并启动 Rocket 应用
    rocket::build()
        .attach(RabbitMQFairing {
            config: config1.mq_config.clone(),
        })
        .attach(MySqlPoolFairing {
            config: config1.mysql_config.clone().unwrap(),
            redis_config: config1.redis_config.clone(),
        })
        .manage(config1.clone())
        .configure(rocket::Config {
            port: config1.node_info.port,
            log_level: rocket::config::LogLevel::Off,
            ..Default::default()
        })
        .mount("/", routes![crate::controller::demo_api::index, user_index])
}
pub struct MySqlPoolFairing {
    pub config: MySQLConfig,
    pub redis_config: RedisConfig,
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

        let redis_pool = create_redis_pool_from_config(&self.redis_config);

        let redis_op = RedisOp { pool: redis_pool };

        let user_biz = UserBiz::new(redis_op.clone(), pool.clone());

        Ok(rocket.manage(pool).manage(redis_op).manage(user_biz))
    }
}
