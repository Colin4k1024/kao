use axum::{
    extract::{Path, State},
    response::Response,
    routing::get,
    Router,
};
use uuid::Uuid;

use crate::common::response::ApiResponse;
use crate::AppState;

use super::model::*;
use super::service::SecurityScanService;

/// Create security scan router
pub fn security_router() -> Router<AppState> {
    Router::new()
        .route("/scan", get(security_scan))
        .route("/scan/configuration", get(config_scan))
        .route("/scan/input-validation", get(input_scan))
        .route("/scan/authentication", get(auth_scan))
        .route("/scan/authorization", get(authz_scan))
        .route("/events", get(security_events))
        .route("/password-health/:user_id", get(password_health))
}

/// GET /api/monitoring/security/scan - Full security scan
pub async fn security_scan(State(state): State<AppState>) -> Response {
    let service = SecurityScanService::new(state.pool.clone());

    match service.run_full_scan().await {
        Ok(result) => ApiResponse::success(result),
        Err(e) => {
            log::error!("Security scan failed: {}", e);
            ApiResponse::error(500, format!("Security scan failed: {}", e))
        }
    }
}

/// GET /api/monitoring/security/scan/configuration - Configuration scan
pub async fn config_scan(State(state): State<AppState>) -> Response {
    let service = SecurityScanService::new(state.pool.clone());

    match service.scan_configuration().await {
        Ok(checks) => {
            let total_checks = checks.len();
            let passed_checks = checks.iter().filter(|c| c.status == "pass").count();
            let failed_checks = checks.iter().filter(|c| c.status == "fail").count();
            let warning_checks = checks.iter().filter(|c| c.status == "warning").count();

            let status = if failed_checks > 0 {
                "critical"
            } else if warning_checks > 0 {
                "warning"
            } else {
                "healthy"
            };

            let result = SecurityScanResult {
                status: status.to_string(),
                checks,
                summary: SecurityScanSummary {
                    total_checks,
                    passed_checks,
                    failed_checks,
                    warning_checks,
                },
            };
            ApiResponse::success(result)
        }
        Err(e) => {
            log::error!("Configuration scan failed: {}", e);
            ApiResponse::error(500, format!("Configuration scan failed: {}", e))
        }
    }
}

/// GET /api/monitoring/security/scan/input-validation - Input validation scan
pub async fn input_scan(State(state): State<AppState>) -> Response {
    let service = SecurityScanService::new(state.pool.clone());

    match service.scan_audit_events().await {
        Ok(checks) => {
            let total_checks = checks.len();
            let passed_checks = checks.iter().filter(|c| c.status == "pass").count();
            let failed_checks = checks.iter().filter(|c| c.status == "fail").count();
            let warning_checks = checks.iter().filter(|c| c.status == "warning").count();

            let status = if failed_checks > 0 {
                "critical"
            } else if warning_checks > 0 {
                "warning"
            } else {
                "healthy"
            };

            let result = SecurityScanResult {
                status: status.to_string(),
                checks,
                summary: SecurityScanSummary {
                    total_checks,
                    passed_checks,
                    failed_checks,
                    warning_checks,
                },
            };
            ApiResponse::success(result)
        }
        Err(e) => {
            log::error!("Input validation scan failed: {}", e);
            ApiResponse::error(500, format!("Input validation scan failed: {}", e))
        }
    }
}

/// GET /api/monitoring/security/scan/authentication - Authentication scan
pub async fn auth_scan(State(state): State<AppState>) -> Response {
    let service = SecurityScanService::new(state.pool.clone());

    // Run authentication-related scans
    let password_checks = service.scan_password_health().await.unwrap_or_default();
    let lockout_checks = service.scan_locked_accounts().await.unwrap_or_default();
    let failed_login_checks = service.scan_failed_logins().await.unwrap_or_default();
    let brute_force_checks = service.scan_brute_force().await.unwrap_or_default();

    let mut all_checks = Vec::new();
    all_checks.extend(password_checks);
    all_checks.extend(lockout_checks);
    all_checks.extend(failed_login_checks);
    all_checks.extend(brute_force_checks);

    let total_checks = all_checks.len();
    let passed_checks = all_checks.iter().filter(|c| c.status == "pass").count();
    let failed_checks = all_checks.iter().filter(|c| c.status == "fail").count();
    let warning_checks = all_checks.iter().filter(|c| c.status == "warning").count();

    let status = if failed_checks > 0 {
        "critical"
    } else if warning_checks > 0 {
        "warning"
    } else {
        "healthy"
    };

    let result = SecurityScanResult {
        status: status.to_string(),
        checks: all_checks,
        summary: SecurityScanSummary {
            total_checks,
            passed_checks,
            failed_checks,
            warning_checks,
        },
    };
    ApiResponse::success(result)
}

/// GET /api/monitoring/security/scan/authorization - Authorization scan
pub async fn authz_scan(State(state): State<AppState>) -> Response {
    let service = SecurityScanService::new(state.pool.clone());

    // Run authorization-related scans (permission denied events)
    let audit_checks = service.scan_audit_events().await.unwrap_or_default();

    // Filter to only authorization-related checks
    let authz_checks: Vec<SecurityCheck> = audit_checks
        .into_iter()
        .filter(|c| {
            matches!(
                c.name.as_str(),
                "permission_denied_events" | "privilege_escalation"
            )
        })
        .collect();

    let total_checks = authz_checks.len();
    let passed_checks = authz_checks.iter().filter(|c| c.status == "pass").count();
    let failed_checks = authz_checks.iter().filter(|c| c.status == "fail").count();
    let warning_checks = authz_checks.iter().filter(|c| c.status == "warning").count();

    let status = if failed_checks > 0 {
        "critical"
    } else if warning_checks > 0 {
        "warning"
    } else {
        "healthy"
    };

    let result = SecurityScanResult {
        status: status.to_string(),
        checks: authz_checks,
        summary: SecurityScanSummary {
            total_checks,
            passed_checks,
            failed_checks,
            warning_checks,
        },
    };
    ApiResponse::success(result)
}

/// GET /api/monitoring/security/events - Get security events
pub async fn security_events(State(state): State<AppState>) -> Response {
    let service = SecurityScanService::new(state.pool.clone());

    // Get security event summary and recent events
    let summary = match service.get_security_event_summary().await {
        Ok(s) => s,
        Err(e) => {
            log::error!("Failed to get security event summary: {}", e);
            return ApiResponse::error(500, format!("Failed to get security events: {}", e));
        }
    };

    let locked_accounts = service.get_locked_accounts().await.unwrap_or_default();
    let failed_attempts = service.get_failed_login_attempts(10).await.unwrap_or_default();
    let brute_force = service.get_brute_force_detection().await.unwrap_or_default();
    let suspicious_inputs = service.get_suspicious_inputs(10).await.unwrap_or_default();
    let permission_denied = service.get_permission_denied_events(10).await.unwrap_or_default();

    #[derive(serde::Serialize)]
    struct SecurityEventsResponse {
        summary: SecurityEventSummary,
        locked_accounts: Vec<LockedAccount>,
        recent_failed_attempts: Vec<FailedLoginAttempt>,
        brute_force_detection: Vec<BruteForceDetection>,
        suspicious_inputs: Vec<SuspiciousInput>,
        permission_denied_events: Vec<PermissionDeniedEvent>,
    }

    let response = SecurityEventsResponse {
        summary,
        locked_accounts,
        recent_failed_attempts: failed_attempts,
        brute_force_detection: brute_force,
        suspicious_inputs,
        permission_denied_events: permission_denied,
    };

    ApiResponse::success(response)
}

/// GET /api/monitoring/security/password-health/:user_id - Get password health for user
pub async fn password_health(
    State(state): State<AppState>,
    Path(user_id): Path<Uuid>,
) -> Response {
    let service = SecurityScanService::new(state.pool.clone());

    match service.get_password_health(user_id).await {
        Ok(Some(health)) => ApiResponse::success(health),
        Ok(None) => ApiResponse::error(404, "User not found".to_string()),
        Err(e) => {
            log::error!("Failed to get password health: {}", e);
            ApiResponse::error(500, format!("Failed to get password health: {}", e))
        }
    }
}
