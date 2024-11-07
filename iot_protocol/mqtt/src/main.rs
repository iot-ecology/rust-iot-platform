mod ctr;
mod mqtt_sample;

use crate::ctr::create_mqtt_client_http;
use crate::ctr::GetNoUseMqttConfig;
use crate::ctr::GetUseMqttConfig;
use crate::ctr::HttpBeat;
use crate::ctr::NodeList;
use crate::ctr::NodeUsingStatus;
use crate::ctr::PubCreateMqttClientHttp;
use crate::ctr::PubRemoveMqttClient;
use crate::ctr::RemoveMqttClient;
use common_lib::config::{get_config, read_config, read_config_tb};
use common_lib::rabbit_utils::init_rabbitmq_with_config;
use common_lib::redis_handler::init_redis;
use common_lib::redis_pool_utils::{create_redis_pool_from_config, RedisOp};
use rocket::{launch, routes};
use tokio::runtime::Runtime;

#[launch]
fn rocket() -> _ {
    common_lib::init_logger(); // 初始化日志记录
    crate::mqtt_sample::init_mqtt_map();
    // 创建异步运行时
    let rt = Runtime::new().unwrap();
    let config1 = read_config_tb("app-local.yml");

    let pool = create_redis_pool_from_config(&config1.redis_config);

    let redis_op = RedisOp { pool };
    // 构建并启动 Rocket 应用
    rocket::build()
        .manage(redis_op)
        .manage(config1.clone())
        .configure(rocket::Config {
            port: config1.node_info.port,
            ..Default::default()
        })
        .mount(
            "/",
            routes![
                HttpBeat,
                create_mqtt_client_http,
                NodeList,
                NodeUsingStatus,
                GetUseMqttConfig,
                GetNoUseMqttConfig,
                RemoveMqttClient,
                PubCreateMqttClientHttp,
                PubRemoveMqttClient,
            ],
        )
}
