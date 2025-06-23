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

use std::str::FromStr;
use chrono::{Local, Utc};
use chrono_tz::Tz;
use cron::Schedule;

pub fn local_to_utc() -> i64 {
    // 获取当前本地时间
    let local_time = Local::now();

    // 获取北京时间（Asia/Shanghai 时区）
    let tz: Tz = "Asia/Shanghai".parse().unwrap();

    // 将本地时间转换为北京时间，再转换为 UTC 时间
    let beijing_time = local_time.with_timezone(&tz);
    let utc_time = beijing_time.with_timezone(&Utc);

    beijing_time.timestamp()
}
pub fn get_next_time(cron_expr: &str) -> Option<i64> {
    // 解析 cron 表达式
    let schedule = Schedule::from_str(cron_expr).ok()?;

    // 计算下一次执行时间
    let time = Utc::now();

    let beijing_time = time.with_timezone(&chrono::FixedOffset::east(8 * 3600));

    println!("当前时间: {}", beijing_time);
    if let Some(next_time) = schedule.after(&time).next() {
        // 返回时间戳（秒）
        Some(next_time.timestamp())
    } else {
        None
    }
}
