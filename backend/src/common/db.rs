use sqlx::{PgPool, PgPoolOptions};
use std::sync::OnceLock;
use std::time::Duration;

static DB_POOL: OnceLock<PgPool> = OnceLock::new();

pub fn get_pool() -> Option<&'static PgPool> {
    DB_POOL.get()
}

/// Initialize database connection pool with configurable settings
pub async fn create_pool_with_options(
    database_url: &str,
    max_connections: u32,
    min_connections: u32,
    connect_timeout: u64,
    idle_timeout: u64,
) -> Result<PgPool, sqlx::Error> {
    tracing::info!(
        "Initializing database connection pool: max_connections={}, min_connections={}, connect_timeout={}, idle_timeout={}",
        max_connections,
        min_connections,
        connect_timeout,
        idle_timeout
    );

    let pool = PgPoolOptions::new()
        .max_connections(max_connections)
        .min_connections(min_connections)
        .connect_timeout(Duration::from_secs(connect_timeout))
        .idle_timeout(Some(Duration::from_secs(idle_timeout)))
        .connect(database_url)
        .await?;

    DB_POOL.set(pool.clone()).ok();
    
    tracing::info!("Database connection pool initialized successfully");
    
    Ok(pool)
}

/// Create pool using Settings struct
pub async fn create_pool(settings: &crate::config::Settings) -> Result<PgPool, sqlx::Error> {
    let database_url = &settings.database.url;
    create_pool_with_options(
        database_url,
        settings.database.max_connections,
        settings.database.min_connections,
        settings.database.connect_timeout,
        settings.database.idle_timeout,
    )
    .await
}

/// Check database connection health
pub async fn check_health() -> Result<(), sqlx::Error> {
    if let Some(pool) = get_pool() {
        pool.acquire().await?;
        Ok(())
    } else {
        tracing::warn!("Database pool not initialized");
        Err(sqlx::Error::Database("Database pool not initialized".into()))
    }
}

/// Get pool stats for metrics
/// Note: SQLx pool stats require the raw method which returns PoolStats
pub fn get_pool_stats() -> Option<PoolStats> {
    // This is a placeholder implementation
    // SQLx 0.8 doesn't expose direct pool stats, but you can get them via
    // pool.raw().num_acquire() etc. when using the raw flag
    None
}

/// Pool statistics struct
#[derive(Debug, Clone, Default)]
pub struct PoolStats {
    pub connections: u64,
    pub available: u64,
    pub busy: u64,
    pub size: u64,
}