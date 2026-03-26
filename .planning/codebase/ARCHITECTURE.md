# Architecture

**Analysis Date:** 2026-03-26

## Pattern Overview

**Overall:** Layered Architecture with Domain-Driven Design principles

**Key Characteristics:**
- Separation of concerns between frontend (React) and backend (Rust/Axum)
- RBAC (Role-Based Access Control) as core security model
- RESTful API design
- Database abstraction with SQLx for Rust and Prisma schema reference

## Layers

**Backend:**

**Controller Layer:**
- Purpose: HTTP request handling and routing
- Location: `backend/src/api/` and `backend/src/features/*/routes.rs`
- Contains: Route definitions and handler functions
- Depends on: Service layer for business logic
- Used by: Axum router in `app.rs`

**Service Layer:**
- Purpose: Business logic orchestration
- Location: `backend/src/features/*/service.rs`
- Contains: Core business rules and workflow coordination
- Depends on: Repository layer for data access
- Used by: Route handlers

**Repository Layer:**
- Purpose: Data access abstraction
- Location: `backend/src/features/*/repo.rs`
- Contains: SQL queries and database operations
- Depends on: SQLx query types
- Used by: Service layer

**Model Layer:**
- Purpose: Data structures and domain models
- Location: `backend/src/features/*/model.rs` and `backend/src/models/`
- Contains: Request/response structs and domain records
- Depends on: None (base layer)

**Database Layer:**
- Purpose: Connection pooling and migration management
- Location: `backend/src/db.rs`, `backend/src/common/db.rs`, `backend/migrations/`
- Contains: Connection pool creation and SQL migrations
- Depends on: SQLx connection pool
- Used by: Repository layer

**Frontend:**

** presentation Layer:**
- Purpose: UI components and pages
- Location: `frontend/src/pages/`
- Contains: React components and pages
- Depends on: Service layer for API calls

**Service Layer:**
- Purpose: API integration
- Location: `frontend/src/services/` and `frontend/src/services/api/`
- Contains: API client and request wrappers
- Depends on: Axios/fetch for HTTP

**State Management:**
- Purpose: Application state
- Location: `frontend/src/useAuth.ts`, `frontend/.umi/model.ts`
- Contains: React hooks for auth state
- Depends on: React Context/Hooks

**Layout Layer:**
- Purpose: Application shell and navigation
- Location: `frontend/src/pages/layout/`
- Contains: MainLayout component
- Used by: All authenticated pages

## Data Flow

**Request Flow (Backend):**

1. Client sends HTTP request → Axum Router (`backend/src/app.rs`)
2. Router matches route → Handler function (`backend/src/features/*/routes.rs`)
3. Handler calls Service (`backend/src/features/*/service.rs`)
4. Service calls Repository (`backend/src/features/*/repo.rs`)
5. Repository executes SQL query via SQLx (`backend/src/common/db.rs`)
6. Database returns data → flows back through layers
7. Handler returns JSON response with统一 error handling

**Request Flow (Frontend):**

1. User interaction → Component event handler
2. Component calls API service (`frontend/src/services/api/`)
3. Service creates HTTP request with auth token
4. Axios/Fetch sends request to backend
5. Response handled → state updated → UI rendered

**State Management:**
- Frontend uses localStorage for auth token persistence
- React Query used for data fetching
- React useState for component-local state
- Authentication state: access_token, refresh_token, user info stored in localStorage

## Key Abstractions

**ApiResponse:**
- Purpose: Unified response format
- Examples:
  - `backend/src/common/response.rs`
  - `frontend/src/types/api.ts`
- Pattern:
  ```typescript
  {
    code: number,      // 200/400/401/etc
    message: string,   // Human-readable message
    data: T            // Response payload
  }
  ```

**AppError:**
- Purpose: Standardized error handling
- Examples: `backend/src/common/error.rs`
- Pattern:
  - Database errors → 500
  - Authentication errors → 401
  - Authorization errors → 403
  - Validation errors → 422

**Authentication:**
- Implementation: JWT tokens with Bearer scheme
- Flow:
  1. User logs in via `/api/auth/login`
  2. Back end issue s JWT token
  3. Frontend stores in localStorage
  4. Token sent in Authorization header for subsequent requests
  5. Backend validates via auth middleware (partially implemented)

**RBAC (Role-Based Access Control):**
- Core Entities:
  - **SysUser**: Users with roles assigned
  - **SysRole**: Roles with permissions and data scopes
  - **SysMenu**: Menus with route mappings
  - **SysDepartment**: Department hierarchy
- Relationships:
  - User ↔ Role (many-to-many via sys_user_roles)
  - Role ↔ Menu (many-to-many via sys_role_menus)
  - Role ↔ Department (many-to-many via sys_role_departments)
  - User ↔ Department (many-to-one)

## Entry Points

**Backend Entry Point:**
- Location: `backend/src/main.rs`
- Triggers: `cargo run` or Docker container startup
- Responsibilities:
  - Load environment config from `.env` files
  - Initialize logging via tracing
  - Create database connection pool
  - Create Axum router with routes
  - Start HTTP server on port 8080

**Frontend Entry Point:**
- Location: `frontend/src/main.tsx`
- Triggers: `npm run dev` or `npm run build`
- Responsibilities:
  - Create React root
  - Mount App component to DOM
  - Initialize QueryClient for data fetching

**Route Entry Point:**
- Location: `frontend/src/routes/index.tsx`
- Triggers: URL navigation
- Responsibilities:
  - Define all application routes
  - Handle authentication redirects
  - Map paths to components

## Error Handling

**Strategy:** Centralized error format with status codes

**Backend Patterns:**
- Result<T, AppError> return type
- Custom error enum with variants
- IntoResponse implementation for automatic HTTP conversion
- Structured logging with tracing

**Frontend Patterns:**
- Axios response interceptors
- Global error messages via ElMessage
- Token expiration handling → redirect to login
- Error boundary components (not yet implemented)

## Cross-Cutting Concerns

**Logging:**
- Backend: tracing crate with env-filter
- Frontend: console.log (minimal), Sentry (not integrated)

**Validation:**
- Backend: Serde deserialization, custom validation in service layer
- Frontend: Zod schemas,react-hook-form

**Authentication:**
- Backend: JWT with jsonwebtoken crate
- Frontend: localStorage token storage
- Token expires in 1 hour by default

---

*Architecture analysis: 2026-03-26*