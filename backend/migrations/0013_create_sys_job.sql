-- Scheduled Job Table
CREATE TABLE sys_job (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    job_name VARCHAR(100) NOT NULL,
    job_group VARCHAR(50) NOT NULL,
    job_type VARCHAR(20) DEFAULT 'http',
    invoke_target VARCHAR(500) NOT NULL,
    cron_expression VARCHAR(50),
    misfire_policy VARCHAR(20) DEFAULT '1',
    concurrent CHAR(1) DEFAULT '1',
    status INTEGER DEFAULT 0,
    remark TEXT,
    created_by UUID,
    updated_by UUID,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_job_group ON sys_job(job_group);
CREATE INDEX idx_job_status ON sys_job(status);
