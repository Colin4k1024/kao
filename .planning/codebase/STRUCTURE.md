# Codebase Structure

**Analysis Date:** 2026-03-26

## Directory Layout

```
kao/
├── frontend/              # React + Vite frontend application
│   ├── src/
│   │   ├── pages/         # Page components and layouts
│   │   │   ├── layout/    # Layout components (MainLayout, etc)
│   │   │   ├── system/    # System management pages
│   │   │   ├── user/      # User-related pages
│   │   │   └── *.tsx      # Page components
│   │   ├── services/      # API service layer
│   │   │   └── api/       # API endpoint wrappers
│   │   ├── components/    # Reusable UI components
│   │   │   ├── ui/        # shadcn/ui components
│   │   │   ├── HeaderDropdown/
│   │   │   ├── RightContent/
│   │   │   └── Footer/
│   │   ├── hooks/         # Custom React hooks
│   │   ├── lib/           # Utility functions
│   │   ├── types/         # TypeScript type definitions
│   │   ├── routes/        # Route configuration
│   │   ├── locales/       # Internationalization
│   │   ├── config/        # Frontend configuration
│   │   ├── main.tsx       # Entry point
│   │   ├── app.tsx        # App component
│   │   └── ...
│   ├── public/            # Static assets
│   ├── package.json       # Frontend dependencies
│   ├── vite.config.ts     # Vite build config
│   ├── tsconfig.json      # TypeScript config
│   └── .env*
│
├── backend/               # Rust Axum backend API
│   ├── src/
│   │   ├── main.rs        # Entry point
│   │   ├── app.rs         # App state and router setup
│   │   ├── lib.rs         # Module declarations
│   │   ├── api/           # API layer (partially used)
│   │   │   └── system/    # System API routes
│   │   │       └── user/  # User API implementation
│   │   ├── common/        # Cross-cutting concerns
│   │   │   ├── auth/      # Authentication utilities
│   │   │   ├── config/    # Configuration
│   │   │   ├── db.rs      # Database pool
│   │   │   ├── error.rs   # Error handling
│   │   │   ├── response.rs # Response formatting
│   │   │   └── permissions/ # RBAC data scope
│   │   ├── config/        # Settings and env config
│   │   ├── middleware/    # Axum middleware
│   │   │   ├── auth.rs    # Auth middleware (disabled)
│   │   │   ├── cors.rs    # CORS configuration
│   │   │   └── logger.rs  # Request logging
│   │   ├── features/      # Domain features
│   │   │   ├── users/     # User feature (full CRUD)
│   │   │   ├── roles/     # Role feature
│   │   │   ├── departments/ # Department feature
│   │   │   ├── menus/     # Menu feature
│   │   │   └── auth/      # Auth feature
│   │   ├── models/        # Domain models
│   │   ├── repositories/  # Data access layer
│   │   ├── services/      # Business logic layer
│   │   └── utils/         # Utility functions
│   ├── migrations/        # SQL migrations (truth source)
│   │   ├── 0001_init_rbac.sql
│   │   ├── 0002*.sql      # Seed and data migrations
│   │   └── ...
│   ├── Cargo.toml         # Rust dependencies
│   ├── .env*
│   └── Dockerfile
│
├── database/              # Prisma schema (reference only)
│   ├── schema.prisma      # Domain model definitions
│   └── README.md
│
├── scripts/               # Deployment scripts
├── tests/                 # Test files
├── .agent/                # AI agent configuration
├── .planning/             # Planning documents
├── docs/                  # Documentation
├── package.json           # Root package config
├── docker-compose.yml     # Docker composition
└── README.md
```

## Directory Purposes

**Frontend/src/pages:**
- Purpose: Contains page components for each route
- Contains:
  - Layout components (MainLayout)
  - System management pages (users, roles, departments, menus)
  - System pages (Dashboard, 404, Admin)
- Key files:
  - `frontend/src/pages/layout/MainLayout.tsx`
  - `frontend/src/pages/system/users/UserList.tsx`
  - `frontend/src/pages/Dashboard.tsx`

**Frontend/src/services/api:**
- Purpose: API endpoint implementations
- Contains:
  - HTTP client configuration (axios/fetch)
  - Endpoints for all backend APIs
  - Request/response interceptors
- Key files:
  - `frontend/src/services/api/index.ts`
  - `frontend/src/services/api/authService.ts`
  - `frontend/src/services/api/systemService.ts`

**Frontend/src/components/ui:**
- Purpose: Reusable UI components
- Contains: shadcn/ui component implementations
- Key files:
  - `frontend/src/components/ui/button.tsx`
  - `frontend/src/components/ui/input.tsx`
  - `frontend/src/components/ui/card.tsx`

**Backend/src/features:**
- Purpose: Domain-driven feature modules
- Contains:
  - model.rs: Request/Response types
  - repo.rs: Database operations
  - service.rs: Business logic
  - routes.rs: API route handlers
- Key features:
  - Users (full CRUD with pagination)
  - Roles (RBAC)
  - Departments (hierarchical)
  - Menus (routing and permissions)
  - Auth (authentication)

**Backend/src/common:**
- Purpose: Shared utilities and cross-cutting concerns
- Contains:
  - auth: JWT utilities, claims, extractor
  - db: Database connection pool
  - error: Error types and handling
  - response: Standardized response format
  - permissions: Data scope control
  - config: Application configuration
- Key files:
  - `backend/src/common/error.rs`
  - `backend/src/common/response.rs`
  - `backend/src/common/db.rs`

**Database/migrations:**
- Purpose: SQL schema migrations (truth source)
- Contains: Incremental SQL migration files
- Naming: `NNNN_description.sql` (4-digit sequence)
- Key migrations:
  - `0001_init_rbac.sql`: Core RBAC tables
  - `0003_create_sys_user.sql`: User table
  - `0004_create_sys_role.sql`: Role table
  - `0005_create_sys_menu.sql`: Menu table
  - `0006-0008`: Junction tables for relationships
  - `0002_seed_admin.sql`: Initial admin user

## Key File Locations

**Entry Points:**
- `backend/src/main.rs`: Rust backend entry point
- `frontend/src/main.tsx`: React frontend entry point
- `frontend/src/app.tsx`: App component with routing
- `frontend/src/routes/index.tsx`: Route definitions

**Configuration:**
- `backend/Cargo.toml`: Rust dependencies and profile
- `backend/src/config/settings.rs`: Backend settings
- `backend/.env.example`: Backend environment template
- `frontend/package.json`: Frontend dependencies and scripts
- `frontend/tsconfig.json`: TypeScript configuration
- `docker-compose.yml`: Docker services composition

**Core Logic:**
- `backend/src/app.rs`: Axum router configuration
- `backend/src/lib.rs`: Module module declarations
- `backend/src/common/auth/jwt.rs`: JWT utilities
- `backend/src/features/users/service.rs`: User business logic
- `backend/src/features/users/repo.rs`: User data access

**Testing:**
- `frontend/jest.config.ts`: Jest configuration
- `playwright.config.ts`: Playwright configuration
- `tests/`: Test files directory
- `test-results/`: Test output location

## Naming Conventions

**Files:**
- Frontend components: PascalCase.tsx (e.g., UserList.tsx)
- Backend modules: snake_case.rs (e.g., user_repo.rs)
- Backend models: snake_case.rs (e.g., user.rs)
- Migration files: NNNN_description.sql

**Functions:**
- Frontend: camelCase (e.g., listUsers, updateUser)
- Backend: snake_case (e.g., list_users, update_user)
- Constants: SCREAMING_SNAKE_CASE

**Variables:**
- Frontend: camelCase (e.g., userId, userName)
- Backend: snake_case (e.g., user_id, username)

**API Endpoints:**
- RESTful: `/api/v1/{resource}` (plural, lowercase)
- Examples:
  - `GET /api/v1/users`
  - `POST /api/v1/users`
  - `PUT /api/v1/users/{id}`
  - `DELETE /api/v1/users/{id}`

## Where to Add New Code

**New Feature (e.g., "permissions"):**

1. Backend:
   - Create `backend/src/features/permissions/` directory
   - Add `model.rs` (request/response types)
   - Add `repo.rs` (SQL queries with SQLx)
   - Add `service.rs` (business logic)
   - Add `routes.rs` (API endpoints)
   - Register in `backend/src/features/mod.rs`
   - Add migration in `backend/migrations/`

2. Frontend:
   - Add page component in `frontend/src/pages/system/permissions/`
   - Add API service in `frontend/src/services/api/`
   - Add type definitions in `frontend/src/types/`
   - Register route in `frontend/src/routes/index.tsx`

**New UI Component:**
- Ant Design component: `frontend/src/components/ui/` (e.g., button.tsx)
- Feature-specific component: `frontend/src/pages/system/users/components/`

**Database Change:**
1. Update `database/schema.prisma` (reference model)
2. Create migration in `backend/migrations/NNNN_description.sql`
3. Run migration: `sqlx migrate run`
4. Update Rust models in `backend/src/models/`

**New API Endpoint:**
1. Backend: Add route in relevant `routes.rs`
2. Create handler function in same file
3. Add service method if business logic needed
4. Add repository method if database access needed
5. Frontend: Add API call in `services/api/`

## Special Directories

**backend/.sqlx:**
- Purpose: SQLx query cache and metadata
- Generated: Yes (by SQLx)
- Committed: No (gitignore)

**frontend/.umi:**
- Purpose: Umi framework generated files (from ant-design-pro)
- Generated: Yes
- Committed: No (gitignore)

**frontend/node_modules:**
- Purpose: npm dependencies
- Generated: Yes
- Committed: No (gitignore)

**backend/target:**
- Purpose: Rust build artifacts
- Generated: Yes
- Committed: No (gitignore)

**database/schema.prisma:**
- Purpose: Prisma schema (reference model)
- Note: Not used at runtime; migrations are truth source
- Generated: No (manual)
- Committed: Yes

---

*Structure analysis: 2026-03-26*