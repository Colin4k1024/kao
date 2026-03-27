# Codebase Structure

**Analysis Date:** 2026-03-27

## Directory Layout

```
backend/
├── src/
│   ├── main.rs                     # Entry point
│   ├── app.rs                      # App assembly and AppState
│   ├── lib.rs                      # Library exports
│   ├── config/                     # Configuration
│   │   └── settings.rs
│   ├── common/                     # Shared utilities
│   │   ├── auth/                   # JWT and credentials
│   │   │   ├── claims.rs          # JWT claims
│   │   │   ├── extractor.rs       # AuthUser extractor
│   │   │   ├── jwt.rs             # Token generation/validation
│   │   │   └── middleware.rs      # Auth middleware
│   │   ├── middleware/             # Axum middleware
│   │   │   ├── caching.rs         # HTTP caching (etag, etc.)
│   │   │   ├── load_balancer.rs   # Sticky sessions
│   │   │   └── openapi.rs         # OpenAPI docs
│   │   ├── metrics/                # Prometheus metrics
│   │   │   ├── alerting.rs        # Alert rules
│   │   │   ├── middleware.rs      # Metrics collection
│   │   │   └── performance_monitor.rs
│   │   ├── security/               # Security utilities
│   │   │   ├── audit_logger.rs    # Security event logging
│   │   │   ├── password_policy.rs
│   │   │   ├── password_expiration.rs
│   │   │   └── scanner.rs
│   │   ├── cache/                  # Redis caching
│   │   │   └── redis.rs
│   │   ├── error.rs                # AppError type
│   │   ├── response.rs             # ApiResponse wrapper
│   │   ├── db.rs                   # Database pool
│   │   └── mod.rs
│   ├── features/                   # Feature modules
│   │   ├── auth/                   # Authentication
│   │   │   ├── handlers.rs
│   │   │   ├── model.rs
│   │   │   ├── repo.rs
│   │   │   ├── service.rs
│   │   │   └── routes.rs
│   │   ├── users/                  # User management
│   │   │   ├── model.rs
│   │   │   ├── repo.rs
│   │   │   ├── service.rs
│   │   │   └── routes.rs
│   │   ├── roles/                  # Role management
│   │   │   ├── model.rs
│   │   │   ├── repo.rs
│   │   │   ├── service.rs
│   │   │   └── routes.rs
│   │   ├── departments/            # Department management
│   │   │   ├── model.rs
│   │   │   ├── repo.rs
│   │   │   ├── service.rs
│   │   │   └── routes.rs
│   │   ├── menus/                  # Menu management
│   │   │   ├── model.rs
│   │   │   ├── repo.rs
│   │   │   ├── service.rs
│   │   │   └── routes.rs
│   │   ├── dictionary/             # Dictionary management
│   │   │   ├── type/
│   │   │   │   └── routes.rs
│   │   │   └── data/
│   │   │       └── routes.rs
│   │   ├── config/                 # System configuration
│   │   │   └── routes.rs
│   │   ├── notice/                 # Notice management
│   │   │   ├── model.rs
│   │   │   ├── repo.rs
│   │   │   ├── service.rs
│   │   │   └── routes.rs
│   │   └── monitoring/             # System monitoring
│   │       ├── operation_log/      # Operation logs
│   │       │   ├── routes.rs
│   │       │   └── mod.rs
│   │       ├── login_log/          # Login logs
│   │       │   ├── routes.rs
│   │       │   └── mod.rs
│   │       ├── online_user/        # Online users
│   │       │   ├── routes.rs
│   │       │   └── mod.rs
│   │       ├── metrics.rs
│   │       ├── health.rs
│   │       └── mod.rs
│   ├── api/                        # API layer (HTTP handlers)
│   │   └── system/
│   │       ├── user/
│   │       │   ├── handlers.rs
│   │       │   └── mod.rs
│   │       ├── role/
│   │       │   ├── handlers.rs
│   │       │   └── mod.rs
│   │       ├── menu/
│   │       │   ├── handlers.rs
│   │       │   └── mod.rs
│   │       ├── department/
│   │       │   ├── handlers.rs
│   │       │   └── mod.rs
│   │       ├── notice/
│   │       │   ├── handlers.rs
│   │       │   └── mod.rs
│   │       ├── monitor/
│   │       │   ├── handlers.rs
│   │       │   └── mod.rs
│   │       ├── config/
│   │       │   ├── handlers.rs
│   │       │   └── mod.rs
│   │       ├── dictionary/
│   │       │   ├── handlers.rs
│   │       │   └── mod.rs
│   │       ├── post/
│   │       │   ├── handlers.rs
│   │       │   └── mod.rs
│   │       ├── job/
│   │       │   ├── handlers.rs
│   │       │   └── mod.rs
│   │       └── handlers.rs
│   ├── models/                     # Shared models
│   │   ├── user.rs
│   │   ├── role.rs
│   │   ├── menu.rs
│   │   └── department.rs
│   ├── repositories/               # Shared repository access
│   │   ├── user_repo.rs
│   │   └── mod.rs
│   ├── services/                   # Shared services
│   │   ├── auth_service.rs
│   │   └── mod.rs
│   └── utils/                      # Utility functions
│       ├── jwt.rs
│       └── response.rs
├── migrations/                     # SQLx migrations
├── tests/                          # Integration tests
├── Cargo.toml                      # Dependencies
└── build.rs                        # Build script (optional)
```

## Directory Purposes

**src/api/system/:**
- Purpose: HTTP handler layer
- Contains: Request/response handling, validation
- Key files: `user/handlers.rs`, `role/handlers.rs`, etc.

**src/features/:**
- Purpose: Feature modules with full CRUD
- Contains: model → service → repo → routes pattern
- Key directories: auth, users, roles, departments, menus

**src/common/:**
- Purpose: Cross-cutting concerns
- Contains: auth, middleware, metrics, security, caching
- Key files: `error.rs`, `middleware/caching.rs`, `metrics/alerting.rs`

**src/models/:**
- Purpose: Shared domain models
- Contains: User, Role, Menu, Department structures

**src/repositories/:**
- Purpose: Shared database access
- Contains: Common query patterns

**src/services/:**
- Purpose: Shared business logic
- Contains: Auth service, other cross-feature logic

## Key File Locations

**Entry Points:**
- `src/main.rs`: Application entry point
- `src/app.rs`: Router assembly and AppState
- `src/lib.rs`: Library exports

**Configuration:**
- `src/config/settings.rs`: Application settings
- `Cargo.toml`: Dependencies and profiles

**Core Logic:**
- `src/features/auth/`: Authentication feature
- `src/features/users/`: User management feature
- `src/features/roles/`: Role management feature
- `src/common/error.rs`: Error handling

**Testing:**
- `tests/`: Integration tests
- Cargotest configuration

## Naming Conventions

**Files:**
- Module files: `mod.rs`
- Models: `model.rs` (singular: `{name}/model.rs`)
- Repositories: `repo.rs` (e.g., `src/features/users/repo.rs`)
- Services: `service.rs`
- Routes: `routes.rs`
- Handlers: `handlers.rs` (API layer)

**Functions:**
- Public: `camelCase` (e.g., `list_users`, `create_user`)
- Private: `snake_case` (e.g., `compute_hash`, `validate_input`)
- Async: `snake_case`suffix with `_async` if needed (typically just underscore)

**Variables:**
- Public: `snake_case` (e.g., `user_id`, `page_size`)
- Private: `snake_case`
- Constants: `SCREAMING_SNAKE_CASE` (e.g., `MAXPageSize`)

**Types:**
- Structs: `PascalCase` (e.g., `UserService`, `AppState`)
- Enums: `PascalCase` (e.g., `AppError`)
- Traits: `PascalCase` `Trait` suffix (e.g., `ResponseExt`)

## Where to Add New Code

**New Feature:**
- Primary code: `src/features/{feature_name}/`
- Models: `src/features/{feature_name}/model.rs`
- Repository: `src/features/{feature_name}/repo.rs`
- Service: `src/features/{feature_name}/service.rs`
- Routes: `src/features/{feature_name}/routes.rs`

**New Component/Module:**
- Implementation: `src/common/{module}/`
- Export in: `src/common/mod.rs`

**Utilities:**
- Shared helpers: `src/utils/`

## Special Directories

**migrations/:**
- Purpose: SQL migrations for schema changes
- Generated: No
- Committed: Yes (database schema tracking)

**tests/:**
- Purpose: Integration and unit tests
- Generated: No
- Committed: Yes

---

*Structure analysis: 2026-03-27*
