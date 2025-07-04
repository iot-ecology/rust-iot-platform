/*
Copyright [2025] [Zen HuiFer]

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
*/

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct CalcCache {
    pub id: i64,
    pub param: Option<Vec<CalcParamCache>>,
    pub cron: String,
    pub script: String,
    pub offset: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CalcParamCache {
    pub protocol: String,
    pub identification_code: String, // 设备标识码
    pub device_uid: i64,             // MQTT客户端表的外键ID

    pub name: String,                                                 // 参数名称
    #[serde(rename = "signal_name")]
    pub signal_name: String, // 信号表 name
    pub reduce: String,       // 数据聚合方式 1. 求和 2. 平均值 3. 最大值 4. 最小值 4. 原始
    pub calc_rule_id: i64,    // CalcRule 主键
    #[serde(rename = "signal_id")]
    pub signal_id: i64, // 信号表的外键ID
}
#[derive(Debug, Serialize, Deserialize)]
pub struct QueryEvent {
    #[serde(rename = "start_time")]
    pub start_time: i64,
    #[serde(rename = "end_time")]
    pub end_time: i64,
    #[serde(rename = "ID")]
    pub id: i64,
}
