CREATE TABLE sys_user (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,
    nickname VARCHAR(100),
    email VARCHAR(100),
    phone VARCHAR(20),
    avatar TEXT,
    status INTEGER DEFAULT 1,
    login_ip VARCHAR(50),
    login_date TIMESTAMP WITH TIME ZONE,
    department_id UUID,
    post_id UUID,
    created_by UUID,
    updated_by UUID,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    deleted_at TIMESTAMP WITH TIME ZONE,
    CONSTRAINT fk_user_department FOREIGN KEY (department_id) REFERENCES sys_department(id),
    CONSTRAINT fk_user_post FOREIGN KEY (post_id) REFERENCES sys_post(id)
);

CREATE INDEX idx_user_username ON sys_user(username);
CREATE INDEX idx_user_department ON sys_user(department_id);
CREATE INDEX idx_user_status ON sys_user(status);
