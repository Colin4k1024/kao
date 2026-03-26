# API Architecture

RESTful API design for Kao admin management system.

## Overview

Kao follows RESTful API design principles with consistent response formats, authentication middleware, and comprehensive API documentation via OpenAPI.

## API Design Principles

### 1. Resource Naming

| Resource | Collection | Item |
|----------|------------|------|
| Users | `GET /api/system/users` | `GET /api/system/users/{id}` |
| Roles | `GET /api/system/roles` | `GET /api/system/roles/{id}` |
| Menus | `GET /api/system/menus` | `GET /api/system/menus/{id}` |
| Departments | `GET /api/system/departments` | `GET /api/system/departments/{id}` |

### 2. HTTP Methods

| Method | Purpose | Examples |
|--------|---------|----------|
| GET | Retrieve resource(s) | `GET /users`, `GET /users/{id}` |
| POST | Create resource | `POST /users` |
| PUT | Update resource | `PUT /users/{id}` |
| DELETE | Delete resource | `DELETE /users/{id}` |
| PATCH | Partial update | *(not used)* |

### 3. Status Codes

| Code | Meaning | Example |
|------|---------|---------|
| 200 | OK | Successful GET/PUT/DELETE |
| 201 | Created | Successful POST |
| 204 | No Content | Successful DELETE |
| 400 | Bad Request | Invalid input |
| 401 | Unauthorized | Missing/invalid token |
| 403 | Forbidden | Insufficient permissions |
| 404 | Not Found | Resource doesn't exist |
| 409 | Conflict | Duplicate resource |
| 422 | Unprocessable Entity | Validation error |
| 500 | Server Error | Internal error |

---

## Response Format

### Success Response

```json
{
  "code": 200,
  "message": "success",
  "data": {
    "id": "uuid",
    "username": "admin",
    "email": "admin@example.com"
  }
}
```

### Error Response

```json
{
  "code": 400,
  "message": "Invalid input",
  "data": null
}
```

### Pagination Response

```json
{
  "code": 200,
  "message": "success",
  "data": {
    "records": [
      {
        "id": "uuid",
        "username": "admin"
      }
    ],
    "total": 100,
    "pageSize": 20,
    "current": 1
  }
}
```

---

## Authentication

### Request with Token

```
GET /api/system/users
Authorization: Bearer <token>
Content-Type: application/json
```

### Token Expiration

| Token Type | Expiration |
|------------|------------|
| Access Token | 1 hour |
| Refresh Token | 7 days |

### Token Refresh

```
POST /api/auth/refresh
Authorization: Bearer <refresh_token>
```

Response:

```json
{
  "code": 200,
  "message": "success",
  "data": {
    "access_token": "new_jwt_token",
    "refresh_token": "new_refresh_token",
    "token_type": "Bearer",
    "expires_in": 3600
  }
}
```

---

## API Endpoints

### Authentication

| Method | Endpoint | Description | Auth |
|--------|----------|-------------|------|
| POST | `/api/auth/login` | User login | No |
| POST | `/api/auth/logout` | User logout | Yes |
| POST | `/api/auth/refresh` | Refresh token | Yes |
| POST | `/api/auth/register` | User registration | No |

#### Login Request

```json
POST /api/auth/login

{
  "username": "admin",
  "password": "password123"
}
```

#### Login Response

```json
{
  "code": 200,
  "message": "登录成功",
  "data": {
    "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "token_type": "Bearer",
    "expires_in": 3600,
    "user": {
      "id": "uuid",
      "username": "admin",
      "nickname": "管理员",
      "email": "admin@example.com",
      "status": "active",
      "roles": ["admin"],
      "permissions": ["*"]
    }
  }
}
```

#### Logout Request

```
POST /api/auth/logout
Authorization: Bearer <token>
```

---

### User Management

| Method | Endpoint | Description | Permission |
|--------|----------|-------------|------------|
| GET | `/api/system/users` | List users | `user:read` |
| POST | `/api/system/users` | Create user | `user:create` |
| GET | `/api/system/users/:id` | Get user | `user:read` |
| PUT | `/api/system/users/:id` | Update user | `user:update` |
| DELETE | `/api/system/users/:id` | Delete user | `user:delete` |
| PUT | `/api/system/users/:id/reset-password` | Reset password | `user:reset-password` |
| PUT | `/api/system/users/:id/roles` | Assign roles | `user:assign-roles` |

#### List Users

```
GET /api/system/users?page=1&pageSize=20&username=admin&status=active
Authorization: Bearer <token>
```

Response:

```json
{
  "code": 200,
  "message": "success",
  "data": {
    "records": [
      {
        "id": "uuid",
        "username": "admin",
        "email": "admin@example.com",
        "status": "active",
        "created_at": "2024-01-01T00:00:00Z"
      }
    ],
    "total": 100,
    "pageSize": 20,
    "current": 1
  }
}
```

#### Create User

```
POST /api/system/users
Authorization: Bearer <token>
Content-Type: application/json

{
  "username": "john.doe",
  "password": "password123",
  "email": "john@example.com",
  "phone": "13800138000",
  "nickname": "John Doe",
  "dept_id": "uuid",
  "role_ids": ["uuid"],
  "status": "active"
}
```

#### Update User

```
PUT /api/system/users/:id
Authorization: Bearer <token>
Content-Type: application/json

{
  "email": "new.email@example.com",
  "phone": "13900139000",
  "nickname": "John Doe II",
  "status": "active"
}
```

---

### Role Management

| Method | Endpoint | Description | Permission |
|--------|----------|-------------|------------|
| GET | `/api/system/roles` | List roles | `role:read` |
| POST | `/api/system/roles` | Create role | `role:create` |
| GET | `/api/system/roles/:id` | Get role | `role:read` |
| PUT | `/api/system/roles/:id` | Update role | `role:update` |
| DELETE | `/api/system/roles/:id` | Delete role | `role:delete` |
| PUT | `/api/system/roles/:id/menus` | Assign menus | `role:assign-menus` |

#### List Roles

```
GET /api/system/roles?page=1&pageSize=20&name=admin
Authorization: Bearer <token>
```

#### Assign Menus to Role

```
PUT /api/system/roles/:id/menus
Authorization: Bearer <token>
Content-Type: application/json

{
  "menu_ids": ["uuid1", "uuid2", "uuid3"]
}
```

---

### Menu Management

| Method | Endpoint | Description | Permission |
|--------|----------|-------------|------------|
| GET | `/api/system/menus` | List menus (tree) | `menu:read` |
| POST | `/api/system/menus` | Create menu | `menu:create` |
| GET | `/api/system/menus/:id` | Get menu | `menu:read` |
| PUT | `/api/system/menus/:id` | Update menu | `menu:update` |
| DELETE | `/api/system/menus/:id` | Delete menu | `menu:delete` |

#### List Menus (Tree)

```
GET /api/system/menus?tree=true
Authorization: Bearer <token>
```

Response:

```json
{
  "code": 200,
  "message": "success",
  "data": [
    {
      "id": "uuid",
      "name": "System",
      "menu_type": "dir",
      "children": [
        {
          "id": "uuid",
          "name": "Users",
          "menu_type": "menu",
          "route_path": "/system/users",
          "component": "system/users/UserList"
        }
      ]
    }
  ]
}
```

---

### Department Management

| Method | Endpoint | Description | Permission |
|--------|----------|-------------|------------|
| GET | `/api/system/departments` | List departments (tree) | `dept:read` |
| POST | `/api/system/departments` | Create department | `dept:create` |
| GET | `/api/system/departments/:id` | Get department | `dept:read` |
| PUT | `/api/system/departments/:id` | Update department | `dept:update` |
| DELETE | `/api/system/departments/:id` | Delete department | `dept:delete` |

#### Create Department

```
POST /api/system/departments
Authorization: Bearer <token>
Content-Type: application/json

{
  "code": "dept1",
  "name": "Engineering",
  "parent_id": "uuid",
  "leader": "John Doe",
  "phone": "13800138000",
  "email": "eng@example.com",
  "status": "active"
}
```

---

### Dictionary Management

| Method | Endpoint | Description | Permission |
|--------|----------|-------------|------------|
| GET | `/api/system/dictionary/types` | List dictionary types | `dict:read` |
| POST | `/api/system/dictionary/types` | Create type | `dict:create` |
| GET | `/api/system/dictionary/data` | List dictionary data | `dict:read` |
| POST | `/api/system/dictionary/data` | Create data | `dict:create` |

---

### Configuration Management

| Method | Endpoint | Description | Permission |
|--------|----------|-------------|------------|
| GET | `/api/system/config` | List config | `config:read` |
| POST | `/api/system/config` | Create config | `config:create` |
| PUT | `/api/system/config/:id` | Update config | `config:update` |
| DELETE | `/api/system/config/:id` | Delete config | `config:delete` |

---

### Job Management

| Method | Endpoint | Description | Permission |
|--------|----------|-------------|------------|
| GET | `/api/system/jobs` | List jobs | `job:read` |
| POST | `/api/system/jobs` | Create job | `job:create` |
| DELETE | `/api/system/jobs/:id` | Delete job | `job:delete` |
| GET | `/api/system/jobs/logs` | List job logs | `job:read` |

---

### System Monitoring

| Method | Endpoint | Description | Permission |
|--------|----------|-------------|------------|
| GET | `/api/system/oper/logs` | List operation logs | `monitor:read` |
| GET | `/api/system/login/logs` | List login logs | `monitor:read` |
| GET | `/api/system/online/users` | List online users | `monitor:read` |

---

## Error Handling

### Common Error Responses

```json
{
  "code": 400,
  "message": "Invalid input",
  "data": {
    "errors": [
      { "field": "username", "message": "Username is required" },
      { "field": "email", "message": "Email format is invalid" }
    ]
  }
}
```

### Unauthorized Error

```json
{
  "code": 401,
  "message": "Unauthorized",
  "data": null
}
```

### Forbidden Error

```json
{
  "code": 403,
  "message": "Permission denied",
  "data": null
}
```

### Not Found Error

```json
{
  "code": 404,
  "message": "Resource not found",
  "data": null
}
```

### Validation Error

```json
{
  "code": 422,
  "message": "Validation failed",
  "data": {
    "errors": [
      { "field": "password", "message": "Password must be at least 8 characters" }
    ]
  }
}
```

---

## Rate Limiting

### Rate Limit Headers

```
X-RateLimit-Limit: 100
X-RateLimit-Remaining: 99
X-RateLimit-Reset: 1704067200
```

### Rate Limit Error

```json
{
  "code": 429,
  "message": "Rate limit exceeded",
  "data": null
}
```

---

## OpenAPI/docs API Documentation

API documentation is available at:
- **Development**: `http://localhost:8080/api-docs`
- **Production**: `https://api.kao-admin.com/api-docs`

### OpenAPI Spec Location

```yaml
# docs/api/openapi.yaml
openapi: 3.0.0
info:
  title: Kao Admin Management System API
  version: 1.0.0
servers:
  - url: http://localhost:8080
    description: Development server
paths:
  /api/auth/login:
    post:
      summary: User login
      requestBody:
        required: true
        content:
          application/json:
            schema:
              $ref: '#/components/schemas/LoginRequest'
      responses:
        '200':
          description: Login successful
          content:
            application/json:
              schema:
                $ref: '#/components/schemas/LoginResponse'
```

---

## API Versioning

### URL Path Versioning

```
/api/v1/users    # Version 1
/api/v2/users    # Version 2
```

### Header-based Versioning

```
GET /api/users
API-Version: 1
```

---

## API Best Practices

### 1. UseNouns for Resources

✅ `GET /api/users`
✅ `POST /api/users`

❌ `GET /api/getUsers`
❌ `POST /api/createUser`

### 2. Use Plural for Collections

✅ `GET /api/users`
✅ `POST /api/users`

❌ `GET /api/user`

### 3. Nest Resources Appropriately

✅ `GET /api/users/:id/roles` - Get user roles
✅ `GET /api/roles/:id/menus` - Get role menus

### 4. Use HTTP Methods Correctly

| Method | Safe | Idempotent | Use Case |
|--------|------|------------|----------|
| GET | ✅ | ✅ | Retrieve data |
| POST | ❌ | ❌ | Create resource |
| PUT | ❌ | ✅ | Update resource |
| DELETE | ❌ | ✅ | Delete resource |
| PATCH | ❌ | ❌ | Partial update |

### 5. Return Appropriate Status Codes

| Code | Use Case |
|------|----------|
| 200 | Successful GET/PUT |
| 201 | Successful POST |
| 204 | Successful DELETE |
| 400 | Invalid input |
| 401 | Unauthorized |
| 403 | Forbidden |
| 404 | Not found |
| 409 | Conflict |
| 422 | Validation error |
| 500 | Server error |

---

## API Testing

### Example API Test

```typescript
// frontend/tests/api/user.test.ts
import request from '@/services/request'

describe('User API', () => {
  it('should list users', async () => {
    const response = await request.get('/api/system/users')
    expect(response.code).toBe(200)
    expect(response.data.records).toBeInstanceOf(Array)
  })
  
  it('should create user', async () => {
    const response = await request.post('/api/system/users', {
      username: 'testuser',
      password: 'password123',
      email: 'test@example.com',
      status: 'active',
    })
    expect(response.code).toBe(201)
    expect(response.data.username).toBe('testuser')
  })
})
```

---

## API Migration Guide

### From Version 1 to 2

| Change | v1 | v2 |
|--------|----|----|
| Auth | Body-based | Header-based |
| Pagination | Page-based | Cursor-based |
| Errors | Simple | Structured |

---

## API Checklist

- [ ] Endpoints are RESTful
- [ ] Responses follow standard format
- [ ] Authentication is enforced
- [ ] Authorization is checked
- [ ] Errors are properly handled
- [ ] OpenAPI specs are updated
- [ ] Tests are written
- [ ] Documentation is complete

---

## Next Steps

1. Review OpenAPI spec in `docs/api/openapi.yaml`
2. Test endpoints with `curl` or Postman
3. Review `backend/src/app/routes.rs` for endpoint implementation
4. Check `backend/src/middleware/auth.rs` for auth implementation
