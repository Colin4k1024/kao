# Requirements

**Project:** Kao - Enterprise Admin Management System

**Date:** 2026-03-26

**Version:** 0.1.0

---

## Overview

This document defines the scoped requirements for Kao, an enterprise-grade admin management system inspired by RuoYi. The system provides RBAC-based access control with dynamic configuration management.

---

## Scope Definition

### In Scope (Phase 1)

1. **User Management**
   - Create, Read, Update, Delete users
   - User status management (active/disabled/locked)
   - Password reset and management
   - Role assignment
   - Department assignment

2. **Department Management**
   - Hierarchical tree structure
   - Department CRUD operations
   - Parent-child relationships
   - Department leader assignment

3. **Role Management**
   - Role CRUD operations
   - Menu permission assignment
   - Data scope configuration
   - Role-based access control

4. **Menu Management**
   - Tree structure navigation
   - Menu CRUD operations
   - Route path mapping
   - Permission string definition

5. **Dynamic Configuration**
   - Dictionary type management
   - Dictionary data management
   - Parameter configuration
   - Notice/announcement management

6. **System Operations**
   - Scheduled job management
   - Job log tracking
   - Operation logging
   - Login logging
   - Online user monitoring

### Out of Scope (Phase 1)

- OAuth2/SAML external authentication
- Multi-tenant architecture
- Advanced caching with Redis
- Message queue integration
- WebSocket real-time communication
- API gateway features

---

## Functional Requirements

### FR1: Authentication & Authorization

**Priority:** Critical

**Description:** System must implement JWT-based authentication with bcrypt password hashing.

**Requirements:**
- JWT token generation with expiration
- Password hashing using bcrypt
- Token refresh mechanism
- Role-based access control
- Permission string validation

### FR2: User Management

**Priority:** High

**Description:** Comprehensive user lifecycle management.

**Requirements:**
- User CRUD operations via REST API
- User status management (ACTIVE/DISABLED/LOCKED)
- Password reset functionality
- Role assignment
- Department assignment
- Search and filter capabilities
- Pagination support

### FR3: Department Management

**Priority:** High

**Description:** Hierarchical department structure management.

**Requirements:**
- Tree structure display and management
- Parent-child relationship维护
- Department CRUD operations
- Leader assignment
- Status management

### FR4: Role Management

**Priority:** High

**Description:** Role-based access control implementation.

**Requirements:**
- Role CRUD operations
- Menu permission assignment
- Data scope configuration (ALL/CUSTOM/DEPT/DEPT_AND_CHILD/SELF)
- Role search and filtering
- Pagination support

### FR5: Menu Management

**Priority:** High

**Description:** Navigation menu and permission configuration.

**Requirements:**
- Menu CRUD operations
- Tree structure management
- Route path mapping
- Component path configuration
- Permission string definition
- Menu visibility control

### FR6: Dynamic Configuration

**Priority:** Medium

**Description:** Dynamic configuration management for system parameters.

**Requirements:**
- Dictionary type CRUD
- Dictionary data CRUD
- Parameter configuration CRUD
- Notice/announcement CRUD
- Cache-friendly configuration

### FR7: System Operations

**Priority:** Medium

**Description:** System operation monitoring and management.

**Requirements:**
- Scheduled job management
- Job log tracking
- Operation logging
- Login logging
- Online user monitoring

---

## Non-Functional Requirements

### NFR1: Security

**Priority:** Critical

**Requirements:**
- Password hashing with bcrypt (cost factor 12+)
- JWT tokens with expiration (default 1 hour)
- SQL injection prevention via parameterized queries
- Cross-origin resource sharing (CORS) configuration
- Rate limiting for authentication endpoints
- Input validation on all API endpoints
- Secure token storage (httpOnly cookies preferred)
- Account lockout after failed login attempts

### NFR2: Performance

**Priority:** High

**Requirements:**
- API response time < 200ms for CRUD operations
- Database connection pooling (min 5, max 20 connections)
- Query optimization with proper indexing
- Caching for frequently accessed data (menus, roles)
- Frontend code splitting and lazy loading
- API response caching headers

### NFR3: Reliability

**Priority:** High

**Requirements:**
- Health check endpoint with dependency status
- Database migration execution on startup
- Error handling with consistent response format
- Structured logging for debugging
- Graceful degradation on non-critical failures
- Backup and recovery procedures

### NFR4: Maintainability

**Priority:** Medium

**Requirements:**
- Consistent code style (TypeScript strict mode)
- ESLint and Prettier configuration
- Code documentation for public APIs
- Version-controlled database schema
- Commit message conventions
- Release note generation

### NFR5: Scalability

**Priority:** Medium

**Requirements:**
- Stateless authentication (JWT)
- Horizontal scaling capability
- Load balancer compatibility
- Database connection pool sizing
- Configuration via environment variables
- Docker containerization

---

## Domain Model

### Entities

1. **SysUser**
   - Identity: UUID
   - Attributes: username, email, password_hash, display_name, phone, avatar_url, dept_id, status, is_super_admin, last_login_at, password_changed_at
   - Relationships: belongs_to department, many-to-many roles

2. **SysDepartment**
   - Identity: UUID
   - Attributes: code, name, parent_id, ancestors, path, sort_order, leader, phone, email, status
   - Relationships: has many users, has many roles, belongs_to parent

3. **SysPost**
   - Identity: UUID
   - Attributes: code, name, sort_order, status, description
   - Relationships: many-to-many users

4. **SysRole**
   - Identity: UUID
   - Attributes: code, name, description, data_scope, status, is_system
   - Relationships: many-to-many users, many-to-many menus, many-to-many departments

5. **SysMenu**
   - Identity: UUID
   - Attributes: parent_id, name, menu_type, route_path, component, permission, icon, sort_order, visible, keep_alive, status
   - Relationships: has many children, many-to-many roles

---

## API Contract

### Authentication

| Method | Endpoint | Description |
|--------|----------|-------------|
| POST | /api/auth/login | User login |
| POST | /api/auth/register | User registration |
| POST | /api/auth/logout | User logout |
| POST | /api/auth/refresh | Refresh token |

### User Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | /api/system/users | List users |
| GET | /api/system/users/:id | Get user detail |
| POST | /api/system/users | Create user |
| PUT | /api/system/users/:id | Update user |
| DELETE | /api/system/users/:id | Delete user |
| PUT | /api/system/users/:id/reset-password | Reset password |
| PUT | /api/system/users/:id/roles | Assign roles |

### Department Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | /api/system/departments | List departments (tree) |
| GET | /api/system/departments/:id | Get department detail |
| POST | /api/system/departments | Create department |
| PUT | /api/system/departments/:id | Update department |
| DELETE | /api/system/departments/:id | Delete department |

### Role Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | /api/system/roles | List roles |
| GET | /api/system/roles/:id | Get role detail |
| POST | /api/system/roles | Create role |
| PUT | /api/system/roles/:id | Update role |
| DELETE | /api/system/roles/:id | Delete role |
| PUT | /api/system/roles/:id/menus | Assign menus |

### Menu Management

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | /api/system/menus | List menus (tree) |
| GET | /api/system/menus/:id | Get menu detail |
| POST | /api/system/menus | Create menu |
| PUT | /api/system/menus/:id | Update menu |
| DELETE | /api/system/menus/:id | Delete menu |

### Dynamic Configuration

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | /api/system/dictionary/types | List dictionary types |
| POST | /api/system/dictionary/types | Create dictionary type |
| GET | /api/system/dictionary/data | List dictionary data |
| POST | /api/system/dictionary/data | Create dictionary data |
| GET | /api/system/config | List parameters |
| POST | /api/system/config | Create parameter |
| GET | /api/system/notice | List notices |
| POST | /api/system/notice | Create notice |

### System Operations

| Method | Endpoint | Description |
|--------|----------|-------------|
| GET | /api/system/jobs | List scheduled jobs |
| POST | /api/system/jobs | Create scheduled job |
| GET | /api/system/jobs/logs | List job logs |
| GET | /api/system/oper/logs | List operation logs |
| GET | /api/system/login/logs | List login logs |
| GET | /api/system/online/users | List online users |

---

## Data Models

### Request Response Format

```json
{
  "code": 200,
  "message": "success",
  "data": {}
}
```

### Error Response Format

```json
{
  "code": 400,
  "message": "error description",
  "data": null
}
```

---

## Dependencies

### Frontend Dependencies

- React 18.2
- TypeScript 5.6
- Vite 5.4
- Ant Design 5.21
- React Router DOM 6.20
- React Query 5.60
- Axios 1.7
- Zod 3.23
- React Hook Form 7.53

### Backend Dependencies

- Rust 1.70+
- Axum 0.7
- Tokio 1.0
- SQLx 0.8
- JWT 9.0
- bcrypt 0.15
- tracing 0.1
- serde 1.0

### Infrastructure

- PostgreSQL 15
- Docker 24+
- Docker Compose 2.0+

---

## Acceptance Criteria

### AC1: Authentication Flow

**Given** User navigates to login page
**When** User enters credentials and submits
**Then** System validates credentials against database
**And** Returns JWT token with user information
**And** Token expires in 1 hour

### AC2: RBAC Permission Check

**Given** User is authenticated
**When** User accesses protected resource
**Then** System validates user roles and permissions
**And** Returns 403 if unauthorized
**And** Returns resource if authorized

### AC3: Department Tree Display

**Given** User accesses department management
**When** Department list is loaded
**Then** System displays hierarchical tree structure
**And** Parent-child relationships are maintained

### AC4: Database Migration

**Given** Application starts
**When** Database exists
**Then** System runs pending migrations
**And** Schema is updated to latest version

---

## Known Issues

1. Multiple authentication implementations exist (app.rs, handlers.rs, auth_service.rs)
2. Hardcoded credentials in app.rs (admin/admin123)
3. JWT secret hardcoded in extractor.rs
4. Table name mismatch: queries use `users` vs schema `sys_users`
5. CORS allows all origins (security risk)
6. No input validation middleware
7. No rate limiting for auth endpoints
8. Frontend stores tokens in localStorage (XSS risk)
9. No health check implementation (only returns static "OK")
10. Missing backend unit tests

See `.planning/codebase/CONCERNS.md` for detailed analysis.
