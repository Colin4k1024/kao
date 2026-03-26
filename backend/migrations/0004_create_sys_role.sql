CREATE TABLE sys_role (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    role_name VARCHAR(100) NOT NULL,
    role_code VARCHAR(50) UNIQUE NOT NULL,
    display_order INTEGER DEFAULT 0,
    status INTEGER DEFAULT 1,
    role_type INTEGER DEFAULT 0,
    data_scope INTEGER DEFAULT 1,
    data_scope_depts TEXT,
    remark TEXT,
    created_by UUID,
    updated_by UUID,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    deleted_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_role_code ON sys_role(role_code);
CREATE INDEX idx_role_status ON sys_role(status);
