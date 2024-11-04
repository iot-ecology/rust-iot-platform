use common_lib::models::DataRowList;
use common_lib::redis_handler::get_redis_instance;
use std::collections::HashMap;

pub async fn handler_waring_once(dt: DataRowList) -> Result<(), Box<dyn std::error::Error>> {
    let device_uid_string = &*dt.device_uid;
    let iden_code = &*dt.identification_code;
    let push_time = dt.time;

    let mapping = get_mapping_signal_waring_config(device_uid_string, iden_code)
        .await
        .unwrap();

    for x in dt.data {
        let x1 = mapping.get(x.name.as_str()).unwrap();

        let floatValue = x.value.parse::<f64>().unwrap();

        for config in x1 {
            if config.in_or_out == 1 {
                if (config.max <= floatValue && floatValue <= config.max) {
                    //     范围内
                }
            } else {
                if (floatValue < config.min || floatValue > config.max) {
                    //     范围外
                }
            }
        }
    }
    Ok(())
}

async fn get_mapping_signal_waring_config(
    device_uid_string: &str,
    iden_code: &str,
) -> Result<HashMap<String, Vec<SignalWaringConfig>>, Box<dyn std::error::Error>> {
    let guard = get_redis_instance().await.unwrap();
    let key = format!("signal:{}:{}", device_uid_string, iden_code);
    let vec = guard.get_list_all(key.as_str()).await.unwrap();
    let mut mapping: HashMap<String, Vec<SignalWaringConfig>> = HashMap::new();
    for str_signal in vec {
        let signal: Signal = match serde_json::from_str(&str_signal) {
            Ok(s) => s,
            Err(_) => continue, // 跳过反序列化失败的信号
        };

        let key_warning = format!("waring:{}", signal.id);
        let warnings = guard.get_list_all(key_warning.as_str()).await.unwrap();

        let mut swcs: Vec<SignalWaringConfig> = vec![];
        for sw in warnings {
            let mut swc: SignalWaringConfig = match serde_json::from_str(&sw) {
                Ok(w) => w,
                Err(_) => continue, // 跳过反序列化失败的警告配置
            };
            swc.unit = signal.unit.clone();
            swcs.push(swc);
        }

        mapping.insert(signal.name, swcs);
    }
    Ok(mapping)
}

#[derive(Deserialize, Serialize, Debug)]
struct Signal {
    #[serde(rename = "ID")] // 在序列化时使用 "ID"
    id: i32,
    name: String,
    unit: String,
}

use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize, Debug)]
struct SignalWaringConfig {
    #[serde(rename = "signal_id")]
    signal_id: i32, // 信号表的外键ID
    #[serde(rename = "min")]
    min: f64, // 范围, 小值
    #[serde(rename = "max")]
    max: f64, // 范围, 大值
    #[serde(rename = "in_or_out")]
    in_or_out: i32, // 1 范围内报警 0 范围外报警
    #[serde(rename = "unit")]
    unit: String, // 单位
    #[serde(rename = "ID")]
    id: i32, // ID
}
