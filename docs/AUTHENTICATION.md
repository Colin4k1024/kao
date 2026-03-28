# Authentication Documentation

## Overview

Kao uses a unified authentication flow based on JWT (JSON Web Tokens) and bcrypt password hashing. This document describes the complete authentication architecture.

## Authentication Flow

### Login Process

```
Client                  Server                     Database
  |                        |                           |
  |---POST /api/auth/login-|                           |
  |   {username, password} |                           |
  |                        |---Query user by username-|
  |                        |<--User record (incl hash)-|
  |                        |                           |
  |                        |---bcrypt::verify(password, hash)
  |                        |<--true/false              |
  |                        |                           |
  |                        |---JWT token generation----|
  |                        |                           |
  |<--200 {token, user}----|                           |
```

### Token Validation Process

```
Client                  Server
  |                        |
  |---GET /api/resource----|
  |   Authorization: Bearer |
  |                        |---Extract & validate JWT--|
  |                        |   (using JWT_SECRET env)  |
  |                        |<--Claims {user_id, ...}  |
  |                        |                           |
  |<--200 {data}-----------|                           |
```

## Components

### Backend Components

| Component | File | Purpose |
|-----------|------|---------|
| Auth Handlers | `backend/src/api/auth/handlers.rs` | Login/logout/refresh endpoints |
| JWT Utils | `backend/src/utils/jwt.rs` | Token creation and validation |
| Auth Extractor | `backend/src/common/auth/extractor.rs` | Request authentication extraction |
| Auth Service | `backend/src/services/auth_service.rs` | Business logic for auth |
| User Repository | `backend/src/repositories/user_repo.rs` | Database user operations |
| Settings | `backend/src/config/settings.rs` | Configuration including JWT secret |

### Password Security

- Passwords are hashed using **bcrypt** with default cost factor (12)
- Plaintext passwords are never stored
- Password verification uses `bcrypt::verify()`
- Common password checks and complexity validation available in `backend/src/common/security/password_policy.rs`

### JWT Configuration

| Setting | Environment Variable | Default | Description |
|---------|----------------------|---------|-------------|
| JWT Secret | `JWT_SECRET` | - | **Required**. Must be at least 32 characters |
| Access Token Expiry | `JWT_ACCESS_TOKEN_EXPIRES_IN` | 3600 | Access token lifetime in seconds |
| Refresh Token Expiry | `JWT_REFRESH_TOKEN_EXPIRES_IN` | 604800 | Refresh token lifetime in seconds (7 days) |

## API Endpoints

### POST /api/auth/login

**Request:**
```json
{
  "username": "admin",
  "password": "your-password"
}
```

**Response (Success):**
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
      "phone": "13800138000",
      "avatar": null,
      "status": 1,
      "roles": ["admin"],
      "permissions": ["*"]
    }
  }
}
```

**Response (Error):**
```json
{
  "code": 401,
  "message": "用户名或密码错误",
  "data": null
}
```

### POST /api/auth/logout

**Headers:** `Authorization: Bearer <token>`

**Response:**
```json
{
  "code": 200,
  "message": "退出成功",
  "data": null
}
```

### POST /api/auth/refresh

**Request:**
```json
{
  "refresh_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
}
```

**Response:**
```json
{
  "code": 200,
  "message": "Token refreshed successfully",
  "data": {
    "access_token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
    "token_type": "Bearer",
    "expires_in": 3600
  }
}
```

## Environment Variables

Required environment variables for authentication:

```bash
# JWT Configuration (REQUIRED)
JWT_SECRET=your-super-secret-key-change-in-production-min-32-chars

# Optional JWT settings
JWT_ACCESS_TOKEN_EXPIRES_IN=3600      # 1 hour
JWT_REFRESH_TOKEN_EXPIRES_IN=604800   # 7 days

# Database (required for user lookup)
DATABASE_URL=postgres://user:pass@localhost:5432/kao_db
```

## Security Best Practices

1. **Never commit .env files** - Use `.env.example` as a template
2. **Use strong JWT secrets** - Minimum 32 characters, randomly generated
3. **Set appropriate token expiry** - Short-lived access tokens (1 hour), longer refresh tokens (7 days)
4. **Use HTTPS in production** - Never transmit tokens over unencrypted connections
5. **Store passwords securely** - Always use bcrypt, never plaintext
6. **Validate input** - All user inputs are validated before processing

## Error Codes

| Code | Message | Description |
|------|---------|-------------|
| 200 | 登录成功 | Login successful |
| 401 | 用户名或密码错误 | Invalid credentials |
| 401 | 用户不存在 | User not found |
| 401 | Token expired | JWT token has expired |
| 401 | Invalid authorization header | Malformed Authorization header |
| 500 | 数据库错误 | Database error |
| 500 | 密码验证失败 | Password verification failed |

## Database Schema

The `sys_user` table must include a `password` column storing bcrypt hashes:

```sql
CREATE TABLE sys_user (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) UNIQUE NOT NULL,
    password VARCHAR(255) NOT NULL,  -- bcrypt hash
    email VARCHAR(100),
    phone VARCHAR(20),
    nickname VARCHAR(50),
    avatar VARCHAR(255),
    status INTEGER DEFAULT 1,
    department_id UUID,
    role_id UUID,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT NOW(),
    deleted_at TIMESTAMP WITH TIME ZONE  -- Soft delete
);
```

## Related Documentation

- [Penetration Testing](./security/penetration-testing.md) - Security testing procedures
- [Password Policy](../backend/src/common/security/password_policy.rs) - Password complexity requirements
- [Deployment Guide](./deployment/README.md) - Production deployment checklist
