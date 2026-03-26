use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;

use super::metrics::{get_metrics, MetricsResponse};

// Re-export for convenience
pub use super::health::HealthStatus;
