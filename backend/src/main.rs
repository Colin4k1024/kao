use ai_coding_backend::app::{router::create_router, state::AppState};
use ai_coding_backend::common::{config::Config, db::create_db_pool};
use std::sync::Arc;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::try_from_default_env().unwrap_or_else(|_| {
            "ai_coding_backend=debug,tower_http=debug".into()
        }))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Load configuration
    let config = Config::from_env()?;
    
    // Create database pool
    let db_pool = create_db_pool(&config.database_url).await?;
    
    // Create application state
    let app_state = AppState {
        db: db_pool,
        config: config.clone(),
    };

    // Create router with state
    let app = create_router(app_state);

    // Run server
    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
        .await
        .unwrap();
    
    println!("Server running on http://0.0.0.0:{}", config.port);
    
    axum::serve(listener, app).await.unwrap();

    Ok(())
}