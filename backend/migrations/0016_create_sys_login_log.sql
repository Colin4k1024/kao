-- Login Log Table
CREATE TABLE sys_login_log (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID,
    username VARCHAR(50),
    login_type VARCHAR(10),
    ip_address VARCHAR(50),
    login_location VARCHAR(255),
    browser VARCHAR(100),
    os VARCHAR(100),
    status INTEGER DEFAULT 0,
    msg TEXT,
    login_time TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_login_log_user_id ON sys_login_log(user_id);
CREATE INDEX idx_login_log_username ON sys_login_log(username);
CREATE INDEX idx_login_log_status ON sys_login_log(status);
CREATE INDEX idx_login_log_login_time ON sys_login_log(login_time);
