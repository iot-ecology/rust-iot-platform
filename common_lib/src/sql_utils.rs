use chrono::{DateTime, NaiveDateTime, Utc};
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
    pub per_page: u64,
}

#[derive(Debug)]
pub struct PaginationResult<T> {
    pub items: Vec<T>,
    pub total: i64,
    pub page: u64,
    pub per_page: u64,
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
    let offset = (pagination.page - 1) * pagination.per_page;

    let mut where_clause = String::new();
    let mut bindings: Vec<String> = Vec::new();

    for filter in filters.iter() {
        let (condition, values) = filter.to_sql();
        where_clause.push_str(&format!(" AND {}", condition));
        bindings.extend(values);
    }

    let mut query = format!("SELECT * FROM {} WHERE 1=1", table_name);
    query.push_str(&where_clause);
    query.push_str(&format!(" LIMIT {} OFFSET {}", pagination.per_page, offset));

    // 构建计数查询
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

    let total_pages = ((total as f64) / (pagination.per_page as f64)).ceil() as u64;

    Ok(PaginationResult {
        items,
        total,
        page: pagination.page,
        per_page: pagination.per_page,
        total_pages,
    })
}

#[tokio::test]
async fn test_get_paginated() {
    let pool = MySqlPool::connect("mysql://root:root123%40@localhost/iot-copy")
        .await
        .unwrap();

    let pagination = PaginationParams {
        page: 1,
        per_page: 10,
    };

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
    for item in result.items {
        println!("User: {:?}", item);
    }
}
