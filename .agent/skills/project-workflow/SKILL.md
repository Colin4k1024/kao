---
name: project-workflow
description: Use when working in this AI coding scaffold repository and you need to understand or execute the current frontend, backend, database, and plan-generation workflow.
---

# Project Workflow

## Overview

This skill is the repo-specific playbook for `/Users/jiafan/Desktop/poc/code-template`.

Use the current codebase as the source of truth, then use `README.md` as the intended workflow. This repository is a full-stack template with a Next.js frontend, a Rust + Axum backend, a Prisma schema, and lightweight JavaScript planning scripts.

## When to Use

- Starting work in this repository
- Turning a natural-language request into frontend/backend/database changes
- Figuring out which files and commands matter for a change
- Reconciling `README.md` with the current implementation

Do not use this as a generic Next.js, Rust, or Prisma skill. It is specific to this repository layout and its current gaps.

## Repo Facts

- `frontend/`: Next.js 16 App Router, React 19, TypeScript, Tailwind 4
- `backend/`: Rust 2021, Axum 0.8, SQLx, JWT auth, PostgreSQL
- `database/`: Prisma schema only; no Prisma Client integration in app code
- `scripts/`: simple plan generation and execution helpers
- `.agent/`: may not exist yet in a fresh checkout; some README references are aspirational

### Important mismatches to remember

- `README.md` describes a richer `.agent/` setup than the repository currently contains.
- `database/schema.prisma` is present, but the Rust backend talks to PostgreSQL with SQLx queries, not Prisma Client.
- `scripts/init-harness.sh` treats `backend/` like a Node project and should not be trusted as-is.
- `docker-compose.yml` references a `frontend/Dockerfile`, but that file is not currently in the repo.
- Backend auth middleware is applied at the router layer in `backend/src/app.rs`, so auth behavior should be verified carefully before assuming public routes work.
- `backend/src/services/auth_service.rs` currently uses a hardcoded JWT secret instead of loading from env.

## Working Map

### Read these first

1. `README.md`
2. `CLAUDE.md`
3. One or more scoped rules:
   - `frontend/CLAUDE.md`
   - `backend/CLAUDE.md`
   - `database/CLAUDE.md`

### Frontend map

- Entry: `frontend/src/app/page.tsx`
- Root layout: `frontend/src/app/layout.tsx`
- Global styles: `frontend/src/app/globals.css`
- Shared utils: `frontend/src/lib/utils.ts`
- Package commands: `frontend/package.json`

### Backend map

- Boot: `backend/src/main.rs`
- App assembly: `backend/src/app.rs`
- DB pool: `backend/src/db.rs`
- Routes and handlers:
  - `backend/src/handlers/auth_handler.rs`
  - `backend/src/handlers/user_handler.rs`
- Business logic:
  - `backend/src/services/auth_service.rs`
  - `backend/src/services/user_service.rs`
- Auth middleware: `backend/src/middleware/auth.rs`
- Models:
  - `backend/src/models/auth.rs`
  - `backend/src/models/user.rs`

### Database map

- Schema source: `database/schema.prisma`
- When schema changes, also inspect backend SQLx queries for table and column compatibility.

### Planning script map

- Create a JSON plan from a natural-language requirement:
  - `scripts/create-plan.js`
- Execute a saved JSON plan:
  - `scripts/implement-plan.js`
- Saved plans go under `.agent/plans/`

## Standard Workflow

### 1. Risk screen first

Before touching code, check the repository rules from `CLAUDE.md`:

- Every API should require authentication unless there is a deliberate public exception.
- Do not introduce direct SQL string concatenation.
- Keep sensitive values in environment variables.
- Validate all external input.
- Preserve TypeScript strict mode and naming conventions.

Also compare the request against the current implementation. This template already has security and workflow gaps, so do not assume the baseline is compliant.

### 2. Build context from code, not just docs

For every task:

1. Read `README.md` for intended architecture.
2. Read the relevant `CLAUDE.md` files for local constraints.
3. Open the real entry points for the layer you are changing.
4. Note any mismatch between docs and code before planning a fix.

If the request changes behavior, trace end-to-end:

- frontend form or page
- API contract
- handler
- service
- database schema or query shape

### 3. Classify the work

Sort the request into one or more buckets:

- Frontend-only: page, component, client validation, state, API integration
- Backend-only: route, handler, service, middleware, auth, DB access
- Database-involved: schema change, new fields, relation changes
- Workflow/tooling: scripts, docs, setup, docker, agent support files

This repo often needs cross-layer changes. If an API payload changes, expect to update both request/response models and the frontend consumer.

### 4. Implement by layer

#### Frontend

- Keep app-router structure under `frontend/src/app/`
- Follow `frontend/CLAUDE.md` naming rules
- Prefer adding validation at the edge of user input
- If calling backend APIs, ensure token handling is explicit

#### Backend

- Route wiring lives through handlers and app assembly
- Keep handler code thin and service code responsible for business logic
- Reuse `AppResult` and existing error flow
- Check whether auth middleware scope matches the intended route visibility

#### Database

- Update `database/schema.prisma` when the data model changes
- Verify SQLx queries in Rust still match the table names and columns
- Do not assume Prisma migration output alone updates backend behavior

#### Planning scripts

- `create-plan.js` is keyword-based and intentionally simple
- `implement-plan.js` simulates task execution and updates plan status
- Treat them as helpers, not a full agent runtime

### 5. Verify before claiming success

Use the lightest command set that matches the touched area.

#### Frontend checks

From `frontend/`:

```bash
npm install
npm run lint
npm test
npm run build
```

#### Backend checks

From `backend/`:

```bash
cargo build
cargo test
```

If the change affects runtime behavior, also run:

```bash
cargo run
```

#### Database checks

From `database/`:

```bash
npx prisma validate
```

If the schema changed, verify the backend SQL still matches the schema.

#### Environment/runtime checks

- Start PostgreSQL with `docker-compose up -d postgres`
- Backend is expected on `http://localhost:3001`
- Frontend is expected on `http://localhost:3000`
- Do not rely on `scripts/init-harness.sh` without reviewing or fixing it first

### 6. Close out clearly

When finishing work in this repo:

- State which layer(s) changed
- State what you verified and what you could not verify
- Call out README/code mismatches if they affected the result
- Suggest follow-up fixes when the repo baseline is misleading or broken

## Quick Start Commands

```bash
docker-compose up -d postgres
cd backend && cargo run
cd frontend && npm install && npm run dev
```

## Common Mistakes

- Trusting `README.md` over the current code
- Updating `schema.prisma` without updating Rust query assumptions
- Assuming auth routes are public without checking middleware scope
- Running `scripts/init-harness.sh` as if it were production-ready
- Treating `.agent/` references in the docs as fully implemented

## Decision Rule

When docs and code disagree:

1. Treat current code as the execution truth.
2. Treat `README.md` as the desired operating model.
3. Either align the code to the docs, or document the mismatch in the final output.
