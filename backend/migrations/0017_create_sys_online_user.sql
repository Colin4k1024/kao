-- Online User Table
CREATE TABLE sys_online_user (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    session_id VARCHAR(100) UNIQUE NOT NULL,
    user_id UUID,
    username VARCHAR(50),
    ip_address VARCHAR(50),
    login_location VARCHAR(255),
    browser VARCHAR(100),
    os VARCHAR(100),
    login_time TIMESTAMP WITH TIME ZONE NOT NULL,
    last_access_time TIMESTAMP WITH TIME ZONE,
    expire_time TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_online_user_session_id ON sys_online_user(session_id);
CREATE INDEX idx_online_user_user_id ON sys_online_user(user_id);
CREATE INDEX idx_online_user_expire_time ON sys_online_user(expire_time);
