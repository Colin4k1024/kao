-- Performance Optimization: Add indexes for caching and query optimization
-- Generated: 2026-03-26

-- =============================================================================
-- User Table Indexes
-- =============================================================================

-- Index for username lookups (frequently queried)
CREATE INDEX IF NOT EXISTS idx_users_username ON sys_users(username);

-- Index for email lookups
CREATE INDEX IF NOT EXISTS idx_users_email ON sys_users(email);

-- Index for department lookups (foreign key)
CREATE INDEX IF NOT EXISTS idx_users_dept_id ON sys_users(dept_id);

-- Composite index for user status and department (common filter combination)
CREATE INDEX IF NOT EXISTS idx_users_status_dept ON sys_users(status, dept_id);

-- =============================================================================
-- Role Table Indexes
-- =============================================================================

-- Index for role code lookups (frequently queried)
CREATE INDEX IF NOT EXISTS idx_roles_code ON sys_roles(code);

-- Index for status lookups
CREATE INDEX IF NOT EXISTS idx_roles_status ON sys_roles(status);

-- Composite index for status and system flag
CREATE INDEX IF NOT EXISTS idx_roles_status_system ON sys_roles(status, is_system);

-- =============================================================================
-- Department Table Indexes
-- =============================================================================

-- Index for parent_id lookups (hierarchical queries)
CREATE INDEX IF NOT EXISTS idx_departments_parent_id ON sys_department(parent_id);

-- Index for department code lookups
CREATE INDEX IF NOT EXISTS idx_departments_code ON sys_department(code);

-- Index for status lookups
CREATE INDEX IF NOT EXISTS idx_departments_status ON sys_department(status);

-- Index for ancestors (in hierarchical queries)
CREATE INDEX IF NOT EXISTS idx_departments_ancestors ON sys_department(ancestors);

-- Composite index for parent status
CREATE INDEX IF NOT EXISTS idx_departments_parent_status ON sys_department(parent_id, status);

-- =============================================================================
-- Menu Table Indexes
-- =============================================================================

-- Index for parent_id lookups (menu树查询)
CREATE INDEX IF NOT EXISTS idx_menus_parent_id ON sys_menu(parent_id);

-- Index for menu type lookups
CREATE INDEX IF NOT EXISTS idx_menus_type ON sys_menu(menu_type);

-- Index for status lookups
CREATE INDEX IF NOT EXISTS idx_menus_status ON sys_menu(status);

-- Index for ancestors (in hierarchical queries)
CREATE INDEX IF NOT EXISTS idx_menus_ancestors ON sys_menu(ancestors);

-- Composite index for parent and sort order
CREATE INDEX IF NOT EXISTS idx_menus_parent_sort ON sys_menu(parent_id, sort_order);

-- =============================================================================
-- Job Table Indexes
-- =============================================================================

-- Index for job group lookups
CREATE INDEX IF NOT EXISTS idx_jobs_group ON sys_job(job_group);

-- Index for status lookups
CREATE INDEX IF NOT EXISTS idx_jobs_status ON sys_job(status);

-- Composite index for status and next run time (scheduler queries)
CREATE INDEX IF NOT EXISTS idx_jobs_status_next_run ON sys_job(status, next_run_time);

-- Composite index for status and trigger time
CREATE INDEX IF NOT EXISTS idx_jobs_status_trigger ON sys_job(status, trigger_time);

-- =============================================================================
-- Log Table Indexes
-- =============================================================================

-- Index for operation log user_id lookups
CREATE INDEX IF NOT EXISTS idx_oper_log_user_id ON sys_oper_log(user_id);

-- Index for operation log business type
CREATE INDEX IF NOT EXISTS idx_oper_log_business_type ON sys_oper_log(business_type);

-- Index for operation log status
CREATE INDEX IF NOT EXISTS idx_oper_log_status ON sys_oper_log(operate_status);

-- Index for operation log time
CREATE INDEX IF NOT EXISTS idx_oper_log_operate_time ON sys_oper_log(operate_time);

-- Composite index for user and time range queries
CREATE INDEX IF NOT EXISTS idx_oper_log_user_time ON sys_oper_log(user_id, operate_time);

-- Index for login log user_id lookups
CREATE INDEX IF NOT EXISTS idx_login_log_user_id ON sys_login_log(user_id);

-- Index for login log username
CREATE INDEX IF NOT EXISTS idx_login_log_username ON sys_login_log(username);

-- Index for login log status
CREATE INDEX IF NOT EXISTS idx_login_log_status ON sys_login_log(status);

-- Index for login log time
CREATE INDEX IF NOT EXISTS idx_login_log_login_time ON sys_login_log(login_time);

-- Composite index for user and time range queries
CREATE INDEX IF NOT EXISTS idx_login_log_user_time ON sys_login_log(user_id, login_time);

-- =============================================================================
-- Dictionary Table Indexes
-- =============================================================================

-- Index for dictionary type code
CREATE INDEX IF NOT EXISTS idx_dict_type_code ON sys_dict_type(code);

-- Index for dictionary data code
CREATE INDEX IF NOT EXISTS idx_dict_data_code ON sys_dict_data(dict_code);

-- Index for dictionary data type ID
CREATE INDEX IF NOT EXISTS idx_dict_data_type_id ON sys_dict_data(type_id);

-- =============================================================================
-- Config Table Indexes
-- =============================================================================

-- Index for config key lookups
CREATE INDEX IF NOT EXISTS idx_config_key ON sys_config(config_key);

-- Index for config name lookups
CREATE INDEX IF NOT EXISTS idx_config_name ON sys_config(config_name);

-- =============================================================================
-- Post Table Indexes
-- =============================================================================

-- Index for post code lookups
CREATE INDEX IF NOT EXISTS idx_post_code ON sys_post(code);

-- Index for post status lookups
CREATE INDEX IF NOT EXISTS idx_post_status ON sys_post(status);

-- =============================================================================
-- User-Role Relationship Indexes
-- =============================================================================

-- Index for user_id in user-role relationships
CREATE INDEX IF NOT EXISTS idx_user_role_user_id ON sys_user_role(user_id);

-- Index for role_id in user-role relationships
CREATE INDEX IF NOT EXISTS idx_user_role_role_id ON sys_user_role(role_id);

-- Composite index for user-role lookups
CREATE INDEX IF NOT EXISTS idx_user_role_user_role ON sys_user_role(user_id, role_id);

-- =============================================================================
-- Role-Menu Relationship Indexes
-- =============================================================================

-- Index for role_id in role-menu relationships
CREATE INDEX IF NOT EXISTS idx_role_menu_role_id ON sys_role_menu(role_id);

-- Index for menu_id in role-menu relationships
CREATE INDEX IF NOT EXISTS idx_role_menu_menu_id ON sys_role_menu(menu_id);

-- Composite index for role-menu lookups
CREATE INDEX IF NOT EXISTS idx_role_menu_role_menu ON sys_role_menu(role_id, menu_id);

-- =============================================================================
-- Role-Department Relationship Indexes
-- =============================================================================

-- Index for role_id in role-department relationships
CREATE INDEX IF NOT EXISTS idx_role_dept_role_id ON sys_role_dept(role_id);

-- Index for department_id in role-department relationships
CREATE INDEX IF NOT EXISTS idx_role_dept_dept_id ON sys_role_dept(department_id);

-- Composite index for role-department lookups
CREATE INDEX IF NOT EXISTS idx_role_dept_role_dept ON sys_role_dept(role_id, department_id);

-- =============================================================================
-- Hash indexes for UUID lookups (PostgreSQL)
-- =============================================================================

-- Create hash index for faster UUID lookups on id columns
CREATE INDEX IF NOT EXISTS idx_users_id_hash ON sys_users USING HASH (id);
CREATE INDEX IF NOT EXISTS idx_roles_id_hash ON sys_roles USING HASH (id);
CREATE INDEX IF NOT EXISTS idx_departments_id_hash ON sys_department USING HASH (id);
CREATE INDEX IF NOT EXISTS idx_menus_id_hash ON sys_menu USING HASH (id);
CREATE INDEX IF NOT EXISTS idx_jobs_id_hash ON sys_job USING HASH (id);
CREATE INDEX IF NOT EXISTS idx_oper_log_id_hash ON sys_oper_log USING HASH (id);
CREATE INDEX IF NOT EXISTS idx_login_log_id_hash ON sys_login_log USING HASH (id);

-- =============================================================================
-- Covering indexes for frequently selected columns
-- =============================================================================

-- Covering index for user list query (username, email, status, dept_id)
CREATE INDEX IF NOT EXISTS idx_users_list ON sys_users(username, email, status, dept_id);

-- Covering index for role list query (code, name, status)
CREATE INDEX IF NOT EXISTS idx_roles_list ON sys_roles(code, name, status);

-- Covering index for menu tree query (parent_id, name, sort_order, status)
CREATE INDEX IF NOT EXISTS idx_menus_tree ON sys_menu(parent_id, name, sort_order, status);
