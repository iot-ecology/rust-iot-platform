use anyhow::Context;
use chrono::{DateTime, NaiveDateTime, Utc};
use log::debug;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use sqlx::{FromRow, MySql, MySqlPool, Pool, Row};
use std::fmt::Debug;

#[derive(Serialize, Deserialize, Debug)]
struct User {
    pub id: u64,
    pub username: Option<String>,
    pub password: Option<String>,
    pub email: Option<String>,
    pub status: Option<String>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub created_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
    )]
    pub updated_at: Option<chrono::NaiveDateTime>,
    #[serde(
        serialize_with = "serialize_naive_datetime",
        deserialize_with = "deserialize_naive_datetime"
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
    pub page: u64,
    pub size: u64,
}

#[derive(Debug)]
pub struct PaginationResult<T> {
    pub data: Vec<T>,
    pub total: i64,
    pub page: u64,
    pub size: u64,
    pub total_pages: u64,
}

#[derive(Debug)]
pub enum FilterOperation {
    Equal,
    NotEqual,
    LeftLike,
    RightLike,
    GreaterThan,
    LessThan,
    GreaterThanOrEqual,
    LessThanOrEqual,
    Between,
}

#[derive(Debug)]
pub struct Filter {
    pub field: String,
    pub value: String,
    pub operation: FilterOperation,
    pub value2: Option<String>,
}

impl Filter {
    fn to_sql(&self) -> (String, Vec<String>) {
        match self.operation {
            FilterOperation::Equal => (format!("{} = ?", self.field), vec![self.value.clone()]),
            FilterOperation::NotEqual => (format!("{} != ?", self.field), vec![self.value.clone()]),
            FilterOperation::LeftLike => (
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
    filters: Vec<Filter>,
    pagination: PaginationParams,
) -> Result<PaginationResult<T>, Error>
where
    T: for<'r> FromRow<'r, sqlx::mysql::MySqlRow> + Send + Unpin + Debug,
{
    let offset = (pagination.page - 1) * pagination.size;

    // 构建 WHERE 子句和绑定值
    let mut where_clause = String::new();
    let mut bindings: Vec<String> = Vec::new();

    for filter in &filters {
        let (condition, values) = filter.to_sql();
        where_clause.push_str(&format!(" AND {}", condition));
        bindings.extend(values);
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

    // 执行主查询
    let mut query_builder = sqlx::query_as::<_, T>(&query);
    for value in &bindings {
        query_builder = query_builder.bind(value);
    }
    let items = query_builder
        .fetch_all(pool)
        .await
        .with_context(|| format!("Failed to fetch paginated records from '{}'", table_name))?;

    // 执行计数查询
    let mut count_query_builder = sqlx::query_scalar(&count_query);
    for value in &bindings {
        count_query_builder = count_query_builder.bind(value);
    }
    let total: i64 = count_query_builder
        .fetch_one(pool)
        .await
        .with_context(|| format!("Failed to fetch record count for '{}'", table_name))?;

    // 计算总页数
    let total_pages = ((total as f64) / (pagination.size as f64)).ceil() as u64;

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
    filters: Vec<Filter>,
) -> Result<Vec<T>, Error>
where
    T: for<'r> FromRow<'r, sqlx::mysql::MySqlRow> + Send + Unpin + Debug,
{
    // 构建 WHERE 子句和绑定值
    let mut where_clause = String::new();
    let mut bindings: Vec<String> = Vec::new();

    for filter in filters.iter() {
        let (condition, values) = filter.to_sql();
        where_clause.push_str(&format!(" AND {}", condition));
        bindings.extend(values);
    }

    // 完成查询语句
    let mut query = format!("SELECT * FROM {} WHERE 1=1{}", table_name, where_clause);
    println!("Executing query: {}", query);

    // 创建查询构建器并绑定参数
    let mut query_builder = sqlx::query_as::<_, T>(&query);
    for value in bindings.iter() {
        query_builder = query_builder.bind(value);
    }

    // 执行查询并返回结果
    let items = query_builder.fetch_all(pool).await.with_context(|| {
        format!(
            "Failed to fetch records from table '{}' with filters {:?}",
            table_name, filters
        )
    })?;

    Ok(items)
}
pub async fn by_id_common<T>(pool: &Pool<MySql>, table_name: &str, id: u64) -> Result<T, Error>
where
    T: for<'r> FromRow<'r, sqlx::mysql::MySqlRow> + Send + Unpin + Debug,
{
    // 构建 SQL 查询
    let query = format!("SELECT * FROM {} WHERE id = ?", table_name);
    println!("Executing query: {}", query);

    // 创建查询构建器并执行查询
    let result = sqlx::query_as::<_, T>(&query)
        .bind(id.clone())
        .fetch_one(pool)
        .await
        .with_context(|| {
            format!(
                "Failed to fetch record from table '{}' with id '{}'",
                table_name, id
            )
        })?;

    Ok(result)
}

pub async fn delete_by_id(pool: &Pool<MySql>, table_name: &str, id: u64) -> Result<(), Error> {
    // 构建逻辑删除 SQL 查询
    let query = format!("UPDATE {} SET deleted_at = NOW() WHERE id = ?", table_name);
    println!("Delete query: {}", query);

    // 执行更新操作并捕获可能的错误
    let affected_rows = sqlx::query(&query)
        .bind(id)
        .execute(pool)
        .await
        .with_context(|| {
            format!(
                "Failed to execute delete on table '{}' with id {}",
                table_name, id
            )
        })?;

    // 检查是否有行受影响
    if affected_rows.rows_affected() == 0 {
        return Err(anyhow::anyhow!("No rows affected for id {}", id));
    }

    Ok(())
}

use anyhow::{Error, Result};

pub async fn update_by_id<T>(
    pool: &Pool<MySql>,
    table_name: &str,
    id: u64,
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
    let affected_rows = query_builder.execute(pool).await.with_context(|| {
        format!(
            "Failed to execute update on table '{}' with id {}",
            table_name, id
        )
    })?;

    if affected_rows.rows_affected() == 0 {
        return Err(anyhow::anyhow!("No rows affected for id {}", id));
    }

    // 构建查询以返回更新后的记录
    let select_query = format!("SELECT * FROM {} WHERE id = ?", table_name);
    let updated_record = sqlx::query_as::<_, T>(&select_query)
        .bind(id)
        .fetch_one(pool)
        .await
        .with_context(|| {
            format!(
                "Failed to fetch updated record from table '{}' with id {}",
                table_name, id
            )
        })?;

    Ok(updated_record)
}

#[async_trait::async_trait]
pub trait CrudOperations<T>
where
    T: for<'r> FromRow<'r, sqlx::mysql::MySqlRow> + Send + Unpin + Debug,
{
    async fn create(&self, item: T) -> Result<T, Error>;
    async fn update(&self, id: u64, item: T) -> Result<T, Error>;
    async fn delete(&self, id: u64) -> Result<(), Error>;
    async fn page(
        &self,
        filters: Vec<Filter>,
        pagination: PaginationParams,
    ) -> Result<PaginationResult<T>, Error>;
    async fn list(&self, filters: Vec<Filter>) -> Result<Vec<T>, Error>;

    async fn by_id(&self, id: u64) -> Result<T, Error>;
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
    query_builder
        .execute(pool)
        .await
        .with_context(|| format!("Failed to execute insert into table '{}'", table_name))?;

    // 插入后获取新插入的记录
    let select_query = format!("SELECT * FROM {} ORDER BY id DESC LIMIT 1", table_name);
    let inserted_record = sqlx::query_as::<_, T>(&select_query)
        .fetch_one(pool)
        .await
        .with_context(|| {
            format!(
                "Failed to fetch newly inserted record from table '{}'",
                table_name
            )
        })?;

    Ok(inserted_record)
}
#[tokio::test]
async fn test_get_paginated() {
    let pool = MySqlPool::connect("mysql://root:root123%40@localhost/iot-copy")
        .await
        .unwrap();

    let pagination = PaginationParams { page: 1, size: 10 };

    let filters = vec![
        Filter {
            field: "username".to_string(),
            value: "1".to_string(),
            operation: FilterOperation::LeftLike,
            value2: None,
        },
        Filter {
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
