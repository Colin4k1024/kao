# Layered Architecture

## Overview

Kao follows a layered architecture pattern that separates concerns into distinct vertical slices. Each layer has a specific responsibility and communicates with adjacent layers through well-defined interfaces.

## Layer Structure

```
┌──────────────────────────────────────────────────────────────────┐
│                     Layered Architecture                         │
└──────────────────────────────────────────────────────────────────┘

┌──────────────────────────────────────────────────────────────────┐
│                   Presentation Layer (Frontend)                  │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐        │
│  │  Pages   │  │Components│  │  Hooks   │  │ Services │        │
│  │  (React) │  │ (AntD)   │  │  (useX)  │  │  (Axios) │        │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘        │
└──────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌──────────────────────────────────────────────────────────────────┐
│                      API Layer (Backend)                         │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐        │
│  │ Routes   │──▶│Middleware│──▶│Response│──▶│ Request  │        │
│  │ (Axum)   │  │  (Tower) │  │Format  │  │Validation│        │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘        │
└──────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌──────────────────────────────────────────────────────────────────┐
│                 Application Layer (Business Logic)               │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐        │
│  │Services  │  │Validators│  │Converters│  │Policies  │        │
│  │  (Business│ │  (Zod)   │  │  (Data)  │  │ (Rules)  │        │
│  │  Logic)  │  │          │  │          │  │          │        │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘        │
└──────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌──────────────────────────────────────────────────────────────────┐
│                    Domain Layer (Entities)                       │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐        │
│  │ User     │  │  Role    │  │  Menu    │  │Department│        │
│  │Entity    │  │ Entity   │  │ Entity   │  │ Entity   │        │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘        │
└──────────────────────────────────────────────────────────────────┘
                              │
                              ▼
┌──────────────────────────────────────────────────────────────────┐
│                 Infrastructure Layer (Data Access)               │
│  ┌──────────┐  ┌──────────┐  ┌──────────┐  ┌──────────┐        │
│  │Database  │  │ Repositories│ │  Cache   │  │External │        │
│  │ (SQLx)   │  │  (Traits) │  │  (Redis) │  │ Services │        │
│  └──────────┘  └──────────┘  └──────────┘  └──────────┘        │
└──────────────────────────────────────────────────────────────────┘
```

## Backend Layer Details

### 1. Presentation Layer (Frontend)

#### Pages
- **Location**: `frontend/src/pages/`
- **Responsibility**: Full-page components
- **Examples**: `UserListPage.tsx`, `RoleFormPage.tsx`

#### Components
- **Location**: `frontend/src/components/`
- **Responsibility**: Reusable UI components
- **Categories**: 
  - `common/` -通用按钮、表单控件
  - `layout/` - Header, Sidebar, Footer
  - `charts/` - Chart components
  - `tables/` - Table components

#### Hooks
- **Location**: `frontend/src/hooks/`
- **Responsibility**: Custom React hooks
- **Examples**: `useAuth()`, `usePermission()`

#### Services
- **Location**: `frontend/src/services/`
- **Responsibility**: API clients
- **Examples**: `userApi.ts`, `roleApi.ts`

### 2. API Layer (Backend)

#### Routes
- **Location**: `backend/src/app/routes.rs`
- **Responsibility**: HTTP routing
- **Pattern**:
  ```rust
  Router::new()
      .route("/users", get(list_users).post(create_user))
      .route("/users/:id", get(get_user).put(update_user))
  ```

#### Middleware
- **Location**: `backend/src/middleware/`
- **Responsibility**: Cross-cutting concerns
- **Types**:
  - `auth.rs` - Authentication
  - `cors.rs` - CORS configuration
  - `logger.rs` - Request logging
  - `openapi.rs` - OpenAPI docs

#### Response Formatting
- **Location**: `backend/src/common/response.rs`
- **Responsibility**: Standardized responses
- **Format**:
  ```rust
  {
      "code": 200,
      "message": "success",
      "data": { ... }
  }
  ```

### 3. Application Layer

#### Services
- **Location**: `backend/src/features/*/service.rs`
- **Responsibility**: Business logic orchestration
- **Pattern**: Transaction management, validate inputs, coordinate repository calls

#### Validators
- **Location**: `backend/src/common/validators.rs`
- **Responsibility**: Input validation (Zod for Rust)
- **Pattern**: Schema validation before service processing

#### Converters
- **Location**: `backend/src/common/converters.rs`
- **Responsibility**: Data transformation between layers
- **Pattern**: Request → DTO → Entity → Response DTO

### 4. Domain Layer

#### Entities
- **Location**: `backend/src/models/`
- **Responsibility**: Business entities
- **Examples**: `User`, `Role`, `Menu`, `Department`

#### Value Objects
- **Location**: `backend/src/common/models/`
- **Responsibility**: Immutable domain values
- **Examples**: `UserId`, `RoleId`, `PhoneNumber`

### 5. Infrastructure Layer

#### Database
- **Location**: `backend/src/db/`
- **Responsibility**: Database connection pool
- **Pattern**: Singleton `PgPool`

#### Repositories
- **Location**: `backend/src/features/*/repo.rs`
- **Responsibility**: Data access abstraction
- **Pattern**: Trait + Implementation
  ```rust
  pub trait UserRepository {
      fn find_by_id(&self, id: Uuid) -> Result<Option<User>>;
  }
  
  pub struct PgUserRepository {
      pool: PgPool,
  }
  ```

#### Caching
- **Location**: `backend/src/common/cache/`
- **Responsibility**: Data caching
- **Pattern**: Cache decorator pattern

## Frontend Layer Details

### 1. Pages Layer

```typescript
// frontend/src/pages/system/users/UserListPage.tsx
import { UserList } from '@/components/system/users/UserList'

export const UserListPage: React.FC = () => {
  return <UserList />
}
```

### 2. Components Layer

```typescript
// frontend/src/components/system/users/UserList.tsx
import { Table, Button } from 'antd'

export const UserList: React.FC = () => {
  return (
    <Table dataSource={data} columns={columns} />
  )
}
```

### 3. Hooks Layer

```typescript
// frontend/src/hooks/useAuth.ts
export const useAuth = () => {
  const [user, setUser] = useState<User | null>(null)
  
  const login = async (username: string, password: string) => {
    // ...
  }
  
  return { user, login, logout }
}
```

### 4. Services Layer

```typescript
// frontend/src/services/api/user.ts
export const userApi = {
  getUsers: async (params: any) => {
    return await request.get('/api/system/users', { params })
  }
}
```

## Data Flow

### Request Flow

```
┌─────────────┐
│   Browser   │
└──────┬──────┘
       │
       ▼
┌─────────────┐
│    Frontend │
│   Service   │  userApi.getUsers()
└──────┬──────┘
       │
       ▼
┌─────────────┐
│   Backend   │
│   Route     │  GET /api/system/users
└──────┬──────┘
       │
       ▼
┌─────────────┐
│   Middleware│  Auth check, CORS
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  Controller │  Parse request, validate
└──────┬──────┘
       │
       ▼
┌─────────────┐
│   Service   │  Business logic
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ Repository  │  Database query
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  Database   │  Return data
└──────┬──────┘
       │
       ▼
┌─────────────┐
│ Repository  │  Map to entity
└──────┬──────┘
       │
       ▼
┌─────────────┐
│   Service   │  Transform data
└──────┬──────┘
       │
       ▼
┌─────────────┐
│  Controller │  Create response
└──────┬──────┘
       │
       ▼
┌─────────────┐
│   Middleware│  Format response, add headers
└──────┬──────┘
       │
       ▼
┌─────────────┐
│    Frontend │
│   Service   │  Parse response
└──────┬──────┘
       │
       ▼
┌─────────────┐
│   Component │  Display data
└─────────────┘
```

## Layer Dependencies

```
┌──────────────────────────────────────────────────────────────────┐
│                Layer Dependencies (Arrows = Depends On)        │
└──────────────────────────────────────────────────────────────────┘

Presentation ──▶ API ──▶ Application ──▶ Domain ──▶ Infrastructure
     │           │          │           │          │
     ▼           ▼          ▼           ▼          ▼
  Components   Routes    Services    Entities    Database
```

## Cross-Cutting Concerns

### Logging

```rust
// Applied across all layers
use tracing::{info, error, debug};

info!("User logged in", username = %username);
error!("Database query failed", error = %e);
```

### Error Handling

```rust
// Standard error conversion
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Not found: {0}")]
    NotFound(String),
}
```

### Validation

```rust
// Validate at each layer boundary
use validator::Validate;

#[derive(Validate)]
pub struct CreateUserRequest {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    
    #[validate(email)]
    pub email: String,
}
```

## Layer Boundaries

### Backward Dependencies Are Forbidden

```
❌ BAD: Infrastructure ──▶ Application
   (Database queries in services)

✅ GOOD: Infrastructure ──▶ Domain
   (Repositories depend on domain entities only)
```

### Dependencies Point Inward

```
   ┌──────────────────────────┐
   │   API Layer              │
   │ (Controllers, Routes)   │
   └──────────────────────────┘
              │
              ▼
   ┌──────────────────────────┐
   │ Application Layer        │
   │    (Services)            │
   └──────────────────────────┘
              │
              ▼
   ┌──────────────────────────┐
   │   Domain Layer           │
   │   (Entities)             │
   └──────────────────────────┘
              │
              ▼
   ┌──────────────────────────┐
   │ Infrastructure Layer     │
   │ (Database, Cache, APIs)  │
   └──────────────────────────┘
```

## Layer Testing

### Unit Tests by Layer

```
┌──────────────────────────────────────────────────────────────────┐
│                      Layer Testing                               │
└──────────────────────────────────────────────────────────────────┘

Presentation (Frontend)
├── Component Tests (Jest)
├── Hook Tests
└── Integration Tests (Playwright)

API Layer (Backend)
├── Route Tests
├── Middleware Tests
└── Response Formatting Tests

Application Layer
├── Service Tests
├── Validator Tests
└── Converter Tests

Domain Layer
├── Entity Tests
├── Value Object Tests
└── Domain Service Tests

Infrastructure Layer
├── Database Tests
├── Repository Tests
└── Cache Tests
```

### Test Locations

```
backend/tests/
├── common/
├── features/
│   ├── auth/
│   ├── users/
│   ├── roles/
│   └── ...
└── mod.rs

frontend/tests/
├── components/
├── hooks/
├── services/
└── utils/
```

---

## Next Steps

1. See [Database Architecture](./database.md) for data layer details
2. See [API Architecture](./api.md) for API design patterns
3. See [Security Architecture](./security.md) for security patterns
4. Review layer-specific code in `backend/src/` and `frontend/src/`
