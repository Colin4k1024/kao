-- Migration: Create sys_job and sys_job_log tables
-- Date: 2026-03-30

-- Create sys_job table
CREATE TABLE IF NOT EXISTS sys_job (
    id BIGSERIAL PRIMARY KEY,
    job_name VARCHAR(100) NOT NULL,
    job_code VARCHAR(100) NOT NULL UNIQUE,
    job_group VARCHAR(50) NOT NULL DEFAULT 'DEFAULT',
    job_status INTEGER NOT NULL DEFAULT 0,
    cron_expression VARCHAR(100) NOT NULL,
    retry_count INTEGER NOT NULL DEFAULT 0,
    retry_interval INTEGER NOT NULL DEFAULT 60,
    timeout INTEGER NOT NULL DEFAULT 300,
    description TEXT,
    created_by VARCHAR(100),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Create sys_job_log table
CREATE TABLE IF NOT EXISTS sys_job_log (
    id BIGSERIAL PRIMARY KEY,
    job_id BIGINT NOT NULL,
    job_name VARCHAR(100) NOT NULL,
    job_code VARCHAR(100) NOT NULL,
    job_group VARCHAR(50) NOT NULL,
    execute_status INTEGER NOT NULL DEFAULT 0,
    execute_message TEXT,
    execute_time VARCHAR(50),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

-- Add comments
COMMENT ON TABLE sys_job IS 'Scheduled job configuration table';
COMMENT ON COLUMN sys_job.job_code IS 'Unique job identifier, used for API calls';
COMMENT ON COLUMN sys_job.job_status IS '0=stopped, 1=running';
COMMENT ON COLUMN sys_job.cron_expression IS 'Cron expression (6-7 fields: second minute hour day month week [year])';
COMMENT ON COLUMN sys_job.retry_count IS 'Number of retries on failure';
COMMENT ON COLUMN sys_job.retry_interval IS 'Retry interval in seconds';
COMMENT ON COLUMN sys_job.timeout IS 'Job execution timeout in seconds';

COMMENT ON TABLE sys_job_log IS 'Job execution log table';
COMMENT ON COLUMN sys_job_log.execute_status IS '0=pending, 1=running, 2=success, 3=failed';
