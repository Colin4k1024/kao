use sqlx::PgPool;
use crate::common::error::AppError;
use super::model::{JobRecord, JobLogRecord, CreateJobRequest, UpdateJobRequest, JobListParams, JobLogListParams};

// ============== Job Repository ==============

/// Create a new job
pub async fn create_job(pool: &PgPool, req: &CreateJobRequest, created_by: Option<&str>) -> Result<i64, AppError> {
    let job_code = req.job_code.clone();
    let job_group = req.job_group.clone();
    let job_status = req.job_status.unwrap_or(0);
    let retry_count = req.retry_count.unwrap_or(0);
    let retry_interval = req.retry_interval.unwrap_or(60);
    let timeout = req.timeout.unwrap_or(300);
    let description = req.description.clone();
    let job_name = req.job_name.clone();
    let cron_expression = req.cron_expression.clone();
    let created_by = created_by.map(String::from);

    let row: (i64,) = sqlx::query_as(
        r#"
        INSERT INTO sys_job (
            job_name, job_code, job_group, job_status,
            cron_expression, retry_count, retry_interval, timeout,
            description, created_by
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        RETURNING id
        "#
    )
    .bind(&job_name)
    .bind(&job_code)
    .bind(&job_group)
    .bind(job_status)
    .bind(&cron_expression)
    .bind(retry_count)
    .bind(retry_interval)
    .bind(timeout)
    .bind(&description)
    .bind(&created_by)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Internal(Some(format!("Failed to create job: {}", e))))?;

    Ok(row.0)
}

/// Get job by ID
pub async fn get_job(pool: &PgPool, id: i64) -> Result<Option<JobRecord>, AppError> {
    let job = sqlx::query_as::<_, JobRecord>(
        r#"
        SELECT
            id, job_name, job_code, job_group, job_status,
            cron_expression, retry_count, retry_interval, timeout,
            description, created_by, created_at, updated_at
        FROM sys_job
        WHERE id = $1
        "#
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::Internal(Some(format!("Failed to get job: {}", e))))?;

    Ok(job)
}

/// Get job by job_code
pub async fn get_job_by_code(pool: &PgPool, job_code: &str) -> Result<Option<JobRecord>, AppError> {
    let job = sqlx::query_as::<_, JobRecord>(
        r#"
        SELECT
            id, job_name, job_code, job_group, job_status,
            cron_expression, retry_count, retry_interval, timeout,
            description, created_by, created_at, updated_at
        FROM sys_job
        WHERE job_code = $1
        "#
    )
    .bind(job_code)
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::Internal(Some(format!("Failed to get job by code: {}", e))))?;

    Ok(job)
}

/// Update job
pub async fn update_job(pool: &PgPool, id: i64, req: &UpdateJobRequest) -> Result<(), AppError> {
    // Build dynamic update query
    let job = get_job(pool, id).await?.ok_or_else(|| AppError::NotFound("Job not found".to_string()))?;

    let job_name = req.job_name.clone().unwrap_or(job.job_name);
    let job_group = req.job_group.clone().unwrap_or(job.job_group);
    let job_status = req.job_status.unwrap_or(job.job_status);
    let cron_expression = req.cron_expression.clone().unwrap_or(job.cron_expression);
    let retry_count = req.retry_count.unwrap_or(job.retry_count);
    let retry_interval = req.retry_interval.unwrap_or(job.retry_interval);
    let timeout = req.timeout.unwrap_or(job.timeout);
    let description = req.description.clone().or(job.description);

    sqlx::query(
        r#"
        UPDATE sys_job
        SET job_name = $1, job_group = $2, job_status = $3,
            cron_expression = $4, retry_count = $5, retry_interval = $6,
            timeout = $7, description = $8, updated_at = NOW()
        WHERE id = $9
        "#
    )
    .bind(&job_name)
    .bind(&job_group)
    .bind(job_status)
    .bind(&cron_expression)
    .bind(retry_count)
    .bind(retry_interval)
    .bind(timeout)
    .bind(&description)
    .bind(id)
    .execute(pool)
    .await
    .map_err(|e| AppError::Internal(Some(format!("Failed to update job: {}", e))))?;

    Ok(())
}

/// Delete job
pub async fn delete_job(pool: &PgPool, id: i64) -> Result<(), AppError> {
    sqlx::query("DELETE FROM sys_job WHERE id = $1")
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| AppError::Internal(Some(format!("Failed to delete job: {}", e))))?;

    Ok(())
}

/// List jobs with pagination
pub async fn list_jobs(pool: &PgPool, params: &JobListParams) -> Result<(Vec<JobRecord>, i64), AppError> {
    let page = params.page.unwrap_or(1).max(1);
    let page_size = params.page_size.unwrap_or(10).min(100);
    let offset = (page - 1) * page_size;

    // Whitelist of allowed filter fields - column names are hardcoded, never from user input
    let job_name = params.job_name.as_ref();
    let job_code = params.job_code.as_ref();
    let job_status = params.job_status;

    // Build parameterized WHERE clause
    let (where_clause, bindings): (String, Vec<String>) = {
        let mut clauses = Vec::new();
        let mut vals = Vec::new();

        if job_name.is_some() {
            clauses.push("job_name LIKE $1".to_string());
            vals.push(format!("%{}%", job_name.unwrap()));
        }
        if job_code.is_some() {
            clauses.push(format!("job_code = ${}", vals.len() + 1));
            vals.push(job_code.unwrap().clone());
        }
        if let Some(status) = job_status {
            clauses.push(format!("job_status = ${}", vals.len() + 1));
            vals.push(status.to_string());
        }

        let where_clause = if clauses.is_empty() {
            String::new()
        } else {
            format!("WHERE {}", clauses.join(" AND "))
        };
        (where_clause, vals)
    };

    // Count total
    let count_query = format!("SELECT COUNT(*) FROM sys_job {}", where_clause);
    let total: i64 = {
        let mut q = sqlx::query_scalar::<_, i64>(&count_query);
        if let Some(ref name) = job_name {
            q = q.bind(format!("%{}%", name));
        }
        if let Some(ref code) = job_code {
            q = q.bind(code);
        }
        if let Some(status) = job_status {
            q = q.bind(status);
        }
        q.fetch_one(pool).await.map_err(|e| AppError::Internal(Some(format!("Failed to count jobs: {}", e))))?
    };

    // Fetch list
    let list_query = format!(
        r#"
        SELECT
            id, job_name, job_code, job_group, job_status,
            cron_expression, retry_count, retry_interval, timeout,
            description, created_by, created_at, updated_at
        FROM sys_job
        {}
        ORDER BY id DESC
        LIMIT $1 OFFSET $2
        "#,
        where_clause
    );

    let mut q = sqlx::query_as::<_, JobRecord>(&list_query).bind(page_size).bind(offset);
    if let Some(ref name) = job_name {
        q = q.bind(format!("%{}%", name));
    }
    if let Some(ref code) = job_code {
        q = q.bind(code);
    }
    if let Some(status) = job_status {
        q = q.bind(status);
    }

    let jobs = q.fetch_all(pool)
        .await
        .map_err(|e| AppError::Internal(Some(format!("Failed to list jobs: {}", e))))?;

    Ok((jobs, total))
}

/// Update job status
pub async fn update_job_status(pool: &PgPool, id: i64, status: i32) -> Result<(), AppError> {
    sqlx::query("UPDATE sys_job SET job_status = $1, updated_at = NOW() WHERE id = $2")
        .bind(status)
        .bind(id)
        .execute(pool)
        .await
        .map_err(|e| AppError::Internal(Some(format!("Failed to update job status: {}", e))))?;

    Ok(())
}

// ============== Job Log Repository ==============

/// Create a job log entry
pub async fn create_job_log(pool: &PgPool, log: &JobLogRecord) -> Result<i64, AppError> {
    let row: (i64,) = sqlx::query_as(
        r#"
        INSERT INTO sys_job_log (
            job_id, job_name, job_code, job_group,
            execute_status, execute_message, execute_time
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        RETURNING id
        "#
    )
    .bind(log.job_id)
    .bind(&log.job_name)
    .bind(&log.job_code)
    .bind(&log.job_group)
    .bind(log.execute_status)
    .bind(&log.execute_message)
    .bind(&log.execute_time)
    .fetch_one(pool)
    .await
    .map_err(|e| AppError::Internal(Some(format!("Failed to create job log: {}", e))))?;

    Ok(row.0)
}

/// List job logs with pagination
pub async fn list_job_logs(pool: &PgPool, params: &JobLogListParams) -> Result<(Vec<JobLogRecord>, i64), AppError> {
    let page = params.page.unwrap_or(1).max(1);
    let page_size = params.page_size.unwrap_or(10).min(100);
    let offset = (page - 1) * page_size;

    // Build WHERE clause
    let mut conditions = Vec::new();
    let mut param_idx = 1;

    if let Some(job_id) = params.job_id {
        conditions.push(format!("job_id = ${}", param_idx));
        param_idx += 1;
    }
    if params.job_name.is_some() {
        conditions.push(format!("job_name LIKE ${}", param_idx));
        param_idx += 1;
    }
    if let Some(status) = params.execute_status {
        conditions.push(format!("execute_status = ${}", param_idx));
        param_idx += 1;
    }

    let where_clause = if conditions.is_empty() {
        String::new()
    } else {
        format!("WHERE {}", conditions.join(" AND "))
    };

    // Count total
    let count_query = format!("SELECT COUNT(*) FROM sys_job_log {}", where_clause);
    let total: i64 = {
        let mut q = sqlx::query_scalar::<_, i64>(&count_query);
        if let Some(job_id) = params.job_id {
            q = q.bind(job_id);
        }
        if let Some(ref name) = params.job_name {
            q = q.bind(format!("%{}%", name));
        }
        if let Some(status) = params.execute_status {
            q = q.bind(status);
        }
        q.fetch_one(pool).await.map_err(|e| AppError::Internal(Some(format!("Failed to count job logs: {}", e))))?
    };

    // Fetch list
    let list_query = format!(
        r#"
        SELECT
            id, job_id, job_name, job_code, job_group,
            execute_status, execute_message, execute_time, created_at
        FROM sys_job_log
        {}
        ORDER BY id DESC
        LIMIT $1 OFFSET $2
        "#,
        where_clause
    );

    let mut q = sqlx::query_as::<_, JobLogRecord>(&list_query).bind(page_size).bind(offset);
    if let Some(job_id) = params.job_id {
        q = q.bind(job_id);
    }
    if let Some(ref name) = params.job_name {
        q = q.bind(format!("%{}%", name));
    }
    if let Some(status) = params.execute_status {
        q = q.bind(status);
    }

    let logs = q.fetch_all(pool)
        .await
        .map_err(|e| AppError::Internal(Some(format!("Failed to list job logs: {}", e))))?;

    Ok((logs, total))
}

/// Get job log by ID
pub async fn get_job_log(pool: &PgPool, id: i64) -> Result<Option<JobLogRecord>, AppError> {
    let log = sqlx::query_as::<_, JobLogRecord>(
        r#"
        SELECT
            id, job_id, job_name, job_code, job_group,
            execute_status, execute_message, execute_time, created_at
        FROM sys_job_log
        WHERE id = $1
        "#
    )
    .bind(id)
    .fetch_optional(pool)
    .await
    .map_err(|e| AppError::Internal(Some(format!("Failed to get job log: {}", e))))?;

    Ok(log)
}

/// Clear job logs
pub async fn clear_job_logs(pool: &PgPool, job_id: Option<i64>) -> Result<u64, AppError> {
    let result = if let Some(id) = job_id {
        sqlx::query("DELETE FROM sys_job_log WHERE job_id = $1")
            .bind(id)
            .execute(pool)
            .await
    } else {
        sqlx::query("DELETE FROM sys_job_log")
            .execute(pool)
            .await
    }
    .map_err(|e| AppError::Internal(Some(format!("Failed to clear job logs: {}", e))))?;

    Ok(result.rows_affected())
}

/// List all running jobs (status = 1)
pub async fn list_running_jobs(pool: &PgPool) -> Result<Vec<JobRecord>, AppError> {
    let jobs = sqlx::query_as::<_, JobRecord>(
        r#"
        SELECT
            id, job_name, job_code, job_group, job_status,
            cron_expression, retry_count, retry_interval, timeout,
            description, created_by, created_at, updated_at
        FROM sys_job
        WHERE job_status = 1
        "#
    )
    .fetch_all(pool)
    .await
    .map_err(|e| AppError::Internal(Some(format!("Failed to list running jobs: {}", e))))?;

    Ok(jobs)
}

/// Update a job log entry
pub async fn update_job_log(pool: &PgPool, id: i64, log: &JobLogRecord) -> Result<(), AppError> {
    sqlx::query(
        r#"
        UPDATE sys_job_log
        SET execute_status = $1, execute_message = $2, execute_time = $3
        WHERE id = $4
        "#
    )
    .bind(log.execute_status)
    .bind(&log.execute_message)
    .bind(&log.execute_time)
    .bind(id)
    .execute(pool)
    .await
    .map_err(|e| AppError::Internal(Some(format!("Failed to update job log: {}", e))))?;

    Ok(())
}
