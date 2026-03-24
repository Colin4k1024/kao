use sqlx::PgPool;
use std::sync::Arc;

use crate::common::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub config: Arc<Config>,
}