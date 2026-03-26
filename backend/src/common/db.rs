use sqlx::{PgPool, Row};
use std::env;
use std::sync::OnceLock;

static DB_POOL: OnceLock<PgPool> = OnceLock::new();

pub fn get_pool() -> Option<&'static PgPool> {
    DB_POOL.get()
}

pub async fn create_pool(settings: &crate::config::Settings) -> Result<PgPool, sqlx::Error> {
    let database_url = &settings.database.url;
    let pool = PgPool::connect(database_url).await?;
    DB_POOL.set(pool.clone()).ok();
    Ok(pool)
}