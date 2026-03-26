-- Scheduled Job Log Table
CREATE TABLE sys_job_log (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    job_id UUID NOT NULL,
    job_name VARCHAR(100) NOT NULL,
    job_group VARCHAR(50) NOT NULL,
    invoke_target VARCHAR(500) NOT NULL,
    job_message TEXT,
    status INTEGER DEFAULT 0,
    exception_info TEXT,
    start_time TIMESTAMP WITH TIME ZONE NOT NULL,
    end_time TIMESTAMP WITH TIME ZONE,
    execute_time INTEGER,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_job_log_job FOREIGN KEY (job_id) REFERENCES sys_job(id)
);

CREATE INDEX idx_job_log_job_id ON sys_job_log(job_id);
CREATE INDEX idx_job_log_status ON sys_job_log(status);
CREATE INDEX idx_job_log_start_time ON sys_job_log(start_time);
