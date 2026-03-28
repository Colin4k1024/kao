# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

React + Rust enterprise admin management system (类若依/RuoYi). A full-stack project with React 19 + Vite frontend and Rust + Axum backend, using PostgreSQL database with RBAC permission model.

## Environment Requirements

- Rust 1.70+
- Node.js 18+
- PostgreSQL 14+
- Docker (optional, for database)

## Common Commands

### Backend (Rust)
```bash
cd backend

# Run database migrations (requires PostgreSQL running)
psql -U postgres -d kao_db -f migrations/0001_create_sys_department.sql
# ... run other migrations in order ...

# Development
cargo run                    # Start backend server (port 8080)
cargo build                  # Build
cargo test                   # Run tests
cargo clippy --all-targets --all-features -- -D warnings  # Lint
cargo fmt                    # Format code

# Single test
cargo test test_name -- --nocapture
```

### Frontend (React)
```bash
cd frontend

npm install                  # Install dependencies
npm run dev                  # Start dev server (port 3000)
npm run build                # Production build
npm run lint                 # ESLint check
npm run format               # Prettier format
npm run type-check          # TypeScript check
```

### Root Level
```bash
make help                    # Show all available commands
make dev                     # Run both backend and frontend
make build                   # Build both
make test                    # Test both
make lint                    # Lint both
make docker-up              # Start Docker containers
```

### E2E Tests
```bash
npm test                     # Run Playwright tests
npm run test:headed          # Run with browser visible
npm run test:ui             # Run with Playwright UI
```

## Architecture

### Backend (Rust + Axum)

```
backend/src/
├── main.rs                 # Entry point - creates app via lib.rs
├── lib.rs                  # Module exports
├── app.rs                  # Router assembly and AppState
├── db.rs                   # PostgreSQL connection pool (SQLx)
├── config/                 # Settings from environment variables
├── common/                 # Shared: auth, config, permissions, cache, logging
├── features/               # Business modules (each has model/repo/service/routes)
│   ├── auth/              # Authentication (login/logout/refresh/session)
│   ├── users/             # User CRUD
│   ├── departments/       # Department tree CRUD
│   ├── roles/             # Role CRUD + menu assignment
│   ├── menus/             # Menu tree CRUD
│   ├── posts/             # Post/position CRUD
│   ├── dictionary/         # Dictionary type + data
│   ├── config/            # System parameters
│   ├── notice/            # Announcements
│   ├── job/               # Scheduled tasks
│   └── monitoring/        # System monitoring
├── middleware/            # Tower HTTP middleware (CORS, logging, auth)
├── models/                # Shared domain models
├── repositories/          # Data access layer
├── services/              # Business logic
└── utils/                # Helpers (JWT, response formatting)
```

**Key Backend Patterns:**
- Each feature module follows: `model.rs` → `repo.rs` → `service.rs` → `routes.rs`
- Database migrations in `backend/migrations/` (SQL files, numbered prefix)
- Auth uses JWT Bearer tokens in Authorization header
- API responses: `{ code: number, message: string, data: T }`

### Frontend (React + Vite)

```
frontend/src/
├── app.tsx                 # Root component with QueryClientProvider
├── routes/                 # Route configuration
├── pages/                  # Page components (system management)
├── features/              # Feature-based components
│   ├── auth/             # Login form
│   └── system/           # User/dept/role/menu/post management
├── services/              # API service layer
│   └── api/             # API client (axios instance)
├── components/            # Shared UI components
├── hooks/                # Custom React hooks
├── lib/                  # Utilities (API client, axios interceptors)
└── locales/              # i18n translations
```

**Key Frontend Patterns:**
- TanStack Query (React Query) for server state management
- Axios with interceptors for auth token injection
- Element Plus UI components
- Ant Design icons
- React Router for routing

## Database

- **ORM**: SQLx (not Prisma) - raw SQL with compile-time checked queries
- **Migrations**: SQL files in `backend/migrations/`
- **Schema reference**: `database/schema.prisma` (documentation only, not used at runtime)
- Tables use `sys_*` prefix with snake_case naming
- All tables have `created_at` and `updated_at` columns

## Security Requirements

- All APIs must require authentication (except `/api/auth/login`)
- Never concatenate SQL directly - use SQLx parameterized queries
- Store sensitive values in environment variables
- Validate all external input

## API Conventions

- RESTful design: `/api/system/users`, `/api/system/departments`, etc.
- Response format: `{ code: 200, message: "success", data: T }`
- Error codes: 200=success, 400=bad request, 401=unauthorized, 403=forbidden, 500=server error
- Backend runs on port 8080, frontend on port 3000

## Testing

- Backend: `cargo test` in backend directory
- Frontend: `npm test` (Vitest)
- E2E: Playwright at root level
