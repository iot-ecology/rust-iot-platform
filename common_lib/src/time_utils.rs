use chrono::{Local, Utc};
use chrono_tz::Tz;
use chrono_tz::Tz::UTC;
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
