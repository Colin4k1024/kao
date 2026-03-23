use sqlx::postgres::PgPoolOptions;
use std::time::Duration;

pub async fn create_db_pool() -> Result<sqlx::PgPool, sqlx::Error> {
    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://user:password@localhost:5432/ai_coding".to_string());

    PgPoolOptions::new()
        .max_connections(5)
        .acquire_timeout(Duration::from_secs(3))
        .connect(&database_url)
        .await
}

// 数据库连接池类型别名
pub type DbPool = sqlx::PgPool;
