use common_lib::config::{get_config, read_config, read_config_tb};
use common_lib::rabbit_utils::init_rabbitmq_with_config;
use common_lib::redis_handler::init_redis;
use common_lib::redis_pool_utils::{create_redis_pool_from_config, RedisOp};
use rocket::{launch, routes};
use tokio::runtime::Runtime;

mod controller;
mod db;

#[launch]
fn rocket() -> _ {
    common_lib::init_logger(); // 初始化日志记录

    // 创建异步运行时
    let rt = Runtime::new().unwrap();
    let config1 = read_config_tb("app-local.yml");
    let pool = create_redis_pool_from_config(&config1.redis_config);

    let redis_op = RedisOp { pool };

    // 构建并启动 Rocket 应用
    rocket::build()
        .manage(redis_op)
        .configure(rocket::Config {
            port: config1.node_info.port,
            ..Default::default()
        })
        .mount("/", routes![crate::controller::demo_api::index]) // 挂载路由
}
