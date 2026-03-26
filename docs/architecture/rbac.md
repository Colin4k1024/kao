# RBAC Architecture

Role-Based Access Control (RBAC) is the foundation of Kao's permission system. This document describes the RBAC model, data structures, and access control flow.

## Overview

```
┌──────────────────────────────────────────────────────────────────┐
│                      RBAC Architecture                           │
└──────────────────────────────────────────────────────────────────┘

┌──────────┐     ┌──────────┐     ┌──────────┐     ┌──────────┐
│  User    │────▶│  Role    │────▶│  Menu    │────▶│Permission│
│ (sys_user)│    │ (sys_role)│    │ (sys_menu)│    │ (String) │
└──────────┘     └──────────┘     └──────────┘     └──────────┘
     │               │      ▲          │      ▲
     │               │      │          │      │
     ▼               ▼      │          ▼      │
┌──────────┐      ┌──────────┐      ┌──────────┐
│ sys_dept │      │sys_role dept│    │ sys_dept │
│(Department)│     │  (Link)  │     │(Linked) │
└──────────┘      └──────────┘      └──────────┘
```

## Data Models

### User (sys_user)

```rust
#[derive(Serialize, Deserialize, Queryable, Insertable)]
pub struct SysUser {
    pub id: Uuid,
    pub username: String,
    pub password_hash: String,
    pub email: Option<String>,
    pub phone: Option<String>,
    pub nickname: Option<String>,
    pub avatar_url: Option<String>,
    pub dept_id: Option<Uuid>,
    pub status: UserStatus,
    pub is_super_admin: bool,
    pub last_login_at: Option<DateTime<Utc>>,
    pub password_changed_at: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub enum UserStatus {
    Active,
    Disabled,
    Locked,
}
```

### Role (sys_role)

```rust
#[derive(Serialize, Deserialize, Queryable, Insertable)]
pub struct SysRole {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub description: Option<String>,
    pub data_scope: DataScope,
    pub status: Status,
    pub is_system: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub enum DataScope {
    All,           // All data accessible
    Custom,        // Custom data scope
    Dept,          // Department data only
    DeptAndChild,  // Department and child departments
    SelfOnly,      // Own data only
}

#[derive(Serialize, Deserialize)]
pub enum Status {
    Active,
    Disabled,
}
```

### Menu (sys_menu)

```rust
#[derive(Serialize, Deserialize, Queryable, Insertable)]
pub struct SysMenu {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub name: String,
    pub menu_type: MenuType,
    pub route_path: String,
    pub component: String,
    pub permission: Option<String>,
    pub icon: Option<String>,
    pub sort: i32,
    pub visible: bool,
    pub keep_alive: bool,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Serialize, Deserialize)]
pub enum MenuType {
    Directory,  // Directory (folder)
    Menu,       // Menu item
    Button,     // Button permission
}
```

### Department (sys_department)

```rust
#[derive(Serialize, Deserialize, Queryable, Insertable)]
pub struct SysDepartment {
    pub id: Uuid,
    pub code: String,
    pub name: String,
    pub parent_id: Option<Uuid>,
    pub ancestors: Vec<Uuid>,
    pub path: String,
    pub sort_order: i32,
    pub leader: Option<String>,
    pub phone: Option<String>,
    pub email: Option<String>,
    pub status: Status,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}
```

## Relationship Tables

### User-Role Link (sys_user_role)

```sql
CREATE TABLE sys_user_role (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    user_id UUID NOT NULL REFERENCES sys_user(id),
    role_id UUID NOT NULL REFERENCES sys_role(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(user_id, role_id)
);
```

### Role-Menu Link (sys_role_menu)

```sql
CREATE TABLE sys_role_menu (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    role_id UUID NOT NULL REFERENCES sys_role(id),
    menu_id UUID NOT NULL REFERENCES sys_menu(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(role_id, menu_id)
);
```

### Role-Department Link (sys_role_department)

```sql
CREATE TABLE sys_role_department (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    role_id UUID NOT NULL REFERENCES sys_role(id),
    department_id UUID NOT NULL REFERENCES sys_department(id),
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    UNIQUE(role_id, department_id)
);
```

## Access Control Flow

### 1. Authentication Flow

```
┌─────────────┐     ┌─────────────┐
│   Request   │────▶│   Middleware│
│  with Token │     │  Check JWT  │
└─────────────┘     └──────┬──────┘
                           │
                    ┌──────┴──────┐
                    │  Token Valid?│
                    └──────┬──────┘
                           │
            ┌──────────────┴──────────────┐
            │          Yes                │     No
            │                             │
            ▼                             ▼
   ┌─────────────┐              ┌─────────────┐
   │  Extract    │              │  Forbidden  │
   │   User      │              │  (403)      │
   │  Claims     │              └─────────────┘
   └──────┬──────┘
          │
          ▼
   ┌─────────────┐
   │  Store in   │
   │   Request   │
   └─────────────┘
```

### 2. Authorization Flow

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│  Request    │────▶│  Extract    │────▶│  Check      │
│  Received   │     │  User/Role  │     │  Permission │
└─────────────┘     └─────────────┘     └─────────────┘
                                             │
                                    ┌────────┴────────┐
                                    │  Has Permission?│
                                    └────────┬────────┘
                                             │
                                    ┌────────┴────────┐
                                    │       Yes       │     No
                                    │                 │
                                    ▼                 ▼
                               ┌─────────────┐  ┌─────────────┐
                               │  Execute    │  │  Forbidden  │
                               │  Request    │  │  (403)      │
                               └─────────────┘  └─────────────┘
```

### 3. Permission Check Implementation

```rust
// Middleware checks permissions
pub async fn check_permission(
    Headers=headers,
    state: State<AppState>,
    request: Request,
) -> Result<Response, AppError> {
    // 1. Extract user from token
    let user = extract_user(&headers.authorization)?;
    
    // 2. Check if user is super admin (bypass check)
    if user.is_super_admin {
        return Ok(request.forward());
    }
    
    // 3. Get user roles
    let roles = get_user_roles(&user.id, &state.pool).await?;
    
    // 4. Get route permissions
    let required_permissions = get_route_permissions(&request.uri().path())?;
    
    // 5. Check if any role has the permission
    let has_permission = roles.iter().any(|role| {
        role_permission_contains(&role, &required_permissions)
    });
    
    if !has_permission {
        return Err(AppError::Unauthorized("No permission".to_string()));
    }
    
    Ok(request.forward())
}
```

## RBAC Types

### 1. Role Types

| Role Type | Description | Example |
|-----------|-------------|---------|
| Super Admin | Full system access | `super-admin` |
| System Admin | System configuration access | `sys-admin` |
| Department Admin | Department-specific access | `dept-admin` |
|普通用户 | Standard user access | `user` |
| Guest | Read-only access | `guest` |

### 2. Permission Patterns

| Pattern | Description | Example |
|---------|-------------|---------|
| Resource:Action | CRUD on resource | `user:create`, `user:read` |
| Action:Resource | Action on resource | `create:user`, `read:user` |
| Domain:Resource:Action | Full specification | `system:user:create` |

### 3. Menu Permission Types

| Type | Description | Example |
|------|-------------|---------|
| Directory | Menu folder | `/system` |
| Menu | Page menu item | `/system/users` |
| Button | Button permission | `user:create` |

## Permission Inheritance

```
┌──────────────────────────────────────────────────────────────────┐
│                    Permission Inheritance                        │
└──────────────────────────────────────────────────────────────────┘

Super Admin (Level 1)
├── System Admin (Level 2)
│   ├── Department Admin (Level 3)
│   │   └──普通用户 (Level 4)
│   └──普通用户 (Level 4)
└──普通用户 (Level 4)

Inheritance Rules:
- Super Admin has ALL permissions
- System Admin has MENU permissions
- Department Admin has DEPT + child permissions
-普通user has assigned permissions only
- Guest has read-only permissions
```

## Data Scope Levels

### Data Scope Types

| Scope | Description | Permission String |
|-------|-------------|------------------|
| ALL | View all data | `data:all` |
| CUSTOM | Custom data scope | `data:custom` |
| DEPT | Own department | `data:dept` |
| DEPT_AND_CHILD | Own and child departments | `data:dept-and-child` |
| SELF | Own data only | `data:self` |

### Example Usage

```rust
// Check data scope
pub async fn check_data_scope(
    user: &SysUser,
    requested_scope: DataScope,
) -> Result<bool, AppError> {
    // Super admin can access all
    if user.is_super_admin {
        return Ok(true);
    }
    
    // Get user's data scope
    let user_scope = get_user_data_scope(&user.id);
    
    match (user_scope, requested_scope) {
        (DataScope::All, _) => Ok(true),
        (DataScope::Custom, DataScope::Custom) => Ok(true),
        (DataScope::Dept, DataScope::Dept) => Ok(true),
        (DataScope::Dept, DataScope::DeptAndChild) => Ok(true),
        (DataScope::DeptAndChild, DataScope::Dept) => Ok(true),
        (DataScope::DeptAndChild, DataScope::DeptAndChild) => Ok(true),
        (DataScope::SelfOnly, DataScope::SelfOnly) => Ok(true),
        _ => Ok(false),
    }
}
```

## RBAC API Endpoints

### User Management

| Method | Endpoint | Permission |
|--------|----------|------------|
| GET | /api/system/users | `user:read` |
| POST | /api/system/users | `user:create` |
| PUT | /api/system/users/:id | `user:update` |
| DELETE | /api/system/users/:id | `user:delete` |
| PUT | /api/system/users/:id/roles | `user:assign-roles` |

### Role Management

| Method | Endpoint | Permission |
|--------|----------|------------|
| GET | /api/system/roles | `role:read` |
| POST | /api/system/roles | `role:create` |
| PUT | /api/system/roles/:id | `role:update` |
| DELETE | /api/system/roles/:id | `role:delete` |
| PUT | /api/system/roles/:id/menus | `role:assign-menus` |

### Menu Management

| Method | Endpoint | Permission |
|--------|----------|------------|
| GET | /api/system/menus | `menu:read` |
| POST | /api/system/menus | `menu:create` |
| PUT | /api/system/menus/:id | `menu:update` |
| DELETE | /api/system/menus/:id | `menu:delete` |

### Department Management

| Method | Endpoint | Permission |
|--------|----------|------------|
| GET | /api/system/departments | `dept:read` |
| POST | /api/system/departments | `dept:create` |
| PUT | /api/system/departments/:id | `dept:update` |
| DELETE | /api/system/departments/:id | `dept:delete` |

## Permission Check Implementation

### Frontend Permission Hook

```typescript
// frontend/src/hooks/usePermission.ts
import { useMemo } from 'react'
import { useQuery } from '@tanstack/react-query'
import { request } from '@/services/request'

export const usePermission = () => {
  const { data: permissions = [] } = useQuery(['permissions'], () =>
    request.get<string[]>('/api/system/permissions')
  )

  const hasPermission = useMemo(() => {
    return (permission: string | string[]): boolean => {
      if (Array.isArray(permission)) {
        return permission.some(p => permissions.includes(p))
      }
      return permissions.includes(permission)
    }
  }, [permissions])

  const hasAnyPermission = useMemo(() => {
    return (...permissions: string[]): boolean => {
      return permissions.some(p => permissions.includes(p))
    }
  }, [permissions])

  const hasAllPermissions = useMemo(() => {
    return (...permissions: string[]): boolean => {
      return permissions.every(p => permissions.includes(p))
    }
  }, [permissions])

  return { hasPermission, hasAnyPermission, hasAllPermissions, permissions }
}
```

### Backend Permission Middleware

```rust
// backend/src/middleware/auth.rs
use axum::{extract::State, http::Request, middleware::Next, response::Response};
use tracing::info;

pub async fn check_permission(
    State(state): State<AppState>,
    mut request: Request,
    next: Next,
) -> Result<Response, AppError> {
    // Check if route requires permission
    let route_info = get_route_info(request.uri().path());
    
    if !route_info.requires_permission {
        return Ok(next.run(request).await);
    }
    
    // Check authentication
    let user_id = extract_user_id(&request)?;
    let user = get_user(&user_id, &state.pool).await?;
    
    // Skip permission check for super admin
    if user.is_super_admin {
        return Ok(next.run(request).await);
    }
    
    // Check permissions
    let user_permissions = get_user_permissions(&user_id, &state.pool).await?;
    
    if !user_permissions.contains(&route_info.permission) {
        return Err(AppError::Unauthorized("Permission denied".to_string()));
    }
    
    info!("Permission check passed for user {}", user.username);
    Ok(next.run(request).await)
}
```

## RBAC Best Practices

### 1. Permission Design

- Use clear, consistent permission naming
- Follow `domain:resource:action` pattern
- Avoid overlapping permissions
- Document all permissions

### 2. Role Design

- Create roles based on job functions
- Avoid creating too many specific roles
- Use permission inheritance when appropriate
- Regularly review role permissions

### 3. Menu Design

- Organize menus hierarchically
- Use descriptive menu names
- Group related menus together
- Keep menu structure flat when possible

### 4. Security

- Always check permissions on backend
- Never rely on frontend-only checks
- Audit permission changes
- Log permission violations

---

## Next Steps

1. See [Database Architecture](./database.md) for table structure
2. See [API Architecture](./api.md) for permission endpoints
3. See [Security Architecture](./security.md) for permission security
4. Review permission checker implementation in `backend/src/middleware/`
