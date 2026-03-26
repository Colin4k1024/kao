use kao_backend::config;
use kao_backend::db;
use kao_backend::middleware::logger::init_logger;
use std::net::SocketAddr;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let settings = config::init();

    init_logger(&settings.app.log_level);

    tracing::info!("Starting kao-backend server...");

    let pool = db::create_pool(&settings).await?;

    tracing::info!("Database connection pool created");

    let app = kao_backend::app::create_app(pool, settings).await;

    let addr = SocketAddr::from(([0, 0, 0, 0], 8080));
    tracing::info!("Server listening on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await;

    Ok(())
}
