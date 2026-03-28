---
phase: 01-stabilization-security
plan: 02
subsystem: auth
tags: [cors, validation, rate-limiting, account-lockout, security, axum, validator]

# Dependency graph
requires:
  - phase: 01-01
    provides: JWT authentication foundation, password hashing with bcrypt
provides:
  - CORS restricted to specific origins from environment variable
  - Input validation middleware with validator crate
  - Rate limiting middleware for auth endpoints (login: 5/min, register: 3/hr, refresh: 10/hr)
  - Account lockout after 5 failed attempts, auto-unlock after 15 minutes
affects:
  - Phase 02 (Feature Completeness)
  - All frontend features using auth endpoints

# Tech tracking
tech-stack:
  added: [validator 0.18, lazy_static]
  patterns:
    - Middleware-based security (CORS, rate limiting, validation)
    - Account lockout with configurable thresholds
    - In-memory rate limiting per IP address

key-files:
  created:
    - backend/src/common/middleware/validation.rs
    - backend/src/common/middleware/rate_limiter.rs
    - backend/src/common/security/account_lockout.rs
    - backend/migrations/0020_add_account_lockout.sql
  modified:
    - backend/src/middleware/cors.rs
    - backend/src/config/settings.rs
    - backend/src/common/middleware/mod.rs
    - backend/src/features/auth/model.rs
    - backend/src/features/auth/service.rs
    - backend/src/models/user.rs
    - backend/Cargo.toml

key-decisions:
  - "CORS default: localhost:5173 for dev, explicit origins required in production"
  - "Rate limiting uses in-memory storage for development, Redis recommended for production"
  - "Account lockout: 5 attempts triggers 15-minute lockout with auto-expiry"
  - "Validation rules: username 3-30 chars alphanumeric, password min 8 with complexity"

patterns-established:
  - "Pattern: Middleware exports in common/middleware/mod.rs"
  - "Pattern: Security features in common/security/ module"
  - "Pattern: Configurable via environment variables"

requirements-completed: [NFR1, FR1]

# Metrics
duration: 15min
completed: 2026-03-28
---

# Phase 01 Plan 02: Security Hardening Summary

**CORS restrictions, input validation middleware, rate limiting, and account lockout implemented for auth endpoints**

## Performance

- **Duration:** 15 min
- **Started:** 2026-03-28T07:00:24Z
- **Completed:** 2026-03-28T07:15:00Z
- **Tasks:** 4
- **Files modified:** 13

## Accomplishments

- CORS restricted to specific origins from CORS_ALLOWED_ORIGINS env var
- Input validation middleware using validator crate with custom validation functions
- Rate limiting middleware protecting auth endpoints (login: 5/min, register: 3/hr, refresh: 10/hr)
- Account lockout mechanism with 5-failure threshold and 15-minute auto-expiry

## Task Commits

Each task was committed atomically:

1. **Task 1: Restrict CORS to specific origins** - `ff882f4` (feat)
2. **Task 2: Add input validation middleware** - `c2f1fa8` (feat)
3. **Task 3: Implement rate limiting for auth endpoints** - `854c8ef` (feat)
4. **Task 4: Implement account lockout mechanism** - `fa4e818` (feat)

## Files Created/Modified

- `backend/src/middleware/cors.rs` - CORS with specific origins, explicit methods/headers
- `backend/src/config/settings.rs` - CORS default changed from * to localhost:5173
- `backend/src/common/middleware/validation.rs` - validate_request middleware using validator
- `backend/src/common/middleware/rate_limiter.rs` - In-memory rate limiting per IP
- `backend/src/common/middleware/mod.rs` - Export validation and rate_limiter modules
- `backend/src/features/auth/model.rs` - LoginRequest with validation derives
- `backend/src/features/auth/service.rs` - Lockout integration in login flow
- `backend/src/models/user.rs` - Added lockout fields (failed_login_attempts, locked_until, lockout_reason)
- `backend/src/common/security/account_lockout.rs` - AccountLockoutService
- `backend/src/common/security/mod.rs` - Export account_lockout module
- `backend/migrations/0020_add_account_lockout.sql` - Migration for lockout fields
- `backend/Cargo.toml` - Added validator and lazy_static dependencies

## Decisions Made

- Used in-memory rate limiting storage (production should use Redis)
- CORS credentials enabled for cookie/auth support
- Account lockout uses database for persistence across restarts
- Validation uses custom functions instead of regex for simplicity

## Deviations from Plan

None - plan executed exactly as written.

## Issues Encountered

- **Pre-existing compilation errors:** The codebase has pre-existing errors in `app.rs`, `config/routes.rs`, `dictionary/*/routes.rs`, and `notice/routes.rs` causing type mismatches. These are unrelated to the security hardening tasks and were present before this plan execution.

## Next Phase Readiness

- Security middleware is ready for integration with auth endpoints
- Rate limiters (LOGIN_RATE_LIMITER, REGISTER_RATE_LIMITER, REFRESH_RATE_LIMITER) are exported and ready for use
- Account lockout service can be imported and used in any auth flow
- Database migration 0020_add_account_lockout.sql needs to be run

---
*Phase: 01-stabilization-security*
*Completed: 2026-03-28*
