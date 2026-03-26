# Codebase Concerns

**Analysis Date:** 2026-03-26

## Security Issues

### Hardcoded Credentials in Login Handler
- **Issue:** `backend/src/app.rs` contains hardcoded admin credentials (`admin` / `admin123`) that bypass JWT authentication entirely
- **Files:** `backend/src/app.rs:45-47`
- **Impact:** Attackers could gain admin access without valid credentials; production systems could be compromised
- **Fix approach:** Replace hardcoded check with proper JWT verification and database lookup; ensure the existing `auth_service.rs` implementation is used instead

### JWT Secret Hardcoded in Token Validation
- **Issue:** `backend/src/common/auth/extractor.rs:43` uses hardcoded secret `"change-me-in-development"` instead of environment variable
- **Files:** `backend/src/common/auth/extractor.rs:43`
- **Impact:** Token validation will fail in production; security vulnerability if secret is weak
- **Fix approach:** Use the same secret from `Settings` or environment variable for both token generation and validation

### Password Validation in API Handler Bypasses bcrypt
- **Issue:** `backend/src/api/auth/handlers.rs:55` uses simple string comparison (`req.password == "admin123"`) instead of bcrypt verification
- **Files:** `backend/src/api/auth/handlers.rs:55`
- **Impact:** Passwords stored in code or logs if they match the hardcoded value; no proper password hashing verification
- **Fix approach:** Use `bcrypt::verify()` to check password hashes against stored hash; ensure consistent with `backend/src/services/auth_service.rs:45`

### CORS Allows All Origins (Overly Permissive)
- **Issue:** `backend/src/middleware/cors.rs:6` uses `.allow_origin(Any)` allowing requests from any origin
- **Files:** `backend/src/middleware/cors.rs:5-10`, `backend/src/app.rs:14-17`
- **Impact:** Cross-origin attacks possible; credentials could be stolen via malicious sites
- **Fix approach:** Configure specific allowed origins from environment variable; only allow trusted frontend domains

### Database Credentials in Connection String
- **Issue:** `backend/src/config/settings.rs:51` defaults to `postgres://postgres:password@localhost:5432/kao_db` with default password
- **Files:** `backend/src/config/settings.rs:51`
- **Impact:** If environment variable not set, default credentials used; weak password
- **Fix approach:** Remove default password requirement; fail fast if `DATABASE_URL` not configured; use strong default password only in development

## Technical Debt

### Inconsistent Authentication Flow
- **Issue:** Multiple authentication implementations coexist:
  - `backend/src/app.rs`: Hardcoded auth with simple token
  - `backend/src/api/auth/handlers.rs`: Password comparison auth
  - `backend/src/services/auth_service.rs`: Proper bcrypt + JWT auth
  - `backend/src/common/auth/extractor.rs`: JWT validation with hardcoded secret
- **Files:** All auth-related files listed above
- **Impact:** Code confusion, security gaps, maintenance burden
- **Fix approach:** Consolidate to single auth flow using bcrypt + JWT; remove duplicate implementations

### Database Connection Not Pooled Properly
- **Issue:** `backend/src/common/db.rs:4-6` creates basic `PgPool` without connection pool configuration
- **Files:** `backend/src/common/db.rs`, `backend/src/main.rs:14`
- **Impact:** No connection pooling configuration (max/min connections, timeouts) from environment
- **Fix approach:** Use `DatabaseSettings` from `config/settings.rs` to configure pool properly; create pool with Settings

### Missing Input Validation middleware
- **Issue:** No input validation middleware present; handlers accept raw `serde_json::Value` without struct validation
- **Files:** `backend/src/app.rs:40-43`, `backend/src/api/auth/handlers.rs:41-42`
- **Impact:** Invalid input accepted; no type safety; potential injection attacks
- **Fix approach:** Implement validation middleware using `validator` crate; use typed request structs with `#[derive(Deserialize, Validate)]`

### No Rate Limiting for Authentication Endpoints
- **Issue:** No rate limiting configured for login/register endpoints
- **Files:** `backend/src/middleware/mod.rs` (empty), `backend/src/middleware/cors.rs`
- **Impact:** Brute force attacks possible on authentication endpoints
- **Fix approach:** Implement rate limiting middleware using `axum-rate-limiter` or similar; store rate limit state in Redis or in-memory for dev

## Operational Complexity

### No Database Migration Script Execution
- **Issue:** Dockerfile copies migrations but no automated migration execution on container startup
- **Files:** `backend/Dockerfile:15`
- **Impact:** Database schema out of sync with code; manual migration required
- **Fix approach:** Add migration execution step in Dockerfile CMD or use a separate migration service; use `sqlx migrate` for Rust

### Missing Health Check Implementation
- **Issue:** `/health` endpoint in `backend/src/app.rs:20` only returns static "OK"; no actual status checks
- **Files:** `backend/src/app.rs:34-36`
- **Impact:** Kubernetes/monitoring cannot determine actual service health
- **Fix approach:** Implement actual health check: database connection, redis (if used), and other dependencies

### No Logging for Authentication Events
- **Issue:** No logging for login attempts, failures, or successful authentications
- **Files:** All auth-related files
- **Impact:** Security audits impossible; no audit trail for incidents
- **Fix approach:** Add logging at INFO level for successful logins, WARN for failures, ERROR for multiple failures

### Frontend Token Storage in localStorage
- **Issue:** `frontend/src/pages/Login.tsx:43-45` stores tokens in localStorage vulnerable to XSS
- **Files:** `frontend/src/pages/Login.tsx:43-45`, `frontend/src/stores/authStore.ts`
- **Impact:** XSS attacks can steal tokens; full account compromise
- **Fix approach:** Use httpOnly cookies for token storage; if localStorage required, implement strict CSP; add token refresh mechanism

## Performance Concerns

### No Database Connection Pool Configuration
- **Issue:** Database pool created without connection limits from environment
- **Files:** `backend/src/common/db.rs`, `backend/src/main.rs:14`
- **Impact:** Under high load, database connections can exhaust; performance degradation
- **Fix approach:** Use `DatabaseSettings` from config to configure max_connections, min_connections, idle_timeout, connect_timeout

### No Caching Layer
- **Issue:** No caching configured for frequently accessed data (menus, roles, departments)
- **Files:** All read-heavy endpoints
- **Impact:** Database load increases with traffic; slow response times
- **Fix approach:** Implement Redis caching for menu trees, role lists, department trees; use TTL-based caching

### No API Response Caching Headers
- **Issue:** No cache-control headers set on read endpoints
- **Files:** `backend/src/api/system/**/*.rs`
- **Impact:** Browser cannot cache responses; repeated requests to backend
- **Fix approach:** Add `Cache-Control` headers to appropriate responses; use ETag for conditional requests

## Scaling Limits

### No Database Connection Limits
- **Issue:** Default database connection pool size not configurable; defaults to 10 connections
- **Files:** `backend/src/config/settings.rs:52-57`
- **Impact:** Under high traffic, connection exhaustion; request queuing; timeouts
- **Fix approach:** Configure DATABASE_MAX_CONNECTIONS env var; document connection pool sizing guide

### No Horizontal Scaling Support
- **Issue:** No sticky session support or load balancer configuration; session state in memory
- **Files:** `backend/src/app.rs`, `backend/src/main.rs`
- **Impact:** Cannot deploy multiple backend instances behind load balancer
- **Fix approach:** Use Redis for session storage if needed; ensure stateless JWT authentication; configure load balancer for sticky sessions if needed

## Missing Critical Features

### No Refresh Token Rotation
- **Issue:** Refresh tokens are not rotated on use; no refresh token revocation mechanism
- **Files:** `backend/src/utils/jwt.rs`, `backend/src/common/auth/jwt.rs`
- **Impact:** Stolen refresh tokens can be used indefinitely; security risk
- **Fix approach:** Implement refresh token rotation; store refresh token hashes in database; revoke on logout

### No Account Lockout Mechanism
- **Issue:** No account lockout after multiple failed login attempts
- **Files:** `backend/src/api/auth/handlers.rs:40-74`
- **Impact:** Brute force attacks possible; accounts can be locked out without notification
- **Fix approach:** Track failed login attempts per IP and username; implement temporary lockout after threshold

### No Password Policy Enforcement
- **Issue:** No password complexity requirements on registration
- **Files:** `backend/src/services/auth_service.rs:66-77`
- **Impact:** Users can set weak passwords; security compromise
- **Fix approach:** Enforce minimum length, complexity requirements; check against common password list

### No Password Expiration Policy
- **Issue:** Passwords never expire; no password rotation required
- **Files:** `backend/src/models/user.rs`
- **Impact:** Long-lived passwords increase breach impact; compliance issues
- **Fix approach:** Add password_changed_at field and expiration policy; require rotation every 90 days

## Test Coverage Gaps

### No Backend Unit Tests
- **Issue:** No backend Rust unit tests present
- **Files:** No `backend/src/**/*_test.rs` or `backend/src/**/*_spec.rs`
- **Impact:** Backend logic cannot be tested; regression risk high
- **Priority:** High

### No Integration Tests for Auth Flow
- **Issue:** Playwright tests only cover frontend; no integration tests for backend APIs
- **Files:** `tests/login.spec.ts`, `tests/basic.spec.ts`
- **Impact:** Auth flow not tested end-to-end; security vulnerabilities not caught
- **Priority:** High

### No Security Tests
- **Issue:** No tests for security scenarios (SQL injection, XSS, CSRF, auth bypass)
- **Files:** All test files
- **Impact:** Security vulnerabilities not prevented; compliance risk
- **Priority:** Critical

## Known Bugs

### Claim Mismatch between JWT Implementations
- **Symptoms:** `Claims` struct in `backend/src/common/auth/claims.rs` vs `backend/src/utils/jwt.rs` have different fields
- **Files:** `backend/src/common/auth/claims.rs:4-13`, `backend/src/utils/jwt.rs:16-24`
- **Impact:** Token generation and validation incompatible; authentication failures
- **Workaround:** None; must consolidate claims structures

### Database Schema Mismatch
- **Symptoms:** `backend/src/repositories/user_repo.rs:27` queries `users` table but migration creates `sys_users`
- **Files:** `backend/src/repositories/user_repo.rs:27-39`
- **Impact:** Database queries fail; users cannot be loaded
- **Workaround:** Table name must match schema; update queries to use `sys_users` instead of `users`

### Token Generation Incompatibility
- **Symptoms:** `backend/src/api/auth/handlers.rs:57` uses simple token format while `backend/src/services/auth_service.rs` uses JWT
- **Files:** `backend/src/api/auth/handlers.rs:57`, `backend/src/services/auth_service.rs:51-56`
- **Impact:** Token format mismatch; frontend cannot parse token consistently
- **Workaround:** Use single token generation mechanism throughout codebase

## Recommendations Summary

### Critical (Do Immediately)
1. Remove hardcoded credentials from `app.rs`; implement proper bcrypt + JWT auth
2. Fix JWT secret to use environment variable consistently
3. Replace hardcoded password check with bcrypt verification
4. Restrict CORS to specific origins
5. Fix database table name mismatches (`users` → `sys_users`)

### High Priority (This Week)
6. Consolidate authentication implementations to single flow
7. Implement proper database connection pool configuration
8. Add rate limiting to auth endpoints
9. Implement account lockout mechanism
10. Add backend unit tests for auth logic

### Medium Priority (Next Sprint)
11. Implement refresh token rotation
12. Add caching layer for frequent data
13. Implement health check with dependency status
14. Add logging for authentication events
15. Move tokens to httpOnly cookies

### Long-term
16. Implement password policy enforcement
17. Add database migration execution automation
18. Enable horizontal scaling support
19. Add comprehensive security tests
20. Document deployment and scaling procedures

---

*Concerns audit: 2026-03-26*