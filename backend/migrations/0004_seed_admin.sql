-- Seed Admin User and Basic Data

-- Insert root department
INSERT INTO sys_departments (id, code, name, sort_order, status) 
VALUES (uuid_generate_v4(), 'ROOT_DEPT', 'Root Department', 0, 'ACTIVE');

-- Get the root department ID for reference
WITH root_dept AS (
    SELECT id FROM sys_departments WHERE code = 'ROOT_DEPT' LIMIT 1
)

-- Insert admin role
INSERT INTO sys_roles (id, code, name, description, data_scope, status, is_system) 
VALUES (uuid_generate_v4(), 'ADMIN', 'Administrator', 'System Administrator Role', 'ALL', 'ACTIVE', true);

-- Insert basic menu entries
INSERT INTO sys_menus (id, name, menu_type, route_path, component, permission, icon, sort_order, visible, status) VALUES
    -- Root menu items
    (uuid_generate_v4(), 'Dashboard', 'MENU', '/dashboard', 'Dashboard', 'dashboard:view', 'dashboard', 1, true, 'ACTIVE'),
    (uuid_generate_v4(), 'System Management', 'DIRECTORY', '/system', '', '', 'settings', 99, true, 'ACTIVE'),

    -- System management submenu items
    (uuid_generate_v4(), 'User Management', 'MENU', '/system/users', 'system/UserManagement', 'system:user:view', 'people', 1, true, 'ACTIVE'),
    (uuid_generate_v4(), 'Role Management', 'MENU', '/system/roles', 'system/RoleManagement', 'system:role:view', 'shield', 2, true, 'ACTIVE'),
    (uuid_generate_v4(), 'Department Management', 'MENU', '/system/departments', 'system/DepartmentManagement', 'system:dept:view', 'business', 3, true, 'ACTIVE'),
    (uuid_generate_v4(), 'Menu Management', 'MENU', '/system/menus', 'system/MenuManagement', 'system:menu:view', 'menu', 4, true, 'ACTIVE');

-- Insert admin user (password is 'Admin123!')
-- The password hash for 'Admin123!' using bcrypt
INSERT INTO sys_users (id, username, email, display_name, password_hash, status, is_super_admin) 
VALUES (
    uuid_generate_v4(), 
    'admin', 
    'admin@example.com', 
    'Administrator', 
    '$2b$12$LQv3c14KNcEIz2xEK8zT.ejTmFTXcZp.5JrfCYxkHfqDSyqz6.G3W', -- bcrypt hash for 'Admin123!'
    'ACTIVE', 
    true
);

-- Assign admin user to admin role
WITH admin_user AS (
    SELECT id FROM sys_users WHERE username = 'admin' LIMIT 1
),
admin_role AS (
    SELECT id FROM sys_roles WHERE code = 'ADMIN' LIMIT 1
)
INSERT INTO sys_users_roles (user_id, role_id)
SELECT admin_user.id, admin_role.id 
FROM admin_user, admin_role;

-- Assign all menus to admin role
WITH admin_role AS (
    SELECT id FROM sys_roles WHERE code = 'ADMIN' LIMIT 1
),
all_menus AS (
    SELECT id FROM sys_menus
)
INSERT INTO sys_roles_menus (role_id, menu_id)
SELECT admin_role.id, all_menus.id
FROM admin_role, all_menus;