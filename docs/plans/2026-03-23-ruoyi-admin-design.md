# RuoYi-Style Admin Framework Design

**Date:** 2026-03-23

**Goal:** Build a RuoYi-style enterprise admin framework with a modern frontend stack and a Rust backend, while preserving strong security defaults, modular boundaries, and long-term maintainability.

## Scope

The framework will be rebuilt around:

- Frontend: `Vite + React 19 + TypeScript + TanStack Router + TanStack Query + React Hook Form + Zod + Tailwind CSS + shadcn/ui`
- Backend: `Rust + Axum + Tokio + SQLx + PostgreSQL + JWT`
- Database: PostgreSQL with SQL migrations as the source of truth

The first delivery phase focuses on framework foundations plus the core administrative domains:

- Authentication
- User Management
- Role Management
- Department Management
- Menu and Permission Management

Later phases can add posts, dictionaries, parameters, notices, jobs, code generation, and observability.

## Architecture Decision Summary

### Frontend

The existing `Next.js` frontend will be replaced with a Vite-based SPA because the requirement document explicitly targets `Vite + React + TanStack Router`.

The frontend will use a feature-oriented structure:

```text
frontend/src/
├── app/
├── routes/
├── features/
├── components/
├── lib/
└── hooks/
```

Key responsibilities:

- `app/`: application bootstrap, providers, router registration
- `routes/`: route tree and page-level composition
- `features/`: domain-specific UI and hooks
- `components/`: shared UI primitives, layout, guards
- `lib/`: HTTP client, auth token helpers, env access, permission utilities
- `hooks/`: shared app hooks

The first frontend milestone includes:

- Login page
- Protected app shell
- Dynamic left navigation from backend menu data
- Permission-aware button and route guards
- Core CRUD pages for users, roles, departments, and menus

### Backend

The backend will remain a single Rust package for now, but its internals will be reorganized into stable domain boundaries:

```text
backend/src/
├── main.rs
├── app/
├── common/
└── features/
```

Each feature module will follow a consistent structure:

- `model.rs`: request and response DTOs, domain types, input validation
- `repo.rs`: SQLx access layer
- `service.rs`: business logic orchestration
- `routes.rs`: Axum routes and handlers

The `common/` area will contain:

- environment config
- database pool bootstrap
- error model
- unified JSON response shape
- auth claims, JWT helpers, extractors, middleware
- permission and data-scope utilities

This layout keeps the current codebase simple enough to evolve quickly while aligning with the requirement document's domain-driven intent.

## Security Model

The framework must satisfy the project rules in `AGENTS.md`:

- All API endpoints except explicit auth bootstrap routes require authentication
- All request inputs are validated
- No direct SQL string concatenation
- Secrets and environment-specific values are loaded from environment variables only

The backend will expose only two unauthenticated endpoints in the first phase:

- `POST /api/v1/auth/login`
- `GET /api/v1/health`

All remaining APIs will require a valid JWT.

Security decisions:

- Passwords are stored as `bcrypt` hashes only
- `JWT_SECRET`, `DATABASE_URL`, and `PORT` come from environment variables
- Permission checks happen at the route or service boundary
- Data scope checks happen in the repo layer through explicit context

## RBAC and Data Scope

The authorization system is modeled after RuoYi-style RBAC:

- users can belong to one department
- users can have multiple roles
- roles can have multiple menus and permissions
- roles define one data scope policy
- roles with custom scope can bind to multiple departments

Supported data scopes:

- `ALL`
- `CUSTOM`
- `DEPT`
- `DEPT_AND_CHILD`
- `SELF`

The system will not attempt hidden SQL rewriting magic in phase one. Instead, repository methods that need data isolation will accept a `DataScopeContext` and apply known SQL fragments in a controlled, testable way.

## Database Model

The current Prisma schema is a blog-style sample and does not match the product requirements. The new phase-one schema will include:

- `sys_users`
- `sys_departments`
- `sys_roles`
- `sys_user_roles`
- `sys_menus`
- `sys_role_menus`
- `sys_role_departments`

Future-ready tables:

- `sys_posts`
- `sys_user_posts`

Recommended structural choices:

- departments store `parent_id` and a normalized `ancestors` path for descendant filtering
- role scope is stored as a stable string enum
- menu entries store both route metadata and permission keys

SQL migrations will become the source of truth. The current Prisma file can remain temporarily for reference, but it will no longer represent the runtime contract.

## API Design

The backend will provide a unified response envelope for business APIs:

```json
{
  "code": 0,
  "message": "ok",
  "data": {}
}
```

Error responses will use the same outer shape with non-zero codes and a stable message.

Phase-one endpoint groups:

- `auth`: login, current profile, current permissions, current menu tree
- `users`: list, create, update, detail
- `roles`: list, create, update, detail
- `departments`: tree, create, update, detail
- `menus`: tree, create, update, detail

Pagination, filtering, and sorting will be added first where they are essential for management views, especially for user and role lists.

## Frontend Interaction Model

The frontend will consume three primary auth bootstrap payloads after login:

- current user profile
- current permission list
- current menu tree

The app shell will derive:

- route access
- navigation rendering
- button visibility

Permission control will be exposed via:

- `usePermissions()`
- `AuthGuard`

Forms will use `React Hook Form + Zod`, while data fetching and cache invalidation will use `TanStack Query`.

## Multi-Subagent Delivery Strategy

Implementation will be coordinated by a main controller agent and executed in dependency-aware batches with multiple subagents.

Planned workstreams:

- frontend foundation
- backend common infrastructure
- database schema and migrations
- auth and permission chain
- core admin features
- tests and engineering polish

These workstreams will not all start simultaneously. They will be grouped to avoid overlapping edits and to keep the critical path moving:

1. foundation batch
2. auth and RBAC batch
3. core feature batch
4. integration and verification batch

## Risks and Constraints

- The current repository is not a git repository, so commit-based checkpoints are unavailable unless the workspace is initialized separately.
- The current README and scripts do not reflect the desired architecture and should be treated as outdated scaffolding.
- The existing backend has auth and routing issues and should not be incrementally patched into the final architecture; it should be reorganized intentionally.
- The existing frontend should be treated as disposable scaffold because it is built on the wrong routing model for the approved requirement.

## Acceptance Criteria

The first framework milestone is successful when:

- the frontend runs on `Vite + React + TanStack Router`
- the backend is organized into `app`, `common`, and `features`
- all non-bootstrap APIs require JWT auth
- all request payloads are validated
- the framework supports users, roles, departments, menus, and menu permissions
- data scope primitives exist and are applied to the relevant queries
- environment variables hold all secrets and runtime configuration
- frontend and backend each have at least one meaningful automated test
- developer commands for lint, build, and test are documented and runnable

## Execution Note

The implementation should proceed using a subagent-driven approach in this session, with explicit ownership boundaries per workstream and verification after each batch.
