# Phase 02: Feature Completeness - Research

**Date:** 2026-03-26
**Phase:** Feature Completeness
**Purpose:** What do I need to know to PLAN this phase well?

---

## Domain Overview

Phase 2 delivers the core business features for the Kao admin management system. Unlike Phase 1 (security foundation), Phase 2 focuses on implementing the main business functionality: dynamic configuration, scheduled jobs, and system monitoring.

---

## Key Technical Decisions

### 1. Dynamic Configuration Module

**Requirements (FR6):**
- Dictionary type CRUD with caching
- Dictionary data CRUD linked to types
- Parameter configuration CRUD
- Notice/announcement CRUD
- Cache-friendly design

**Technical Approach:**
- Dictionary types and data follow the same pattern as roles/menus in Phase 1
- Use caching layer for frequent reads (dictionary types, parameters)
- Cache invalidation strategy needed on updates
- Consider Redis for caching or in-memory for Phase 2

**Reference Implementations:**
- Check existing `backend/src/features/dictionary/` if exists
- Check existing `backend/src/features/config/` if exists
- Follow RBAC patterns from Phase 1 for permissions

### 2. Scheduled Job Management

**Requirements (FR7):**
- Background job scheduler integration
- Job management API
- Job log tracking
- Cron expression validation
- Job status monitoring

**Technical Approach:**
- Evaluate available Rust job scheduling crates:
  - `tokio-cron-scheduler` - lightweight, scheduled tasks
  - `cron` - cron表达式 parsing
  - `kombucha` - job queue with retry
  - `async-cron` - cron-like scheduling
- Choose based on: cron support, persistence, retry logic
- Job logs need separate table from jobs

**Key Decisions:**
- Job scheduler must integrate with existing database
- Jobs stored in database for persistence
- Cron validation before job creation
- Job execution logs for debugging

### 3. System Monitoring

**Requirements (FR7, NFR3):**
- Metrics endpoint (/metrics)
- Health check with dependency status
- Operation logging
- Login logging
- Online user monitoring
- System resource monitoring

**Technical Approach:**
- Metrics endpoint follows Prometheus format
- Health check must check database connection
- Operation logs track all API actions
- Login logs track authentication events
- Online users from JWT token tracking
- System resources from system information library

**Reference Implementations:**
- Check Phase 1 health check implementation
- Reuse logging infrastructure from Phase 1
- Metrics format: Prometheus text format
- Health status: healthy/degraded/unhealthy

### 4. Frontend Enhancement

**Requirements (Phase 2 deliverable 4):**
- Complete dynamic configuration UI
- Completed scheduled job UI
- System monitoring dashboard
- Responsive design improvements
- Loading states and error boundaries

**Technical Approach:**
- Use existing React + Ant Design patterns
- Follow dynamic layout patterns from Phase 1
- Add loading states for all async operations
- Error boundaries for exception handling
- Responsive layout using Ant Design grid

### 5. Documentation

**Requirements (Phase 2 deliverable 5):**
- API documentation (OpenAPI/Swagger)
- Deployment guide
- Development guide
- Architecture documentation
- User manual

**Technical Approach:**
- Use `utoipa` crate for OpenAPI/Swagger in Rust backend
- Generate API docs from code annotations
- Deploy docs as static site or separate endpoint
- User manual as markdown documentation

---

## Codebase Patterns to Follow

### Backend Patterns
1. **Layered Architecture:**
   ```
   Controller (routes.rs) → Service (service.rs) → Repository (repo.rs) → Database
   ```

2. **Response Format:**
   ```rust
   {
     "code": 200,
     "message": "success",
     "data": {...}
   }
   ```

3. **Error Format:**
   ```rust
   {
     "code": 400,
     "message": "error description",
     "data": null
   }
   ```

4. **Authentication:**
   - JWT tokens in Authorization header
   - Bearer scheme
   - 1-hour expiration
   - Role-based access control

### Frontend Patterns
1. **Component Structure:**
   ```
   pages/ → [Feature]Page.tsx
   components/ → [Feature]Component.tsx
   services/api/ → [feature].ts
   ```

2. **API Integration:**
   - Axios for HTTP requests
   - React Query for data fetching
   - Zod for validation
   - React Hook Form for forms

3. **State Management:**
   - localStorage for auth tokens
   - React Context/Hooks for auth state
   - React Query for data state

---

## Database Schema Considerations

### New Tables for Phase 2
1. **sys_dictionary_type** - Dictionary type metadata
2. **sys_dictionary_data** - Dictionary data records
3. **sys_config** - Parameter configuration
4. **sys_notice** - Notice/announcement records
5. **sys_job** - Scheduled job definitions
6. **sys_job_log** - Job execution logs
7. **sys_oper_log** - Operation logs
8. **sys_login_log** - Login event logs
9. **sys_online_user** - Online user tracking

### Existing Patterns
- Follow same naming convention: `sys_*` prefix
- UUID primary keys
- Timestamp columns (created_at, updated_at)
- Status field for soft delete

---

## Security Considerations

1. **Input Validation:**
   - Validate all API inputs
   - Use Zod schemas for frontend
   - Use Serde validation for backend
   - Sanitize user inputs

2. **Authentication:**
   - All endpoints require authentication
   - Role-based authorization
   - Permission string validation

3. **Logging:**
   - Log sensitive events (login, permission changes)
   - Don't log passwords or secrets
   - Structured logging with tracing

4. **Error Handling:**
   - Don't expose internal errors to users
   - Log detailed errors server-side
   - Return consistent error format

---

## Testing Requirements

1. **Backend Unit Tests:**
   - Business logic in services
   - Repository queries
   - Validation logic

2. **Integration Tests:**
   - API endpoint behavior
   - Database operations
   - Authentication flow

3. **E2E Tests:**
   - Critical user flows
   - End-to-end workflows
   - Error scenarios

4. **Coverage Target:**
   - >50% for new code
   - Critical paths >70%

---

## Deployment Considerations

1. **Database Migrations:**
   - Apply on startup
   - Version controlled
   - Rollback capability

2. **Environment Variables:**
   - Database URL
   - Cache configuration
   - Job scheduler settings
   - Monitoring endpoints

3. **Health Checks:**
   - Database connection
   - Job scheduler status
   - Service dependencies

---

## Known Gaps from Phase 1

1. **No caching layer yet** - Plan for Redis in Phase 3
2. **Basic logging only** - Enhance for Phase 2
3. **No metrics endpoint** - Add Prometheus format
4. **Health check incomplete** - Add dependency checks

---

## Success Criteria

- All Phase 2 deliverables implemented
- API endpoints match requirements
- Frontend UI complete for all features
- Documentation available
- Test coverage >50%
- No hardcoded credentials
- Security vulnerabilities fixed in Phase 1 addressed

---

*Research completed: 2026-03-26*
