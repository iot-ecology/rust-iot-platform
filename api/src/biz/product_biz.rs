use crate::biz::mqtt_client_biz::MqttClientBiz;
use crate::biz::user_biz::UserBiz;
use crate::db::db_model::{Product, Signal, WebSocketHandler};
use anyhow::{Context, Error, Result};
use common_lib::redis_pool_utils::RedisOp;
use common_lib::sql_utils::{CrudOperations, Filter, PaginationParams, PaginationResult};
use sqlx::MySqlPool;

pub struct ProductBiz {
    pub redis: RedisOp,
    pub mysql: MySqlPool,
}
impl ProductBiz {
    pub fn new(redis: RedisOp, mysql: MySqlPool) -> Self {
        ProductBiz { redis, mysql }
    }
}

#[async_trait::async_trait]
impl CrudOperations<Product> for ProductBiz {
    async fn create(&self, item: Product) -> Result<Product, Error> {
        let mut updates = vec![];

        if let Some(name) = item.name {
            updates.push(("name", name));
        }

        if let Some(description) = item.description {
            updates.push(("description", description));
        }

        if let Some(sku) = item.sku {
            updates.push(("sku", sku));
        }

        if let Some(price) = item.price {
            updates.push(("price", price.to_string()));
        }

        if let Some(cost) = item.cost {
            updates.push(("cost", cost.to_string()));
        }

        if let Some(quantity) = item.quantity {
            updates.push(("quantity", quantity.to_string()));
        }

        if let Some(minimum_stock) = item.minimum_stock {
            updates.push(("minimum_stock", minimum_stock.to_string()));
        }

        if let Some(warranty_period) = item.warranty_period {
            updates.push(("warranty_period", warranty_period.to_string()));
        }

        if let Some(status) = item.status {
            updates.push(("status", status));
        }

        if let Some(tags) = item.tags {
            updates.push(("tags", tags));
        }

        if let Some(image_url) = item.image_url {
            updates.push(("image_url", image_url));
        }

        log::info!("Creating product with updates: {:?}", updates);

        let result =
            common_lib::sql_utils::insert::<Product>(&self.mysql, "products", updates).await;

        result
    }

    async fn update(&self, id: u64, item: Product) -> Result<Product, Error> {
        let mut updates = vec![];

        if let Some(name) = item.name {
            updates.push(("name", name));
        }

        if let Some(description) = item.description {
            updates.push(("description", description));
        }

        if let Some(sku) = item.sku {
            updates.push(("sku", sku));
        }

        if let Some(price) = item.price {
            updates.push(("price", price.to_string()));
        }

        if let Some(cost) = item.cost {
            updates.push(("cost", cost.to_string()));
        }

        if let Some(quantity) = item.quantity {
            updates.push(("quantity", quantity.to_string()));
        }

        if let Some(minimum_stock) = item.minimum_stock {
            updates.push(("minimum_stock", minimum_stock.to_string()));
        }

        if let Some(warranty_period) = item.warranty_period {
            updates.push(("warranty_period", warranty_period.to_string()));
        }

        if let Some(status) = item.status {
            updates.push(("status", status));
        }

        if let Some(tags) = item.tags {
            updates.push(("tags", tags));
        }

        if let Some(image_url) = item.image_url {
            updates.push(("image_url", image_url));
        }

        log::info!("Updating product with ID {}: {:?}", id, updates);

        let result =
            common_lib::sql_utils::update_by_id::<Product>(&self.mysql, "products", id, updates)
                .await;

        return match result {
            Ok(it) => Ok(it),
            Err(err) => Err(err),
        };
    }

    async fn delete(&self, id: u64) -> Result<(), Error> {
        log::info!("Deleting product with ID {}", id);

        common_lib::sql_utils::delete_by_id(&self.mysql, "products", id).await
    }

    async fn page(
        &self,
        filters: Vec<Filter>,
        pagination: PaginationParams,
    ) -> Result<PaginationResult<Product>, Error> {
        log::info!(
            "Fetching page of products with filters: {:?} and pagination: {:?}",
            filters,
            pagination
        );

        let result = common_lib::sql_utils::paginate::<Product>(
            &self.mysql,
            "products",
            filters,
            pagination,
        )
        .await;

        result
    }

    async fn list(&self, filters: Vec<Filter>) -> Result<Vec<Product>, Error> {
        log::info!("Fetching list of products with filters: {:?}", filters);
        let result = common_lib::sql_utils::list::<Product>(&self.mysql, "products", filters).await;
        return result;
    }

    async fn by_id(&self, id: u64) -> Result<Product, Error> {
        let result =
            common_lib::sql_utils::by_id_common::<Product>(&self.mysql, "products", id).await;
        result
    }
}
