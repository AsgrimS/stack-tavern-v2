use serde::Serialize;

use sqlx::FromRow;
use time::OffsetDateTime;

// use crate::db::get_connection_pool;

#[derive(FromRow, Serialize, Debug)]
pub struct Technology {
    pub id: i32,
    pub name: String,
    pub description: Option<String>,
    pub purpose: String,
    pub created_at: OffsetDateTime,
}
