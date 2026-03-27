// System Monitoring module
// 
// This module provides system monitoring and logging features:
// - Metrics endpoint (Prometheus format)
// - Health check with dependency status
// - Operation logging
// - Login logging
// - Online user monitoring

pub mod metrics;
pub mod health;
pub mod operation_log;
pub mod login_log;
pub mod online_user;
pub mod routes;

// Re-export for convenience
pub use crate::common::metrics::MetricsMiddleware;
pub use metrics::MetricsResponse as CommonMetricsResponse;
pub use health::{check_health, HealthStatus, HealthCheckResponse};
pub use operation_log::{OperationLogController, OperationLogService, OperationLog, CreateOperationLogRequest, OperationLogQueryParams};
pub use login_log::{LoginLogController, LoginLogService, LoginLog, CreateLoginLogRequest, LoginLogQueryParams};
pub use online_user::{OnlineUserController, OnlineUserService, OnlineUser, OnlineUsersResponse, ForceLogoutRequest};
pub use routes::monitoring_router;
