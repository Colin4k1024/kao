-- Migration: Create indexes for sys_job and sys_job_log tables
-- Date: 2026-03-30

-- Indexes for sys_job
CREATE INDEX IF NOT EXISTS idx_sys_job_job_code ON sys_job(job_code);
CREATE INDEX IF NOT EXISTS idx_sys_job_job_group ON sys_job(job_group);
CREATE INDEX IF NOT EXISTS idx_sys_job_job_status ON sys_job(job_status);
CREATE INDEX IF NOT EXISTS idx_sys_job_created_at ON sys_job(created_at DESC);

-- Indexes for sys_job_log
CREATE INDEX IF NOT EXISTS idx_sys_job_log_job_id ON sys_job_log(job_id);
CREATE INDEX IF NOT EXISTS idx_sys_job_log_execute_status ON sys_job_log(execute_status);
CREATE INDEX IF NOT EXISTS idx_sys_job_log_created_at ON sys_job_log(created_at DESC);
CREATE INDEX IF NOT EXISTS idx_sys_job_log_job_id_created ON sys_job_log(job_id, created_at DESC);
