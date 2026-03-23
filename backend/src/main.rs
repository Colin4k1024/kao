use ai_coding_backend::app::create_app;
use ai_coding_backend::common::config::AppConfig;
use std::error::Error;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "ai_coding_backend=debug,tower_http=debug".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting AI Coding Backend");
    dotenv::dotenv().ok();

    let config = AppConfig::from_env()?;
    let bind_addr = config.bind_addr()?;
    let app = create_app(config).await?;

    tracing::info!("Server listening on http://{}", bind_addr);

    let listener = tokio::net::TcpListener::bind(bind_addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}
