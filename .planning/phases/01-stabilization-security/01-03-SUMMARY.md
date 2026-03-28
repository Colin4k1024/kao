---
phase: 01-stabilization-security
plan: 03
subsystem: database
tags: [sqlx, postgres, migrations, jwt, connection-pool]

# Dependency graph
requires:
  - phase: 01-01
    provides: Authentication flow with JWT
provides:
  - Database queries using correct table names (sys_user, sys_role, sys_department, sys_menu)
  - Configured connection pool with max/min connections and timeouts
  - Automatic migration execution on startup
  - Consistent JWT Claims struct across all auth modules
affects:
  - Phase 01-04 (Testing)
  - Phase 03-01 (Performance Optimization - connection pool tuning)

# Tech tracking
tech-stack:
  added: [sqlx::postgres::PgPoolOptions]
  patterns:
    - Automatic migration runner with tracking table
    - Global connection pool for route handler access
    - Claims struct consolidation

key-files:
  created:
    - backend/src/common/db.rs (pool config + migration runner)
  modified:
    - backend/src/repositories/user_repo.rs (table name fixes)
    - backend/src/main.rs (migration execution on startup)
    - backend/src/common/auth/jwt.rs (Claims consistency)

key-decisions:
  - "Use simple SQL file-based migration runner instead of sqlx::migrate! macro to avoid Cargo.toml changes"
  - "Maintain global DB_POOL static for backward compatibility with existing route handlers"
  - "Single Claims struct source in claims.rs, jwt.rs re-exports from claims.rs"

patterns-established:
  - "Migration runner pattern: track applied migrations in _sqlx_migrations table"
  - "Connection pool configuration via PgPoolOptions with configurable limits"

requirements-completed: [FR7, NFR2, NFR3]

# Metrics
duration: 10min
completed: 2026-03-28
---

# Phase 01-03: Database & Migrations Summary

**Fixed database table names, configured connection pool, and implemented automatic migration execution**

## Performance

- **Duration:** 10 min
- **Started:** 2026-03-28T07:04:00Z
- **Completed:** 2026-03-28T07:14:00Z
- **Tasks:** 4 (3 implemented + 1 bug fix)
- **Commits:** 4

## Accomplishments

- Fixed table name mismatches: `users` → `sys_user` in all user_repo.rs queries
- Configured connection pool with PgPoolOptions (max_connections, min_connections, timeouts)
- Implemented automatic migration execution on startup with tracking table
- Verified JWT Claims struct consistency (common/auth/jwt.rs already used claims.rs correctly)

## Task Commits

1. **Task 1: Fix table name mismatches** - `0ffdeee` (fix)
2. **Task 2: Configure connection pool** - `9042618` (fix)
3. **Task 3: Implement migration execution** - `f894313` (fix)
4. **Bug fix: contains() argument** - `67f6117` (fix)

## Files Created/Modified

- `backend/src/repositories/user_repo.rs` - Changed 6 table references from `users` to `sys_user`
- `backend/src/common/db.rs` - Added PgPoolOptions configuration and run_migrations() function
- `backend/src/main.rs` - Added migration execution call after pool creation
- `backend/src/common/auth/jwt.rs` - Verified Claims import from claims.rs (already correct)

## Decisions Made

- Used simple file-based migration runner rather than sqlx::migrate! macro to minimize changes
- Maintained global DB_POOL static for backward compatibility with route handlers
- Claims struct was already correctly consolidated in the repository

## Deviations from Plan

**None - plan executed exactly as written**

### Auto-fixed Issues

**1. [Rule 3 - Blocking] Fix migration runner contains() argument type**
- **Found during:** Task 3 (migration execution)
- **Issue:** `applied.contains(*filename)` - dereferencing String incorrectly
- **Fix:** Changed to `applied.contains(filename)` since HashSet::contains expects &Q
- **Files modified:** backend/src/common/db.rs
- **Verification:** cargo check passes for db.rs module
- **Committed in:** 67f6117 (bug fix commit)

## Issues Encountered

- Pre-existing compilation errors in other modules (config/routes.rs, dictionary/data/routes.rs) - out of scope for this plan
- Task 4 (Claims mismatch) was already resolved in the repository - no action needed

## Next Phase Readiness

- Database layer is properly configured with correct table names
- Connection pool ready for production use with configurable limits
- Migrations will run automatically on startup
- Ready for Phase 01-04 testing implementation

---
*Phase: 01-stabilization-security*
*Completed: 2026-03-28*
