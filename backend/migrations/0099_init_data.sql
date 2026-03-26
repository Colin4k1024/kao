-- System Initial Data Migration
-- This script inserts initial data for system tables

-- ===========================
-- 1. Initialize Department Data
-- ===========================
INSERT INTO sys_department (id, parent_id, ancestors, department_name, display_order, leader, phone, email, status, created_by, created_at)
VALUES 
('00000000-0000-0000-0000-000000000001', NULL, ',00000000-0000-0000-0000-000000000001,', '某某科技有限公司', 1, 'admin', '13800138000', 'admin@example.com', 1, NULL, NOW())
ON CONFLICT (id) DO NOTHING;

INSERT INTO sys_department (id, parent_id, ancestors, department_name, display_order, leader, phone, email, status, created_by, created_at)
VALUES 
('00000000-0000-0000-0000-000000000002', '00000000-0000-0000-0000-000000000001', ',00000000-0000-0000-0000-000000000001,00000000-0000-0000-0000-000000000002,', '研发部', 1, 'dev_leader', '13800138001', 'dev@example.com', 1, NULL, NOW())
ON CONFLICT (id) DO NOTHING;

INSERT INTO sys_department (id, parent_id, ancestors, department_name, display_order, leader, phone, email, status, created_by, created_at)
VALUES 
('00000000-0000-0000-0000-000000000003', '00000000-0000-0000-0000-000000000001', ',00000000-0000-0000-0000-000000000001,00000000-0000-0000-0000-000000000003,', '销售部', 2, 'sales_leader', '13800138002', 'sales@example.com', 1, NULL, NOW())
ON CONFLICT (id) DO NOTHING;

-- ===========================
-- 2. Initialize Post Data
-- ===========================
INSERT INTO sys_post (id, post_code, post_name, display_order, status, created_by, created_at)
VALUES 
('00000000-0000-0000-0000-000000000010', 'ceo', '首席执行官', 1, 1, NULL, NOW()),
('00000000-0000-0000-0000-000000000011', 'cto', '技术总监', 2, 1, NULL, NOW()),
('00000000-0000-0000-0000-000000000012', 'dev', '开发工程师', 3, 1, NULL, NOW()),
('00000000-0000-0000-0000-000000000013', 'test', '测试工程师', 4, 1, NULL, NOW()),
('00000000-0000-0000-0000-000000000014', 'sales', '销售经理', 5, 1, NULL, NOW())
ON CONFLICT (id) DO NOTHING;

-- ===========================
-- 3. Initialize Role Data
-- ===========================
INSERT INTO sys_role (id, role_name, role_code, display_order, status, role_type, data_scope, remark, created_by, created_at)
VALUES 
('00000000-0000-0000-0000-000000000100', '超级管理员', 'admin', 1, 1, 1, 1, '系统内置角色，拥有所有权限', NULL, NOW()),
('00000000-0000-0000-0000-000000000101', '普通用户', 'user', 2, 1, 0, 2, '普通用户角色', NULL, NOW()),
('00000000-0000-0000-0000-000000000102', '部门管理员', 'dept_admin', 3, 1, 0, 3, '部门管理员角色', NULL, NOW())
ON CONFLICT (id) DO NOTHING;

-- ===========================
-- 4. Initialize Menu Data
-- ===========================
-- Top-level Menus
INSERT INTO sys_menu (id, parent_id, ancestors, menu_name, menu_type, icon, route_name, route_path, display_order, is_visible, status, created_by, created_at)
VALUES 
('00000000-0000-0000-0000-000000001000', NULL, ',00000000-0000-0000-0000-000000001000,', '系统管理', 'M', 'Setting', 'System', '/system', 1, '1', 1, NULL, NOW()),
('00000000-0000-0000-0000-000000001001', NULL, ',00000000-0000-0000-0000-000000001001,', '用户管理', 'C', 'Users', 'User', '/system/users', 2, '1', 1, NULL, NOW()),
('00000000-0000-0000-0000-000000001002', NULL, ',00000000-0000-0000-0000-000000001002,', '角色管理', 'C', 'Shield', 'Role', '/system/roles', 3, '1', 1, NULL, NOW()),
('00000000-0000-0000-0000-000000001003', NULL, ',00000000-0000-0000-0000-000000001003,', '菜单管理', 'C', 'Menu', 'Menu', '/system/menus', 4, '1', 1, NULL, NOW()),
('00000000-0000-0000-0000-000000001004', NULL, ',00000000-0000-0000-0000-000000001004,', '部门管理', 'C', 'Building', 'Department', '/system/departments', 5, '1', 1, NULL, NOW()),
('00000000-0000-0000-0000-000000001005', NULL, ',00000000-0000-0000-0000-000000001005,', '岗位管理', 'C', 'Briefcase', 'Post', '/system/posts', 6, '1', 1, NULL, NOW())
ON CONFLICT (id) DO NOTHING;

-- Sub-menus (Button Permissions) for User Management
INSERT INTO sys_menu (id, parent_id, ancestors, menu_name, menu_type, permission, display_order, is_visible, status, created_by, created_at)
VALUES 
('00000000-0000-0000-0000-000000001010', '00000000-0000-0000-0000-000000001001', ',00000000-0000-0000-0000-000000001001,00000000-0000-0000-0000-000000001010,', '用户新增', 'F', 'system:user:add', 1, '1', 1, NULL, NOW()),
('00000000-0000-0000-0000-000000001011', '00000000-0000-0000-0000-000000001001', ',00000000-0000-0000-0000-000000001001,00000000-0000-0000-0000-000000001011,', '用户编辑', 'F', 'system:user:edit', 2, '1', 1, NULL, NOW()),
('00000000-0000-0000-0000-000000001012', '00000000-0000-0000-0000-000000001001', ',00000000-0000-0000-0000-000000001001,00000000-0000-0000-0000-000000001012,', '用户删除', 'F', 'system:user:remove', 3, '1', 1, NULL, NOW()),
('00000000-0000-0000-0000-000000001013', '00000000-0000-0000-0000-000000001001', ',00000000-0000-0000-0000-000000001001,00000000-0000-0000-0000-000000001013,', '重置密码', 'F', 'system:user:resetPwd', 4, '1', 1, NULL, NOW())
ON CONFLICT (id) DO NOTHING;

-- Sub-menus for Role Management
INSERT INTO sys_menu (id, parent_id, ancestors, menu_name, menu_type, permission, display_order, is_visible, status, created_by, created_at)
VALUES 
('00000000-0000-0000-0000-000000001020', '00000000-0000-0000-0000-000000001002', ',00000000-0000-0000-0000-000000001002,00000000-0000-0000-0000-000000001020,', '角色新增', 'F', 'system:role:add', 1, '1', 1, NULL, NOW()),
('00000000-0000-0000-0000-000000001021', '00000000-0000-0000-0000-000000001002', ',00000000-0000-0000-0000-000000001002,00000000-0000-0000-0000-000000001021,', '角色编辑', 'F', 'system:role:edit', 2, '1', 1, NULL, NOW()),
('00000000-0000-0000-0000-000000001022', '00000000-0000-0000-0000-000000001002', ',00000000-0000-0000-0000-000000001002,00000000-0000-0000-0000-000000001022,', '角色删除', 'F', 'system:role:remove', 3, '1', 1, NULL, NOW()),
('00000000-0000-0000-0000-000000001023', '00000000-0000-0000-0000-000000001002', ',00000000-0000-0000-0000-000000001002,00000000-0000-0000-0000-000000001023,', '分配权限', 'F', 'system:role:assign', 4, '1', 1, NULL, NOW())
ON CONFLICT (id) DO NOTHING;

-- ===========================
-- 5. Initialize Super Admin User
-- ===========================
-- Password: admin123 (bcrypt encrypted)
INSERT INTO sys_user (id, username, password, nickname, email, phone, status, department_id, post_id, created_by, created_at)
VALUES 
('00000000-0000-0000-0000-000000000001', 'admin', '$2b$12$LQv3c1yqBWVHxkd0LHAkCOYz6TtxMQJqhN8/LewY5CsIIc.x5T0S2', '管理员', 'admin@example.com', '13800138000', 1, '00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000010', NULL, NOW())
ON CONFLICT (id) DO NOTHING;

-- ===========================
-- 6. Initialize Dictionary Data
-- ===========================
-- Dictionary Types
INSERT INTO sys_dict_type (id, dict_name, dict_type, status, remark, created_by, created_at)
VALUES 
('00000000-0000-0000-0000-000000000100', '用户性别', 'sys_user_sex', 1, '用户性别列表', NULL, NOW()),
('00000000-0000-0000-0000-000000000101', '菜单状态', 'sys_show_hide', 1, '菜单状态列表', NULL, NOW()),
('00000000-0000-0000-0000-000000000102', '系统开关', 'sys_normal_disable', 1, '系统开关列表', NULL, NOW())
ON CONFLICT (id) DO NOTHING;

-- Dictionary Data
INSERT INTO sys_dict_data (id, dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, created_by, created_at)
VALUES 
('00000000-0000-0000-0000-000000000110', 1, '男', '0', 'sys_user_sex', NULL, 'default', 'N', 1, NULL, NOW()),
('00000000-0000-0000-0000-000000000111', 2, '女', '1', 'sys_user_sex', NULL, 'default', 'N', 1, NULL, NOW()),
('00000000-0000-0000-0000-000000000112', 3, '未知', '2', 'sys_user_sex', NULL, 'default', 'N', 1, NULL, NOW()),
('00000000-0000-0000-0000-000000000120', 1, '显示', '1', 'sys_show_hide', NULL, 'primary', 'Y', 1, NULL, NOW()),
('00000000-0000-0000-0000-000000000121', 2, '隐藏', '0', 'sys_show_hide', NULL, 'danger', 'N', 1, NULL, NOW()),
('00000000-0000-0000-0000-000000000130', 1, '正常', '1', 'sys_normal_disable', NULL, 'primary', 'Y', 1, NULL, NOW()),
('00000000-0000-0000-0000-000000000131', 2, '停用', '0', 'sys_normal_disable', NULL, 'danger', 'N', 1, NULL, NOW())
ON CONFLICT (id) DO NOTHING;

-- ===========================
-- 7. Initialize System Configuration
-- ===========================
INSERT INTO sys_config (id, config_name, config_key, config_value, config_type, is_encrypt, status, remark, created_by, created_at)
VALUES 
('00000000-0000-0000-0000-000000000200', '用户管理-初始密码', 'sys_user_initPassword', '123456', 'Y', 'N', 1, '用户初始化密码', NULL, NOW()),
('00000000-0000-0000-0000-000000000201', '是否开启用户注册', 'sys_account_registerEnabled', 'false', 'Y', 'N', 1, '是否允许新用户注册', NULL, NOW())
ON CONFLICT (id) DO NOTHING;

-- ===========================
-- 8. Assign All Menus to Super Admin Role
-- ===========================
INSERT INTO sys_role_menu (id, role_id, menu_id, created_at)
SELECT 
    uuid_generate_v4(),
    '00000000-0000-0000-0000-000000000100',
    id,
    NOW()
FROM sys_menu WHERE deleted_at IS NULL
ON CONFLICT DO NOTHING;

-- ===========================
-- 9. Assign Super Admin Role to Admin User
-- ===========================
INSERT INTO sys_user_role (id, user_id, role_id, created_at)
VALUES 
('00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000001', '00000000-0000-0000-0000-000000000100', NOW())
ON CONFLICT DO NOTHING;

-- ===========================
-- 10. Initialize System Notices
-- ===========================
INSERT INTO sys_notice (id, notice_title, notice_type, notice_content, notice_status, is_top, priority, publisher_id, publisher_name, publish_time, created_by, created_at)
VALUES 
('00000000-0000-0000-0000-000000000300', '系统上线通知', '1', '欢迎使用本系统，系统现已正式上线运行。', '1', '1', 1, '00000000-0000-0000-0000-000000000001', '管理员', NOW(), NULL, NOW()),
('00000000-0000-0000-0000-000000000301', '系统维护通知', '2', '系统将于本周日凌晨2:00-4:00进行维护升级，届时系统将暂停服务。', '1', '0', 2, '00000000-0000-0000-0000-000000000001', '管理员', NOW(), NULL, NOW())
ON CONFLICT (id) DO NOTHING;
