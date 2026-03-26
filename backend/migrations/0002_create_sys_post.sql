CREATE TABLE sys_post (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    post_code VARCHAR(50) UNIQUE NOT NULL,
    post_name VARCHAR(100) NOT NULL,
    display_order INTEGER DEFAULT 0,
    status INTEGER DEFAULT 1,
    created_by UUID,
    updated_by UUID,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    deleted_at TIMESTAMP WITH TIME ZONE
);

CREATE INDEX idx_post_code ON sys_post(post_code);
CREATE INDEX idx_post_status ON sys_post(status);
