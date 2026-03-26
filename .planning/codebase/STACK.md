# Technology Stack

**Analysis Date:** 2026-03-26

## Languages

**Primary:**
- Rust 1.70+ - Backend server
- TypeScript 5.6+ - Frontend application

**Secondary:**
- SQL (PostgreSQL-specific) - Database queries and migrations

## Runtime

**Environment:**
- Rust (backend)
- Node.js (frontend with Vite)

**Package Manager:**
- Cargo (Rust)
- npm (frontend)

## Frameworks

**Core:**
- Axum 0.7 - Rust web framework with macros support
- React 18.2 - UI library
- Next.js - Not used; Vite is used instead

**Testing:**
- Playwright 1.58 - E2E testing (configured in root package.json)
- Rust `tokio-test` - Unit testing utilities

**Build/Dev:**
- Vite 5.4 - Frontend build tool
- PostgreSQL 15 Alpine - Database container
- Docker Compose - Container orchestration

## Key Dependencies

**Backend Critical:**
- `axum` 0.7 - Web framework
- `tokio` 1.0 - Async runtime
- `sqlx` 0.8 - Async PostgreSQL ORM with compile-time SQL checking
- `jsonwebtoken` 9 - JWT authentication

**Backend Infrastructure:**
- `bcrypt` 0.15 - Password hashing
- `serde`/`serde_json` 1.0 - Serialization
- `tracing`/`tracing-subscriber` - Logging
- `uuid` 1.0 - UUID generation
- `chrono` 0.4 - Date/time handling

**Frontend:**
- `react`/`react-dom` 18.2 - UI
- `axios` 1.7 - HTTP client
- `@tanstack/react-query` 5.60 - Data fetching
- `react-router-dom` 6.20 - Routing
- `antd` 5.21 - UI component library
- `element-plus` 2.13 - Alternative UI components
- `zod` 3.23 - Schema validation
- `react-hook-form` 7.53 - Form handling
- `@hookform/resolvers` 3.9 - Zod resolver for forms
- `lucide-react` 1.6 - Icon library
- `@ant-design/icons` 5.5 - Ant Design icons

## Configuration

**Environment:**
- Environment variables configured via `.env` files
- Frontend: `VITE_API_BASE_URL`
- Backend: Database config, JWT secrets, CORS settings

**Build:**
- Frontend: `vite.config.ts` with path aliases and proxy
- Backend: Cargo.toml with release profile optimization (LTO, opt-level 3)

## Platform Requirements

**Development:**
- Rust 1.70+ toolchain
- Node.js 18+
- PostgreSQL 15 (running locally or in Docker)

**Production:**
- Docker containers
- PostgreSQL database
- Configured environment variables

---

*Stack analysis: 2026-03-26*