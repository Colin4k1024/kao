# Codebase Concerns

**Analysis Date:** 2026-03-27

## Tech Debt

**AppState Type Mismatch:**
- Issue: Router<State> vs Router<()> type mismatch
- Files: `src/features/*/routes.rs`, `src/app.rs`
- Impact: Compilation failures preventing project build
- Fix approach: Standardize on Router<()> or create app-wide state

**AppState Missing `db` Field:**
- Issue: AppState uses `pool` field but some code references `db`
- Files: Many service files
- Impact: Cannot access database
- Fix approach: Rename all `db` references to `pool` or use a type alias

**AppState Missing `Settings` Field:**
- Issue: AppState structure missing in some files
- Files: Configuration-dependent code
- Impact: Cannot access application settings
- Fix approach: Ensure AppState is properly imported and used

**Audit Logger Database Implementation:**
- Issue: `sqlx::query!` macros fail at compile time due to missing database
- Files: `src/common/security/audit_logger.rs`
- Impact: Cannot compile without database connection
- Fix approach: Comment out or use `sqlx::query_as!` with type annotations

**Missing MetricsMiddleware Exports:**
- Issue: MetricsMiddleware struct doesn't exist, replaced with metrics_middleware function
- Files: `src/common/metrics/mod.rs`
- Impact: Middleware cannot be used
- Fix approach: Update imports to reference metrics_middleware function

**FromRow Trait Missing:**
- Issue: LoginLog, OnlineUser, OperationLog structs don't implement FromRow trait
- Files: `src/features/monitoring/login_log/mod.rs`, `src/features/monitoring/online_user/mod.rs`, `src/features/monitoring/operation_log/mod.rs`
- Impact: Cannot use sqlx query macros with these types
- Fix approach: Implement FromRow trait or use manual row iteration

## Known Bugs

**Hex Encoding Mismatch:**
- Symptoms: `hex::encode` function not found
- Files: `src/features/menus/routes.rs`, `src/features/roles/routes.rs`
- Trigger: Building project with hex 0.4.x
- Workaround: Ensure correct hex version in Cargo.toml

**MD5 Digest Conversion:**
- Symptoms: `md5::Digest` doesn't implement `AsRef<[u8]>`
- Files: `src/features/roles/routes.rs`
- Trigger: Using md5::compute on body
- Workaround: Use hex::encode(md5::compute(body).0) for bytes conversion

## Security Considerations

**JWT Secret Default:**
- Risk: Default secret "change-me-in-production" in auth middleware
- Files: `src/common/auth/middleware.rs`
- Current mitigation: Environment variable override
- Recommendations: Add compile-time check for production environment

**Password Hashing Simplification:**
- Risk: Some login handlers bypass password verification
- Files: `src/api/auth/handlers.rs`, `src/app.rs`
- Current mitigation: Commented as "In production..."
- Recommendations: Implement proper bcrypt verification

## Performance Bottlenecks

**Database Query Compilation:**
- Problem: `sqlx::query!` macros require runtime database connection
- Files: Multiple database query macros
- Cause: SQLx macro validation requires database at compile time
- Improvement path: Use `sqlx::query_as!` with runtime query building

**Static Pool Mutability:**
- Problem: Cannot borrow static DB_POOL as mutable
- Files: `src/common/db.rs`
- Cause: Static variable requires unsafe for mutation
- Improvement path: Use_once_lock with proper initialization

## Fragile Areas

**AppState Field Access:**
- Files: Multiple files accessing AppState fields
- Why fragile: AppState definitions may diverge across modules
- Safe modification: Use a type alias or central configuration
- Test coverage: Low, requires integration tests

**Metrics Type Changes:**
- Files: `src/common/metrics/mod.rs`, `src/common/metrics/performance_monitor.rs`
- Why fragile: prometheus crate API changes between versions
- Safe modification: Pin exact versions in Cargo.toml
- Test coverage: Basic unit tests present

## Scaling Limits

**Single Database Pool:**
- Current capacity: Limited by PgPool configuration
- Limit: No pooling strategy implemented
- Scaling path: Implement connection pooling with metrics

**No Caching Strategy:**
- Current capacity: Every request hits database
- Limit: High latency for frequently accessed data
- Scaling path: Implement Redis caching layer

## Dependencies at Risk

**Hex Version Conflict:**
- Risk: Two versions of hex crate (0.2 and 0.4)
- Impact: Compile errors when using encode
- Migration plan: Pin to exactly hex = "0.4"

**Prometheus API Changes:**
- Risk: prometheus crate breaking changes
- Impact: Metrics collection may fail
- Migration plan: Pin to exact version prometheus = "0.13"

## Missing Critical Features

**Authentication Middleware:**
- Problem: AuthUser extractor not fully implemented
- Files: `src/common/auth/extractor.rs`
- Blocks: Route protection, user-specific operations
- Priority: High

**Database Seed Data:**
- Problem: No seed data script
- Blocks: Initial deployment
- Priority: Medium

## Test Coverage Gaps

**No Route Tests:**
- What's not tested: Route definitions, middleware chain
- Files: `src/features/*/routes.rs`
- Risk: Routes may break without detection
- Priority: Medium

**No Integration Tests:**
- What's not tested: Full request/response cycle
- Files: None
- Risk: Integration issues undetected
- Priority: Medium

**No Database Migrations:**
- What's not tested: Schema changes
- Files: None (migrations/ directory empty)
- Risk: Production schema drift
- Priority: High

---

*Concerns audit: 2026-03-27*
