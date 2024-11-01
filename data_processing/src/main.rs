use common_lib::init_logger;
use common_lib::protocol_config::read_config;
use common_lib::rabbit_utils::{get_rabbitmq_instance, init_rabbitmq_with_config};
use common_lib::redis_handler::init_redis;
use tokio::runtime::Runtime;

mod js_test;

#[tokio::main]
async fn main() {
    init_logger();
    let result = read_config("app-local.yml").unwrap();
    init_redis(result.redis_config).await.unwrap();
    init_rabbitmq_with_config(result.mq_config).await.unwrap();
    let rabbit = get_rabbitmq_instance().await.unwrap();
    let should_ack = |data: &[u8]| -> bool {
        let message = String::from_utf8_lossy(data);
        println!("Received message: {}", message);
        message.contains("ack");
        return true;
    };

    rabbit.consume("queue1", should_ack).await.unwrap();

    // 保证 main 不会提前退出
    tokio::signal::ctrl_c()
        .await
        .expect("Failed to listen for ctrl-c signal");
}
