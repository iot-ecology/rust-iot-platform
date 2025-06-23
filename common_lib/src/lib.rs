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

pub mod config;
pub mod influxdb_utils;
pub mod models;
pub mod mongo_utils;
pub mod mysql_utils;
pub mod rabbit_utils;
pub mod redis_handler;
pub mod redis_lock;
pub mod redis_pool_utils;
pub mod sql_utils;
pub mod time_utils;
pub mod ut;
pub mod servlet_common;

pub fn init_logger() {
    log4rs::init_file("log4rs.yml", Default::default()).unwrap();
}
