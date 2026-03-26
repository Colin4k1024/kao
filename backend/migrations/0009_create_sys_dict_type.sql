-- Dictionary Type Table
CREATE TABLE sys_dict_type (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    dict_name VARCHAR(100) NOT NULL,
    dict_type VARCHAR(100) UNIQUE NOT NULL,
    status INTEGER DEFAULT 1,
    remark TEXT,
    created_by UUID,
    updated_by UUID,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_dict_type_dict_type ON sys_dict_type(dict_type);
CREATE INDEX idx_dict_type_status ON sys_dict_type(status);
