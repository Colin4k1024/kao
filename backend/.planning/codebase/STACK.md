# Technology Stack

**Analysis Date:** 2026-03-27

## Languages

**Primary:**
- Rust 2021 Edition - All backend code

## Runtime

**Environment:**
- Rust stable (edition 2021)
- Tokio async runtime

**Package Manager:**
- Cargo (builtin)
- Lockfile: Cargo.lock (present)

## Frameworks

**Core:**
- axum 0.7 - Web framework
- tower - Middleware library

**Database:**
- sqlx 0.8 - Async query library for PostgreSQL
- PostgreSQL - Database

**Authentication:**
- jsonwebtoken 9 - JWT implementation
- bcrypt 0.15 - Password hashing

**Monitoring:**
- prometheus 0.13 - Metrics collection
- tracing 0.1 - Logging
- tracing-subscriber 0.3 - Logging subscriber

**Validation:**
- thiserror 1 - Error definitions
- anyhow 1 - Error handling

## Key Dependencies

**Critical:**
- axum 0.7 - Web framework core
- sqlx 0.8 - Database access
- serde 1 - Serialization
- uuid 1 - Unique ID generation
- chrono 0.4 - Date/time handling

**Infrastructure:**
- tokio 1 - Async runtime
- futures 0.3 - Future utilities
- hex 0.4 - Hex encoding
- sha2 0.10 - SHA-256 hashing
- md5 0.7 - MD5 hashing

## Configuration

**Environment:**
- Uses .env file for secrets
- Database URL configured via environment variable

**Build:**
- Cargo.toml with profiles: dev, release, production

## Platform Requirements

**Development:**
- Rust 1.75+
- PostgreSQL
- tokio-console (optional for debugging)

**Production:**
- Linux/macOS
- PostgreSQL database
- Environment variables for configuration

---

*Stack analysis: 2026-03-27*
