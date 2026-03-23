BEGIN;

CREATE OR REPLACE FUNCTION set_updated_at_timestamp()
RETURNS TRIGGER
LANGUAGE plpgsql
AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$;

CREATE TABLE sys_departments (
  id UUID PRIMARY KEY,
  code VARCHAR(64) NOT NULL UNIQUE,
  name VARCHAR(128) NOT NULL,
  parent_id UUID NULL REFERENCES sys_departments(id) ON DELETE SET NULL,
  ancestors TEXT NOT NULL DEFAULT '',
  path TEXT NOT NULL DEFAULT '',
  sort_order INTEGER NOT NULL DEFAULT 0,
  leader VARCHAR(64),
  phone VARCHAR(32),
  email VARCHAR(255),
  status VARCHAR(16) NOT NULL DEFAULT 'ACTIVE' CHECK (status IN ('ACTIVE', 'DISABLED')),
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE sys_roles (
  id UUID PRIMARY KEY,
  code VARCHAR(64) NOT NULL UNIQUE,
  name VARCHAR(128) NOT NULL,
  description TEXT,
  data_scope VARCHAR(32) NOT NULL DEFAULT 'DEPT' CHECK (data_scope IN ('ALL', 'CUSTOM', 'DEPT', 'DEPT_AND_CHILD', 'SELF')),
  status VARCHAR(16) NOT NULL DEFAULT 'ACTIVE' CHECK (status IN ('ACTIVE', 'DISABLED')),
  is_system BOOLEAN NOT NULL DEFAULT FALSE,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE sys_users (
  id UUID PRIMARY KEY,
  username VARCHAR(64) NOT NULL UNIQUE,
  email VARCHAR(255) UNIQUE,
  display_name VARCHAR(128) NOT NULL,
  password_hash VARCHAR(255) NOT NULL,
  avatar_url VARCHAR(255),
  phone VARCHAR(32),
  dept_id UUID REFERENCES sys_departments(id) ON DELETE SET NULL,
  status VARCHAR(16) NOT NULL DEFAULT 'ACTIVE' CHECK (status IN ('ACTIVE', 'DISABLED', 'LOCKED')),
  is_super_admin BOOLEAN NOT NULL DEFAULT FALSE,
  last_login_at TIMESTAMPTZ,
  last_login_ip VARCHAR(64),
  password_changed_at TIMESTAMPTZ,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  deleted_at TIMESTAMPTZ
);

CREATE TABLE sys_menus (
  id UUID PRIMARY KEY,
  parent_id UUID NULL REFERENCES sys_menus(id) ON DELETE SET NULL,
  name VARCHAR(128) NOT NULL,
  menu_type VARCHAR(16) NOT NULL CHECK (menu_type IN ('DIRECTORY', 'MENU', 'BUTTON')),
  route_path VARCHAR(255),
  component VARCHAR(255),
  permission VARCHAR(128),
  icon VARCHAR(64),
  sort_order INTEGER NOT NULL DEFAULT 0,
  visible BOOLEAN NOT NULL DEFAULT TRUE,
  keep_alive BOOLEAN NOT NULL DEFAULT FALSE,
  status VARCHAR(16) NOT NULL DEFAULT 'ACTIVE' CHECK (status IN ('ACTIVE', 'DISABLED')),
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE sys_user_roles (
  user_id UUID NOT NULL REFERENCES sys_users(id) ON DELETE CASCADE,
  role_id UUID NOT NULL REFERENCES sys_roles(id) ON DELETE CASCADE,
  assigned_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY (user_id, role_id)
);

CREATE TABLE sys_role_menus (
  role_id UUID NOT NULL REFERENCES sys_roles(id) ON DELETE CASCADE,
  menu_id UUID NOT NULL REFERENCES sys_menus(id) ON DELETE CASCADE,
  assigned_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY (role_id, menu_id)
);

CREATE TABLE sys_role_departments (
  role_id UUID NOT NULL REFERENCES sys_roles(id) ON DELETE CASCADE,
  dept_id UUID NOT NULL REFERENCES sys_departments(id) ON DELETE CASCADE,
  assigned_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  PRIMARY KEY (role_id, dept_id)
);

CREATE INDEX idx_sys_departments_parent_id ON sys_departments(parent_id);
CREATE INDEX idx_sys_departments_ancestors ON sys_departments(ancestors);
CREATE INDEX idx_sys_departments_ancestors_pattern ON sys_departments(ancestors text_pattern_ops);
CREATE INDEX idx_sys_departments_status ON sys_departments(status);

CREATE INDEX idx_sys_roles_status ON sys_roles(status);
CREATE INDEX idx_sys_roles_data_scope ON sys_roles(data_scope);

CREATE INDEX idx_sys_users_dept_id ON sys_users(dept_id);
CREATE INDEX idx_sys_users_status ON sys_users(status);
CREATE INDEX idx_sys_users_is_super_admin ON sys_users(is_super_admin);

CREATE INDEX idx_sys_menus_parent_id ON sys_menus(parent_id);
CREATE INDEX idx_sys_menus_menu_type ON sys_menus(menu_type);
CREATE INDEX idx_sys_menus_status ON sys_menus(status);
CREATE INDEX idx_sys_menus_route_path ON sys_menus(route_path);
CREATE UNIQUE INDEX uq_sys_menus_permission ON sys_menus(permission);

CREATE TRIGGER trg_sys_departments_updated_at
BEFORE UPDATE ON sys_departments
FOR EACH ROW
EXECUTE FUNCTION set_updated_at_timestamp();

CREATE TRIGGER trg_sys_roles_updated_at
BEFORE UPDATE ON sys_roles
FOR EACH ROW
EXECUTE FUNCTION set_updated_at_timestamp();

CREATE TRIGGER trg_sys_users_updated_at
BEFORE UPDATE ON sys_users
FOR EACH ROW
EXECUTE FUNCTION set_updated_at_timestamp();

CREATE TRIGGER trg_sys_menus_updated_at
BEFORE UPDATE ON sys_menus
FOR EACH ROW
EXECUTE FUNCTION set_updated_at_timestamp();

COMMIT;
