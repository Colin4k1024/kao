CREATE TABLE sys_role_department (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    role_id UUID NOT NULL,
    department_id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    CONSTRAINT fk_rd_role FOREIGN KEY (role_id) REFERENCES sys_role(id),
    CONSTRAINT fk_rd_dept FOREIGN KEY (department_id) REFERENCES sys_department(id),
    UNIQUE(role_id, department_id)
);

CREATE INDEX idx_rd_role ON sys_role_department(role_id);
CREATE INDEX idx_rd_dept ON sys_role_department(department_id);
