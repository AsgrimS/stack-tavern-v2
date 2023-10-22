use super::STModel;
use crate::api::db::get_connection_pool;
use axum::async_trait;
use sqlx::{postgres::PgRow, Error, FromRow};

#[async_trait]
pub trait Get {
    /// Gets an item from the database by id.
    /// Returns a Result with a Boxed item or sqlx::Error.
    async fn get(id: &i32) -> Result<Box<Self>, Error>
    where
        Self: Send + Unpin + STModel + for<'r> FromRow<'r, PgRow>,
    {
        let pool = get_connection_pool().await;
        let query = format!("SELECT * FROM {} WHERE id = {}", Self::TABLE_NAME, id);
        let item: Self = sqlx::query_as(query.as_str()).fetch_one(pool).await?;

        Ok(Box::new(item))
    }
}

#[async_trait]
pub trait GetAll {
    /// Gets all items from the database.
    /// Returns a Result with a Vec of Boxed items or sqlx::Error.
    async fn get_all() -> Result<Vec<Box<Self>>, Error>
    where
        Self: Send + Unpin + STModel + for<'r> FromRow<'r, PgRow>,
    {
        let pool = get_connection_pool().await;
        let query = format!("SELECT * FROM {}", Self::TABLE_NAME);
        let items: Vec<Self> = sqlx::query_as(query.as_str()).fetch_all(pool).await?;

        Ok(items.into_iter().map(|i| Box::new(i)).collect())
    }
}

#[async_trait]
pub trait Delete {
    /// Deletes an item from the database by id.
    /// Returns a Result with the number of affected rows or sqlx::Error.
    async fn delete(id: &i32) -> Result<u64, Error>
    where
        Self: STModel,
    {
        let pool = get_connection_pool().await;
        let query = format!("DELETE FROM {} WHERE id = {}", Self::TABLE_NAME, id);
        let rows = sqlx::query(query.as_str()).execute(pool).await?;

        Ok(rows.rows_affected())
    }
}
