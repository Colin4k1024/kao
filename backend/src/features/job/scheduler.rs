//! Job scheduler module
//!
//! This module provides cron-based job scheduling using tokio-cron-scheduler.

use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;
use tokio_cron_scheduler::{Job, JobScheduler};

use crate::common::error::AppError;

use super::model::{JobLogRecord, JobRecord};
use super::repo;

/// Scheduler state shared across the application
pub struct SchedulerState {
    pub running_jobs: Arc<RwLock<std::collections::HashSet<i64>>>,
}

impl SchedulerState {
    pub fn new() -> Self {
        Self {
            running_jobs: Arc::new(RwLock::new(std::collections::HashSet::new())),
        }
    }
}

impl Default for SchedulerState {
    fn default() -> Self {
        Self::new()
    }
}

/// Initialize and start the job scheduler
pub async fn init_scheduler(pool: &PgPool) -> Result<(JobScheduler, SchedulerState), AppError> {
    let scheduler = JobScheduler::new()
        .await
        .map_err(|e| AppError::Internal(Some(e.to_string())))?;

    let state = SchedulerState::new();

    // Load and schedule all running jobs from database
    load_and_schedule_jobs(&scheduler, pool, &state.running_jobs).await?;

    // Start the scheduler
    scheduler
        .start()
        .await
        .map_err(|e| AppError::Internal(Some(e.to_string())))?;

    tracing::info!("Job scheduler started");

    Ok((scheduler, state))
}

/// Load all running jobs from database and schedule them
pub async fn load_and_schedule_jobs(
    scheduler: &JobScheduler,
    pool: &PgPool,
    running_jobs: &Arc<RwLock<std::collections::HashSet<i64>>>,
) -> Result<(), AppError> {
    // Get all jobs with status = 1 (running)
    let jobs = repo::list_running_jobs(pool)
        .await
        .map_err(|e| AppError::Internal(Some(e.to_string())))?;

    let count = jobs.len();
    for job in jobs {
        add_job_to_scheduler(scheduler, pool, running_jobs, &job).await?;
    }

    tracing::info!("Loaded {} running jobs into scheduler", count);
    Ok(())
}

/// Add a single job to the scheduler
pub async fn add_job_to_scheduler(
    scheduler: &JobScheduler,
    pool: &PgPool,
    running_jobs: &Arc<RwLock<std::collections::HashSet<i64>>>,
    job: &JobRecord,
) -> Result<(), AppError> {
    let job_id = job.id;
    let cron_expr = job.cron_expression.clone();
    let pool = pool.clone();
    let running_jobs_clone = running_jobs.clone();

    // Mark job as running BEFORE creating the closure
    {
        let mut jobs = running_jobs.write().await;
        jobs.insert(job_id);
    }

    // Create a job using tokio-cron-scheduler
    let job = Job::new_async(cron_expr.as_str(), move |_uuid, _l| {
        let pool = pool.clone();
        let running_jobs = running_jobs_clone.clone();
        Box::pin(async move {
            execute_job(&pool, job_id, &running_jobs).await;
        })
    })
    .map_err(|e| AppError::Internal(Some(format!("Failed to create cron job: {}", e))))?;

    scheduler
        .add(job)
        .await
        .map_err(|e| AppError::Internal(Some(format!("Failed to add job to scheduler: {}", e))))?;

    tracing::info!("Added job {} to scheduler with cron: {}", job_id, cron_expr);
    Ok(())
}

/// Remove a job from the scheduler (by marking it as not running)
pub async fn remove_job_from_scheduler(
    job_id: i64,
    running_jobs: &Arc<RwLock<std::collections::HashSet<i64>>>,
) -> Result<(), AppError> {
    // Remove from running jobs set
    let mut jobs = running_jobs.write().await;
    jobs.remove(&job_id);

    tracing::info!("Removed job {} from scheduler", job_id);
    Ok(())
}

/// Execute a job
async fn execute_job(
    pool: &PgPool,
    job_id: i64,
    running_jobs: &Arc<RwLock<std::collections::HashSet<i64>>>,
) {
    tracing::info!("Executing job {}", job_id);

    // Check if job is still scheduled to run
    {
        let jobs = running_jobs.read().await;
        if !jobs.contains(&job_id) {
            tracing::info!("Job {} was unscheduled, skipping execution", job_id);
            return;
        }
    }

    // Get job details
    let job = match repo::get_job(pool, job_id).await {
        Ok(Some(j)) => j,
        Ok(None) => {
            tracing::warn!("Job {} not found, skipping execution", job_id);
            return;
        }
        Err(e) => {
            tracing::error!("Failed to get job {}: {}", job_id, e);
            return;
        }
    };

    // Create log entry - job starting
    let start_time = chrono::Utc::now();
    let log = JobLogRecord {
        id: 0,
        job_id,
        job_name: job.job_name.clone(),
        job_code: job.job_code.clone(),
        job_group: job.job_group.clone(),
        execute_status: 1, // Running
        execute_message: Some("Job started".to_string()),
        execute_time: Some(start_time.to_rfc3339()),
        created_at: start_time,
    };

    let log_id = match repo::create_job_log(pool, &log).await {
        Ok(id) => id,
        Err(e) => {
            tracing::error!("Failed to create job log: {}", e);
            return;
        }
    };

    // Simulate job execution
    // In a real implementation, this would execute the actual job logic
    // For now, we just sleep for a bit and log success
    tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;

    let end_time = chrono::Utc::now();
    let duration = end_time.signed_duration_since(start_time);

    // Update log entry - job completed
    let update_log = JobLogRecord {
        id: log_id,
        job_id,
        job_name: job.job_name.clone(),
        job_code: job.job_code.clone(),
        job_group: job.job_group.clone(),
        execute_status: 2, // Success
        execute_message: Some(format!("Job completed successfully in {:?}", duration)),
        execute_time: Some(end_time.to_rfc3339()),
        created_at: start_time,
    };

    if let Err(e) = repo::update_job_log(pool, log_id, &update_log).await {
        tracing::error!("Failed to update job log: {}", e);
    }

    tracing::info!("Job {} completed successfully in {:?}", job_id, duration);
}
