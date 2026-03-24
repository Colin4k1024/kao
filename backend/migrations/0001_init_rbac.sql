-- RBAC Schema Migration

-- Enable UUID extension
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Create sys_departments table
CREATE TABLE sys_departments (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    code VARCHAR(64) UNIQUE NOT NULL,
    name VARCHAR(128) NOT NULL,
    parent_id UUID REFERENCES sys_departments(id) ON DELETE SET NULL,
    ancestors TEXT DEFAULT '',
    path VARCHAR(255) DEFAULT '',
    sort_order INTEGER DEFAULT 0,
    leader VARCHAR(64),
    phone VARCHAR(32),
    email VARCHAR(255),
    status VARCHAR(20) NOT NULL DEFAULT 'ACTIVE',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create sys_roles table
CREATE TABLE sys_roles (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    code VARCHAR(64) UNIQUE NOT NULL,
    name VARCHAR(128) NOT NULL,
    description TEXT,
    data_scope VARCHAR(20) NOT NULL DEFAULT 'DEPT', -- ALL, CUSTOM, DEPT, DEPT_AND_CHILD, SELF
    status VARCHAR(20) NOT NULL DEFAULT 'ACTIVE',
    is_system BOOLEAN DEFAULT FALSE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create sys_users table
CREATE TABLE sys_users (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    username VARCHAR(64) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE,
    display_name VARCHAR(128) NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    avatar_url VARCHAR(255),
    phone VARCHAR(32),
    dept_id UUID REFERENCES sys_departments(id) ON DELETE SET NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'ACTIVE',
    is_super_admin BOOLEAN DEFAULT FALSE,
    last_login_at TIMESTAMP WITH TIME ZONE,
    last_login_ip VARCHAR(64),
    password_changed_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    deleted_at TIMESTAMP WITH TIME ZONE
);

-- Create sys_menus table
CREATE TABLE sys_menus (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    parent_id UUID REFERENCES sys_menus(id) ON DELETE SET NULL,
    name VARCHAR(128) NOT NULL,
    menu_type VARCHAR(20) NOT NULL, -- DIRECTORY, MENU, BUTTON
    route_path VARCHAR(255),
    component VARCHAR(255),
    permission VARCHAR(128) UNIQUE,
    icon VARCHAR(64),
    sort_order INTEGER DEFAULT 0,
    visible BOOLEAN DEFAULT TRUE,
    keep_alive BOOLEAN DEFAULT FALSE,
    status VARCHAR(20) NOT NULL DEFAULT 'ACTIVE',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);

-- Create sys_user_roles junction table
CREATE TABLE sys_user_roles (
    user_id UUID NOT NULL REFERENCES sys_users(id) ON DELETE CASCADE,
    role_id UUID NOT NULL REFERENCES sys_roles(id) ON DELETE CASCADE,
    assigned_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (user_id, role_id)
);

-- Create sys_role_menus junction table
CREATE TABLE sys_role_menus (
    role_id UUID NOT NULL REFERENCES sys_roles(id) ON DELETE CASCADE,
    menu_id UUID NOT NULL REFERENCES sys_menus(id) ON DELETE CASCADE,
    assigned_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (role_id, menu_id)
);

-- Create sys_role_departments junction table
CREATE TABLE sys_role_departments (
    role_id UUID NOT NULL REFERENCES sys_roles(id) ON DELETE CASCADE,
    dept_id UUID NOT NULL REFERENCES sys_departments(id) ON DELETE CASCADE,
    assigned_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    PRIMARY KEY (role_id, dept_id)
);

-- Create indexes for better performance
CREATE INDEX idx_sys_departments_parent_id ON sys_departments(parent_id);
CREATE INDEX idx_sys_departments_ancestors ON sys_departments(ancestors);
CREATE INDEX idx_sys_users_dept_id ON sys_users(dept_id);
CREATE INDEX idx_sys_users_status ON sys_users(status);
CREATE INDEX idx_sys_users_is_super_admin ON sys_users(is_super_admin);
CREATE INDEX idx_sys_roles_status ON sys_roles(status);
CREATE INDEX idx_sys_roles_data_scope ON sys_roles(data_scope);
CREATE INDEX idx_sys_menus_parent_id ON sys_menus(parent_id);
CREATE INDEX idx_sys_menus_menu_type ON sys_menus(menu_type);
CREATE INDEX idx_sys_menus_status ON sys_menus(status);