# Phase 1: Stabilization & Security - Context

**Gathered:** 2026-03-26
**Status:** Ready for planning
**Source:** Phase 1 Roadmap Entry

<domain>
## Phase Boundary

Phase 1 addresses critical security vulnerabilities and establishes a stable foundation for the Kao admin management system. This is a foundational phase with no dependencies on prior phases.

### What this phase delivers:
1. **Authentication Consolidation** - Remove hardcoded credentials, fix JWT secrets, replace password bypass, consolidate to single auth flow
2. **Security Hardening** - Restrict CORS, fix database table mismatches, add validation middleware, rate limiting, account lockout, refresh token rotation
3. **Database & Migrations** - Audit queries, automatic migration execution, connection pool config, fix JWT Claims mismatch
4. **Testing** - Backend unit tests, integration tests, security tests, increase coverage to >50%
5. **Observability** - Proper health check endpoint, logging for auth events, structured logging config, request tracking

### Out of Scope:
- OAuth2/SAML external authentication
- Multi-tenant architecture
- Advanced caching with Redis (PostgreSQL only)
- Frontend UI redesigns
- Performance optimization (covered in Phase 3)

</domain>

<decisions>
## Implementation Decisions

### Locked Decisions (from Phase 1 Requirements)

1. **Authentication Strategy: JWT + bcrypt (locked)**
   - Use bcrypt for password hashing with cost factor 12+
   - JWT tokens with 1-hour expiration
   - JWT secret must come from environment variable (NO hardcoded secrets)
   - Refresh token mechanism required
   - Authorization header for access tokens (Bearer scheme)
   
   **Reason:** Security requirements in REQUIREMENTS.md NFR1 and phase goal
   **Reference:** REQUIREMENTS.md FR1, NFR1

2. **Security First Approach (locked)**
   - Zero hardcoded credentials or secrets allowed
   - All API endpoints must include authentication
   -禁止 direct SQL concatenation - must use ORM (SQLx)
   - All user input must be validated
   
   **Reason:** Multiple security vulnerabilities found in codebase (CONCERNS.md)
   **Reference:** AGENTS.md "安全" section

3. **CORS Must Be Restricted (locked)**
   -禁止 allow-all-origins configuration
   - Configure specific allowed origins from environment variable
   - Only allow trusted frontend domains (localhost:5173, production domain)
   
   **Reason:** CORS allows all origins is a security vulnerability (CONCERNS.md line 25-29)
   **Reference:** CONCERNS.md line 25-29

4. **Database Table Names Must Match Schema (locked)**
   - Queries must use `sys_users`, `sys_roles`, `sys_departments`, etc.
   - Table name mismatches cause query failures
   - Audit all queries and fix to match migration tables
   
   **Reason:** Database table name mismatches cause query failures (CONCERNS.md line 181-185)
   **Reference:** CONCERNS.md database schema mismatch section

5. **Testing Minimum 50% Coverage (locked)**
   - Critical paths must have tests
   - Security tests required (SQL injection, XSS, CSRF)
   - Integration tests for auth flow required
   
   **Reason:** Phase 1 success criteria specifies >50% test coverage
   **Reference:** ROADMAP.md "Phase 1: Stabilization & Security" success criteria

6. **Health Check Must Check Dependencies (locked)**
   - /health endpoint must return actual status checks
   - Not just static "OK" response
   - Must include database, Redis (if used) status
   - Kubernetes/monitoring compatibile
   
   **Reason:** Existing health check returns static "OK" with no dependency checks (CONCERNS.md line 75-79)
   **Reference:** REQUIREMENTS.md NFR3 reliability section

### the agent's Discretion

1. **Frontend Token Storage Strategy**
   - Current: localStorage (vulnerable to XSS)
   - Recommended: httpOnly cookies (secure)
   - Decision: Use httpOnly cookies for tokens, implement CSRF protection
   - **Reason:** Security best practice; localStorage is XSS vulnerability
   - **Note:** This requires backend cookie configuration

2. **Rate Limiting Implementation**
   - Options: axum-rate-limiter, custom middleware
   -Decision: Use axum-rate-limiter crate for simplicity and proven implementation
   - **Reason:** Proven library with good API configurability

3. **Caching Strategy**
   - Options: Redis, in-memory cache, response headers
   - Decision: Phase 1 uses response caching headers only; Phase 3 introduces Redis
   - **Reason:** Keep Phase 1 focused on security; caching is performance optimization (Phase 3)

4. **Logging Structure**
   - Options: tracing, log, custom
   - Decision: Continue using tracing crate (already implemented in backend)
   - **Reason:** Existing codebase uses tracing; minimal change needed

5. **Database Migration Strategy**
   - Options: sqlx migrate, custom migration runner
   - Decision: Use sqlx migrate for automated migration execution
   - **Reason:** Native integration with SQLx; automatic on startup

</decisions>

<canonical_refs>
## Canonical References

**Downstream agents MUST read these before planning or implementing.**

### Phase Requirements
- `.planning/REQUIREMENTS.md` — Phase 1 functional and non-functional requirements
- `.planning/ROADMAP.md` — Phase 1 deliverables and success criteria
- `.planning/STATE.md` — Project decisions and progress tracking

### Codebase Analysis
- `.planning/codebase/CONCERNS.md` — Critical security issues to fix in Phase 1
- `.planning/codebase/STACK.md` — Technology stack (Rust, Axum, PostgreSQL)
- `.planning/codebase/ARCHITECTURE.md` — Layered architecture with RBAC
- `.planning/codebase/STRUCTURE.md` — Directory structure patterns
- `.planning/codebase/CONVENTIONS.md` — Code style and conventions
- `.planning/codebase/TESTING.md` — Testing guidelines and patterns

### Project Documentation
- `AGENTS.md` — AI coding project core rules and security requirements
- `DEPLOYMENT.md` — Deployment guide for production
- `docker-compose.yml` — Docker Compose configuration
- `backend/Cargo.toml` — Backend dependencies and configuration
- `frontend/package.json` — Frontend dependencies and scripts

</canonical_refs>

<specifics>
## Specific Ideas

### Security Fixes Priority Order:
1. Remove hardcoded credentials from `backend/src/app.rs` (line 45-47)
2. Fix JWT secret to use environment variable in `backend/src/common/auth/extractor.rs:43`
3. Replace hardcoded password check with bcrypt in `backend/src/api/auth/handlers.rs:55`
4. Restrict CORS to specific origins in `backend/src/middleware/cors.rs`
5. Fix database table names (`users` → `sys_users`)
6. Consolidate authentication implementations

### Testing Requirements:
- Backend unit tests for auth logic (bcrypt, JWT token operations)
- Integration tests for auth flow (login, token refresh, logout)
- Security tests (SQL injection, XSS prevention, CSRF protection)
- Test coverage >50% for critical paths

### Code Quality Gates:
- ESLint + Prettier for frontend
- Clippy warnings fixed for Rust
- TypeScript strict mode enabled
- All API responses follow unified format

</specifics>

<deferred>
## Deferred Ideas

These items are explicitly deferred to future phases:

- OAuth2/SAML external authentication (Phase 3)
- Multi-tenant architecture (not planned)
- Advanced caching with Redis (Phase 3)
- Message queue integration (not planned)
- WebSocket real-time communication (not planned)
- API gateway features (not planned)
- Frontend UI redesigns (Phase 2 focuses on feature completion)
- Performance optimization (Phase 3)

</deferred>

---

**Phase:** 01-stabilization-security
**Context gathered:** 2026-03-26
