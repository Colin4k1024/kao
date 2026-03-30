-- Post/Position Table
CREATE TABLE sys_post (
    id BIGSERIAL PRIMARY KEY,
    post_name VARCHAR(50) NOT NULL,
    post_code VARCHAR(50) UNIQUE NOT NULL,
    post_group VARCHAR(50),
    sort INTEGER DEFAULT 0,
    status INTEGER DEFAULT 1,
    created_by VARCHAR(100),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX idx_post_code ON sys_post(post_code);
CREATE INDEX idx_post_status ON sys_post(status);
CREATE INDEX idx_post_sort ON sys_post(sort);
