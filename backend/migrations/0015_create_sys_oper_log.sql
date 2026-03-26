-- Operation Log Table
CREATE TABLE sys_oper_log (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    title VARCHAR(100),
    business_type VARCHAR(20),
    method VARCHAR(200),
    request_method VARCHAR(10),
    request_url VARCHAR(500),
    request_params TEXT,
    request_body TEXT,
    response_result TEXT,
    user_id UUID,
    username VARCHAR(50),
    ip_address VARCHAR(50),
    location VARCHAR(255),
    operate_status INTEGER DEFAULT 0,
    error_msg TEXT,
    operate_time TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_oper_log_user_id ON sys_oper_log(user_id);
CREATE INDEX idx_oper_log_business_type ON sys_oper_log(business_type);
CREATE INDEX idx_oper_log_operate_status ON sys_oper_log(operate_status);
CREATE INDEX idx_oper_log_operate_time ON sys_oper_log(operate_time);
