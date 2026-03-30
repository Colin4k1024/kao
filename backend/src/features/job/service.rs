use sqlx::PgPool;
use crate::common::error::AppError;
use super::model::*;
use super::repo;

/// Validate cron expression (basic validation)
fn validate_cron_expression(cron: &str) -> Result<(), AppError> {
    let parts: Vec<&str> = cron.trim().split_whitespace().collect();
    // Cron expression should have 6 or 7 fields: second minute hour day month week [year]
    if parts.len() != 6 && parts.len() != 7 {
        return Err(AppError::Validation {
            field: "cron_expression".to_string(),
            message: "Cron expression must have 6 or 7 fields".to_string(),
        });
    }
    // Basic format validation - just check that fields don't contain invalid characters
    for part in &parts {
        if !part.chars().all(|c| c.is_ascii_digit() || "*?-,/".contains(c)) {
            return Err(AppError::Validation {
                field: "cron_expression".to_string(),
                message: format!("Invalid cron field: {}", part),
            });
        }
    }
    Ok(())
}

pub struct JobService;

impl JobService {
    pub fn new() -> Self {
        Self
    }

    /// List jobs with pagination
    pub async fn list_jobs(
        &self,
        db: &PgPool,
        params: JobListParams,
    ) -> Result<JobListResponse, AppError> {
        let page = params.page.unwrap_or(1).max(1);
        let page_size = params.page_size.unwrap_or(10).min(100);

        let (jobs, total) = repo::list_jobs(db, &params).await?;

        Ok(JobListResponse {
            list: jobs.into_iter().map(Job::from).collect(),
            total,
            page,
            page_size,
        })
    }

    /// Get job by ID
    pub async fn get_job(&self, db: &PgPool, id: i64) -> Result<Job, AppError> {
        repo::get_job(db, id)
            .await?
            .map(Job::from)
            .ok_or_else(|| AppError::NotFound("Job not found".to_string()))
    }

    /// Create a new job
    pub async fn create_job(
        &self,
        db: &PgPool,
        req: CreateJobRequest,
        created_by: Option<&str>,
    ) -> Result<Job, AppError> {
        // Validate cron expression
        validate_cron_expression(&req.cron_expression)?;

        // Check if job_code already exists
        if repo::get_job_by_code(db, &req.job_code).await?.is_some() {
            return Err(AppError::Validation {
                field: "job_code".to_string(),
                message: "Job code already exists".to_string(),
            });
        }

        // Validate retry_count, retry_interval, timeout
        if req.retry_count.unwrap_or(0) > 10 {
            return Err(AppError::Validation {
                field: "retry_count".to_string(),
                message: "Retry count must not exceed 10".to_string(),
            });
        }

        if let Some(interval) = req.retry_interval {
            if interval < 1 || interval > 3600 {
                return Err(AppError::Validation {
                    field: "retry_interval".to_string(),
                    message: "Retry interval must be between 1 and 3600 seconds".to_string(),
                });
            }
        }

        if let Some(timeout) = req.timeout {
            if timeout < 1 || timeout > 3600 {
                return Err(AppError::Validation {
                    field: "timeout".to_string(),
                    message: "Timeout must be between 1 and 3600 seconds".to_string(),
                });
            }
        }

        let job_id = repo::create_job(db, &req, created_by).await?;
        self.get_job(db, job_id).await
    }

    /// Update an existing job
    pub async fn update_job(
        &self,
        db: &PgPool,
        id: i64,
        req: UpdateJobRequest,
    ) -> Result<Job, AppError> {
        // Validate cron expression if provided
        if let Some(ref cron) = req.cron_expression {
            validate_cron_expression(cron)?;
        }

        // Validate retry_count
        if let Some(count) = req.retry_count {
            if count > 10 {
                return Err(AppError::Validation {
                    field: "retry_count".to_string(),
                    message: "Retry count must not exceed 10".to_string(),
                });
            }
        }

        // Validate retry_interval
        if let Some(interval) = req.retry_interval {
            if interval < 1 || interval > 3600 {
                return Err(AppError::Validation {
                    field: "retry_interval".to_string(),
                    message: "Retry interval must be between 1 and 3600 seconds".to_string(),
                });
            }
        }

        // Validate timeout
        if let Some(timeout) = req.timeout {
            if timeout < 1 || timeout > 3600 {
                return Err(AppError::Validation {
                    field: "timeout".to_string(),
                    message: "Timeout must be between 1 and 3600 seconds".to_string(),
                });
            }
        }

        repo::update_job(db, id, &req).await?;
        self.get_job(db, id).await
    }

    /// Delete a job
    pub async fn delete_job(&self, db: &PgPool, id: i64) -> Result<(), AppError> {
        // Verify job exists
        let _ = self.get_job(db, id).await?;
        repo::delete_job(db, id).await
    }

    /// Schedule a job (set status to running)
    pub async fn schedule_job(&self, db: &PgPool, id: i64) -> Result<(), AppError> {
        let job = repo::get_job(db, id).await?.ok_or_else(|| AppError::NotFound("Job not found".to_string()))?;
        if job.job_status == 1 {
            return Err(AppError::Validation {
                field: "job_status".to_string(),
                message: "Job is already running".to_string(),
            });
        }
        repo::update_job_status(db, id, 1).await
    }

    /// Unschedule a job (set status to stopped)
    pub async fn unschedule_job(&self, db: &PgPool, id: i64) -> Result<(), AppError> {
        let job = repo::get_job(db, id).await?.ok_or_else(|| AppError::NotFound("Job not found".to_string()))?;
        if job.job_status == 0 {
            return Err(AppError::Validation {
                field: "job_status".to_string(),
                message: "Job is already stopped".to_string(),
            });
        }
        repo::update_job_status(db, id, 0).await
    }

    /// Run job once (trigger immediate execution and log it)
    pub async fn run_job_once(&self, db: &PgPool, id: i64) -> Result<(), AppError> {
        let job = repo::get_job(db, id).await?.ok_or_else(|| AppError::NotFound("Job not found".to_string()))?;

        // Create a job log entry
        let log = JobLogRecord {
            id: 0, // Will be set by database
            job_id: job.id,
            job_name: job.job_name.clone(),
            job_code: job.job_code.clone(),
            job_group: job.job_group.clone(),
            execute_status: 1, // Running
            execute_message: Some("Job triggered manually".to_string()),
            execute_time: Some(chrono::Utc::now().to_rfc3339()),
            created_at: chrono::Utc::now(),
        };

        repo::create_job_log(db, &log).await?;

        // Note: In a real implementation, this would trigger the actual job execution
        // For now, we just log the execution request

        Ok(())
    }

    /// List job logs with pagination
    pub async fn list_job_logs(
        &self,
        db: &PgPool,
        params: JobLogListParams,
    ) -> Result<JobLogListResponse, AppError> {
        let page = params.page.unwrap_or(1).max(1);
        let page_size = params.page_size.unwrap_or(10).min(100);

        let (logs, total) = repo::list_job_logs(db, &params).await?;

        Ok(JobLogListResponse {
            list: logs.into_iter().map(JobLog::from).collect(),
            total,
            page,
            page_size,
        })
    }

    /// Get job log by ID
    pub async fn get_job_log(&self, db: &PgPool, id: i64) -> Result<JobLog, AppError> {
        repo::get_job_log(db, id)
            .await?
            .map(JobLog::from)
            .ok_or_else(|| AppError::NotFound("Job log not found".to_string()))
    }

    /// Clear job logs
    pub async fn clear_job_logs(&self, db: &PgPool, job_id: Option<i64>) -> Result<u64, AppError> {
        repo::clear_job_logs(db, job_id).await
    }
}

impl Default for JobService {
    fn default() -> Self {
        Self::new()
    }
}
