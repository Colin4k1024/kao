# External Integrations

**Analysis Date:** 2026-03-26

## APIs & External Services

**Backend API:**
- Axum Rust web framework serving RESTful API
  - Port: 8080 (default)
  - Base path: `/api/v1/*` for versioned endpoints
  - Authentication: Bearer JWT tokens
  - Frontend API URL configured in `frontend/.env`

## Data Storage

**Databases:**
- PostgreSQL 15 Alpine
  - Connection: `DATABASE_URL` environment variable
  - Client: SQLx (async Rust ORM)
  - Database name: `kao_db` (default)

**Migrations:**
- SQLx-compatible manual migrations in `backend/migrations/`
- Migrations executed at container startup
- Schema includes RBAC (users, roles, departments, menus)
- Tables: sys_users, sys_roles, sys_departments, sys_menus, sys_user_roles, sys_role_menus, sys_role_departments, sys_config, sys_notice, sys_job, sys_job_log, sys_oper_log, sys_login_log, sys_online_user

**File Storage:**
- Local filesystem only (no external file storage service)

**Caching:**
- None detected in current implementation

## Authentication & Identity

**Auth Provider:**
- Custom JWT implementation
  - Implementation: `jsonwebtoken` crate with custom claims
  - Claims include: sub (user ID), username, permissions, dept_id, roles
  - Token expiration: 24 hours (configured via JWT_ACCESS_TOKEN_EXPIRES_IN)
  - Refresh tokens: Supported via JWT_REFRESH_TOKEN_EXPIRES_IN

**Token Flow:**
1. Login via POST `/api/v1/auth/login`
2. Receive access_token and refresh_token
3. Store in localStorage
4. AttachBearer token to subsequent requests via Authorization header

**Current State:**
- **WARNING**: Implementation has two conflicting login endpoints:
  - `backend/src/app.rs` contains simplified password comparison (insecure)
  - `backend/src/common/auth/jwt.rs` contains proper JWT generate/validation
  - Password hashing uses bcrypt in migrations (not consistently applied)
  - Hardcoded secret in `backend/src/common/auth/extractor.rs`

## Monitoring & Observability

**Error Tracking:**
- None detected (no external error tracking service integrated)

**Logs:**
- `tracing` crate for structured logging
- Log level configurable via RUST_LOG environment variable
- No Log aggregation or visualization (ELK, Loki, etc.)

## CI/CD & Deployment

**Hosting:**
- Docker Compose (development)
- Docker containers for frontend, backend, and postgres

**Current Docker Setup:**
```
 services:
   postgres: PostgreSQL 15 Alpine
   backend: Custom Rust image (rust:1.70-slim builder, debian:bookworm-slim runtime)
   frontend: Custom image with Vite
```

**CI Pipeline:**
- None detected (no GitHub Actions or CI configuration found)

## Environment Configuration

**Required env vars:**

**Backend:**
- `DATABASE_URL` - PostgreSQL connection string (required)
- `JWT_SECRET` - JWT signing secret (required)
- `JWT_ACCESS_TOKEN_EXPIRES_IN` - Access token TTL in seconds (default: 3600)
- `JWT_REFRESH_TOKEN_EXPIRES_IN` - Refresh token TTL in seconds (default: 604800)
- `APP_HOST` - Backend bind address (default: 0.0.0.0)
- `APP_PORT` - Backend port (default: 8080)
- `RUST_LOG` - Logging level (default: info)

**Frontend:**
- `VITE_API_BASE_URL` - Backend server URL (default: http://localhost:8080)

**Secrets location:**
- `.env` files in root of each directory
- `.env.example` provides templates
- **WARNING**: Production should use more secure secret management (Vault, AWS Secrets Manager, etc.)

## Webhooks & Callbacks

**Incoming:**
- None detected (no webhook endpoints)

**Outgoing:**
- None detected (no external webhook integrations)

## API Structure

**Versioning:**
- API versioned under `/api/v1/*` path prefix
- Backend routes defined in `backend/src/features/*/routes.rs`

**Public API Endpoints (v1):**

**Authentication:**
- POST `/api/v1/auth/login` - User login
- GET `/api/v1/auth/profile` - Get current user profile
- GET `/api/v1/auth/session` - Get current session (user + menu tree + permissions)
- GET `/api/v1/auth/permissions` - Get user permissions
- GET `/api/v1/auth/menus` - Get user menu tree

**System Management:**
- Users: GET/POST PUT, DELETE `/api/v1/users`
- Roles: GET/POST PUT, DELETE `/api/v1/roles`
- Departments: GET/POST PUT, DELETE `/api/v1/departments`
- Menus: GET/POST PUT, DELETE `/api/v1/menus`
- Posts: GET/POST PUT, DELETE `/api/v1/posts`
- Config: GET/POST PUT, DELETE `/api/v1/config` (detected in DB, routes TBD)
- Notices: GET/POST PUT, DELETE `/api/v1/notices` (detected in DB, routes TBD)
- Jobs: GET/POST PUT, DELETE `/api/v1/jobs` (detected in DB, routes TBD)

**Frontend-Backend Integration:**

**API Client:**
- Location: `frontend/src/lib/api.ts`
- Axios instance with interceptors
- Automatic token injection from localStorage
- Global error handling with Element Plus notifications
- Response format: `{ code, message, data }`

**Data Fetching:**
- API service layer in `frontend/src/services/api/*`
- React Query integration for data caching
- PageResult type for paginated lists

**Database Integration:**

**SQLx Usage:**
- Connection pool management in `backend/src/common/db.rs`
- Parameterized queries to prevent SQL injection
- Compile-time SQL checking (when enabled)
- Migrations manage schema versioning

**Security Configuration:**
- CORS enabled for all origins (development)
- HTTPS recommended for production
- JWT tokens with expiration
- Password hashing with bcrypt in migrations
- Password field named `password_hash` or `password` (inconsistent)

---

*Integration audit: 2026-03-26*