use sqlx::postgres::{PgPool, PgPoolOptions};
use std::env;

pub async fn initialize_pool() -> PgPool {
    let url = env::var("DATABASE_URL").expect("database URL to be in .env");
    PgPoolOptions::new()
        .max_connections(5)
        .connect_lazy(url.as_str())
        .unwrap()
}
