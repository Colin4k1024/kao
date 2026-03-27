# External Integrations

**Analysis Date:** 2026-03-27

## APIs & External Services

**PostgreSQL:**
- Database for user, role, menu, department management
- Used by: auth, users, roles, departments, monitoring features
- Client: sqlx
- Connection config: DATABASE_URL env var

## Data Storage

**Databases:**
- PostgreSQL (main)
  - Connection: DATABASE_URL environment variable
  - Client: sqlx (async query library)

**Caching:**
- Redis (configurable, disabled by default)
  - TTL-based caching for users, menus, roles
  - Implementation: common/cache/redis.rs

## Authentication & Identity

**Auth Provider:**
- Custom JWT implementation
  - Implementation: common/auth/jwt.rs
  - Token generation and validation
  - Claims validation without database lookup

## Monitoring & Observability

**Error Tracking:**
- Logging via tracing crate
- Error formatting: common/error.rs

**Logs:**
- Structured logging via tracing
- Environment-based filtering via tracing-subscriber
- Audit logging: common/security/audit_logger.rs

## CI/CD & Deployment

**Hosting:**
- Generic deployment to any platform running Rust

**CI Pipeline:**
- Cargo build + test framework
- No external CI configured

## Environment Configuration

**Required env vars:**
- DATABASE_URL - PostgreSQL connection string
- JWT_SECRET - JWT signing secret
- JWT_EXPIRES_IN - Token expiration time

**Secrets location:**
- .env file (development)
- Environment variables (production)

## Webhooks & Callbacks

**Incoming:**
- None currently implemented

**Outgoing:**
- Alert webhook support in common/metrics/alerting.rs
- Webhook URL configured via alert rules

---

*Integration audit: 2026-03-27*
