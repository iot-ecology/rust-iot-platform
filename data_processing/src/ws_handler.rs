use crate::storage_handler::storage_data_row;
use chrono::Utc;
use common_lib::config::{get_config, Config, InfluxConfig};
use common_lib::influxdb_utils::InfluxDBManager;
use common_lib::models::{CoapMessage, DataRowList, DataValue, MQTTMessage, Signal, SignalMapping};
use common_lib::rabbit_utils::RabbitMQ;
use common_lib::redis_handler::{get_redis_instance, RedisWrapper};
use futures_util::StreamExt;
use lapin::options::{BasicAckOptions, BasicConsumeOptions, BasicPublishOptions};
use lapin::types::FieldTable;
use lapin::{BasicProperties, Channel, Connection};
use log::{error, info};
use quick_js::{Context, ContextError};
use serde::{Deserialize, Serialize};
use serde_json::from_str;
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::{Mutex, MutexGuard};

async fn handler_data_storage_string(
    result: String,
    jsc: Context,
    config: InfluxConfig,
    redis: RedisWrapper,
    rabbit_conn: &Connection,
) -> Result<(), Box<dyn Error>> {
    info!("message : {:?}", result);

    let mqtt_message: CoapMessage = serde_json::from_str(&result)?;

    // 获取存储的脚本
    let option = redis
        .get_hash("struct:Websocket", mqtt_message.uid.as_str())
        .await?;

    if let Some(string) = option {
        // 在这里创建 JavaScript 上下文
        jsc.eval(&string)
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        // 调用 main 函数
        let value = jsc
            .call_function("main", [mqtt_message.message])
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        // 定义并调用 JavaScript 代码
        let js_code_2 = r#"
        function main2(data) {
            return JSON.stringify(data);
        }"#;
        jsc.eval(js_code_2)
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;
        let value2 = jsc
            .call_function("main2", [value])
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        let x = value2.as_str().unwrap_or("");

        info!("Java Script Result = {:?}", x);
        let dt: Vec<DataRowList> = from_str(&x).map_err(|e| Box::new(e) as Box<dyn Error>)?;
        info!("{:?}", dt);

        // 存储数据行
        for mut data_row in dt {
            data_row.Protocol = Some("WebSocket".to_string());
            storage_data_row(
                data_row,
                "WebSocket",
                config.host.clone().unwrap().as_str(),
                config.port.clone().unwrap(),
                config.org.clone().unwrap().as_str(),
                config.token.clone().unwrap().as_str(),
                config.bucket.clone().unwrap().as_str(),
                redis.clone(),
            )
            .await
            .expect("storage_data_row error");
        }

        // 创建 RabbitMQ 通道
        let rabbit_channel = rabbit_conn
            .create_channel()
            .await
            .map_err(|e| Box::new(e) as Box<dyn Error>)?;

        // 消息推送
        for queue in ["waring_handler", "waring_delay_handler", "transmit_handler"].iter() {
            rabbit_channel
                .basic_publish(
                    "",
                    *queue,
                    BasicPublishOptions::default(),
                    x.clone().as_bytes(),
                    BasicProperties::default(),
                )
                .await
                .map_err(|e| Box::new(e) as Box<dyn Error>)?;
        }

        // fixme: 处理最后推送时间（如果需要的话）
    } else {
        info!("未找到脚本 for uid: {}", mqtt_message.uid);
    }

    Ok(())
}

pub async fn pre_ws_handler(
    guard1: &Config,
    guard: &RedisWrapper,
    rabbit_conn: &Connection,
    channel1: &Channel,
) {
    let mut consumer = channel1
        .basic_consume(
            "pre_coap_handler",
            "",
            BasicConsumeOptions::default(),
            FieldTable::default(),
        )
        .await
        .unwrap();

    info!("rmq consumer connected, waiting for messages");
    while let Some(delivery_result) = consumer.next().await {
        match delivery_result {
            Ok(delivery) => {
                info!("received msg: {:?}", delivery);

                let result = String::from_utf8(delivery.data).unwrap();

                match handler_data_storage_string(
                    result,
                    Context::new().unwrap(),
                    guard1.influx_config.clone().unwrap(),
                    guard.clone(),
                    rabbit_conn,
                )
                .await
                {
                    Ok(_) => {
                        info!("msg processed");
                    }
                    Err(error) => {
                        error!("{}", error);
                    }
                };

                match channel1
                    .basic_ack(delivery.delivery_tag, BasicAckOptions::default())
                    .await
                {
                    Ok(_) => {
                        info!("消息已成功确认。");
                    }
                    Err(e) => {
                        error!("确认消息时发生错误: {}", e);
                        // 这里可以添加进一步的错误处理逻辑
                    }
                }
            }
            Err(err) => {
                error!("Error receiving message: {:?}", err);
            }
        }
    }
}
