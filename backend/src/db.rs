use sqlx::postgres::{PgPool, PgPoolOptions};
use crate::config::Settings;

pub async fn create_pool(settings: &Settings) -> Result<PgPool, sqlx::Error> {
    let pool = PgPoolOptions::new()
        .max_connections(settings.database.max_connections)
        .min_connections(settings.database.min_connections)
        .acquire_timeout(std::time::Duration::from_secs(settings.database.connect_timeout))
        .idle_timeout(std::time::Duration::from_secs(settings.database.idle_timeout))
        .connect(&settings.database.url)
        .await?;

    Ok(pool)
}
