use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;
use tokio::sync::OnceCell;

pub async fn initialize_pool() -> PgPool {
    let url = env::var("DATABASE_URL").expect("database URL to be in .env");
    PgPoolOptions::new()
        .max_connections(5)
        .connect_lazy(url.as_str())
        .unwrap()
}

static POOL: OnceCell<PgPool> = OnceCell::const_new();

pub async fn get_connection_pool<'a>() -> &'a PgPool {
    POOL.get_or_init(initialize_pool).await
}
