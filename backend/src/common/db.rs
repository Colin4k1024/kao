use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;
use std::path::Path;
use tokio::fs;

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

/// Run database migrations from the migrations directory
pub async fn run_migrations(pool: &PgPool) -> Result<(), MigrationError> {
    let migrations_dir = std::path::Path::new("migrations");

    if !migrations_dir.exists() {
        tracing::info!("No migrations directory found, skipping migrations");
        return Ok(());
    }

    // Create migrations tracking table if it doesn't exist
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS _sqlx_migrations (
            version VARCHAR(255) PRIMARY KEY,
            applied_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
        )
        "#,
    )
    .execute(pool)
    .await
    .map_err(|e| MigrationError::Setup(e.to_string()))?;

    // Collect all migration files
    let mut entries = fs::read_dir(migrations_dir).await.map_err(|e| {
        MigrationError::ReadDir(e.to_string())
    })?;

    let mut migration_files: Vec<String> = Vec::new();
    while let Some(entry) = entries.next_entry().await.map_err(|e| MigrationError::ReadDir(e.to_string()))? {
        let path = entry.path();
        if path.extension().and_then(|s| s.to_str()) == Some("sql") {
            if let Some(name) = path.file_name().and_then(|s| s.to_str()) {
                migration_files.push(name.to_string());
            }
        }
    }

    // Sort migration files to ensure order
    migration_files.sort();

    // Track applied migrations
    let applied: std::collections::HashSet<String> = sqlx::query_scalar::<_, String>(
        "SELECT version FROM _sqlx_migrations"
    )
    .fetch_all(pool)
    .await
    .map_err(|e| MigrationError::Query(e.to_string()))?
    .into_iter()
    .collect();

    // Run pending migrations
    for filename in &migration_files {
        if applied.contains(*filename) {
            tracing::info!("Migration {} already applied, skipping", filename);
            continue;
        }

        let path = migrations_dir.join(filename);
        let sql = fs::read_to_string(&path).await.map_err(|e| {
            MigrationError::ReadFile(format!("{}: {}", filename, e))
        })?;

        tracing::info!("Applying migration: {}", filename);

        // Execute each statement in the migration
        for statement in sql.split(';') {
            let trimmed = statement.trim();
            if !trimmed.is_empty() {
                sqlx::query(trimmed)
                    .execute(pool)
                    .await
                    .map_err(|e| MigrationError::Execution(format!("{}: {}", filename, e)))?;
            }
        }

        // Record the migration
        sqlx::query("INSERT INTO _sqlx_migrations (version) VALUES ($1)")
            .bind(filename)
            .execute(pool)
            .await
            .map_err(|e| MigrationError::Record(format!("{}: {}", filename, e)))?;

        tracing::info!("Migration {} applied successfully", filename);
    }

    Ok(())
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

/// Migration error types
#[derive(Debug, thiserror::Error)]
pub enum MigrationError {
    #[error("Failed to setup migrations table: {0}")]
    Setup(String),
    #[error("Failed to read migrations directory: {0}")]
    ReadDir(String),
    #[error("Failed to read migration file: {0}")]
    ReadFile(String),
    #[error("Failed to execute migration: {0}")]
    Execution(String),
    #[error("Failed to record migration: {0}")]
    Record(String),
    #[error("Failed to query applied migrations: {0}")]
    Query(String),
}
