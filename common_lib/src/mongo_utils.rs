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

use crate::config::MongoConfig;
use futures_util::StreamExt;
use mongodb::bson::{Bson, Document};
use mongodb::options::{ClientOptions, ResolverConfig};
use mongodb::{bson, Client, Collection, Database};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::error::Error;

#[derive(Clone, Debug)]
pub struct MongoDBManager {
    pub client: Client,
    pub db: Database,
}

impl MongoDBManager {
    pub async fn new(config: MongoConfig) -> Result<Self, Box<dyn Error>> {
        let uri = format!(
            "mongodb://{}:{}@{}:{}/?maxPoolSize=20",
            config.username.unwrap(),
            config.password.unwrap(),
            config.host.unwrap(),
            config.port.unwrap(),
        );
        let mut client_options = ClientOptions::parse(uri).await?;
        // let credential = Credential::builder()
        //     .username(Some("your_username".to_string()))
        //     .password(Some("your_password".to_string()))
        //     .mechanism(Some(mongodb::options::AuthMechanism::ScramSha1))
        //     .build();

        // client_options.credential = Some(credential);

        let client = Client::with_options(client_options)?;
        let option = config.db.clone();
        let db = client.database(option.unwrap().as_ref());

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
        collection.insert_one(document).await?;
        Ok(())
    }

    pub async fn create_collection(&self, name: &str) -> Result<(), Box<dyn Error>> {
        let collections = self.db.list_collection_names().await?;

        if collections.contains(&name.to_string()) {
            info!("Collection '{}' already exists, skipping creation.", name);
            return Ok(()); // 如果集合已存在，则返回，不再创建
        }

        self.db.create_collection(name).await?;
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

        let mut cursor = collection.find(filter).await?;
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

use crate::rabbit_utils::RabbitMQ;
use log::info;
use std::sync::Arc;
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
