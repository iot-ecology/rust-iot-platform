use crate::biz::calc_param_biz::CalcParamBiz;
use crate::biz::calc_rule_biz::CalcRuleBiz;
use crate::biz::coap_biz::CoapHandlerBiz;
use crate::biz::dashboard_biz::DashboardBiz;
use crate::biz::dept_biz::DeptBiz;
use crate::biz::device_group_biz::DeviceGroupBiz;
use crate::biz::device_info_biz::DeviceInfoBiz;
use crate::biz::http_biz::HttpHandlerBiz;
use crate::biz::message_list_biz::MessageListBiz;
use crate::biz::mqtt_client_biz::MqttClientBiz;
use crate::biz::notice::dingding_biz::DingDingBiz;
use crate::biz::notice::feishu_biz::FeiShuBiz;
use crate::biz::product_biz::ProductBiz;
use crate::biz::production_plan_biz::ProductionPlanBiz;
use crate::biz::repair_record_biz::RepairRecordBiz;
use crate::biz::role_biz::RoleBiz;
use crate::biz::shipment_record_biz::ShipmentRecordBiz;
use crate::biz::signal_biz::SignalBiz;
use crate::biz::signal_delay_waring_biz::SignalDelayWaringBiz;
use crate::biz::signal_delay_waring_param_biz::SignalDelayWaringParamBiz;
use crate::biz::sim_card_biz::SimCardBiz;
use crate::biz::tcp_biz::TcpHandlerBiz;
use crate::biz::transmit::bind::cassandra_bind_biz::CassandraTransmitBindBiz;
use crate::biz::transmit::bind::clickhouse_bind_biz::ClickhouseTransmitBindBiz;
use crate::biz::transmit::bind::influxdb_bind_biz::InfluxDbTransmitBindBiz;
use crate::biz::transmit::bind::mongo_bind_biz::MongoTransmitBindBiz;
use crate::biz::transmit::bind::mysql_bind_biz::MysqlTransmitBindBiz;
use crate::biz::transmit::cassandra_transmit_biz::CassandraTransmitBiz;
use crate::biz::transmit::clickhouse_transmit_biz::ClickhouseTransmitBiz;
use crate::biz::transmit::influxdb_transmit_biz::InfluxDbTransmitBiz;
use crate::biz::transmit::mongo_transmit_biz::MongoTransmitBiz;
use crate::biz::transmit::mysql_transmit_biz::MysqlTransmitBiz;
use crate::biz::user_biz::UserBiz;
use crate::biz::ws_biz::WebSocketHandlerBiz;
use crate::controller::notice::dingding_router;
use crate::controller::notice::feishu_router;
use crate::controller::transmit::bind::cassandra_bind_router;
use crate::controller::transmit::bind::clickhouse_bind_router;
use crate::controller::transmit::bind::influxdb_bind_router;
use crate::controller::transmit::bind::mongo_bind_router;
use crate::controller::transmit::bind::mysql_bind_router;
use crate::controller::transmit::cassandra_transmit_router;
use crate::controller::transmit::clickhouse_transmit_router;
use crate::controller::transmit::influxdb_transmit_router;
use crate::controller::transmit::mongo_transmit_router;
use crate::controller::transmit::mysql_transmit_router;
use crate::controller::user_router::{
    bind_dept, bind_device_info, bind_role, by_id_user, create_user, delete_user, list_user,
    page_user, query_bind_dept, query_bind_device_info, query_bind_role, update_user, user_index,
};
use common_lib::config::{read_config_tb, InfluxConfig, MongoConfig, MySQLConfig, RedisConfig};
use common_lib::mysql_utils::gen_mysql_url;
use common_lib::rabbit_utils::RabbitMQFairing;
use common_lib::redis_pool_utils::{create_redis_pool_from_config, RedisOp};
use rocket::fairing::{Info, Kind};
use rocket::request::FromRequest;
use rocket::{launch, routes, Build, Rocket};
use serde::Serialize;
use sqlx::MySqlPool;
use std::collections::HashSet;

use crate::controller::calc_param_router;
use crate::controller::calc_rule_router;
use crate::controller::coap_handler_router;
use crate::controller::dashboard_router;
use crate::controller::dept_router;
use crate::controller::device_group_router;
use crate::controller::device_info_router;
use crate::controller::http_handler_router;
use crate::controller::login_router::login;
use crate::controller::login_router::userinfo;
use crate::controller::message_list_router;
use crate::controller::mqtt_client_router;
use crate::controller::product_router;
use crate::controller::production_plan_router;
use crate::controller::repair_record_router;
use crate::controller::role_router;
use crate::controller::shipment_record_router;
use crate::controller::signal_delay_waring_param_router;
use crate::controller::signal_delay_waring_router;
use crate::controller::signal_router;
use crate::controller::sim_card_router;
use crate::controller::tcp_handler_router;
use crate::controller::ws_handler_router;

mod biz;
mod controller;
mod db;
mod ut;

use rocket::http::Header;
use rocket::{Request, Response};

pub struct CORS;

#[rocket::async_trait]
impl rocket::fairing::Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, PUT, DELETE, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "Content-Type, Authorization"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}
use rocket::options;

#[options("/<_..>")]
pub fn all_options() {
    // 空函数体，响应会被 CORS fairing 处理
}

#[launch]
fn rocket() -> _ {
    common_lib::init_logger(); // 初始化日志记录

    let config1 = read_config_tb("app-local.yml");

    // 构建并启动 Rocket 应用
    rocket::build()
        .attach(CORS)  // 添加 CORS 支持
        .mount("/", routes![all_options])  // 添加全局 OPTIONS 路由处理
        .attach(RabbitMQFairing {
            config: config1.mq_config.clone(),
        })
        .attach(MySqlPoolFairing {
            config: config1.mysql_config.clone().unwrap(),
            redis_config: config1.redis_config.clone(),
            mongo_config: config1.mongo_config.clone().unwrap(),
            influxd_config: config1.influx_config.clone().unwrap(),
        })
        .manage(config1.clone())
        .configure(rocket::Config {
            port: config1.node_info.port,
            log_level: rocket::config::LogLevel::Off,
            ..Default::default()
        })
        .mount(
            "/",
            routes![
                // Existing routes
                crate::controller::demo_api::index,
                crate::controller::demo_api::beat,
                user_index,
                create_user,
                update_user,
                page_user,
                delete_user,
                by_id_user,
                list_user,
                bind_role,
                bind_dept,
                query_bind_role,
                query_bind_dept,
                bind_device_info,
                query_bind_device_info,
                login,
                userinfo,
                // Device related routes
                device_info_router::create_device_info,
                device_info_router::list_device_info,
                device_info_router::update_device_info,
                device_info_router::by_id_device_info,
                device_info_router::page_device_info,
                device_info_router::delete_device_info,
                device_info_router::bind_mqtt,
                device_info_router::bind_tcp,
                device_info_router::bind_http,
                device_info_router::bind_hcoap,
                device_info_router::bind_websocket,
                device_info_router::query_bind_mqtt,
                device_info_router::query_bind_tcp,
                device_info_router::query_bind_http,
                device_info_router::query_bind_coap,
                device_info_router::query_bind_websocket,
                // Device group routes
                device_group_router::create_device_group,
                device_group_router::update_device_group,
                device_group_router::delete_device_group,
                device_group_router::page_device_group,
                device_group_router::by_id_device_group,
                device_group_router::query_bind_device_info,
                device_group_router::bind_device_info,
                device_group_router::bind_mqtt,
                device_group_router::query_bind_mqtt,
                // Product routes
                product_router::create_product,
                product_router::update_product,
                product_router::delete_product,
                product_router::page_product,
                product_router::by_id_product,
                product_router::list_product,
                // MQTT client routes
                mqtt_client_router::page_mqtt,
                mqtt_client_router::list_mqtt,
                mqtt_client_router::by_id_mqtt,
                mqtt_client_router::start_mqtt,
                mqtt_client_router::stop_mqtt,
                mqtt_client_router::update_mqtt,
                mqtt_client_router::delete_mqtt,
                mqtt_client_router::node_using_status,
                mqtt_client_router::set_script,
                mqtt_client_router::check_script,
                mqtt_client_router::send_mqtt_message,
                // Signal routes
                signal_router::create_signal,
                signal_router::update_signal,
                signal_router::delete_signal,
                signal_router::page_signal,
                signal_router::signal_by_id,
                // Department routes
                dept_router::create_dept,
                dept_router::update_dept,
                dept_router::delete_dept,
                dept_router::page_dept,
                dept_router::by_id_dept,
                // Role routes
                role_router::create_role,
                role_router::update_role,
                role_router::delete_role,
                role_router::page_role,
                role_router::by_id_role,
                // Handler routes
                http_handler_router::create_http_handler,
                http_handler_router::update_http_handler,
                http_handler_router::delete_http_handler,
                http_handler_router::page_http_handler,
                http_handler_router::by_id_http_handler,
                ws_handler_router::create_websocket_handler,
                ws_handler_router::update_websocket_handler,
                ws_handler_router::delete_websocket_handler,
                ws_handler_router::page_websocket_handler,
                ws_handler_router::by_id_websocket_handler,
                tcp_handler_router::create_tcp_handler,
                tcp_handler_router::update_tcp_handler,
                tcp_handler_router::delete_tcp_handler,
                tcp_handler_router::page_tcp_handler,
                tcp_handler_router::by_id_tcp_handler,
                coap_handler_router::create_coap_handler,
                coap_handler_router::update_coap_handler,
                coap_handler_router::delete_coap_handler,
                coap_handler_router::page_coap_handler,
                coap_handler_router::by_id_coap_handler,
                // Transmit routes
                mysql_transmit_router::create_mysql_transmit,
                mysql_transmit_router::update_mysql_transmit,
                mysql_transmit_router::delete_mysql_transmit,
                mysql_transmit_router::page_mysql_transmit,
                mysql_transmit_router::by_id_mysql_transmit,
                mongo_transmit_router::create_mongo_transmit,
                mongo_transmit_router::update_mongo_transmit,
                mongo_transmit_router::delete_mongo_transmit,
                mongo_transmit_router::page_mongo_transmit,
                mongo_transmit_router::by_id_mongo_transmit,
                influxdb_transmit_router::create_influxdb_transmit,
                influxdb_transmit_router::update_influxdb_transmit,
                influxdb_transmit_router::delete_influxdb_transmit,
                influxdb_transmit_router::page_influxdb_transmit,
                influxdb_transmit_router::by_id_influxdb_transmit,
                clickhouse_transmit_router::create_clickhouse_transmit,
                clickhouse_transmit_router::update_clickhouse_transmit,
                clickhouse_transmit_router::delete_clickhouse_transmit,
                clickhouse_transmit_router::page_clickhouse_transmit,
                clickhouse_transmit_router::by_id_clickhouse_transmit,
                cassandra_transmit_router::create_cassandra_transmit,
                cassandra_transmit_router::update_cassandra_transmit,
                cassandra_transmit_router::delete_cassandra_transmit,
                cassandra_transmit_router::page_cassandra_transmit,
                cassandra_transmit_router::by_id_cassandra_transmit,
                // MySQL bind
                mysql_bind_router::create_mysql_transmit_bind,
                mysql_bind_router::update_mysql_transmit_bind,
                mysql_bind_router::delete_mysql_transmit_bind,
                mysql_bind_router::page_mysql_transmit_bind,
                mysql_bind_router::by_id_mysql_transmit_bind,
                // MongoDB bind
                mongo_bind_router::create_mongo_transmit_bind,
                mongo_bind_router::update_mongo_transmit_bind,
                mongo_bind_router::delete_mongo_transmit_bind,
                mongo_bind_router::page_mongo_transmit_bind,
                mongo_bind_router::by_id_mongo_transmit_bind,
                // InfluxDB bind
                influxdb_bind_router::create_influxdb_transmit_bind,
                influxdb_bind_router::update_influxdb_transmit_bind,
                influxdb_bind_router::delete_influxdb_transmit_bind,
                influxdb_bind_router::page_influxdb_transmit_bind,
                influxdb_bind_router::by_id_influxdb_transmit_bind,
                // Clickhouse bind
                clickhouse_bind_router::create_clickhouse_transmit_bind,
                clickhouse_bind_router::update_clickhouse_transmit_bind,
                clickhouse_bind_router::delete_clickhouse_transmit_bind,
                clickhouse_bind_router::page_clickhouse_transmit_bind,
                clickhouse_bind_router::by_id_clickhouse_transmit_bind,
                // Cassandra bind
                cassandra_bind_router::create_cassandra_transmit_bind,
                cassandra_bind_router::update_cassandra_transmit_bind,
                cassandra_bind_router::delete_cassandra_transmit_bind,
                cassandra_bind_router::page_cassandra_transmit_bind,
                cassandra_bind_router::by_id_cassandra_transmit_bind,
                // Notice routes
                dingding_router::create_dingding,
                dingding_router::update_dingding,
                dingding_router::delete_dingding,
                dingding_router::page_dingding,
                dingding_router::by_id_dingding,
                dingding_router::bind_dingding,
                feishu_router::create_feishu,
                feishu_router::update_feishu,
                feishu_router::delete_feishu,
                feishu_router::page_feishu,
                feishu_router::by_id_feishu,
                feishu_router::bind_feishu,
                // Calculation rules
                calc_rule_router::create_calc_rule,
                calc_rule_router::update_calc_rule,
                calc_rule_router::delete_calc_rule,
                calc_rule_router::page_calc_rule,
                calc_rule_router::start_calc_rule,
                calc_rule_router::stop_calc_rule,
                calc_rule_router::refresh_calc_rule,
                calc_rule_router::mock_calc_rule,
                calc_rule_router::calc_rule_result,
                // Calculation parameters
                calc_param_router::create_calc_param,
                calc_param_router::update_calc_param,
                calc_param_router::delete_calc_param,
                calc_param_router::page_calc_param,
                // Production plans
                production_plan_router::create_production_plan,
                production_plan_router::update_production_plan,
                production_plan_router::delete_production_plan,
                production_plan_router::page_production_plan,
                production_plan_router::by_id_production_plan,
                // Repair records
                repair_record_router::create_repair_record,
                repair_record_router::update_repair_record,
                repair_record_router::delete_repair_record,
                repair_record_router::page_repair_record,
                repair_record_router::by_id_repair_record,
                // Shipment records
                shipment_record_router::create_shipment_record,
                shipment_record_router::update_shipment_record,
                shipment_record_router::delete_shipment_record,
                shipment_record_router::page_shipment_record,
                shipment_record_router::by_id_shipment_record,
                shipment_record_router::find_by_shipment_product_detail,
                // SIM cards
                sim_card_router::create_sim_card,
                sim_card_router::update_sim_card,
                sim_card_router::delete_sim_card,
                sim_card_router::page_sim_card,
                sim_card_router::by_id_sim_card,
                sim_card_router::bind_device_info,
                // Signal delay warnings
                signal_delay_waring_router::create_signal_delay_waring,
                signal_delay_waring_router::update_signal_delay_waring,
                signal_delay_waring_router::delete_signal_delay_waring,
                signal_delay_waring_router::page_signal_delay_waring,
                signal_delay_waring_router::by_id_signal_delay_waring,
                signal_delay_waring_router::mock_signal_delay_waring,
                signal_delay_waring_router::gen_param_signal_delay_waring,
                signal_delay_waring_router::query_waring_list,
                signal_delay_waring_router::list_signal_delay_waring,
                // Signal delay warning parameters
                signal_delay_waring_param_router::create_signal_delay_waring_param,
                signal_delay_waring_param_router::update_signal_delay_waring_param,
                signal_delay_waring_param_router::delete_signal_delay_waring_param,
                signal_delay_waring_param_router::page_signal_delay_waring_param,
                message_list_router::page_message_list,
                dashboard_router::create_dashboard,
                dashboard_router::update_dashboard,
                dashboard_router::delete_dashboard,
                dashboard_router::page_dashboard,
                dashboard_router::by_id_dashboard
            ],
        )
}
pub struct MySqlPoolFairing {
    pub config: MySQLConfig,
    pub redis_config: RedisConfig,
    pub mongo_config: MongoConfig,

    pub influxd_config: InfluxConfig,

}
#[rocket::async_trait]
impl rocket::fairing::Fairing for MySqlPoolFairing {
    fn info(&self) -> Info {
        Info {
            name: "Mysql Initializer",
            kind: Kind::Ignite,
        }
    }
    async fn on_ignite(&self, rocket: Rocket<Build>) -> Result<Rocket<Build>, Rocket<Build>> {
        let string = gen_mysql_url(&self.config);
        log::info!("mysql url = {}",string);

        let pool = MySqlPool::connect(&string).await.unwrap();

        let redis_pool = create_redis_pool_from_config(&self.redis_config);

        let redis_op = RedisOp { pool: redis_pool };


        let mongo_db_manager = Box::leak(Box::new(MongoDBManager::new(self.mongo_config.clone()).await.unwrap()));

        let user_biz = UserBiz::new(redis_op.clone(), pool.clone());
        let websocket_handler_biz = WebSocketHandlerBiz::new(redis_op.clone(), pool.clone());
        let tcp_handler_biz = TcpHandlerBiz::new(redis_op.clone(), pool.clone());
        let sim_card_biz = SimCardBiz::new(redis_op.clone(), pool.clone());
        let signal_delay_waring_param_biz =
            SignalDelayWaringParamBiz::new(redis_op.clone(), pool.clone());
        let signal_delay_waring_biz = SignalDelayWaringBiz::new(redis_op.clone(), pool.clone(),
                                                                mongo_db_manager,
                                                                self.mongo_config.clone(),
        );
        let calc_run_biz = CalcRunBiz::new(redis_op.clone(), pool.clone(), mongo_db_manager, self.influxd_config.clone());


        let signal_biz = SignalBiz::new(redis_op.clone(), pool.clone());
        let signal_waring_config_biz = SignalWaringConfigBiz::new(redis_op.clone(), pool.clone(), mongo_db_manager,
                                                                  self.mongo_config.clone(),
        );
        let shipment_record_biz = ShipmentRecordBiz::new(redis_op.clone(), pool.clone());
        let role_biz = RoleBiz::new(redis_op.clone(), pool.clone());
        let repair_record_biz = RepairRecordBiz::new(redis_op.clone(), pool.clone());
        let production_plan_biz = ProductionPlanBiz::new(redis_op.clone(), pool.clone());
        let product_biz = ProductBiz::new(redis_op.clone(), pool.clone());
        let mqtt_client_biz = MqttClientBiz::new(redis_op.clone(), pool.clone());
        let message_list_biz = MessageListBiz::new(redis_op.clone(), pool.clone());
        let http_handler_biz = HttpHandlerBiz::new(redis_op.clone(), pool.clone());
        let device_info_biz = DeviceInfoBiz::new(redis_op.clone(), pool.clone());
        let device_group_biz = DeviceGroupBiz::new(redis_op.clone(), pool.clone());
        let dept_biz = DeptBiz::new(redis_op.clone(), pool.clone());
        let dashboard_biz = DashboardBiz::new(redis_op.clone(), pool.clone());
        let coap_handler_biz = CoapHandlerBiz::new(redis_op.clone(), pool.clone());
        let calc_rule_biz = CalcRuleBiz::new(redis_op.clone(), pool.clone());
        let calc_param_biz = CalcParamBiz::new(redis_op.clone(), pool.clone());
        let mysql_transmit_biz = MysqlTransmitBiz::new(redis_op.clone(), pool.clone());
        let mongo_transmit_biz = MongoTransmitBiz::new(redis_op.clone(), pool.clone());
        let influx_db_transmit_biz = InfluxDbTransmitBiz::new(redis_op.clone(), pool.clone());
        let clickhouse_transmit_biz = ClickhouseTransmitBiz::new(redis_op.clone(), pool.clone());
        let cassandra_transmit_biz = CassandraTransmitBiz::new(redis_op.clone(), pool.clone());
        let mysql_transmit_bind_biz = MysqlTransmitBindBiz::new(redis_op.clone(), pool.clone());
        let mongo_transmit_bind_biz = MongoTransmitBindBiz::new(redis_op.clone(), pool.clone());
        let influx_db_transmit_bind_biz =
            InfluxDbTransmitBindBiz::new(redis_op.clone(), pool.clone());
        let clickhouse_transmit_bind_biz =
            ClickhouseTransmitBindBiz::new(redis_op.clone(), pool.clone());
        let cassandra_transmit_bind_biz =
            CassandraTransmitBindBiz::new(redis_op.clone(), pool.clone());
        let fei_shu_biz = FeiShuBiz::new(redis_op.clone(), pool.clone());
        let ding_ding_biz = DingDingBiz::new(redis_op.clone(), pool.clone());

        Ok(rocket
            .manage(pool)
            .manage(redis_op.clone())
            .manage(user_biz)
            .manage(calc_run_biz)
            .manage(websocket_handler_biz)
            .manage(tcp_handler_biz)
            .manage(sim_card_biz)
            .manage(signal_delay_waring_param_biz)
            .manage(signal_delay_waring_biz)
            .manage(signal_biz)
            .manage(signal_waring_config_biz)
            .manage(shipment_record_biz)
            .manage(role_biz)
            .manage(repair_record_biz)
            .manage(production_plan_biz)
            .manage(product_biz)
            .manage(mqtt_client_biz)
            .manage(message_list_biz)
            .manage(http_handler_biz)
            .manage(device_info_biz)
            .manage(device_group_biz)
            .manage(dept_biz)
            .manage(dashboard_biz)
            .manage(coap_handler_biz)
            .manage(calc_rule_biz)
            .manage(calc_param_biz)
            .manage(mysql_transmit_biz)
            .manage(mongo_transmit_biz)
            .manage(influx_db_transmit_biz)
            .manage(clickhouse_transmit_biz)
            .manage(cassandra_transmit_biz)
            .manage(mysql_transmit_bind_biz)
            .manage(mongo_transmit_bind_biz)
            .manage(influx_db_transmit_bind_biz)
            .manage(clickhouse_transmit_bind_biz)
            .manage(cassandra_transmit_bind_biz)
            .manage(fei_shu_biz)
            .manage(ding_ding_biz))
    }
}
pub struct AuthToken(String);
use crate::biz::calc_run_biz::CalcRunBiz;
use crate::biz::signal_waring_config_biz::SignalWaringConfigBiz;
use common_lib::mongo_utils::MongoDBManager;
use rocket::serde::json::Json;
use rocket::{
    self,
    http::Status,
    request::{Outcome, },
};

#[rocket::async_trait]
impl<'r> FromRequest<'r> for AuthToken {
    type Error = Json<ErrorResponse>;

    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let whitelist: HashSet<String> = vec![
            "/login".to_string(),
            // "/userinfo".to_string(),
        ]
            .into_iter()
            .collect();

        let path = request.uri().path();

        if whitelist.contains(path.as_str()) {
            Outcome::Success(AuthToken("".to_string()))
        } else {
            if let Some(value) = request.headers().get_one("Authorization") {
                Outcome::Success(AuthToken(value.to_string()))
            } else {
                Outcome::Error((
                    Status::Unauthorized,
                    Json(ErrorResponse {
                        message: "Authorization header missing".to_string(),
                    }),
                ))
            }
        }
    }
}

#[derive(Serialize, Debug)]
pub struct ErrorResponse {
    message: String,
}
