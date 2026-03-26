CREATE TABLE sys_menu (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    parent_id UUID,
    menu_name VARCHAR(100) NOT NULL,
    menu_type CHAR(1) NOT NULL,
    icon VARCHAR(100),
    route_name VARCHAR(100),
    route_path VARCHAR(255),
    component VARCHAR(255),
    permission VARCHAR(100),
    display_order INTEGER DEFAULT 0,
    is_cache CHAR(1) DEFAULT '0',
    is_visible CHAR(1) DEFAULT '1',
    status INTEGER DEFAULT 1,
    ancestors TEXT,
    created_by UUID,
    updated_by UUID,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    deleted_at TIMESTAMP WITH TIME ZONE,
    CONSTRAINT fk_menu_parent FOREIGN KEY (parent_id) REFERENCES sys_menu(id)
);

CREATE INDEX idx_menu_parent ON sys_menu(parent_id);
CREATE INDEX idx_menu_type ON sys_menu(menu_type);
CREATE INDEX idx_menu_status ON sys_menu(status);
CREATE INDEX idx_menu_ancestors ON sys_menu(ancestors);
