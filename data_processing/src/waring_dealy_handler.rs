use common_lib::models::{DataRow, DataRowList, SignalDelayWaring, SignalDelayWaringParam, Tv};
use common_lib::redis_handler::{get_redis_instance, RedisWrapper};
use log::info;
use serde_json::from_str;
use std::collections::HashMap;
use std::error::Error;

pub async fn handler_waring_once(
    dt: DataRowList,
    waring_collection: String,
) -> Result<(), Box<dyn std::error::Error>> {
    let device_uid_string = &*dt.DeviceUid;
    let iden_code = &*dt.IdentificationCode;
    let push_time = dt.Time;
    let guard = get_redis_instance().await.unwrap();

    let mapping = get_delay_param(device_uid_string, iden_code, dt.DataRows, guard).await?;
    let mut script_param: HashMap<String, Vec<Tv>> = HashMap::new();

    let guard = get_redis_instance().await.unwrap();

    for param in &mapping {
        let key = format!(
            "signal_delay_warning:{}:{}:{}",
            param.device_uid, param.identification_code, param.signal_id
        );
        info!("key = {}", key);

        let members = guard.get_zset(&key).await.unwrap();

        let mut vs = Vec::new();

        for member in members {
            let t: Result<i64, _> = member.0.parse(); // 将字符串转换为 i64

            let time: i64 = t.unwrap_or_default();
            let value: f64 = member.1;
            vs.push(Tv { time, value });
        }

        script_param.insert(param.name.clone(), vs);
    }

    let script = get_delay_script(mapping, guard).await.unwrap();
    for x in script {
        let js = call_js(x.script, &script_param);
        info!("js = {}", js);
    }

    Ok(())
}

pub async fn get_delay_param(
    uid: &str,
    code: &str,
    rows: Vec<DataRow>,
    guard: MutexGuard<'_, RedisWrapper>,
) -> Result<Vec<SignalDelayWaringParam>, Box<dyn std::error::Error>> {
    let values = guard.get_list_all("delay_param").await?;

    let mut mapping = Vec::new();
    for value in values {
        if let Ok(param) = serde_json::from_str::<SignalDelayWaringParam>(&value) {
            if param.device_uid.to_string() == uid
                && param.identification_code == code
                && name_in_data_row(param.signal_name.clone(), &rows)
            {
                mapping.push(param);
            }
        }
    }

    Ok(mapping)
}

// 将 &rows 的类型改为 &Vec<DataRow>，并使用迭代器来简化循环
fn name_in_data_row(name: String, rows: &Vec<DataRow>) -> bool {
    rows.iter().any(|row| row.Name == name)
}

async fn get_delay_script(
    mapping: Vec<SignalDelayWaringParam>,
    redis: MutexGuard<'_, RedisWrapper>,
) -> Result<Vec<SignalDelayWaring>, Box<dyn std::error::Error>> {
    let mut res = Vec::new();

    for param in mapping {
        let id = param.signal_delay_waring_id;
        let key = "signal_delay_config";
        let val: Option<String> = redis.get_hash(key, &id.to_string()).await?;

        if let Some(value) = val {
            match from_str::<SignalDelayWaring>(&value) {
                Ok(singw) => {
                    res.push(singw);
                }
                Err(err) => {
                    eprintln!("解析 JSON 异常: {:?}", err);
                }
            }
        }
    }

    // 使用 HashMap 来存储已经出现过的 ID
    let mut id_map = HashMap::new();
    let mut unique_res = Vec::new();

    for item in res {
        if !id_map.contains_key(&item.id) {
            id_map.insert(item.id, true);
            unique_res.push(item);
        }
    }

    Ok(unique_res)
}
use quick_js::{Context, JsValue};
use tokio::sync::MutexGuard;

fn call_js(js: String, param: &HashMap<String, Vec<Tv>>) -> bool {
    // 创建新的 JavaScript 上下文
    let context = Context::new().unwrap();

    // 执行 JavaScript 代码
    context.eval(js.as_str()).unwrap();

    // 将 HashMap 转换为 JSON 字符串
    let json_param = serde_json::to_string(&param).unwrap();

    let js_code_2 = r#"
        function main2(data) {
            return JSON.parse(data);
        }"#;
    context.eval(js_code_2).unwrap();
    let value2 = context.call_function("main2", [json_param]).unwrap();
    info!("value2 = {:?}", value2);

    // 调用 JavaScript 函数，传递 JSON 参数
    let value = context.call_function("main", [value2]).unwrap();

    match value {
        JsValue::Bool(b) => b,
        _ => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Utc;
    use common_lib::config::{get_config, read_config};
    use common_lib::init_logger;
    use common_lib::mongo_utils::init_mongo;
    use common_lib::rabbit_utils::init_rabbitmq_with_config;
    use common_lib::redis_handler::init_redis;

    #[tokio::test]
    async fn cc() {
        init_logger();

        let result = read_config("app-local.yml").await.unwrap();
        let config = get_config().await.unwrap();

        let redis_config = config.redis_config.clone();
        let mongo_config = config.mongo_config.clone().unwrap();

        let influxdb = config.influx_config.clone().unwrap();
        init_redis(redis_config).await.unwrap();
        init_rabbitmq_with_config(config.mq_config.clone())
            .await
            .unwrap();

        init_mongo(mongo_config.clone()).await.unwrap();
        let now = Utc::now();
        let dt = DataRowList {
            Time: now.timestamp(),
            DeviceUid: "102".to_string(),
            IdentificationCode: "102".to_string(),
            DataRows: vec![DataRow {
                Name: "Temperature".to_string(),
                Value: "2".to_string(),
            }],
            Nc: "1".to_string(),
            Protocol: Some("MQTT".to_string()),
        };

        handler_waring_once(dt, "asf".to_string()).await.unwrap();
    }
    #[test]
    fn test_call_js() {
        // 定义 JavaScript 代码，其中包含需要测试的函数
        let js_code = r#"
            function main(param) {
                if (param["test"] && param["test"].length > 0) {
                    return true; // 返回 true
                }
                return false; // 返回 false
            }
        "#;

        // 创建测试用的参数
        let mut param: HashMap<String, Vec<Tv>> = HashMap::new();
        param.insert(
            "test".to_string(),
            vec![Tv {
                time: 1234567890,
                value: 42.0,
            }],
        );

        // 调用 call_js 函数
        let result = call_js(js_code.to_string(), &param);
        info!("{:?}", result);
        // 断言返回结果
        assert!(result); // 期望返回 true
    }
}
