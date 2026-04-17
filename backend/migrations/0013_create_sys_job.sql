--定时任务表
CREATE TABLE sys_job (
    id BIGSERIAL PRIMARY KEY,
    job_name VARCHAR(100) NOT NULL,
    job_code VARCHAR(100) NOT NULL UNIQUE,
    job_group VARCHAR(50) DEFAULT 'default',
    job_status INTEGER DEFAULT 0,
    cron_expression VARCHAR(100) NOT NULL,
    retry_count INTEGER DEFAULT 0,
    retry_interval INTEGER DEFAULT 60,
    timeout INTEGER DEFAULT 0,
    description TEXT,
    created_by VARCHAR(50),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_job_code ON sys_job(job_code);
CREATE INDEX idx_job_group ON sys_job(job_group);
CREATE INDEX idx_job_status ON sys_job(job_status);
CREATE INDEX idx_job_deleted ON sys_job(deleted_at);
