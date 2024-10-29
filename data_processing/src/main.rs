mod fff;

use crate::fff::{f1, f2};
use log::{error, info, warn};
use log4rs;

fn init_logger() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
}

fn main() {
    init_logger();
    f1();
    f2();
    // 示例日志
    info!("这是一条信息日志");
    warn!("这是一条警告日志");
    error!("这是一条错误日志");
}
