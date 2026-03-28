use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

// Database pool variable for global access
static mut DB_POOL: Option<PgPool> = None;

#[allow(static_mut_refs)]
pub fn get_pool() -> Option<&'static PgPool> {
    unsafe { DB_POOL.as_ref() }
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
        "Initializing database connection pool: max_connections={},
         min_connections={}, connect_timeout={}, idle_timeout={}",
        max_connections, min_connections, connect_timeout, idle_timeout
    );

    let pool_options = PgPoolOptions::new()
        .max_connections(max_connections)
        .min_connections(min_connections)
        .acquire_timeout(std::time::Duration::from_secs(connect_timeout))
        .idle_timeout(std::time::Duration::from_secs(idle_timeout));

    let pool = pool_options.connect(database_url).await?;

    // Store pool globally for backward compatibility with route handlers
    unsafe {
        DB_POOL = Some(pool.clone());
    }

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
        // In production, return a more specific error
        Err(sqlx::Error::PoolTimedOut)
    }
}

/// Get pool stats for metrics
pub fn get_pool_stats() -> Option<PoolStats> {
    // This is a placeholder implementation
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
