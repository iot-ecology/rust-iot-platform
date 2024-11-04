use crate::protocol_config::MongoConfig;
use futures_util::StreamExt;
use mongodb::bson::{Bson, Document};
use mongodb::options::{ClientOptions, ResolverConfig};
use mongodb::{bson, Client, Collection, Database};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

pub struct MongoDBManager {
    client: Client,
    db: Database,
}

impl MongoDBManager {
    pub async fn new(config: MongoConfig) -> Result<Self, Box<dyn Error>> {
        let mut client_options = ClientOptions::parse_with_resolver_config(
            &format!(
                "mongodb://{}:{}/",
                config.host.as_deref().unwrap_or("localhost"),
                config.port.unwrap_or(27017)
            ),
            ResolverConfig::cloudflare(),
        )
        .await?;

        if let (Some(username), Some(password)) = (config.username, config.password) {
            client_options.credential = Some(
                mongodb::options::Credential::builder()
                    .username(username)
                    .password(password)
                    .build(),
            );
        }

        let client = Client::with_options(client_options)?;
        let db = client.database(config.db.as_deref().unwrap_or("test"));

        Ok(MongoDBManager { client, db })
    }

    pub fn collection(&self, name: &str) -> Collection<HashMap<String, serde_json::Value>> {
        self.db.collection(name)
    }

    pub async fn insert_document(
        &self,
        collection_name: &str,
        document: HashMap<String, serde_json::Value>,
    ) -> Result<(), Box<dyn Error>> {
        let collection = self.collection(collection_name);
        collection.insert_one(document, None).await?;
        Ok(())
    }

    pub async fn create_collection(&self, name: &str) -> Result<(), Box<dyn Error>> {
        self.db.create_collection(name, None).await?;
        Ok(())
    }

    pub async fn find_document(
        &self,
        collection_name: &str,
        filter: Option<HashMap<String, serde_json::Value>>,
    ) -> Result<Vec<HashMap<String, String>>, Box<dyn Error>> {
        let collection = self.collection(collection_name);

        let filter = filter
            .map(|f| {
                f.into_iter()
                    .map(|(k, v)| (k, Bson::try_from(v).unwrap_or(Bson::Null)))
                    .collect::<Document>()
            })
            .unwrap_or_default();

        let mut cursor = collection.find(filter, None).await?;
        let mut results = Vec::new();

        while let Some(doc) = cursor.next().await {
            let doc = doc?;

            let string_map: HashMap<String, String> =
                doc.into_iter().map(|(k, v)| (k, v.to_string())).collect();
            results.push(string_map);
        }

        Ok(results)
    }
}

use std::sync::Arc;

use crate::rabbit_utils::RabbitMQ;
use tokio::sync::{Mutex, MutexGuard, OnceCell};

static DB_MANAGER: OnceCell<Arc<Mutex<MongoDBManager>>> = OnceCell::const_new();

pub async fn init_mongo(config: MongoConfig) -> Result<(), Box<dyn Error>> {
    let db_manager = MongoDBManager::new(config).await?;
    DB_MANAGER
        .set(Arc::new(Mutex::new(db_manager)))
        .map_err(|_| "DB Manager is already initialized".into())
}

pub async fn get_mongo() -> Result<MutexGuard<'static, MongoDBManager>, Box<dyn Error>> {
    let instance = DB_MANAGER
        .get()
        .ok_or("DB Manager has not been initialized")?;
    Ok(instance.lock().await)
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_mongodb_operations() -> Result<(), Box<dyn std::error::Error>> {
        let config = MongoConfig {
            host: Some("localhost".to_string()),
            port: Some(27017),
            username: Some("admin".to_string()),
            password: Some("admin".to_string()),
            db: Some("test_db".to_string()),
            collection: Some("test_collection".to_string()),
            waring_collection: None,
            script_waring_collection: None,
        };
        init_mongo(config).await?;

        let db_manager = get_mongo().await?;
        // 创建集合
        db_manager.create_collection("test_collection").await?;

        // 准备要插入的文档
        let mut document = HashMap::new();
        document.insert("name".to_string(), serde_json::json!("John Doe"));
        document.insert("age".to_string(), serde_json::json!(30));

        // 插入文档
        db_manager
            .insert_document("test_collection", document)
            .await?;

        // 准备过滤器查询
        let filter = Some(HashMap::from([(
            "name".to_string(),
            serde_json::json!("John Doe"),
        )]));

        // 执行查询
        let documents = db_manager.find_document("test_collection", filter).await?;

        // 打印查询结果
        for doc in documents {
            println!("{:?}", doc);
        }
        Ok(())
    }
}
