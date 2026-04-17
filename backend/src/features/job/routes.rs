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
use super::scheduler;

// ============== Job Handlers ==============

/// GET /api/jobs - List jobs
#[utoipa::path(
    get,
    path = "/api/jobs",
    tag = "jobs",
    security (
        ("bearer_auth" = [])
    ),
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("page_size" = Option<i64>, Query, description = "Page size"),
        ("keyword" = Option<String>, Query, description = "Search keyword"),
        ("job_name" = Option<String>, Query, description = "Job name filter"),
        ("job_code" = Option<String>, Query, description = "Job code filter"),
        ("job_status" = Option<i32>, Query, description = "Job status filter")
    ),
    responses(
        (status = 200, description = "Job list retrieved successfully", body = ApiResponse),
        (status = 401, description = "Not authenticated")
    )
)]
pub async fn list_jobs(
    State(state): State<AppState>,
    Query(params): Query<JobListParams>,
) -> Result<impl IntoResponse, AppError> {
    let service = JobService::new();
    let result = service.list_jobs(&state.pool, params).await?;
    Ok(ApiResponse::success(result))
}

/// GET /api/jobs/{id} - Get job by ID
#[utoipa::path(
    get,
    path = "/api/jobs/{id}",
    tag = "jobs",
    security (
        ("bearer_auth" = [])
    ),
    params(
        ("id" = i64, Path, description = "Job ID")
    ),
    responses(
        (status = 200, description = "Job found", body = ApiResponse),
        (status = 404, description = "Job not found"),
        (status = 401, description = "Not authenticated")
    )
)]
pub async fn get_job(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let service = JobService::new();
    let job = service.get_job(&state.pool, id).await?;
    Ok(ApiResponse::success(job))
}

/// POST /api/jobs - Create job
#[utoipa::path(
    post,
    path = "/api/jobs",
    tag = "jobs",
    security (
        ("bearer_auth" = [])
    ),
    request_body = CreateJobRequest,
    responses(
        (status = 200, description = "Job created successfully", body = ApiResponse),
        (status = 400, description = "Validation error"),
        (status = 401, description = "Not authenticated")
    )
)]
pub async fn create_job(
    State(state): State<AppState>,
    _auth_user: AuthUser,
    Json(req): Json<CreateJobRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = JobService::new();
    let job = service.create_job(&state.pool, req, Some(&_auth_user.username)).await?;
    Ok(ApiResponse::success(job))
}

/// PUT /api/jobs/{id} - Update job
#[utoipa::path(
    put,
    path = "/api/jobs/{id}",
    tag = "jobs",
    security (
        ("bearer_auth" = [])
    ),
    params(
        ("id" = i64, Path, description = "Job ID")
    ),
    request_body = UpdateJobRequest,
    responses(
        (status = 200, description = "Job updated successfully", body = ApiResponse),
        (status = 404, description = "Job not found"),
        (status = 401, description = "Not authenticated")
    )
)]
pub async fn update_job(
    State(state): State<AppState>,
    Path(id): Path<i64>,
    Json(req): Json<UpdateJobRequest>,
) -> Result<impl IntoResponse, AppError> {
    let service = JobService::new();
    let job = service.update_job(&state.pool, id, req).await?;
    Ok(ApiResponse::success(job))
}

/// DELETE /api/jobs/{id} - Delete job
#[utoipa::path(
    delete,
    path = "/api/jobs/{id}",
    tag = "jobs",
    security (
        ("bearer_auth" = [])
    ),
    params(
        ("id" = i64, Path, description = "Job ID")
    ),
    responses(
        (status = 200, description = "Job deleted successfully", body = ApiResponse),
        (status = 404, description = "Job not found"),
        (status = 401, description = "Not authenticated")
    )
)]
pub async fn delete_job(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let service = JobService::new();
    service.delete_job(&state.pool, id).await?;
    Ok(ApiResponse::success_no_data())
}

/// PUT /api/jobs/{id}/schedule - Schedule job (start running)
#[utoipa::path(
    put,
    path = "/api/jobs/{id}/schedule",
    tag = "jobs",
    security (
        ("bearer_auth" = [])
    ),
    params(
        ("id" = i64, Path, description = "Job ID")
    ),
    responses(
        (status = 200, description = "Job scheduled successfully", body = ApiResponse),
        (status = 404, description = "Job not found"),
        (status = 401, description = "Not authenticated")
    )
)]
pub async fn schedule_job(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let service = JobService::new();
    service.schedule_job(&state.pool, id).await?;

    // Add job to the scheduler
    if let Some(job) = super::repo::get_job(&state.pool, id).await? {
        scheduler::add_job_to_scheduler(
            state.scheduler.as_ref(),
            &state.pool,
            &state.running_jobs,
            &job,
        ).await?;
    }

    Ok(ApiResponse::success_no_data())
}

/// PUT /api/jobs/{id}/unschedule - Unschedule job (stop running)
#[utoipa::path(
    put,
    path = "/api/jobs/{id}/unschedule",
    tag = "jobs",
    security (
        ("bearer_auth" = [])
    ),
    params(
        ("id" = i64, Path, description = "Job ID")
    ),
    responses(
        (status = 200, description = "Job unscheduled successfully", body = ApiResponse),
        (status = 404, description = "Job not found"),
        (status = 401, description = "Not authenticated")
    )
)]
pub async fn unschedule_job(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let service = JobService::new();
    service.unschedule_job(&state.pool, id).await?;

    // Remove job from the scheduler
    scheduler::remove_job_from_scheduler(id, &state.running_jobs).await?;

    Ok(ApiResponse::success_no_data())
}

/// POST /api/jobs/{id}/run - Run job once (trigger immediate execution)
#[utoipa::path(
    post,
    path = "/api/jobs/{id}/run",
    tag = "jobs",
    security (
        ("bearer_auth" = [])
    ),
    params(
        ("id" = i64, Path, description = "Job ID")
    ),
    responses(
        (status = 200, description = "Job triggered successfully", body = ApiResponse),
        (status = 404, description = "Job not found"),
        (status = 401, description = "Not authenticated")
    )
)]
pub async fn run_job(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let service = JobService::new();
    service.run_job_once(&state.pool, id).await?;
    Ok(ApiResponse::success_no_data())
}

// ============== Job Log Handlers ==============

/// GET /api/jobs/logs - List job logs
#[utoipa::path(
    get,
    path = "/api/jobs/logs",
    tag = "jobs",
    security (
        ("bearer_auth" = [])
    ),
    params(
        ("page" = Option<i64>, Query, description = "Page number"),
        ("page_size" = Option<i64>, Query, description = "Page size"),
        ("job_id" = Option<i64>, Query, description = "Job ID filter"),
        ("job_name" = Option<String>, Query, description = "Job name filter"),
        ("execute_status" = Option<i32>, Query, description = "Execute status filter")
    ),
    responses(
        (status = 200, description = "Job log list retrieved successfully", body = ApiResponse),
        (status = 401, description = "Not authenticated")
    )
)]
pub async fn list_job_logs(
    State(state): State<AppState>,
    Query(params): Query<JobLogListParams>,
) -> Result<impl IntoResponse, AppError> {
    let service = JobService::new();
    let result = service.list_job_logs(&state.pool, params).await?;
    Ok(ApiResponse::success(result))
}

/// GET /api/jobs/logs/{id} - Get job log by ID
#[utoipa::path(
    get,
    path = "/api/jobs/logs/{id}",
    tag = "jobs",
    security (
        ("bearer_auth" = [])
    ),
    params(
        ("id" = i64, Path, description = "Job log ID")
    ),
    responses(
        (status = 200, description = "Job log found", body = ApiResponse),
        (status = 404, description = "Job log not found"),
        (status = 401, description = "Not authenticated")
    )
)]
pub async fn get_job_log(
    State(state): State<AppState>,
    Path(id): Path<i64>,
) -> Result<impl IntoResponse, AppError> {
    let service = JobService::new();
    let log = service.get_job_log(&state.pool, id).await?;
    Ok(ApiResponse::success(log))
}

/// DELETE /api/jobs/logs/clear - Clear job logs
#[utoipa::path(
    delete,
    path = "/api/jobs/logs/clear",
    tag = "jobs",
    security (
        ("bearer_auth" = [])
    ),
    params(
        ("job_id" = Option<i64>, Query, description = "Job ID to clear logs for")
    ),
    responses(
        (status = 200, description = "Job logs cleared successfully", body = ApiResponse),
        (status = 401, description = "Not authenticated")
    )
)]
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
        .route("/jobs", get(list_jobs))
        .route("/jobs", post(create_job))
        .route("/jobs/:id", get(get_job))
        .route("/jobs/:id", put(update_job))
        .route("/jobs/:id", delete(delete_job))
        .route("/jobs/:id/schedule", put(schedule_job))
        .route("/jobs/:id/unschedule", put(unschedule_job))
        .route("/jobs/:id/run", post(run_job))
        .route("/jobs/logs", get(list_job_logs))
        .route("/jobs/logs/clear", delete(clear_job_logs))
        .route("/jobs/logs/:id", get(get_job_log))
}
