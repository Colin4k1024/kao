# Kao Architecture Documentation

Enterprise Admin Management System Architecture

## Table of Contents
- [Overview](#overview)
- [Architecture Principles](#architecture-principles)
- [Technology Stack](#technology-stack)
- [Key Decisions](#key-decisions)
- [References](#references)

---

## Overview

Kao is an enterprise-grade admin management system built with a layered architecture pattern. This document describes the architectural principles, technology choices, and design patterns used throughout the project.

### System Architecture

```
┌──────────────────────────────────────────────────────────────────┐
│                        Presentation Layer                        │
│                   React SPA + Ant Design                         │
│                    Frontend Components                           │
└──────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌──────────────────────────────────────────────────────────────────┐
│                         API Gateway                              │
│                   Axum HTTP Framework                            │
│                    RESTful Endpoints                             │
└──────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌──────────────────────────────────────────────────────────────────┐
│                    Application Layer                             │
│                   Service Layer + Business Logic                 │
│                     Request Validation                           │
└──────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌──────────────────────────────────────────────────────────────────┐
│                      Domain Layer                                │
│                    Domain Models + Entities                      │
│                     Domain Services                              │
└──────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌──────────────────────────────────────────────────────────────────┐
│                    Infrastructure Layer                          │
│               Database (PostgreSQL) + ORM (SQLx)                │
│                 Caching + External APIs                          │
└──────────────────────────────────────────────────────────────────┘
```

---

## Architecture Principles

### 1. Separation of Concerns

Each layer has a distinct responsibility:

| Layer | Responsibility |
|-------|----------------|
| Presentation | UI components, routing, state management |
| API | HTTP routing, request handling, validation |
| Application | Business logic, workflow orchestration |
| Domain | Core business entities, domain rules |
| Infrastructure | Database, caching, external services |

### 2. Layered Architecture

```
┌─────────────┐
│  Presentation  │  UI Components
├─────────────┤
│    API Layer   │  Routes, Controllers
├─────────────┤
│ Application Layer│  Services, Business Logic
├─────────────┤
│  Domain Layer  │  Entities, Domain Models
├─────────────┤
│Infrastructure  │  Database, Caching, APIs
└─────────────┘
```

### 3. Dependency Inversion

Higher layers depend on abstractions, not implementations:

```rust
// Service depends on trait, not concrete implementation
pub trait UserRepository {
    fn find_by_id(&self, id: &Uuid) -> Result<Option<User>>;
}

// Concrete implementation
pub struct PgUserRepository {
    pool: PgPool,
}

impl UserRepository for PgUserRepository {
    fn find_by_id(&self, id: &Uuid) -> Result<Option<User>> {
        // Implementation
    }
}
```

### 4. Domain-Driven Design

Core business concepts drive the architecture:

- **User**: Primary entity with authentication
- **Role**: Permission aggregation
- **Menu**: Navigation structure
- **Department**: Organizational hierarchy

---

## Technology Stack

### Backend

| Component | Technology | Purpose |
|-----------|------------|---------|
| Language | Rust 1.70+ | Type-safe, performance |
| Web Framework | Axum 0.7 | HTTP routing |
| Async Runtime | Tokio | Async execution |
| Database ORM | SQLx 0.8 | Runtime database queries |
| Database | PostgreSQL 14+ | Relational data storage |
| Authentication | JWTjsonwebtoken | Token-based auth |
| Password Hashing | bcrypt 0.15 | Password security |
| Logging | tracing 0.1 | Structured logging |
| Validation | Validator | Input validation |
| Error Handling | thiserror, anyhow | Error management |

### Frontend

| Component | Technology | Purpose |
|-----------|------------|---------|
| Framework | React 18.2 | UI library |
| Language | TypeScript 5.6 | Type-safe development |
| Build Tool | Vite 5.4 | Development server |
| UI Library | Ant Design 5.21 | Component library |
| Routing | React Router 6.20 | Navigation |
| State | React Query 5.60 | Data fetching |
| HTTP Client | Axios 1.7 | API calls |
| Validation | Zod 3.23 | Schema validation |
| Styling | Tailwind CSS | Styling |

### Infrastructure

| Component | Technology | Purpose |
|-----------|------------|---------|
| Container | Docker 20+ | Containerization |
| Orchestration | Docker Compose | Multi-container |
| Database | PostgreSQL 14+ | Data storage |
| CI/CD | GitHub Actions | Automation |
| Monitoring | Prometheus (future) | Metrics |
| Logging |ELK Stack (future) | Log aggregation |

---

## Key Decisions

### 1. Rust for Backend

**Decision**: Use Rust over Node.js/Python for backend

**Reasoning**:
- Performance critical APIs
- Memory safety guarantees
- Concurrent request handling
- Type safety catches bugs early

**Trade-offs**:
- Steeper learning curve
- Slower development compared to JavaScript
- Smaller ecosystem

### 2. Axum Framework

**Decision**: Use Axum overactix-web or warp

**Reasoning**:
- Built by Tokio team (async expertise)
- Type-safe routing
- Good middleware ecosystem
- Active community

### 3. SQLx Over Diesel

**Decision**: Use SQLx over Diesel ORM

**Reasoning**:
- Runtime queries (not compile-time)
- Better async support
- More flexible query building
- Active development

### 4. PostgreSQL as Primary Database

**Decision**: Use PostgreSQL over MySQL/MongoDB

**Reasoning**:
- Advanced JSON support
- ACID compliance
- Strong ecosystem
- Full-text search
- GIS support (PostGIS)

### 5. React Query Over Redux

**Decision**: Use React Query for data fetching

**Reasoning**:
- Less boilerplate
- Built-in caching
- Automatic refetching
- Simple API

### 6. Vite Over webpack

**Decision**: Use Vite over webpack

**Reasoning**:
- Faster dev server
- Native ESM
- Instant server restart
- Simpler configuration

---

## API Design

### RESTful Endpoints

| Method | Endpoint | Description | Security |
|--------|----------|-------------|----------|
| GET | /api/resource | List resources | Auth required |
| GET | /api/resource/:id | Get resource | Auth required |
| POST | /api/resource | Create resource | Auth required |
| PUT | /api/resource/:id | Update resource | Auth required |
| DELETE | /api/resource/:id | Delete resource | Auth required |

### Response Format

```json
{
  "code": 200,
  "message": "success",
  "data": { ... }
}
```

### Error Response Format

```json
{
  "code": 400,
  "message": "Invalid input",
  "data": null
}
```

---

## Authentication Flow

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Frontend  │────▶│   Backend   │────▶│  Database   │────▶│   Backend   │
│  (Browser)  │     │  (Axum)     │     │ (PostgreSQL)│     │  (Axum)     │
└─────────────┘     └─────────────┘     └─────────────┘     └─────────────┘
       │                   │                     │                   │
       │  1. Login Request │                     │                   │
       │───────────────────▶                                     │
       │                   │                                     │
       │                   │  2. Query User                     │
       │                   │─────────────────────────────────────▶
       │                   │                                     │
       │                   │                                     │  3. User Found
       │                   │◀─────────────────────────────────────│
       │                   │                                     │
       │                   │  4. Verify Password (bcrypt)        │
       │                   │─────────────────────────────────────▶
       │                   │                                     │
       │                   │                                     │  5. Password Match
       │                   │◀─────────────────────────────────────│
       │                   │                                     │
       │                   │  6. Generate JWT                    │
       │                   │─────────────────────────────────────▶
       │                   │                                     │
       │                   │                                     │  7. Token Created
       │                   │◀─────────────────────────────────────│
       │                   │                                     │
       │  8. Login Response│                                     │
       │◀──────────────────│                                     │
       │                   │                                     │
       │  9. Store Token   │                                     │
       │───────────────────▶                                     │
       │                   │                                     │
┌────────────────────────────────────────────────────────────────┐
│                    Token Stored in Memory                      │
└────────────────────────────────────────────────────────────────┘
```

## RBAC Architecture

### User-Role-Menu Relationship

```
┌──────────┐     ┌──────────┐     ┌──────────┐
│  User    │────▶│  Role    │◀────│  Menu    │
│ (sys_user)│    │ (sys_role)│    │ (sys_menu)│
└──────────┘     └──────────┘     └──────────┘
                                      │
                                      │
                              ┌──────────┐
                              │Department│
                              │           │
                              └──────────┘
```

### Permission Check Flow

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│  Request    │────▶│  Middleware │────▶│  Service    │
│  Received   │     │  Check Auth │     │  Check Perm │
└─────────────┘     └─────────────┘     └─────────────┘
                                        │
                                        ▼
                              ┌─────────────┐
                              │  Authenticated│
                              │  & Authorized │
                              └─────────────┘
                                        │
                              ┌─────────────┐
                              │   Execute   │
                              │  Operation  │
                              └─────────────┘
```

---

## Database Schema

### Entity-Relationship Diagram

```
┌──────────────────────────────────────────────────────────────────┐
│                         Database Schema                          │
└──────────────────────────────────────────────────────────────────┘

┌──────────┐          ┌──────────┐          ┌──────────┐
│ sys_user │─────────▶│ sys_role │◀─────────│ sys_menu │
│ (User)   │          │ (Role)   │          │ (Menu)   │
└──────────┘          └──────────┘          └──────────┘
     │                      │       ▲          │       ▲
     │                      │       │          │       │
     │         ┌────────────┼───────┴──────────┴───────┤
     │         │            │                            │
     ▼         ▼            ▼                            ▼
┌──────────┐ ┌──────────┐ ┌──────────┐          ┌──────────┐
│sys_post  │ │sys_dept  │ │sys_config│          │sys_noti  │
│ (Post)   │ │(Department│ │(Config)  │          │(Notice)  │
└──────────┘ │  )       │ └──────────┘          └──────────┘
             └──────────┘
                    │
                    ▼
            ┌──────────┐
            │sys_rol_dept│
            │ (Role-Dept)│
            └──────────┘
```

---

## Data Flow

### User Management Flow

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│   Frontend  │────▶│   Backend   │────▶│  Repository │────▶│  Database   │
│ User List   │     │  Service    │     │  (SQLx)    │     │ (PostgreSQL)│
└─────────────┘     └─────────────┘     └─────────────┘     └─────────────┘
       │                   │                     │                   │
       │  1. Request Users │                     │                   │
       │───────────────────▶                                     │
       │                   │                                     │
       │                   │  2. Query Database                 │
       │                   │─────────────────────────────────────▶
       │                   │                                     │
       │                   │                                     │  3. Return Users
       │                   │◀─────────────────────────────────────│
       │                   │                                     │
       │                   │  4. Format Response                 │
       │                   │─────────────────────────────────────▶
       │                   │                                     │
       │                   │                                     │  5. Response Ready
       │                   │◀─────────────────────────────────────│
       │                   │                                     │
       │  6. Return Data   │                                     │
       │◀──────────────────│                                     │
       │                   │                                     │
       │  7. Display UI    │                                     │
       │───────────────────▶                                     │
       │                   │                                     │
```

---

## Security Architecture

### Security Layers

```
┌──────────────────────────────────────────────────────────────────┐
│                    Security Layers                               │
└──────────────────────────────────────────────────────────────────┘

┌─────────────┐
│  Input      │  1. Validation (Zod/Validator)
│  Validation │     - Schema validation
└─────────────┘     - Type checking
                    - Length limits

┌─────────────┐
│  Auth       │  2. Authentication (JWT)
│  Middleware │     - JWT parsing
└─────────────┘     - Token verification
                    - User extraction

┌─────────────┐
│  Authorize  │  3. Authorization (RBAC)
│  Check      │     - Role validation
└─────────────┘     - Permission validation
                    - Data scope check

┌─────────────┐
│  Rate Limit │  4. Rate Limiting
│  Middleware │     - Request throttling
└─────────────┘     - Brute force protection

┌─────────────┐
│  Input San  │  5. Input Sanitization
│  itization  │     - XSS prevention
└─────────────┘     - SQL injection prevention
```

### Security Best Practices

1. **Password Hashing**: bcrypt with cost factor 12+
2. **Token Expiration**: JWT tokens expire after 1 hour
3. **Rate Limiting**: 100 requests per minute per IP
4. **CORS**: Whitelist allowed origins
5. **SQL Injection**: Parameterized queries
6. **XSS**: Sanitize all user inputs
7. **CSRF**: Token-based protection
8. **HTTPS**: Enforce HTTPS in production

---

## Performance Considerations

### Database Optimization

1. **Connection Pooling**: Minimum 5, maximum 20 connections
2. **Query Optimization**: Use indexes, avoid N+1
3. **Caching**: Cache frequently accessed data
4. **Pagination**: Always paginate list queries

### Frontend Optimization

1. **Code Splitting**: Lazy load components
2. **Image Optimization**: Use WebP format
3. **Caching**: Cache API responses
4. **Bundle Size**: Keep under 200KB gzipped

---

## Monitoring & Logging

### Structured Logging

```rust
use tracing::{info, error, debug};

info!("User logged in", username = %username);
error!("Failed to process request", error = %e);
debug!("Request payload", payload = ?payload);
```

### Metrics

```prometheus
# HELP.http_requests_total Total HTTP requests
# TYPE http_requests_total counter
http_requests_total{method="GET",status="200"} 100
http_requests_total{method="POST",status="201"} 50

# HELP.http_request_duration_seconds Request duration in seconds
# TYPE http_request_duration_seconds histogram
http_request_duration_seconds{method="GET",path="/api/users"} 0.05
```

---

## API Documentation

### OpenAPI Specification

- Location: `docs/api/openapi.yaml`
- URL: `/api-docs`
- Tools: utoipa-rs, Swagger UI

### Documentation Generation

```bash
# Generate OpenAPI spec
cargo doc --open

# View API docs
open http://localhost:8080/api-docs
```

---

## Testing Strategy

### Test Pyramid

```
                ┌─────────────┐
                │    E2E      │  10%
                │  Tests      │
                └─────────────┘
                        ▲
                        │
                ┌─────────────┐
                │ Integration │  30%
                │   Tests     │
                └─────────────┘
                        ▲
                        │
                ┌─────────────┐
                │   Unit      │  60%
                │   Tests     │
                └─────────────┘
```

### Test Coverage

- **Backend**: > 70% for critical paths
- **Frontend**: > 50% for components
- **Integration**: Critical flows 100%

---

## Deployment Architecture

### Development

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│  Frontend   │────▶│   Backend   │────▶│  Database   │
│  (Vite)     │     │  (Cargo)    │     │(PostgreSQL) │
│  Port 3000  │     │  Port 8080  │     │  Port 5432  │
└─────────────┘     └─────────────┘     └─────────────┘
```

### Production

```
┌─────────────┐     ┌─────────────┐     ┌─────────────┐
│  Browser    │────▶│   Nginx     │────▶│  Backend    │
│             │     │   (Port 80) │     │  (Port 8080)│
│             │     │             │     │             │
│             │     │  Static     │     │  PostgreSQL │
│             │     │  Files      │     │  (Internal) │
└─────────────┘     └─────────────┘     └─────────────┘
```

---

## Version Control

### Branching Strategy

```
main
├── release/1.0.0
├── develop
│   ├── feature/user-management
│   ├── feature/role-management
│   └── bugfix/login-bug
```

### Commit Convention

```
<type>(<scope>): <subject>

<body>

<footer>
```

Types: `feat`, `fix`, `docs`, `style`, `refactor`, `test`, `chore`

---

## Next Steps

1. Review individual architecture documents:
   - [Layered Architecture](./layered.md)
   - [RBAC Architecture](./rbac.md)
   - [Database Architecture](./database.md)
   - [API Architecture](./api.md)
   - [Security Architecture](./security.md)

2. Set up development environment
3. Run application locally
4. Review test coverage

---

## References

- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [Axum Documentation](https://docs.rs/axum/)
- [SQLx Documentation](https://docs.rs/sqlx/)
- [React Documentation](https://react.dev/)
- [Ant Design Documentation](https://ant.design/)
- [OpenAPI Specification](https://swagger.io/specification/)
