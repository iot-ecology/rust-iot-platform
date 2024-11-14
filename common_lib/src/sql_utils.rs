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
) -> Result<PaginationResult<T>, sqlx::Error>
where
    T: for<'r> FromRow<'r, sqlx::mysql::MySqlRow> + Send + Unpin + Debug,
{
    let offset = (pagination.page - 1) * pagination.size;

    let mut where_clause = String::new();
    let mut bindings: Vec<String> = Vec::new();

    for filter in filters.iter() {
        let (condition, values) = filter.to_sql();
        where_clause.push_str(&format!(" AND {}", condition));
        bindings.extend(values);
    }

    let mut query = format!("SELECT * FROM {} WHERE 1=1", table_name);
    query.push_str(&where_clause);
    query.push_str(&format!(" LIMIT {} OFFSET {}", pagination.size, offset));

    let mut count_query = format!("SELECT COUNT(1) FROM {} WHERE 1=1", table_name);
    count_query.push_str(&where_clause);

    let mut query_builder = sqlx::query_as::<_, T>(&query);
    for value in bindings.iter() {
        query_builder = query_builder.bind(value);
    }
    let items = query_builder.fetch_all(pool).await?;

    let mut count_query_builder = sqlx::query_as(&count_query);
    for value in bindings.iter() {
        count_query_builder = count_query_builder.bind(value);
    }
    let (total,): (i64,) = count_query_builder.fetch_one(pool).await?;

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
) -> Result<Vec<T>, sqlx::Error>
where
    T: for<'r> FromRow<'r, sqlx::mysql::MySqlRow> + Send + Unpin + Debug,
{
    let mut where_clause = String::new();
    let mut bindings: Vec<String> = Vec::new();

    for filter in filters.iter() {
        let (condition, values) = filter.to_sql();
        where_clause.push_str(&format!(" AND {}", condition));
        bindings.extend(values);
    }

    let mut query = format!("SELECT * FROM {} WHERE 1=1", table_name);
    query.push_str(&where_clause);

    let mut query_builder = sqlx::query_as::<_, T>(&query);
    for value in bindings.iter() {
        query_builder = query_builder.bind(value);
    }
    let items = query_builder.fetch_all(pool).await?;

    Ok(items)
}

pub async fn by_id<T>(pool: &Pool<MySql>, table_name: &str, id: String) -> Result<T, sqlx::Error>
where
    T: for<'r> FromRow<'r, sqlx::mysql::MySqlRow> + Send + Unpin + Debug,
{
    let mut query = format!("SELECT * FROM {} WHERE 1=1 and id = ?", table_name);

    let mut query_builder = sqlx::query_as::<_, T>(&query);
    let result = query_builder.bind(id).fetch_one(pool).await;

    result
}

pub async fn delete_by_id(
    pool: &Pool<MySql>,
    table_name: &str,
    id: u64,
) -> Result<(), sqlx::Error> {
    let query = format!("UPDATE {} SET deleted_at = NOW() WHERE id = ?", table_name);

    sqlx::query(&query).bind(id).execute(pool).await?;

    Ok(())
}

pub async fn update_by_id<T>(
    pool: &Pool<MySql>,
    table_name: &str,
    id: u64,
    updates: Vec<(&str, String)>,
) -> Result<T, sqlx::Error>
where
    T: for<'r> FromRow<'r, sqlx::mysql::MySqlRow> + Send + Unpin + Debug,
{
    let mut set_clause = String::new();
    let mut bindings: Vec<String> = Vec::new();

    for (field, value) in updates.iter() {
        if !set_clause.is_empty() {
            set_clause.push_str(", ");
        }
        set_clause.push_str(&format!("{} = ?", field));
        bindings.push(value.clone());
    }

    set_clause.push_str(", updated_at = NOW()");

    let query = format!("UPDATE {} SET {} WHERE id = ?", table_name, set_clause);
    println!("Update query: {}", query);
    println!("Bindings: {:?}", bindings);

    let mut query_builder = sqlx::query(&query);
    for value in bindings.iter() {
        println!("{}", value);
        query_builder = query_builder.bind(value);
    }
    query_builder = query_builder.bind(id);

    let affected_rows = query_builder.execute(pool).await?;

    if affected_rows.rows_affected() == 0 {
        return Err(sqlx::Error::RowNotFound);
    }

    let select_query = format!("SELECT * FROM {} WHERE id = ?", table_name);
    let updated_record = sqlx::query_as::<_, T>(&select_query)
        .bind(id)
        .fetch_one(pool)
        .await?;

    Ok(updated_record)
}

#[async_trait::async_trait]
pub trait CrudOperations<T> {
    async fn create(&self, pool: &Pool<MySql>, item: T) -> Result<(), sqlx::Error>;
    async fn update(&self, pool: &Pool<MySql>, id: u64, item: T) -> Result<(), sqlx::Error>;
    async fn delete(&self, pool: &Pool<MySql>, id: u64) -> Result<(), sqlx::Error>;
    async fn page(
        &self,
        pool: &Pool<MySql>,
        filters: Vec<Filter>,
        pagination: PaginationParams,
    ) -> Result<PaginationResult<T>, sqlx::Error>;
    async fn list(&self, pool: &Pool<MySql>, filters: Vec<Filter>) -> Result<Vec<T>, sqlx::Error>;
}

pub async fn insert<T>(
    pool: &Pool<MySql>,
    table_name: &str,
    updates: Vec<(&str, String)>,
) -> Result<T, sqlx::Error>
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

    // 执行插入操作
    let mut query_builder = sqlx::query(&query);
    for value in bindings.iter() {
        query_builder = query_builder.bind(value);
    }

    // 执行插入
    query_builder.execute(pool).await?;

    // 插入后获取新插入的记录
    let select_query = format!("SELECT * FROM {} ORDER BY id DESC LIMIT 1", table_name);
    let inserted_record = sqlx::query_as::<_, T>(&select_query)
        .fetch_one(pool)
        .await?;

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
