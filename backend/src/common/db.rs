use sqlx::{PgPool, Row};
use std::env;

pub async fn create_db_pool(database_url: &str) -> Result<PgPool, sqlx::Error> {
    PgPool::connect(database_url).await
}