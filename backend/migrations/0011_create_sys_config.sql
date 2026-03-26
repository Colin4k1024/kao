-- System Configuration Table
CREATE TABLE sys_config (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    config_name VARCHAR(100) NOT NULL,
    config_key VARCHAR(100) UNIQUE NOT NULL,
    config_value TEXT NOT NULL,
    config_type VARCHAR(10) DEFAULT 'Y',
    is_encrypt CHAR(1) DEFAULT 'N',
    status INTEGER DEFAULT 1,
    remark TEXT,
    created_by UUID,
    updated_by UUID,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_config_key ON sys_config(config_key);
CREATE INDEX idx_config_status ON sys_config(status);
