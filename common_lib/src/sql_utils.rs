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

use anyhow::Context;
use chrono::{DateTime, NaiveDateTime, Utc};
use log::{debug, info};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sqlx::{FromRow, MySql, MySqlPool, Pool, Row};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    pub id: i64,
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub status: Option<String>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime",
        default
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime",
        default
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime",
        default
    )]
    pub deleted_at: Option<chrono::NaiveDateTime>,
}
impl FromRow<'_, sqlx::mysql::MySqlRow> for User {
    fn from_row(row: &'_ sqlx::mysql::MySqlRow) -> Result<Self, sqlx::Error> {
        Ok(User {
            id: row.try_get("id")?,
            username: row.try_get("username")?,
            password: row.try_get("password")?,
            email: row.try_get("email")?,
            status: row.try_get("status")?,
            created_at: row.try_get("created_at")?,
            updated_at: row.try_get("updated_at")?,
            deleted_at: row.try_get("deleted_at")?,
        })
    }
}
fn serialize_naive_datetime<S>(
    naive: &Option<NaiveDateTime>,
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    match naive {
        Some(ndt) => {
            // 将 NaiveDateTime 转为 DateTime<Utc> 并序列化
            let dt = DateTime::<Utc>::from_utc(*ndt, Utc);
            dt.serialize(serializer)
        }
        None => serializer.serialize_none(),
    }
}

fn deserialize_naive_datetime<'de, D>(deserializer: D) -> Result<Option<NaiveDateTime>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let dt: Option<DateTime<Utc>> = Option::deserialize(deserializer)?;
    Ok(dt.map(|d| d.naive_utc()))
}

#[derive(Debug)]
pub struct PaginationParams {
    pub page: i64,
    pub size: i64,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PaginationResult<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: i64,
    pub size: i64,
    pub total_pages: i64,
}

#[derive(Debug)]
pub enum FilterOperation {
    Equal,
    NotEqual,
    LeftLike,
    RightLike,
    AllLike,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Between,
}

#[derive(Debug)]
pub struct FilterInfo {
    pub field: String,
    pub value: String,
    pub operation: FilterOperation,
    pub value2: Option<String>,
}

impl FilterInfo {
    fn to_sql(&self) -> (String, Vec<String>) {
        match self.operation {
            FilterOperation::Equal => (format!("{} = ?", self.field), vec![self.value.clone()]),
            FilterOperation::NotEqual => (format!("{} != ?", self.field), vec![self.value.clone()]),
            FilterOperation::LeftLike => (
                format!("{} LIKE ?", self.field),
                vec![format!("%{}", self.value)],
            ),
            FilterOperation::AllLike => (
                format!("{} LIKE ?", self.field),
                vec![format!("%{}%", self.value)],
            ),
            FilterOperation::RightLike => (
                format!("{} LIKE ?", self.field),
                vec![format!("{}%", self.value)],
            ),
            FilterOperation::GreaterThan => {
                (format!("{} > ?", self.field), vec![self.value.clone()])
            }
            FilterOperation::LessThan => (format!("{} < ?", self.field), vec![self.value.clone()]),
            FilterOperation::GreaterThanOrEqual => {
                (format!("{} >= ?", self.field), vec![self.value.clone()])
            }
            FilterOperation::LessThanOrEqual => {
                (format!("{} <= ?", self.field), vec![self.value.clone()])
            }
            FilterOperation::Between => {
                if let Some(ref value2) = self.value2 {
                    (
                        format!("{} BETWEEN ? AND ?", self.field),
                        vec![self.value.clone(), value2.clone()],
                    )
                } else {
                    panic!("`value2` must be provided for `Between` operation");
                }
            }
        }
    }
}

pub async fn paginate<T>(
    pool: &Pool<MySql>,
    table_name: &str,
    filters: Vec<FilterInfo>,
    pagination: PaginationParams,
) -> Result<PaginationResult<T>, Error>
where
    T: for<'r> FromRow<'r, sqlx::mysql::MySqlRow> + Send + Unpin + Debug,
{
    let offset = (pagination.page - 1) * pagination.size;

    // 构建 WHERE 子句和绑定值
    let mut where_clause = String::new();
    let mut bindings: Vec<String> = Vec::new();

    for filter in filters.iter() {
        if !filter.value.as_str().eq("0") || filter.value.as_str().eq("") {
            let (condition, values) = filter.to_sql();
            where_clause.push_str(&format!(" AND {}", condition));
            bindings.extend(values);
        }
    }

    // 主查询语句
    let query = format!(
        "SELECT * FROM {} WHERE 1=1{} LIMIT {} OFFSET {}",
        table_name, where_clause, pagination.size, offset
    );

    // 计数查询语句
    let count_query = format!(
        "SELECT COUNT(1) FROM {} WHERE 1=1{}",
        table_name, where_clause
    );

    info!("sql = {:?}", query);
    info!("where = {:?}", bindings);

    // 执行主查询
    let mut query_builder = sqlx::query_as::<_, T>(&query);
    for value in &bindings {
        query_builder = query_builder.bind(value);
    }
    let items = match query_builder.fetch_all(pool).await.with_context(|| {
        format!("Failed to fetch paginated records from '{}'", table_name)
    }) {
        Ok(results) => {
            log::info!(
                "Successfully fetched {} records from table '{}' with filters {:?}",
                results.len(),
                table_name,
                filters
            );
            results
        }
        Err(e) => {
            log::error!(
                "Database query error: table='{}', filters={:?}, query='{}', error={}",
                table_name,
                filters,
                query,
                e
            );
            return Err(e);
        }
    };

    // 执行计数查询
    let mut count_query_builder = sqlx::query_scalar(&count_query);
    for value in &bindings {
        count_query_builder = count_query_builder.bind(value);
    }
    let total: i64 = match count_query_builder.fetch_one(pool).await.with_context(|| {
        format!("Failed to fetch record count for '{}'", table_name)
    }) {
        Ok(total) => {
            log::info!("Successfully fetched record count for table '{}'", table_name);
            total
        }
        Err(e) => {
            log::error!(
                "Database query error: table='{}', filters={:?}, query='{}', error={}",
                table_name,
                filters,
                count_query,
                e
            );
            return Err(e);
        }
    };

    // 计算总页数
    let total_pages = ((total as f64) / (pagination.size as f64)).ceil() as i64;

    Ok(PaginationResult {
        data: items,
        total,
        page: pagination.page,
        size: pagination.size,
        total_pages,
    })
}
pub async fn list<T>(
    pool: &Pool<MySql>,
    table_name: &str,
    filters: Vec<FilterInfo>,
) -> Result<Vec<T>, Error>
where
    T: for<'r> FromRow<'r, sqlx::mysql::MySqlRow> + Send + Unpin + Debug,
{
    // 构建 WHERE 子句和绑定值
    let mut where_clause = String::new();
    let mut bindings: Vec<String> = Vec::new();

    for filter in filters.iter() {
        if !filter.value.as_str().eq("0") || filter.value.as_str().eq("") {
            let (condition, values) = filter.to_sql();
            where_clause.push_str(&format!(" AND {}", condition));
            bindings.extend(values);
        }
    }

    // 完成查询语句
    let mut query = format!("SELECT * FROM {} WHERE 1=1 {}", table_name, where_clause);
    println!("Executing query: {}", query);

    // 创建查询构建器并绑定参数
    let mut query_builder = sqlx::query_as::<_, T>(&query);
    for value in bindings.iter() {
        query_builder = query_builder.bind(value);
    }

    // 执行查询并返回结果
    match query_builder.fetch_all(pool).await {
        Ok(results) => {
            log::info!(
                "Successfully fetched {} records from table '{}' with filters {:?}",
                results.len(),
                table_name,
                filters
            );
            Ok(results)
        }
        Err(e) => {
            log::error!(
                "Database query error: table='{}', filters={:?}, query='{}', error={}",
                table_name,
                filters,
                query,
                e
            );
            Err(e.into())
        }
    }
}
pub async fn by_id_common<T>(pool: &Pool<MySql>, table_name: &str, id: i64) -> Result<T, Error>
where
    T: for<'r> FromRow<'r, sqlx::mysql::MySqlRow> + Send + Unpin + Debug,
{
    // 构建 SQL 查询
    let query = format!("SELECT * FROM {} WHERE id = ?", table_name);
    println!("Executing query: {}", query);

    // 创建查询构建器并执行查询
    let result = match sqlx::query_as::<_, T>(&query)
        .bind(id.clone())
        .fetch_one(pool)
        .await
        .with_context(|| {
            format!(
                "Failed to fetch record from table '{}' with id '{}'",
                table_name, id
            )
        }) {
        Ok(result) => {
            log::info!("Successfully fetched record from table '{}' with id '{}'", table_name, id);
            result
        }
        Err(e) => {
            log::error!(
                "Database query error: table='{}', id={}, query='{}', error={}",
                table_name,
                id,
                query,
                e
            );
            return Err(e);
        }
    };

    Ok(result)
}

pub async fn delete_by_id<T>(pool: &Pool<MySql>, table_name: &str, id: i64) -> Result<T, Error>
where
    T: for<'r> FromRow<'r, sqlx::mysql::MySqlRow> + Send + Unpin + Debug,
{
    // 构建逻辑删除 SQL 查询
    let query = format!("UPDATE {} SET deleted_at = NOW() WHERE id = ?", table_name);
    println!("Delete query: {}", query);

    let result = by_id_common(pool, table_name, id).await;
    // 执行更新操作并捕获可能的错误
    let affected_rows = match sqlx::query(&query)
        .bind(id)
        .execute(pool)
        .await
        .with_context(|| {
            format!(
                "Failed to execute delete on table '{}' with id {}",
                table_name, id
            )
        }) {
        Ok(affected_rows) => {
            log::info!("Successfully executed delete on table '{}' with id '{}'", table_name, id);
            affected_rows
        }
        Err(e) => {
            log::error!(
                "Database query error: table='{}', id={}, query='{}', error={}",
                table_name,
                id,
                query,
                e
            );
            return Err(e);
        }
    };

    // 检查是否有行受影响
    if affected_rows.rows_affected() == 0 {
        return Err(anyhow::anyhow!("No rows affected for id {}", id));
    }
    result
}

use anyhow::{Error, Result};

pub async fn update_by_id<T>(
    pool: &Pool<MySql>,
    table_name: &str,
    id: i64,
    updates: Vec<(&str, String)>,
) -> Result<T, Error>
where
    T: for<'r> FromRow<'r, sqlx::mysql::MySqlRow> + Send + Unpin + Debug,
{
    let mut set_clause = String::new();
    let mut bindings: Vec<String> = Vec::new();

    // 构建 SET 子句和绑定值
    for (field, value) in updates.iter() {
        if !set_clause.is_empty() {
            set_clause.push_str(", ");
        }
        set_clause.push_str(&format!("{} = ?", field));
        bindings.push(value.clone());
    }

    // 更新 updated_at 字段
    set_clause.push_str(", updated_at = NOW()");

    // 构建更新 SQL 查询
    let query = format!("UPDATE {} SET {} WHERE id = ?", table_name, set_clause);
    println!("Update query: {}", query);
    println!("Bindings: {:?}", bindings);

    // 构建查询并绑定值
    let mut query_builder = sqlx::query(&query);
    for value in bindings.iter() {
        query_builder = query_builder.bind(value);
    }
    query_builder = query_builder.bind(id);

    // 执行更新操作并捕获可能的错误
    let affected_rows = match query_builder.execute(pool).await.with_context(|| {
        format!(
            "Failed to execute update on table '{}' with id {}",
            table_name, id
        )
    }) {
        Ok(affected_rows) => {
            log::info!("Successfully executed update on table '{}' with id '{}'", table_name, id);
            affected_rows
        }
        Err(e) => {
            log::error!(
                "Database query error: table='{}', id={}, query='{}', error={}",
                table_name,
                id,
                query,
                e
            );
            return Err(e);
        }
    };

    if affected_rows.rows_affected() == 0 {
        return Err(anyhow::anyhow!("No rows affected for id {}", id));
    }

    // 构建查询以返回更新后的记录
    let select_query = format!("SELECT * FROM {} WHERE id = ?", table_name);
    let updated_record = match sqlx::query_as::<_, T>(&select_query)
        .bind(id)
        .fetch_one(pool)
        .await
        .with_context(|| {
            format!(
                "Failed to fetch updated record from table '{}' with id {}",
                table_name, id
            )
        }) {
        Ok(updated_record) => {
            log::info!("Successfully fetched updated record from table '{}' with id '{}'", table_name, id);
            updated_record
        }
        Err(e) => {
            log::error!(
                "Database query error: table='{}', id={}, query='{}', error={}",
                table_name,
                id,
                select_query,
                e
            );
            return Err(e);
        }
    };

    Ok(updated_record)
}

#[async_trait::async_trait]
pub trait CrudOperations<T>
where
    T: for<'r> FromRow<'r, sqlx::mysql::MySqlRow> + Send + Unpin + Debug,
{
    async fn create(&self, item: T) -> Result<T, Error>;
    async fn update(&self, id: i64, item: T) -> Result<T, Error>;
    async fn delete(&self, id: i64) -> Result<T, Error>;
    async fn page(
        &self,
        filters: Vec<FilterInfo>,
        pagination: PaginationParams,
    ) -> Result<PaginationResult<T>, Error>;
    async fn list(&self, filters: Vec<FilterInfo>) -> Result<Vec<T>, Error>;

    async fn by_id(&self, id: i64) -> Result<T, Error>;
}

pub async fn insert<T>(
    pool: &Pool<MySql>,
    table_name: &str,
    updates: Vec<(&str, String)>,
) -> anyhow::Result<T, anyhow::Error>
where
    T: for<'r> FromRow<'r, sqlx::mysql::MySqlRow> + Send + Unpin + Debug,
{
    let mut columns = String::new();
    let mut values = String::new();
    let mut bindings: Vec<String> = Vec::new();

    // 遍历 updates 列表，构建列名和值的字符串
    for (i, (field, value)) in updates.iter().enumerate() {
        if i > 0 {
            columns.push_str(", ");
            values.push_str(", ");
        }
        columns.push_str(field);
        values.push_str("?");
        bindings.push(value.clone());
    }

    columns.push_str(", created_at");
    values.push_str(", NOW()");

    let query = format!(
        "INSERT INTO {} ({}) VALUES ({})",
        table_name, columns, values
    );

    println!("Insert query: {}", query);
    println!("Bindings: {:?}", bindings);

    // 构建查询并绑定值
    let mut query_builder = sqlx::query(&query);
    for value in bindings.iter() {
        query_builder = query_builder.bind(value);
    }

    // 执行插入操作并捕获可能的数据库错误
    match query_builder.execute(pool).await.with_context(|| {
        format!("Failed to execute insert into table '{}'", table_name)
    }) {
        Ok(_) => {
            log::info!("Successfully executed insert into table '{}'", table_name);
        }
        Err(e) => {
            log::error!(
                "Database query error: table='{}', query='{}', error={}",
                table_name,
                query,
                e
            );
            return Err(e);
        }
    };

    // 插入后获取新插入的记录
    let select_query = format!("SELECT * FROM {} ORDER BY id DESC LIMIT 1", table_name);
    let inserted_record = match sqlx::query_as::<_, T>(&select_query)
        .fetch_one(pool)
        .await
        .with_context(|| {
            format!(
                "Failed to fetch newly inserted record from table '{}'",
                table_name
            )
        }) {
        Ok(inserted_record) => {
            log::info!("Successfully fetched newly inserted record from table '{}'", table_name);
            inserted_record
        }
        Err(e) => {
            log::error!(
                "Database query error: table='{}', query='{}', error={}",
                table_name,
                select_query,
                e
            );
            return Err(e);
        }
    };

    Ok(inserted_record)
}
#[tokio::test]
async fn test_get_paginated() {
    let pool = MySqlPool::connect("mysql://root:root123%40@localhost/iot-copy")
        .await
        .unwrap();

    let pagination = PaginationParams { page: 1, size: 10 };

    let filters = vec![
        FilterInfo {
            field: "username".to_string(),
            value: "1".to_string(),
            operation: FilterOperation::LeftLike,
            value2: None,
        },
        FilterInfo {
            field: "id".to_string(),
            value: "1".to_string(),
            operation: FilterOperation::Between,
            value2: Some("300".to_string()),
        },
    ];

    let result = paginate::<User>(&pool, "users", filters, pagination)
        .await
        .unwrap();

    println!(
        "Total records: {}, Pages: {}",
        result.total, result.total_pages
    );
    for item in result.data {
        println!("User: {:?}", item);
    }
    let updates = vec![("username", "new_username".to_string())];
    let x = update_by_id::<User>(&pool, "users", 1, updates)
        .await
        .unwrap();
    println!("{:?}", x);
    let updates = vec![
        ("username", "new_user".to_string()),
        ("password", "new_pass".to_string()),
        ("email", "new_email@example.com".to_string()),
    ];
    let user = insert::<User>(&pool, "users", updates).await.unwrap();
    println!("User: {:?}", user);
}
