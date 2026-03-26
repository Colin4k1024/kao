# Database Architecture

Database schema design for Kao admin management system.

## Overview

Kao uses PostgreSQL as the primary database with SQLx for Rust integration. All tables follow a consistent naming convention with `sys_` prefix.

## Database Schema

### RBAC Tables

```
┌──────────────────────────────────────────────────────────────────┐
│                        RBAC Tables                               │
└──────────────────────────────────────────────────────────────────┘

┌──────────────┐    ┌──────────────┐    ┌──────────────┐
│  sys_user    │───▶│ sys_user_role│◀───│   sys_role   │
│  (用户)       │    │  (用户角色关联)│    │  (角色)      │
└──────────────┘    └──────────────┘    └──────────────┘
                                       │
                                       ▼
                              ┌──────────────┐
                              │ sys_role_menu│
                              │ (角色菜单关联)│
                              └──────────────┘
                                       │
                                       ▼
                              ┌──────────────┐
                              │   sys_menu   │
                              │  (菜单)       │
                              └──────────────┘
                                       │
                                       ▼
                              ┌──────────────┐    ┌──────────────┐
                              │sys_role_dept│───▶│sys_department│
                              │(角色部门关联)│    │  (部门)      │
                              └──────────────┘    └──────────────┘
```

### Data Dictionary Tables

```
┌──────────────────────────────────────────────────────────────────┐
│                     Data Dictionary Tables                       │
└──────────────────────────────────────────────────────────────────┘

┌─────────────────┐    ┌─────────────────┐
│sys_dict_type    │───▶│ sys_dict_data   │
│ (字典类型)       │    │  (字典数据)     │
└─────────────────┘    └─────────────────┘
```

### System Operations Tables

```
┌──────────────────────────────────────────────────────────────────┐
│                  System Operations Tables                        │
└──────────────────────────────────────────────────────────────────┘

┌──────────────┐    ┌──────────────┐    ┌──────────────┐
│   sys_notice │    │  sys_oper_log│    │ sys_login_log│
│ (通知公告)    │    │ (操作日志)   │    │  (登录日志)  │
└──────────────┘    └──────────────┘    └──────────────┘

┌──────────────┐
│   sys_job    │
│ (定时任务)    │
└──────────────┘

┌──────────────┐
│ sys_job_log  │
│ (任务日志)    │
└──────────────┘
```

## Table Specifications

### sys_user (用户表)

```sql
CREATE TABLE sys_user (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) NOT NULL UNIQUE,
    password_hash VARCHAR(100) NOT NULL,
    email VARCHAR(100) UNIQUE,
    phone VARCHAR(20) UNIQUE,
    nickname VARCHAR(50),
    avatar_url VARCHAR(500),
    dept_id UUID REFERENCES sys_department(id),
    status VARCHAR(20) NOT NULL DEFAULT 'active',
    is_super_admin BOOLEAN NOT NULL DEFAULT false,
    last_login_at TIMESTAMP WITH TIME ZONE,
    password_changed_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    CONSTRAINT chk_user_status CHECK (status IN ('active', 'disabled', 'locked'))
);

CREATE INDEX idx_user_username ON sys_user(username);
CREATE INDEX idx_user_dept_id ON sys_user(dept_id);
CREATE INDEX idx_user_status ON sys_user(status);
```

### sys_department (部门表)

```sql
CREATE TABLE sys_department (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(50) NOT NULL UNIQUE,
    name VARCHAR(100) NOT NULL,
    parent_id UUID REFERENCES sys_department(id),
    ancestors UUID[] NOT NULL DEFAULT '{}',
    path VARCHAR(500) NOT NULL,
    sort_order INT NOT NULL DEFAULT 0,
    leader VARCHAR(50),
    phone VARCHAR(20),
    email VARCHAR(100),
    status VARCHAR(20) NOT NULL DEFAULT 'active',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    CONSTRAINT chk_dept_status CHECK (status IN ('active', 'disabled'))
);

CREATE INDEX idx_dept_code ON sys_department(code);
CREATE INDEX idx_dept_parent_id ON sys_department(parent_id);
CREATE INDEX idx_dept_path ON sys_department(path);
```

### sys_post (岗位表)

```sql
CREATE TABLE sys_post (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(50) NOT NULL UNIQUE,
    name VARCHAR(100) NOT NULL,
    sort_order INT NOT NULL DEFAULT 0,
    status VARCHAR(20) NOT NULL DEFAULT 'active',
    description TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    CONSTRAINT chk_post_status CHECK (status IN ('active', 'disabled'))
);

CREATE INDEX idx_post_code ON sys_post(code);
```

### sys_role (角色表)

```sql
CREATE TABLE sys_role (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    code VARCHAR(50) NOT NULL UNIQUE,
    name VARCHAR(100) NOT NULL,
    description TEXT,
    data_scope VARCHAR(20) NOT NULL DEFAULT 'self',
    status VARCHAR(20) NOT NULL DEFAULT 'active',
    is_system BOOLEAN NOT NULL DEFAULT false,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    CONSTRAINT chk_role_status CHECK (status IN ('active', 'disabled')),
    CONSTRAINT chk_role_scope CHECK (data_scope IN ('all', 'custom', 'dept', 'dept-and-child', 'self-only'))
);

CREATE INDEX idx_role_code ON sys_role(code);
CREATE INDEX idx_role_status ON sys_role(status);
```

### sys_menu (菜单表)

```sql
CREATE TABLE sys_menu (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    parent_id UUID REFERENCES sys_menu(id),
    name VARCHAR(50) NOT NULL,
    menu_type VARCHAR(20) NOT NULL,
    route_path VARCHAR(200) NOT NULL,
    component VARCHAR(200),
    permission VARCHAR(100),
    icon VARCHAR(50),
    sort_order INT NOT NULL DEFAULT 0,
    visible BOOLEAN NOT NULL DEFAULT true,
    keep_alive BOOLEAN NOT NULL DEFAULT true,
    status VARCHAR(20) NOT NULL DEFAULT 'active',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    CONSTRAINT chk_menu_type CHECK (menu_type IN ('dir', 'menu', 'btn')),
    CONSTRAINT chk_menu_status CHECK (status IN ('active', 'disabled'))
);

CREATE INDEX idx_menu_parent_id ON sys_menu(parent_id);
CREATE INDEX idx_menu_route_path ON sys_menu(route_path);
CREATE INDEX idx_menu_sort ON sys_menu(sort_order);
```

### sys_user_role (用户角色关联表)

```sql
CREATE TABLE sys_user_role (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES sys_user(id) ON DELETE CASCADE,
    role_id UUID NOT NULL REFERENCES sys_role(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    UNIQUE(user_id, role_id)
);

CREATE INDEX idx_user_role_user_id ON sys_user_role(user_id);
CREATE INDEX idx_user_role_role_id ON sys_user_role(role_id);
```

### sys_role_menu (角色菜单关联表)

```sql
CREATE TABLE sys_role_menu (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    role_id UUID NOT NULL REFERENCES sys_role(id) ON DELETE CASCADE,
    menu_id UUID NOT NULL REFERENCES sys_menu(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    UNIQUE(role_id, menu_id)
);

CREATE INDEX idx_role_menu_role_id ON sys_role_menu(role_id);
CREATE INDEX idx_role_menu_menu_id ON sys_role_menu(menu_id);
```

### sys_role_department (角色部门关联表)

```sql
CREATE TABLE sys_role_department (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    role_id UUID NOT NULL REFERENCES sys_role(id) ON DELETE CASCADE,
    department_id UUID NOT NULL REFERENCES sys_department(id) ON DELETE CASCADE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    UNIQUE(role_id, department_id)
);

CREATE INDEX idx_role_dept_role_id ON sys_role_department(role_id);
CREATE INDEX idx_role_dept_dept_id ON sys_role_department(department_id);
```

### sys_dict_type (字典类型表)

```sql
CREATE TABLE sys_dict_type (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    type_code VARCHAR(50) NOT NULL UNIQUE,
    type_name VARCHAR(100) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'active',
    description TEXT,
    sort INT NOT NULL DEFAULT 0,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    CONSTRAINT chk_dict_type_status CHECK (status IN ('active', 'disabled'))
);

CREATE INDEX idx_dict_type_code ON sys_dict_type(type_code);
```

### sys_dict_data (字典数据表)

```sql
CREATE TABLE sys_dict_data (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    dict_type_code VARCHAR(50) NOT NULL REFERENCES sys_dict_type(type_code) ON DELETE CASCADE,
    dict_label VARCHAR(100) NOT NULL,
    dict_value VARCHAR(100) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'active',
    sort INT NOT NULL DEFAULT 0,
    color_type VARCHAR(20),
    css_class VARCHAR(50),
    css_style VARCHAR(200),
    remark TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    CONSTRAINT chk_dict_data_status CHECK (status IN ('active', 'disabled'))
);

CREATE INDEX idx_dict_data_type_code ON sys_dict_data(dict_type_code);
CREATE INDEX idx_dict_data_label ON sys_dict_data(dict_label);
```

### sys_config (参数配置表)

```sql
CREATE TABLE sys_config (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    config_key VARCHAR(100) NOT NULL UNIQUE,
    config_name VARCHAR(100) NOT NULL,
    config_value TEXT,
    config_type VARCHAR(10) NOT NULL DEFAULT 'yes',
    remark TEXT,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    CONSTRAINT chk_config_type CHECK (config_type IN ('yes', 'no'))
);

CREATE INDEX idx_config_key ON sys_config(config_key);
```

### sys_notice (通知公告表)

```sql
CREATE TABLE sys_notice (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    notice_title VARCHAR(200) NOT NULL,
    notice_type VARCHAR(20) NOT NULL,
    notice_content TEXT NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'active',
    remark TEXT,
    created_by UUID REFERENCES sys_user(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    CONSTRAINT chk_notice_type CHECK (notice_type IN ('notice', 'announcement')),
    CONSTRAINT chk_notice_status CHECK (status IN ('active', 'disabled'))
);

CREATE INDEX idx_notice_type ON sys_notice(notice_type);
CREATE INDEX idx_notice_status ON sys_notice(status);
```

### sys_job (定时任务表)

```sql
CREATE TABLE sys_job (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    job_name VARCHAR(100) NOT NULL,
    job_group VARCHAR(50) NOT NULL DEFAULT 'default',
    method VARCHAR(200) NOT NULL,
    method_params VARCHAR(200),
    cron_expression VARCHAR(100) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'active',
    remark TEXT,
    created_by UUID REFERENCES sys_user(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    
    CONSTRAINT chk_job_status CHECK (status IN ('active', 'disabled'))
);

CREATE INDEX idx_job_name ON sys_job(job_name);
CREATE INDEX idx_job_status ON sys_job(status);
```

### sys_job_log (定时任务日志表)

```sql
CREATE TABLE sys_job_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    job_id UUID NOT NULL REFERENCES sys_job(id) ON DELETE CASCADE,
    job_name VARCHAR(100) NOT NULL,
    job_group VARCHAR(50) NOT NULL,
    method VARCHAR(200) NOT NULL,
    method_params VARCHAR(200),
    job_message VARCHAR(500),
    job_status VARCHAR(20) NOT NULL,
    exception_info TEXT,
    execute_time BIGINT,
    start_time TIMESTAMP WITH TIME ZONE,
    stop_time TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_job_log_job_id ON sys_job_log(job_id);
CREATE INDEX idx_job_log_create_time ON sys_job_log(created_at);
CREATE INDEX idx_job_log_status ON sys_job_log(job_status);
```

### sys_oper_log (操作日志表)

```sql
CREATE TABLE sys_oper_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    oper_title VARCHAR(100) NOT NULL,
    oper_type VARCHAR(20) NOT NULL,
    oper_name VARCHAR(50),
    method VARCHAR(200),
    method_params TEXT,
    return_message TEXT,
    status VARCHAR(20) NOT NULL DEFAULT 'success',
    oper_ip VARCHAR(50),
    oper_location VARCHAR(100),
    error_msg TEXT,
    created_by UUID REFERENCES sys_user(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_oper_log_type ON sys_oper_log(oper_type);
CREATE INDEX idx_oper_log_status ON sys_oper_log(status);
CREATE INDEX idx_oper_log_create_time ON sys_oper_log(created_at);
```

### sys_login_log (登录日志表)

```sql
CREATE TABLE sys_login_log (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) NOT NULL,
    ipaddr VARCHAR(50),
    login_location VARCHAR(100),
    browser VARCHAR(100),
    os VARCHAR(50),
    status VARCHAR(20) NOT NULL,
    msg VARCHAR(200),
    login_time TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_login_log_username ON sys_login_log(username);
CREATE INDEX idx_login_log_status ON sys_login_log(status);
CREATE INDEX idx_login_log_login_time ON sys_login_log(login_time);
```

### sys_online_user (在线用户表)

```sql
CREATE TABLE sys_online_user (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES sys_user(id),
    username VARCHAR(50) NOT NULL,
    nick_name VARCHAR(50),
    ipaddr VARCHAR(50),
    login_location VARCHAR(100),
    browser VARCHAR(100),
    os VARCHAR(50),
    login_time TIMESTAMP WITH TIME ZONE NOT NULL,
    expire_time TIMESTAMP WITH TIME ZONE NOT NULL,
    session_id VARCHAR(100) NOT NULL UNIQUE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_online_user_session_id ON sys_online_user(session_id);
CREATE INDEX idx_online_user_user_id ON sys_online_user(user_id);
CREATE INDEX idx_online_user_expire_time ON sys_online_user(expire_time);
```

## Database Migrations

### Migration File Naming

```
backend/migrations/
├── 0001_create_sys_department.sql
├── 0002_create_sys_post.sql
├── 0003_create_sys_user.sql
├── 0004_create_sys_role.sql
├── 0005_create_sys_menu.sql
├── 0006_create_sys_user_role.sql
├── 0007_create_sys_role_menu.sql
├── 0008_create_sys_role_department.sql
├── 0009_create_sys_dict_type.sql
├── 0010_create_sys_dict_data.sql
├── 0011_create_sys_config.sql
├── 0012_create_sys_notice.sql
├── 0013_create_sys_job.sql
├── 0014_create_sys_job_log.sql
├── 0015_create_sys_oper_log.sql
├── 0016_create_sys_login_log.sql
├── 0017_create_sys_online_user.sql
└── 0099_init_data.sql
```

### Migration Template

```sql
-- 00XX_create_example_table.sql
-- migrate:up
CREATE TABLE sys_example (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    name VARCHAR(100) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'active',
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW()
);

CREATE INDEX idx_example_name ON sys_example(name);

-- migrate:down
DROP TABLE sys_example;
```

## Database Connection Pool

```rust
// backend/src/db/pool.rs
use deadpool_postgres::{Config, Pool};
use tokio_postgres::NoTls;

pub async fn create_pool() -> Pool {
    let config = Config::from_json(&config::database_url())
        .expect("Failed to create config");
    
    let pool = config
        .create_pool(Some(NoTls))
        .expect("Failed to create pool");
    
    // Ensure we have the right pool size
    pool.set_max_size(20);
    pool.set_min_idle(Some(5));
    
    pool
}
```

## Database Query Example

```rust
// backend/src/features/users/repo.rs
use sqlx::PgPool;
use crate::common::error::AppError;

pub async fn find_user_by_id(pool: &PgPool, user_id: &str) -> Result<User, AppError> {
    let user = sqlx::query_as!(
        User,
        r#"
        SELECT 
            id,
            username,
            password_hash,
            email,
            phone,
            nickname,
            avatar_url,
            dept_id,
            status,
            is_super_admin,
            last_login_at,
            password_changed_at,
            created_at,
            updated_at
        FROM sys_user
        WHERE id = $1
        "#,
        user_id
    )
    .fetch_one(pool)
    .await?;
    
    Ok(user)
}

pub async fn list_users(
    pool: &PgPool,
    page: i64,
    page_size: i64,
    username: Option<&str>,
) -> Result<Vec<User>, AppError> {
    let offset = (page - 1) * page_size;
    
    let users = sqlx::query_as!(
        User,
        r#"
        SELECT 
            id,
            username,
            email,
            phone,
            nickname,
            status,
            created_at,
            updated_at
        FROM sys_user
        WHERE ($1::VARCHAR IS NULL OR username LIKE '%' || $1 || '%')
        ORDER BY created_at DESC
        LIMIT $2 OFFSET $3
        "#,
        username,
        page_size,
        offset
    )
    .fetch_all(pool)
    .await?;
    
    Ok(users)
}
```

## Index Strategy

### Single-Column Indexes

```sql
CREATE INDEX idx_table_column ON table_name(column_name);
```

### Composite Indexes

```sql
CREATE INDEX idx_users_status_dept ON sys_user(status, dept_id);
```

### Unique Indexes

```sql
CREATE UNIQUE INDEX idx_username_unique ON sys_user(username);
```

### Covering Indexes

```sql
CREATE INDEX idx_users_covering ON sys_user(id, username, email, status);
```

## Query Optimization

### Common Patterns

```sql
-- ✅ Good: Use indexes
SELECT * FROM sys_user WHERE username = $1;

-- ✅ Good: Use LIMIT
SELECT * FROM sys_user ORDER BY created_at DESC LIMIT 20;

-- ❌ Bad: Avoid SELECT *
SELECT * FROM sys_user;  -- Fetch only required columns

-- ❌ Bad: Avoid LIKE with leading wildcard
SELECT * FROM sys_user WHERE username LIKE '%admin%';
```

### EXPLAIN ANALYZE

```sql
EXPLAIN ANALYZE
SELECT * FROM sys_user WHERE username = 'admin';
```

## Common Queries

### Get User with Roles

```sql
SELECT 
    u.id,
    u.username,
    u.email,
    array_agg(r.name) AS roles
FROM sys_user u
LEFT JOIN sys_user_role ur ON u.id = ur.user_id
LEFT JOIN sys_role r ON ur.role_id = r.id
WHERE u.id = $1
GROUP BY u.id;
```

### Get Menu Tree

```sql
WITH RECURSIVE menu_tree AS (
    SELECT 
        id,
        parent_id,
        name,
        route_path,
        component,
        permission,
        icon,
        sort_order,
        menu_type,
        visible,
        keep_alive,
        status,
        0 AS level
    FROM sys_menu
    WHERE parent_id IS NULL AND status = 'active'
    
    UNION ALL
    
    SELECT 
        m.id,
        m.parent_id,
        m.name,
        m.route_path,
        m.component,
        m.permission,
        m.icon,
        m.sort_order,
        m.menu_type,
        m.visible,
        m.keep_alive,
        m.status,
        mt.level + 1
    FROM sys_menu m
    INNER JOIN menu_tree mt ON m.parent_id = mt.id
    WHERE m.status = 'active'
)
SELECT * FROM menu_tree ORDER BY sort_order;
```

---

## Migration Commands

```bash
# Run all pending migrations
sqlx migrate run --database-url postgres://user:pass@localhost:5432/kao_db

# Revert last migration
sqlx migrate revert --database-url postgres://user:pass@localhost:5432/kao_db

# Check migration status
sqlx migrate status --database-url postgres://user:pass@localhost:5432/kao_db

# Create new migration
sqlx migrate add --database-url postgres://user:pass@localhost:5432/kao_db add_new_column
```

---

## Next Steps

1. Review [API Architecture](./api.md) for database API endpoints
2. Review [Security Architecture](./security.md) for data protection
3. Run migrations on development database
4. Test queries with SQLx
