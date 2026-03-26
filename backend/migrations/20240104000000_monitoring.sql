CREATE TABLE IF NOT EXISTS sys_oper_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    username VARCHAR(64) NOT NULL,
    module VARCHAR(64) NOT NULL,
    action_type VARCHAR(32) NOT NULL,
    method VARCHAR(16) NOT NULL,
    path VARCHAR(255) NOT NULL,
    request_method VARCHAR(10) NOT NULL,
    request_params TEXT,
    response_code INTEGER NOT NULL,
    response_message TEXT,
    execution_time BIGINT NOT NULL DEFAULT 0,
    ip_address VARCHAR(45) NOT NULL,
    user_agent TEXT,
    status INTEGER NOT NULL DEFAULT 1,
    create_time TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW(),
    update_time TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_oper_log_user_id ON sys_oper_log(user_id);
CREATE INDEX idx_oper_log_create_time ON sys_oper_log(create_time);
CREATE INDEX idx_oper_log_module ON sys_oper_log(module);
CREATE INDEX idx_oper_log_action_type ON sys_oper_log(action_type);
CREATE INDEX idx_oper_log_status ON sys_oper_log(status);

-- Create sys_login_log table for login logs
CREATE TABLE IF NOT EXISTS sys_login_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    username VARCHAR(64) NOT NULL,
    ip_address VARCHAR(45) NOT NULL,
    user_agent TEXT,
    status INTEGER NOT NULL DEFAULT 1,
    message TEXT,
    login_time TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW(),
    create_time TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW(),
    update_time TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_login_log_user_id ON sys_login_log(user_id);
CREATE INDEX idx_login_log_create_time ON sys_login_log(create_time);
CREATE INDEX idx_login_log_status ON sys_login_log(status);

-- Create sys_online_user table for tracking active sessions
CREATE TABLE IF NOT EXISTS sys_online_user (
    session_id VARCHAR(255) PRIMARY KEY,
    user_id UUID NOT NULL,
    username VARCHAR(64) NOT NULL,
    dept_name VARCHAR(64),
    ip_address VARCHAR(45) NOT NULL,
    user_agent TEXT,
    login_time TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW(),
    last_activity_time TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW(),
    expire_time TIMESTAMP WITHOUT TIME ZONE NOT NULL,
    status INTEGER NOT NULL DEFAULT 1,
    remark TEXT,
    create_time TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW(),
    update_time TIMESTAMP WITHOUT TIME ZONE NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_online_user_user_id ON sys_online_user(user_id);
CREATE INDEX idx_online_user_expire_time ON sys_online_user(expire_time);
CREATE INDEX idx_online_user_status ON sys_online_user(status);

-- Sample operation logs for testing
INSERT INTO sys_oper_log (id, user_id, username, module, action_type, method, path, request_method, response_code, execution_time, ip_address, status)
VALUES 
    ('a1b2c3d4-e5f6-7890-abcd-ef1234567890', '11111111-1111-1111-1111-111111111111', 'admin', '用户管理', '查询用户列表', 'GET', '/api/system/users', 'GET', 200, 45, '127.0.0.1', 1),
    ('b2c3d4e5-f6a7-8901-bcde-f12345678901', '11111111-1111-1111-1111-111111111111', 'admin', '角色管理', '查询角色列表', 'GET', '/api/system/roles', 'GET', 200, 32, '127.0.0.1', 1),
    ('c3d4e5f6-a7b8-9012-cdef-123456789012', '11111111-1111-1111-1111-111111111111', 'admin', '菜单管理', '查询菜单列表', 'GET', '/api/system/menus', 'GET', 200, 28, '127.0.0.1', 1),
    ('d4e5f6a7-b8c9-0123-defa-234567890123', '11111111-1111-1111-1111-111111111111', 'admin', '字典管理', '查询字典类型', 'GET', '/api/system/dictionary/types', 'GET', 200, 38, '127.0.0.1', 1),
    ('e5f6a7b8-c9d0-1234-efab-345678901234', '11111111-1111-1111-1111-111111111111', 'admin', '系统监控', '查看在线用户', 'GET', '/api/system/online/users', 'GET', 200, 22, '127.0.0.1', 1);

-- Sample login logs for testing
INSERT INTO sys_login_log (id, user_id, username, ip_address, status, message)
VALUES 
    ('f1a2b3c4-d5e6-7890-abcd-ef1234567890', '11111111-1111-1111-1111-111111111111', 'admin', '127.0.0.1', 1, '登录成功'),
    ('f2a3b4c5-d6e7-8901-bcde-f12345678901', '22222222-2222-2222-2222-222222222222', 'test_user', '192.168.1.100', 1, '登录成功'),
    ('f3a4b5c6-d7e8-9012-cdef-123456789012', '33333333-3333-3333-3333-333333333333', 'guest', '192.168.1.101', 0, '用户名或密码错误'),
    ('f4a5b6c7-d8e9-0123-defa-234567890123', '11111111-1111-1111-1111-111111111111', 'admin', '127.0.0.1', 1, '登录成功'),
    ('f5a6b7c8-d9e0-1234-efab-345678901234', '44444444-4444-4444-4444-444444444444', 'unknown_user', '10.0.0.1', 0, '用户不存在');

-- Sample online users for testing
INSERT INTO sys_online_user (session_id, user_id, username, ip_address, expire_time, status)
VALUES 
    ('sess_001', '11111111-1111-1111-1111-111111111111', 'admin', '127.0.0.1', NOW() + INTERVAL '1 hour', 1),
    ('sess_002', '22222222-2222-2222-2222-222222222222', 'test_user', '192.168.1.100', NOW() + INTERVAL '1 hour', 1),
    ('sess_003', '11111111-1111-1111-1111-111111111111', 'admin', '192.168.1.1', NOW() + INTERVAL '30 minutes', 1);
