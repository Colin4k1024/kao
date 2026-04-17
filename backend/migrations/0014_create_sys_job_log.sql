--定时任务执行日志表
CREATE TABLE sys_job_log (
    id BIGSERIAL PRIMARY KEY,
    job_id BIGINT NOT NULL,
    execute_status INTEGER DEFAULT 0,
    execute_message TEXT,
    execute_time INTEGER DEFAULT 0,
    trigger_time TIMESTAMP WITH TIME ZONE,
    start_time TIMESTAMP WITH TIME ZONE,
    end_time TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_job_log_job_id ON sys_job_log(job_id);
CREATE INDEX idx_job_log_status ON sys_job_log(execute_status);
CREATE INDEX idx_job_log_trigger_time ON sys_job_log(trigger_time);
