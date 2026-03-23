# RuoYi Admin Framework Implementation Plan

> **For Claude:** REQUIRED SUB-SKILL: Use superpowers:executing-plans to implement this plan task-by-task.

**Goal:** Rebuild this template into a RuoYi-style admin framework with a Vite React frontend and a modular Rust SQLx backend.

**Architecture:** Replace the current Next.js scaffold with a Vite SPA, reorganize the backend into `app/common/features`, introduce SQL migrations for RBAC entities, then implement auth, permissions, and the first admin modules in dependency-aware batches.

**Tech Stack:** Vite, React 19, TypeScript, TanStack Router, TanStack Query, React Hook Form, Zod, Tailwind CSS, shadcn/ui, Rust, Axum, SQLx, PostgreSQL, JWT, bcrypt, Vitest

---

### Task 1: Rebuild Frontend Foundation

**Files:**
- Create: `frontend/index.html`
- Create: `frontend/vite.config.ts`
- Create: `frontend/src/app/main.tsx`
- Create: `frontend/src/app/providers.tsx`
- Create: `frontend/src/app/router.tsx`
- Create: `frontend/src/routes/__root.tsx`
- Create: `frontend/src/routes/login.tsx`
- Create: `frontend/src/routes/index.tsx`
- Create: `frontend/src/components/layout/app-shell.tsx`
- Create: `frontend/src/components/guards/auth-guard.tsx`
- Create: `frontend/src/lib/http.ts`
- Create: `frontend/src/lib/auth.ts`
- Create: `frontend/src/lib/env.ts`
- Modify: `frontend/package.json`
- Modify: `frontend/tsconfig.json`
- Delete or replace references to: `frontend/next.config.ts`

**Step 1: Write the failing frontend smoke test**

```tsx
import { describe, expect, it } from "vitest";
import { render, screen } from "@testing-library/react";
import { AppRouterProvider } from "../src/app/providers";

describe("app bootstrap", () => {
  it("renders login route", () => {
    render(<AppRouterProvider initialPath="/login" />);
    expect(screen.getByText("登录")).toBeInTheDocument();
  });
});
```

**Step 2: Run test to verify it fails**

Run: `npm test -- --run`
Expected: FAIL because the Vite router bootstrap and test utilities do not exist yet.

**Step 3: Write minimal implementation**

```tsx
export function AppRouterProvider() {
  return <div>登录</div>;
}
```

Then replace it with the real provider, Vite config, router, login route, and guarded app shell.

**Step 4: Run test to verify it passes**

Run: `npm test -- --run`
Expected: PASS for the login bootstrap test.

**Step 5: Verify the app builds**

Run: `npm run build`
Expected: PASS and output for a Vite production build.

### Task 2: Create Database Schema and Migrations for RBAC

**Files:**
- Create: `backend/migrations/0001_init_rbac.sql`
- Create: `backend/migrations/0002_seed_admin.sql`
- Modify: `database/schema.prisma`
- Create: `database/README.md`

**Step 1: Write the failing schema verification check**

```sql
select table_name
from information_schema.tables
where table_name in (
  'sys_users',
  'sys_departments',
  'sys_roles',
  'sys_user_roles',
  'sys_menus',
  'sys_role_menus',
  'sys_role_departments'
);
```

**Step 2: Run migration status check to verify it fails**

Run: `cargo test schema_tables_exist -- --nocapture`
Expected: FAIL because migrations and test fixtures are not present yet.

**Step 3: Write minimal implementation**

```sql
create table sys_departments (...);
create table sys_roles (...);
create table sys_users (...);
```

Then complete all phase-one tables, indexes, foreign keys, and admin seed data.

**Step 4: Run schema verification again**

Run: `cargo test schema_tables_exist -- --nocapture`
Expected: PASS once the test database applies the migrations.

### Task 3: Build Backend Common Infrastructure

**Files:**
- Create: `backend/src/app/mod.rs`
- Create: `backend/src/app/router.rs`
- Create: `backend/src/app/state.rs`
- Create: `backend/src/common/mod.rs`
- Create: `backend/src/common/config.rs`
- Create: `backend/src/common/db.rs`
- Create: `backend/src/common/error.rs`
- Create: `backend/src/common/response.rs`
- Create: `backend/src/common/auth/jwt.rs`
- Create: `backend/src/common/auth/claims.rs`
- Create: `backend/src/common/auth/extractor.rs`
- Create: `backend/src/common/auth/middleware.rs`
- Create: `backend/src/common/permissions/data_scope.rs`
- Modify: `backend/src/lib.rs`
- Modify: `backend/src/main.rs`

**Step 1: Write the failing backend bootstrap test**

```rust
#[tokio::test]
async fn health_route_returns_ok() {
    let app = test_app().await;
    let response = app.get("/api/v1/health").await;
    assert_eq!(response.status(), 200);
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test health_route_returns_ok`
Expected: FAIL because the new app assembly and route wiring do not exist.

**Step 3: Write minimal implementation**

```rust
Router::new().route("/api/v1/health", get(health_handler))
```

Then expand it into config loading, DB pool wiring, unified responses, auth helpers, and protected router composition.

**Step 4: Run backend bootstrap tests**

Run: `cargo test health_route_returns_ok`
Expected: PASS.

### Task 4: Implement Auth Feature and Session Bootstrap

**Files:**
- Create: `backend/src/features/auth/model.rs`
- Create: `backend/src/features/auth/repo.rs`
- Create: `backend/src/features/auth/service.rs`
- Create: `backend/src/features/auth/routes.rs`
- Create: `backend/src/features/auth/mod.rs`
- Create: `backend/tests/auth_login.rs`

**Step 1: Write the failing auth test**

```rust
#[tokio::test]
async fn login_returns_access_token_and_profile() {
    let app = seeded_app().await;
    let response = login(&app, "admin", "Admin123!").await;
    assert_eq!(response.status(), 200);
    assert!(response.body().contains("access_token"));
}
```

**Step 2: Run auth test to verify it fails**

Run: `cargo test login_returns_access_token_and_profile`
Expected: FAIL because login flow and seeded admin auth are not implemented yet.

**Step 3: Write minimal implementation**

```rust
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}
```

Then add validation, bcrypt verification, JWT issue, current-user profile, current permissions, and current menu-tree endpoints.

**Step 4: Run auth tests**

Run: `cargo test auth_login -- --nocapture`
Expected: PASS.

### Task 5: Implement Permissions, Menus, and Route Guards

**Files:**
- Create: `backend/src/features/menus/model.rs`
- Create: `backend/src/features/menus/repo.rs`
- Create: `backend/src/features/menus/service.rs`
- Create: `backend/src/features/menus/routes.rs`
- Create: `frontend/src/features/auth/use-current-session.ts`
- Create: `frontend/src/features/auth/use-permissions.ts`
- Create: `frontend/src/components/guards/permission-guard.tsx`
- Create: `frontend/src/routes/system/menus.tsx`

**Step 1: Write the failing permission test**

```tsx
it("hides protected action without permission", () => {
  render(<PermissionGuard required="system:user:add">Add</PermissionGuard>);
  expect(screen.queryByText("Add")).toBeNull();
});
```

**Step 2: Run test to verify it fails**

Run: `npm test -- --run`
Expected: FAIL because permission context and guard components do not exist.

**Step 3: Write minimal implementation**

```tsx
export function PermissionGuard(props: PropsWithChildren<{ required: string }>) {
  return null;
}
```

Then connect it to current-session permission data and menu-driven navigation.

**Step 4: Run permission tests**

Run: `npm test -- --run`
Expected: PASS.

### Task 6: Implement User, Role, and Department Features

**Files:**
- Create: `backend/src/features/users/model.rs`
- Create: `backend/src/features/users/repo.rs`
- Create: `backend/src/features/users/service.rs`
- Create: `backend/src/features/users/routes.rs`
- Create: `backend/src/features/roles/model.rs`
- Create: `backend/src/features/roles/repo.rs`
- Create: `backend/src/features/roles/service.rs`
- Create: `backend/src/features/roles/routes.rs`
- Create: `backend/src/features/departments/model.rs`
- Create: `backend/src/features/departments/repo.rs`
- Create: `backend/src/features/departments/service.rs`
- Create: `backend/src/features/departments/routes.rs`
- Create: `frontend/src/routes/system/users.tsx`
- Create: `frontend/src/routes/system/roles.tsx`
- Create: `frontend/src/routes/system/departments.tsx`

**Step 1: Write the failing domain tests**

```rust
#[tokio::test]
async fn list_users_requires_permission() {
    let app = seeded_app().await;
    let response = get_users_without_token(&app).await;
    assert_eq!(response.status(), 401);
}
```

**Step 2: Run domain tests to verify they fail**

Run: `cargo test list_users_requires_permission`
Expected: FAIL because the feature routes and permission checks are not in place yet.

**Step 3: Write minimal implementation**

```rust
Router::new().route("/users", get(list_users))
```

Then complete the user list, role list, department tree, create and update endpoints, along with the matching frontend pages and query hooks.

**Step 4: Run domain tests**

Run: `cargo test --tests`
Expected: PASS for the new feature integration tests.

### Task 7: Add Data Scope Enforcement

**Files:**
- Modify: `backend/src/common/permissions/data_scope.rs`
- Modify: `backend/src/features/users/repo.rs`
- Modify: `backend/src/features/departments/repo.rs`
- Create: `backend/tests/data_scope.rs`

**Step 1: Write the failing data-scope test**

```rust
#[tokio::test]
async fn dept_scope_only_returns_same_department_records() {
    let app = seeded_app().await;
    let users = list_users_as_dept_admin(&app).await;
    assert!(users.iter().all(|u| u.dept_id == "dept-a"));
}
```

**Step 2: Run test to verify it fails**

Run: `cargo test dept_scope_only_returns_same_department_records`
Expected: FAIL because repo filtering does not yet apply department scope.

**Step 3: Write minimal implementation**

```rust
where_clauses.push("u.dept_id = $CURRENT_DEPT");
```

Then replace the placeholder logic with parameterized SQLx query construction for `ALL`, `CUSTOM`, `DEPT`, `DEPT_AND_CHILD`, and `SELF`.

**Step 4: Run data-scope tests**

Run: `cargo test data_scope -- --nocapture`
Expected: PASS.

### Task 8: Add Tooling, Validation, and Docs

**Files:**
- Create: `.gitignore`
- Create: `.env.example`
- Create: `frontend/eslint.config.js`
- Create: `frontend/prettier.config.js`
- Create: `frontend/vitest.config.ts`
- Create: `frontend/src/test/setup.ts`
- Modify: `README.md`
- Modify: `docker-compose.yml`
- Modify: `scripts/init-harness.sh`

**Step 1: Write the failing tooling check**

```bash
npm run lint
cargo test
```

**Step 2: Run checks to verify they fail**

Run: `cd frontend && npm run lint`
Expected: FAIL because the Vite frontend lint setup is missing.

Run: `cd backend && cargo test`
Expected: FAIL or be incomplete because the new test harness and environment files are not aligned yet.

**Step 3: Write minimal implementation**

```env
VITE_API_BASE_URL=http://localhost:3001
PORT=3001
DATABASE_URL=postgresql://postgres:postgres@localhost:5432/ruoyi_admin
JWT_SECRET=change-me
```

Then complete lint config, Vitest setup, README rewrite, Docker compose alignment, and harness script fixes.

**Step 4: Run full verification**

Run: `cd frontend && npm run build && npm test -- --run && npm run lint`
Expected: PASS.

Run: `cd backend && cargo test`
Expected: PASS.

### Task 9: Execute With Subagents in Batches

**Files:**
- Use and coordinate the files above

**Step 1: Batch the work**

Batch A:
- Task 1
- Task 2
- Task 3

Batch B:
- Task 4
- Task 5

Batch C:
- Task 6
- Task 7
- Task 8

**Step 2: Dispatch implementation subagents with non-overlapping ownership**

Run one subagent per ownership boundary:

- frontend foundation
- database and migrations
- backend common
- auth and permissions
- admin features
- tooling and tests

**Step 3: Review each batch before merging**

Expected:
- no overlapping file ownership within the same batch
- tests pass for touched areas
- user-facing behavior matches the approved design

**Step 4: Run final verification**

Run:
- `cd frontend && npm run build && npm test -- --run && npm run lint`
- `cd backend && cargo test`

Expected: PASS across both applications.
