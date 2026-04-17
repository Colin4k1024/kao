CREATE TABLE sys_roles_menu (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    role_id UUID NOT NULL,
    menu_id UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    CONSTRAINT fk_rm_role FOREIGN KEY (role_id) REFERENCES sys_roles(id),
    CONSTRAINT fk_rm_menu FOREIGN KEY (menu_id) REFERENCES sys_menus(id),
    UNIQUE(role_id, menu_id)
);

CREATE INDEX idx_rm_role ON sys_roles_menu(role_id);
CREATE INDEX idx_rm_menu ON sys_roles_menu(menu_id);
