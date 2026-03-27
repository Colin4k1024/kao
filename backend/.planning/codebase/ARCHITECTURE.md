# Architecture

**Analysis Date:** 2026-03-27

## Pattern Overview

**Overall:** Layered Architecture withAxum Router

**Key Characteristics:**
- Separation of concerns: routes → services → repositories → database
- Feature-based module organization
- Async/await pattern throughout
- Result<T, AppError> for error handling

## Layers

**Handler Layer (routes):**
- Purpose: HTTP request routing and response formatting
- Location: `src/api/*/handlers.rs`, `src/features/*/routes.rs`
- Contains: Router definitions, route handlers
- Depends on: Services
- Used by: App state assembly in `src/app.rs`

**Service Layer:**
- Purpose: Business logic and orchestration
- Location: `src/features/*/service.rs`
- Contains: Business rules, validation, composition
- Depends on: Repositories, error types
- Used by: Handlers

**Repository Layer (Repo):**
- Purpose: Database access abstraction
- Location: `src/features/*/repo.rs`
- Contains: SQL queries using sqlx macros
- Depends on: sqlx, models
- Used by: Services

**Model Layer:**
- Purpose: Data structures and types
- Location: `src/features/*/model.rs`, `src/models/`
- Contains: Request/response DTOs, domain models
- Depends on: serde, chrono, uuid

**Common Layer:**
- Purpose: Shared utilities, middleware, authentication
- Location: `src/common/`
- Contains: error handling, middleware, metrics
- Used by: All application layers

## Data Flow

**Request Flow:**

1. HTTP request arrives at Router
2. Middleware chain processes request
3. Route handler extracts parameters
4. Handler calls Service method
5. Service validates input and calls Repository
6. Repository executes SQL query via sqlx
7. Results transformed to response DTOs
8. Response sent back through middleware
9. HTTP response returned

**State Management:**
- AppState: Holds database pool and configuration
- State extraction: axum::extract::State<AppState>
- Immutable access to shared state

## Key Abstractions

**AppError:**
- Purpose: Unified error type for all operations
- Pattern: Enum-based error types
- Example: `src/common/error.rs`
- Variants: Database, Authentication, Authorization, Validation, Internal

**AppState:**
- Purpose: Shared application state across requests
- Pattern: Cloneable struct with Arc internally
- Fields: pool (PgPool), settings (Settings)
- Example: `src/app.rs`

**MetricsMiddleware:**
- Purpose: Automatic metrics collection for HTTP requests
- Pattern: Axum middleware function
- Features: Request counts, duration, error rates
- Example: `src/common/metrics/middleware.rs`

## Entry Points

**Main Entry (src/main.rs):**
- Location: `src/main.rs`
- Triggers: Program execution
- Responsibilities: Initialize logging, create pool, build router

**App Builder (src/app.rs):**
- Location: `src/app.rs`
- Triggers: Application startup
- Responsibilities: Assembles Router with middleware

**API Endpoints (src/api/*/):**
- Location: `src/api/system/*/handlers.rs`
- Triggers: HTTP request matching route
- Responsibilities: Request validation, response creation

## Error Handling

**Strategy:** 
- All public functions return `Result<T, AppError>`
- Automatic conversion via From traits
- Type-specific error variants

**Patterns:**
- Database errors → AppError::Database
- Authentication errors → AppError::Authentication
- Validation errors → AppError::Validation
- Internal errors → AppError::Internal

## Cross-Cutting Concerns

**Logging:** tracing crate with env-filter
**Validation:** semantic validation, custom validators
**Authentication:** JWT with claims extraction
**Authorization:** Permission-based access control
**Metrics:** Prometheus-compatible metrics
**Caching:** Redis (configurable, disabled by default)

---

*Architecture analysis: 2026-03-27*
