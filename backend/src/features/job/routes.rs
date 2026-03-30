use axum::{
    extract::{Path, Query, State},
    routing::{get, post, put, delete},
    response::IntoResponse,
    Json, Router,
};

use crate::app::AppState;
use crate::common::auth::extractor::AuthUser;
use crate::common::response::ApiResponse;
use crate::common::error::AppError;
use super::model::*;
use super::service::JobService;

// ============== Job Handlers ==============

/// List jobs
pub async fn list_jobs(
    State(state): State<AppState>,
    Query(params): Query<JobListParams>,
) -> Result<impl IntoResponse, AppError> {
    let service = JobService::new();
    let result = service.list_jobs(&state.pool, params).await?;
    Ok(ApiResponse::success(result))
}

/// Get job by ID
pub async fn get_job(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let service = JobService::new();
    let job = service.get_job(&state.pool, id).await?;
    Ok(ApiResponse::success(job))
}

/// Create job
pub async fn create_job(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Json(req): Json<CreateJobRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = JobService::new();
    let job = service.create_job(&state.pool, req, Some(&_auth_user.username)).await?;
    Ok(ApiResponse::success(job))
}

/// Update job
pub async fn update_job(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateJobRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = JobService::new();
    let job = service.update_job(&state.pool, id, req).await?;
    Ok(ApiResponse::success(job))
}

/// Delete job
pub async fn delete_job(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let service = JobService::new();
    service.delete_job(&state.pool, id).await?;
    Ok(ApiResponse::success_no_data())
}

/// Schedule job (start running)
pub async fn schedule_job(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let service = JobService::new();
    service.schedule_job(&state.pool, id).await?;
    Ok(ApiResponse::success_no_data())
}

/// Unschedule job (stop running)
pub async fn unschedule_job(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let service = JobService::new();
    service.unschedule_job(&state.pool, id).await?;
    Ok(ApiResponse::success_no_data())
}

/// Run job once (trigger immediate execution)
pub async fn run_job(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let service = JobService::new();
    service.run_job_once(&state.pool, id).await?;
    Ok(ApiResponse::success_no_data())
}

// ============== Job Log Handlers ==============

/// List job logs
pub async fn list_job_logs(
    State(state): State<AppState>,
    Query(params): Query<JobLogListParams>,
) -> Result<impl IntoResponse, AppError> {
    let service = JobService::new();
    let result = service.list_job_logs(&state.pool, params).await?;
    Ok(ApiResponse::success(result))
}

/// Get job log by ID
pub async fn get_job_log(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let service = JobService::new();
    let log = service.get_job_log(&state.pool, id).await?;
    Ok(ApiResponse::success(log))
}

/// Clear job logs
pub async fn clear_job_logs(
    State(state): State<AppState>,
    Query(params): Query<JobLogListParams>,
) -> Result<impl IntoResponse, AppError> {
    let service = JobService::new();
    let count = service.clear_job_logs(&state.pool, params.job_id).await?;
    Ok(ApiResponse::success(count))
}

// ============== Router ==============

pub fn job_routes() -> Router<AppState> {
    Router::new()
        .route("/api/jobs", get(list_jobs))
        .route("/api/jobs", post(create_job))
        .route("/api/jobs/:id", get(get_job))
        .route("/api/jobs/:id", put(update_job))
        .route("/api/jobs/:id", delete(delete_job))
        .route("/api/jobs/:id/schedule", put(schedule_job))
        .route("/api/jobs/:id/unschedule", put(unschedule_job))
        .route("/api/jobs/:id/run", post(run_job))
        .route("/api/jobs/logs", get(list_job_logs))
        .route("/api/jobs/logs/clear", delete(clear_job_logs))
        .route("/api/jobs/logs/:id", get(get_job_log))
}
