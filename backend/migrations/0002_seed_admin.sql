BEGIN;

INSERT INTO sys_departments (
  id, code, name, parent_id, ancestors, path, sort_order, leader, phone, email, status, created_at, updated_at
) VALUES (
  '00000000-0000-0000-0000-000000000001',
  'ROOT',
  '总部',
  NULL,
  '',
  '/总部',
  0,
  '系统管理员',
  NULL,
  NULL,
  'ACTIVE',
  NOW(),
  NOW()
) ON CONFLICT (id) DO NOTHING;

INSERT INTO sys_departments (
  id, code, name, parent_id, ancestors, path, sort_order, leader, phone, email, status, created_at, updated_at
) VALUES (
  '00000000-0000-0000-0000-000000000010',
  'EAST_REGION',
  '华东大区',
  '00000000-0000-0000-0000-000000000001',
  '00000000-0000-0000-0000-000000000001',
  '/总部/华东大区',
  10,
  '区域负责人',
  NULL,
  NULL,
  'ACTIVE',
  NOW(),
  NOW()
) ON CONFLICT (id) DO NOTHING;

INSERT INTO sys_roles (
  id, code, name, description, data_scope, status, is_system, created_at, updated_at
) VALUES (
  '00000000-0000-0000-0000-000000000002',
  'SUPER_ADMIN',
  '超级管理员',
  '系统内置超级管理员角色',
  'ALL',
  'ACTIVE',
  TRUE,
  NOW(),
  NOW()
) ON CONFLICT (id) DO NOTHING;

INSERT INTO sys_roles (
  id, code, name, description, data_scope, status, is_system, created_at, updated_at
) VALUES (
  '00000000-0000-0000-0000-000000000011',
  'REGIONAL_AUDITOR',
  '区域审计员',
  '用于验证 CUSTOM 数据范围的内置示例角色',
  'CUSTOM',
  'ACTIVE',
  TRUE,
  NOW(),
  NOW()
) ON CONFLICT (id) DO NOTHING;

INSERT INTO sys_users (
  id, username, email, display_name, password_hash, avatar_url, phone, dept_id, status, is_super_admin, last_login_at, last_login_ip, password_changed_at, created_at, updated_at, deleted_at
) VALUES (
  '00000000-0000-0000-0000-000000000003',
  'admin',
  'admin@example.com',
  '系统管理员',
  '$2y$10$wiqFP6.PI24n4JHUhumU..IEqBbveyBvuEbiz9aPTMOfuxlQ2oDzC',
  NULL,
  NULL,
  '00000000-0000-0000-0000-000000000001',
  'ACTIVE',
  TRUE,
  NULL,
  NULL,
  NOW(),
  NOW(),
  NOW(),
  NULL
) ON CONFLICT (id) DO NOTHING;

INSERT INTO sys_menus (
  id, parent_id, name, menu_type, route_path, component, permission, icon, sort_order, visible, keep_alive, status, created_at, updated_at
) VALUES
  (
    '00000000-0000-0000-0000-000000000004',
    NULL,
    '工作台',
    'MENU',
    '/dashboard',
    'dashboard/index',
    NULL,
    'dashboard',
    1,
    TRUE,
    TRUE,
    'ACTIVE',
    NOW(),
    NOW()
  ),
  (
    '00000000-0000-0000-0000-000000000005',
    NULL,
    '系统管理',
    'DIRECTORY',
    '/system',
    NULL,
    NULL,
    'settings',
    2,
    TRUE,
    FALSE,
    'ACTIVE',
    NOW(),
    NOW()
  ),
  (
    '00000000-0000-0000-0000-000000000006',
    '00000000-0000-0000-0000-000000000005',
    '用户管理',
    'MENU',
    '/system/users',
    'system/users',
    'system:user:list',
    'users',
    1,
    TRUE,
    TRUE,
    'ACTIVE',
    NOW(),
    NOW()
  ),
  (
    '00000000-0000-0000-0000-000000000007',
    '00000000-0000-0000-0000-000000000005',
    '角色管理',
    'MENU',
    '/system/roles',
    'system/roles',
    'system:role:list',
    'shield',
    2,
    TRUE,
    TRUE,
    'ACTIVE',
    NOW(),
    NOW()
  ),
  (
    '00000000-0000-0000-0000-000000000008',
    '00000000-0000-0000-0000-000000000005',
    '部门管理',
    'MENU',
    '/system/departments',
    'system/departments',
    'system:dept:list',
    'building',
    3,
    TRUE,
    TRUE,
    'ACTIVE',
    NOW(),
    NOW()
  ),
  (
    '00000000-0000-0000-0000-000000000009',
    '00000000-0000-0000-0000-000000000005',
    '菜单管理',
    'MENU',
    '/system/menus',
    'system/menus',
    'system:menu:list',
    'menu',
    4,
    TRUE,
    TRUE,
    'ACTIVE',
    NOW(),
    NOW()
  ),
  (
    '00000000-0000-0000-0000-000000000012',
    '00000000-0000-0000-0000-000000000006',
    '新增用户',
    'BUTTON',
    NULL,
    NULL,
    'system:user:add',
    NULL,
    10,
    TRUE,
    FALSE,
    'ACTIVE',
    NOW(),
    NOW()
  ),
  (
    '00000000-0000-0000-0000-000000000013',
    '00000000-0000-0000-0000-000000000007',
    '编辑角色',
    'BUTTON',
    NULL,
    NULL,
    'system:role:edit',
    NULL,
    10,
    TRUE,
    FALSE,
    'ACTIVE',
    NOW(),
    NOW()
  )
ON CONFLICT (id) DO NOTHING;

INSERT INTO sys_user_roles (user_id, role_id, assigned_at)
VALUES (
  '00000000-0000-0000-0000-000000000003',
  '00000000-0000-0000-0000-000000000002',
  NOW()
) ON CONFLICT (user_id, role_id) DO NOTHING;

INSERT INTO sys_role_menus (role_id, menu_id, assigned_at)
VALUES
  ('00000000-0000-0000-0000-000000000002', '00000000-0000-0000-0000-000000000004', NOW()),
  ('00000000-0000-0000-0000-000000000002', '00000000-0000-0000-0000-000000000005', NOW()),
  ('00000000-0000-0000-0000-000000000002', '00000000-0000-0000-0000-000000000006', NOW()),
  ('00000000-0000-0000-0000-000000000002', '00000000-0000-0000-0000-000000000007', NOW()),
  ('00000000-0000-0000-0000-000000000002', '00000000-0000-0000-0000-000000000008', NOW()),
  ('00000000-0000-0000-0000-000000000002', '00000000-0000-0000-0000-000000000009', NOW()),
  ('00000000-0000-0000-0000-000000000002', '00000000-0000-0000-0000-000000000012', NOW()),
  ('00000000-0000-0000-0000-000000000002', '00000000-0000-0000-0000-000000000013', NOW())
ON CONFLICT (role_id, menu_id) DO NOTHING;

INSERT INTO sys_role_departments (role_id, dept_id, assigned_at)
VALUES (
  '00000000-0000-0000-0000-000000000011',
  '00000000-0000-0000-0000-000000000010',
  NOW()
) ON CONFLICT (role_id, dept_id) DO NOTHING;

COMMIT;
