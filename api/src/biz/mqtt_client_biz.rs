use crate::biz::user_biz::UserBiz;
use crate::db::db_model::{MqttClient, Signal, WebSocketHandler};
use anyhow::{Context, Error, Result};
use common_lib::redis_pool_utils::RedisOp;
use common_lib::sql_utils::{CrudOperations, Filter, PaginationParams, PaginationResult};
use sqlx::MySqlPool;

pub struct MqttClientBiz {
    pub redis: RedisOp,
    pub mysql: MySqlPool,
}

#[async_trait::async_trait]
impl CrudOperations<MqttClient> for MqttClientBiz {
    async fn create(&self, item: MqttClient) -> Result<MqttClient, Error> {
        let mut updates = vec![];

        if let Some(host) = item.host {
            updates.push(("host", host));
        }

        if let Some(port) = item.port {
            updates.push(("port", port.to_string()));
        }

        if let Some(client_id) = item.client_id {
            updates.push(("client_id", client_id));
        }

        if let Some(username) = item.username {
            updates.push(("username", username));
        }

        if let Some(password) = item.password {
            updates.push(("password", password));
        }

        if let Some(subtopic) = item.subtopic {
            updates.push(("subtopic", subtopic));
        }

        if let Some(start) = item.start {
            updates.push(("start", start.to_string()));
        }

        if let Some(script) = item.script {
            updates.push(("script", script));
        }

        log::info!("Creating mqtt client with updates: {:?}", updates);

        let result =
            common_lib::sql_utils::insert::<MqttClient>(&self.mysql, "mqtt_clients", updates).await;

        result
    }

    async fn update(&self, id: u64, item: MqttClient) -> Result<MqttClient, Error> {
        let mut updates = vec![];

        if let Some(host) = item.host {
            updates.push(("host", host));
        }

        if let Some(port) = item.port {
            updates.push(("port", port.to_string()));
        }

        if let Some(client_id) = item.client_id {
            updates.push(("client_id", client_id));
        }

        if let Some(username) = item.username {
            updates.push(("username", username));
        }

        if let Some(password) = item.password {
            updates.push(("password", password));
        }

        if let Some(subtopic) = item.subtopic {
            updates.push(("subtopic", subtopic));
        }

        if let Some(start) = item.start {
            updates.push(("start", start.to_string()));
        }

        if let Some(script) = item.script {
            updates.push(("script", script));
        }

        log::info!("Updating mqtt client with ID {}: {:?}", id, updates);

        let result = common_lib::sql_utils::update_by_id::<MqttClient>(
            &self.mysql,
            "mqtt_clients",
            id,
            updates,
        )
        .await;

        return match result {
            Ok(it) => Ok(it),
            Err(err) => Err(err),
        };
    }

    async fn delete(&self, id: u64) -> Result<(), Error> {
        log::info!("Deleting mqtt client with ID {}", id);

        common_lib::sql_utils::delete_by_id(&self.mysql, "mqtt_clients", id).await
    }

    async fn page(
        &self,
        filters: Vec<Filter>,
        pagination: PaginationParams,
    ) -> Result<PaginationResult<MqttClient>, Error> {
        log::info!(
            "Fetching page of mqtt clients with filters: {:?} and pagination: {:?}",
            filters,
            pagination
        );

        let result = common_lib::sql_utils::paginate::<MqttClient>(
            &self.mysql,
            "mqtt_clients",
            filters,
            pagination,
        )
        .await;

        result
    }

    async fn list(&self, filters: Vec<Filter>) -> Result<Vec<MqttClient>, Error> {
        log::info!("Fetching list of mqtt clients with filters: {:?}", filters);
        let result =
            common_lib::sql_utils::list::<MqttClient>(&self.mysql, "mqtt_clients", filters).await;
        return result;
    }

    async fn by_id(&self, id: u64) -> Result<MqttClient, Error> {
        let result =
            common_lib::sql_utils::by_id_common::<MqttClient>(&self.mysql, "mqtt_clients", id)
                .await;
        result
    }
}
