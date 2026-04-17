-- Create sys_posts table
CREATE TABLE sys_posts (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    dept_id UUID REFERENCES sys_departments(id) ON DELETE SET NULL,
    code VARCHAR(64) UNIQUE NOT NULL,
    name VARCHAR(128) NOT NULL,
    sort_order INTEGER DEFAULT 0,
    status VARCHAR(20) NOT NULL DEFAULT 'ACTIVE',
    created_by UUID,
    updated_by UUID,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP WITH TIME ZONE,
    CONSTRAINT fk_post_department FOREIGN KEY (dept_id) REFERENCES sys_departments(id)
);

CREATE INDEX idx_post_code ON sys_posts(code);
CREATE INDEX idx_post_dept ON sys_posts(dept_id);
CREATE INDEX idx_post_status ON sys_posts(status);
