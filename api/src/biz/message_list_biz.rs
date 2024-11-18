use crate::biz::http_biz::HttpHandlerBiz;
use crate::biz::user_biz::UserBiz;
use crate::db::db_model::{MessageList, Signal, WebSocketHandler};
use anyhow::{Context, Error, Result};
use common_lib::redis_pool_utils::RedisOp;
use common_lib::sql_utils::{CrudOperations, FilterInfo, PaginationParams, PaginationResult};
use sqlx::MySqlPool;

pub struct MessageListBiz {
    pub redis: RedisOp,
    pub mysql: MySqlPool,
}
impl MessageListBiz {
    pub fn new(redis: RedisOp, mysql: MySqlPool) -> Self {
        MessageListBiz { redis, mysql }
    }
}

#[async_trait::async_trait]
impl CrudOperations<MessageList> for MessageListBiz {
    async fn create(&self, item: MessageList) -> Result<MessageList, Error> {
        let mut updates = vec![];

        if let Some(content) = item.content {
            updates.push(("content", content));
        }

        if let Some(en_content) = item.en_content {
            updates.push(("en_content", en_content));
        }

        if let Some(message_type_id) = item.message_type_id {
            updates.push(("message_type_id", message_type_id.to_string()));
        }

        if let Some(ref_id) = item.ref_id {
            updates.push(("ref_id", ref_id));
        }

        log::info!("Creating message list with updates: {:?}", updates);

        let result =
            common_lib::sql_utils::insert::<MessageList>(&self.mysql, "message_lists", updates)
                .await;

        result
    }

    async fn update(&self, id: u64, item: MessageList) -> Result<MessageList, Error> {
        let mut updates = vec![];

        if let Some(content) = item.content {
            updates.push(("content", content));
        }

        if let Some(en_content) = item.en_content {
            updates.push(("en_content", en_content));
        }

        if let Some(message_type_id) = item.message_type_id {
            updates.push(("message_type_id", message_type_id.to_string()));
        }

        if let Some(ref_id) = item.ref_id {
            updates.push(("ref_id", ref_id));
        }

        log::info!("Updating message list with ID {}: {:?}", id, updates);

        let result = common_lib::sql_utils::update_by_id::<MessageList>(
            &self.mysql,
            "message_lists",
            id,
            updates,
        )
        .await;

        return match result {
            Ok(it) => Ok(it),
            Err(err) => Err(err),
        };
    }

    async fn delete(&self, id: u64) -> Result<MessageList, Error> {
        log::info!("Deleting message list with ID {}", id);

        common_lib::sql_utils::delete_by_id(&self.mysql, "message_lists", id).await
    }

    async fn page(
        &self,
        filters: Vec<FilterInfo>,
        pagination: PaginationParams,
    ) -> Result<PaginationResult<MessageList>, Error> {
        log::info!(
            "Fetching page of message lists with filters: {:?} and pagination: {:?}",
            filters,
            pagination
        );

        let result = common_lib::sql_utils::paginate::<MessageList>(
            &self.mysql,
            "message_lists",
            filters,
            pagination,
        )
        .await;

        result
    }

    async fn list(&self, filters: Vec<FilterInfo>) -> Result<Vec<MessageList>, Error> {
        log::info!("Fetching list of message lists with filters: {:?}", filters);
        let result =
            common_lib::sql_utils::list::<MessageList>(&self.mysql, "message_lists", filters).await;
        return result;
    }

    async fn by_id(&self, id: u64) -> Result<MessageList, Error> {
        let result =
            common_lib::sql_utils::by_id_common::<MessageList>(&self.mysql, "message_lists", id)
                .await;
        result
    }
}
