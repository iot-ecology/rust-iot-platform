use rumqttc::{Client, LastWill, MqttOptions, QoS};
use std::thread;
use std::time::Duration;

/*
 * 这是程序的主函数。
 * 在该函数中，将初始化 MQTT 客户端、设置连接选项和遗嘱消息。
 * 然后，创建客户端和连接、并在新线程中调用发布函数。
 * 最后，使用 connection.iter() 方法遍历并处理接中的每个通知。
 */
fn main() {
    // 初始化日志记录器

    // 设置 MQTT 连接选项和遗嘱消息
    let mut mqttoptions = MqttOptions::new("test-1", "localhost", 1883);
    mqttoptions.set_credentials("admin", "admin");

    // 创建 MQTT 客户端和连接，并启动新线程进行消息发布
    let (client, mut connection) = Client::new(mqttoptions, 10);

    let x = client.subscribe("#", QoS::AtLeastOnce).unwrap();

    // 遍历并处理连接中的每个通知
    for (i, notification) in connection.iter().enumerate() {
        match notification {
            Ok(notif) => {
                println!("{i}. Notification = {notif:?}");
            }
            Err(error) => {
                println!("{i}. Notification = {error:?}");
                return;
            }
        }
    }
    thread::sleep(Duration::from_secs(30));
    println!("Done with the stream!!");
}
