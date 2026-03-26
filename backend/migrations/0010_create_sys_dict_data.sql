-- Dictionary Data Table
CREATE TABLE sys_dict_data (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    dict_sort INTEGER DEFAULT 0,
    dict_label VARCHAR(100) NOT NULL,
    dict_value VARCHAR(100) NOT NULL,
    dict_type VARCHAR(100) NOT NULL,
    css_class VARCHAR(100),
    list_class VARCHAR(100),
    is_default CHAR(1) DEFAULT 'N',
    status INTEGER DEFAULT 1,
    remark TEXT,
    created_by UUID,
    updated_by UUID,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP WITH TIME ZONE,
    CONSTRAINT fk_dict_data_type FOREIGN KEY (dict_type) REFERENCES sys_dict_type(dict_type)
);

CREATE INDEX idx_dict_data_type ON sys_dict_data(dict_type);
CREATE INDEX idx_dict_data_status ON sys_dict_data(status);
CREATE INDEX idx_dict_data_sort ON sys_dict_data(dict_sort);
