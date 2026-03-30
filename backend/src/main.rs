use kao_backend::config;
use kao_backend::db;
use kao_backend::common::logging::init_logger_with_level;
use kao_backend::common::cache::redis::RedisCache;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let settings = config::init();

    let log_level = match settings.app.log_level.to_lowercase().as_str() {
        "trace" => kao_backend::common::logging::LogLevel::Trace,
        "debug" => kao_backend::common::logging::LogLevel::Debug,
        "info" => kao_backend::common::logging::LogLevel::Info,
        "warn" => kao_backend::common::logging::LogLevel::Warn,
        "error" => kao_backend::common::logging::LogLevel::Error,
        _ => kao_backend::common::logging::LogLevel::Info,
    };
    let _logger = init_logger_with_level(log_level);

    tracing::info!("Starting kao-backend server...");

    let pool = db::create_pool(&settings).await?;

    tracing::info!("Database connection pool created");

    // Run database migrations
    tracing::info!("Running database migrations...");
    if let Err(e) = kao_backend::common::db::run_migrations(&pool).await {
        tracing::error!("Failed to run migrations: {}", e);
        return Err(anyhow::anyhow!("Migration failed: {}", e));
    }
    tracing::info!("Database migrations completed");

    // Create Redis cache instance
    let cache = if let Some(redis_url) = &settings.redis.url {
        tracing::info!("Initializing Redis cache...");
        RedisCache::from_url(redis_url, settings.redis.cache_ttl)
    } else {
        tracing::info!("Redis URL not configured, running without cache");
        RedisCache::new(None, kao_backend::common::cache::redis::CacheConfig::default())
    };

    let app = kao_backend::app::create_app(pool, settings, cache);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
