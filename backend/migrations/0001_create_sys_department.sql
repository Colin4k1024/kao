CREATE TABLE sys_department (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    parent_id UUID,
    ancestors TEXT,
    department_name VARCHAR(100) NOT NULL,
    display_order INTEGER DEFAULT 0,
    leader VARCHAR(50),
    phone VARCHAR(20),
    email VARCHAR(100),
    status INTEGER DEFAULT 1,
    created_by UUID,
    updated_by UUID,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    deleted_at TIMESTAMP WITH TIME ZONE,
    CONSTRAINT fk_dept_parent FOREIGN KEY (parent_id) REFERENCES sys_department(id)
);

CREATE INDEX idx_dept_parent ON sys_department(parent_id);
CREATE INDEX idx_dept_status ON sys_department(status);
CREATE INDEX idx_dept_ancestors ON sys_department(ancestors);
