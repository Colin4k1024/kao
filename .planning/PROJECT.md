# Project Context

**Name:** Kao - Enterprise RuoYi-style Admin Management System

**Date:** 2026-03-26

**Status:** Project Initialization - Brownfield Migration

---

## Executive Summary

Kao is an enterprise-grade admin management system built with React frontend and Rust backend, inspired by the RuoYi system. The codebase contains existing implementations of core RBAC functionality (users, departments, roles, menus) but requires architectural consolidation and security hardening before it can be considered production-ready.

---

## Project Vision

Create a highly scalable, security-first, and long-term maintainable admin management system.

---

## Domain Model

### Core Entities

1. User (sys_user) - Primary anchor entity
2. Department (sys_department) - Hierarchical tree structure
3. Post (sys_post) - Job positions
4. Role (sys_role) - Permission aggregation hub
5. Menu (sys_menu) - Frontend/Backend mapping

---

## Current Architecture

### Technology Stack

Backend: Rust 1.70+, Axum, Tokio, SQLx, JWT
Frontend: React 18.2, TypeScript, Vite, Ant Design, React Query
Infrastructure: PostgreSQL 15, Docker

---

## Project Goals

### Phase 1: Stabilization & Security (Current)
- Consolidate authentication implementations
- Fix security vulnerabilities (hardcoded credentials, JWT secrets)
- Implement proper database migrations
- Add comprehensive tests

### Phase 2: Feature Completeness
- Complete dynamic configuration module
- Implement scheduled job management
- Add system monitoring endpoints
- Complete logging infrastructure

### Phase 3: Production Readiness
- Performance optimization
- Horizontal scaling support
- Health check and metrics endpoints
- Comprehensive documentation

---

## Scope Constraints

**In Scope:** RBAC permission system, Dynamic configuration management, Scheduled task system, System monitoring and logging, Docker deployment

**Out of Scope (Phase 1):** OAuth2/SAML external authentication, Multi-tenant architecture, Advanced caching with Redis

---

## Success Criteria

1. All authentication flows use consistent bcrypt + JWT
2. Zero hardcoded credentials or secrets
3. Code coverage > 70% for critical paths
4. Health check endpoint returns actual dependency status
5. Database migrations run automatically on startup

---

## Known Risks

| Risk | Impact | Mitigation |
|------|--------|------------|
| Multiple auth implementations causing confusion | High | Consolidate to single flow in Phase 1 |
| Hardcoded secrets in code | Critical | Replace with environment variables |
| Database table name mismatches | High | Audit all queries against schema |

---

## References

- Requirements: require.md
- Quick Start: QUICKSTART.md
- Deployment: DEPLOYMENT.md
