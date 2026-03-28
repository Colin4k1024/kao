use kao_backend::config;
use kao_backend::db;
use kao_backend::common::logging::init_logger_with_level;
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
    if let Err(e) = db::run_migrations(&pool).await {
        tracing::error!("Failed to run migrations: {}", e);
        return Err(anyhow::anyhow!("Migration failed: {}", e));
    }
    tracing::info!("Database migrations completed");

    let app = kao_backend::app::create_app(pool, settings);

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app.into_make_service()).await?;

    Ok(())
}
