CREATE TABLE sys_users_role (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL,
    role_id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    CONSTRAINT fk_ur_user FOREIGN KEY (user_id) REFERENCES sys_users(id),
    CONSTRAINT fk_ur_role FOREIGN KEY (role_id) REFERENCES sys_roles(id),
    UNIQUE(user_id, role_id)
);

CREATE INDEX idx_ur_user ON sys_users_role(user_id);
CREATE INDEX idx_ur_role ON sys_users_role(role_id);
