-- Sample dictionary types
INSERT INTO sys_dict_type (dict_name, dict_type, status, remark, created_by, updated_by)
VALUES 
    ('用户性别', 'sys_user_gender', 1, '用户性别列表', NULL, NULL),
    ('菜单类型', 'sys_menu_type', 1, '菜单类型列表', NULL, NULL),
    ('状态', 'sys_normal_disable', 1, '状态列表', NULL, NULL),
    ('通知类型', 'sys_notice_type', 1, '通知类型列表', NULL, NULL),
    ('系统是否', 'sys_yes_no', 1, '是/否选项', NULL, NULL);

-- Sample dictionary data
INSERT INTO sys_dict_data (dict_sort, dict_label, dict_value, dict_type, css_class, list_class, is_default, status, remark, created_by, updated_by)
VALUES 
    (1, '男', '0', 'sys_user_gender', NULL, NULL, 'Y', 1, '男性', NULL, NULL),
    (2, '女', '1', 'sys_user_gender', NULL, NULL, 'N', 1, '女性', NULL, NULL),
    (1, '目录', 'M', 'sys_menu_type', NULL, 'primary', 'Y', 1, '目录', NULL, NULL),
    (2, '菜单', 'C', 'sys_menu_type', NULL, 'info', 'N', 1, '菜单', NULL, NULL),
    (3, '按钮', 'F', 'sys_menu_type', NULL, '', 'N', 1, '按钮', NULL, NULL),
    (1, '正常', '1', 'sys_normal_disable', NULL, 'success', 'Y', 1, '正常', NULL, NULL),
    (2, '停用', '0', 'sys_normal_disable', NULL, 'danger', 'N', 1, '停用', NULL, NULL),
    (1, '通知', '1', 'sys_notice_type', NULL, 'warning', 'Y', 1, '通知', NULL, NULL),
    (2, '公告', '2', 'sys_notice_type', NULL, 'info', 'N', 1, '公告', NULL, NULL),
    (1, '是', 'Y', 'sys_yes_no', NULL, 'primary', 'Y', 1, '系统默认是', NULL, NULL),
    (2, '否', 'N', 'sys_yes_no', NULL, '', 'N', 1, '否', NULL, NULL);
