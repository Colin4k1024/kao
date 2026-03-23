use crate::common::{
    config::AppConfig,
    db::{create_pool, DbPool},
    error::AppError,
};

#[derive(Clone)]
pub struct AppState {
    pub config: AppConfig,
    pub db: DbPool,
}

impl AppState {
    pub async fn bootstrap(config: AppConfig) -> Result<Self, AppError> {
        let db = create_pool(&config.database_url).await?;
        Ok(Self { config, db })
    }
}
