# Coding Conventions

**Analysis Date:** 2026-03-26

## Language Standards

### TypeScript (Frontend)

**Strict Mode:** TypeScript is used in strict mode with the following configuration:
- Edition: ES2020+
- Strict compilation enabled
- ESLint with `@typescript-eslint/recommended` rules

**TypeScript Configuration:**
- Version: 5.6.3+
- ESLint: 9.13.0 with `@typescript-eslint/parser`
- Strict mode enforced via ESLint configuration

### Rust (Backend)

**Edition:** Rust 2021

**Key Dependencies:**
- `axum` 0.7 - Web framework with macros
- `tokio` 1 - Async runtime with full features
- `sqlx` 0.8 - Async PostgreSQL with compile-time SQL validation
- `serde` 1 - Serialization/deserialization with derive macros
- `thiserror` 1 - Custom error types
- `anyhow` 1 - Flexible error handling

## Naming Conventions

### Files

**TypeScript:**
- Components: `PascalCase.tsx` (e.g., `Login.tsx`, `Dashboard.tsx`)
- Hooks: `useXxx.ts` (e.g., `useAuth.ts`, `useDebounce.ts`)
- Services: `xxxService.ts` (e.g., `authService.ts`, `systemService.ts`)
- Types: `xxx.ts` inside `types/` directory
- Libraries: `xxx.ts` inside `lib/` directory

**Rust:**
- Modules: `mod.rs` in organized directories
- Structs: `PascalCase` (e.g., `User`, `LoginDto`, `ApiResponse`)
- Enums: `PascalCase` (e.g., `AppError`, `UserStatus`)
- Functions/Methods: `snake_case` (e.g., `find_user_by_username`)
- Constants: `SCREAMING_SNAKE_CASE`

### Functions

**TypeScript:**
- camelCase naming (e.g., `getUserInfo`, `updatePassword`)
- Async functions use `async` keyword
- Return types explicitly declared
- Hook functions prefixed with `use`

**Rust:**
- snake_case naming (e.g., `find_user_by_username`, `hash_password`)
- Return type explicitly declared with `-> Type`
- Functions in modules organized by concern

### Variables

**TypeScript:**
- camelCase naming
- Explicit type annotation for function parameters and return values
- Prefer const over let

**Rust:**
- snake_case naming
- Type inference used where obvious
- Explicit types for public interfaces

## Code Style

### Formatting

**Primary Tool:** Biome (with Prettier fallback)

**Configuration:** `.biome.json`
```json
{
  "formatter": {
    "enabled": true,
    "indentStyle": "space"
  },
  "javascript": {
    "jsxRuntime": "reactClassic",
    "formatter": {
      "quoteStyle": "single"
    }
  }
}
```

**Settings:**
- Indent: 2 spaces
- Line endings: LF
- Single quotes for strings
- Trailing commas: ES5
- Print width: 100 characters
- Arrow parentheses: always

**Files excluded from Biome:**
- `.umi/` directories
- `src/services/` (Swagger-generated)
- `mock/` directories
- `dist/`, `public/`, `coverage/` directories

**EditorConfig:** `.editorconfig`
- Root indicator present
- All files: indent with spaces, size 2
- Markdown files: no trailing whitespace
- Makefiles: tab indentation

### Linting

**TypeScript:**
- ESLint configured with:
  - `eslint:recommended`
  - `plugin:@typescript-eslint/recommended`
  - `plugin:react-hooks/recommended`
  - `react-refresh` plugin

**Key Rules:**
- `react-refresh/only-export-components`: warn (allow constant export)
- `@typescript-eslint/no-unused-vars`: warn

**npm script:** `npm run lint`

**Rust:**
- Standard `cargo fmt` for formatting
- `clippy` for linting
- No explicit clippy configuration found

## Import Organization

**TypeScript:**
1. External dependencies (react, axios, etc.)
2. Internal libraries (`@/lib/*`)
3. Types (`@/types/*`)
4. Services (`@/services/*`)
5. Components

**Path Aliases:**
- `@/` resolves to `./src/`
- Configured in Vite and Biome

**Rust:**
- Standard module system with `use` statements
- Organized by feature: `crate::features::auth`, `crate::common::auth`, etc.
- Relative imports for test modules: `super::`, `crate::`

## Type Safety

**TypeScript:**
- Strict mode enabled
- Explicit return types on functions
- Generic types used for API responses
- `ApiResponse<T>` pattern for standardized responses

**Rust:**
- Strong static typing
- Custom error types using `thiserror`
- `Result<T, AppError>` return pattern
- Strongly-typed DTOs (e.g., `LoginDto`, `CreateUserDto`)

## Error Handling

**TypeScript Patterns:**
```typescript
// API error handling via axios interceptors
api.interceptors.response.use(
  (response) => {
    if (res.code !== 200) {
      ElMessage.error(res.message);
      if (res.code === 401) {
        localStorage.removeItem('access_token');
        window.location.href = '/login';
      }
      return Promise.reject(new Error(res.message));
    }
    return response;
  },
  (error) => {
    ElMessage.error(error.message || '网络错误');
    return Promise.reject(error);
  }
);
```

**Rust Patterns:**
```rust
#[derive(Debug)]
pub enum AppError {
    Database(String),
    Authentication(String),
    Authorization(String),
    Validation(String),
    Internal(String),
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err.to_string())
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        // Converts to JSON with structured error format
    }
}
```

**Error Response Format:**
```json
{
  "code": 401,
  "message": "Authentication error: Invalid token",
  "data": null
}
```

## Logging

**TypeScript:**
- console.log for development
- No structured logging configured
- Error messages shown via Element Plus notifications

**Rust:**
- `tracing` crate for structured logging
- `tracing-subscriber` with env-filter for filtering
- Structured logs with context

## Documentation

**JSDoc/TSDoc:**
- Not commonly used in codebase
- TypeScript files lack JSDoc comments
- Function purpose inferred from naming

**Rust:**
- Standard `///` doc comments used
- Example: `/// Handles user login request`

## React Patterns

**Component Structure:**
- Functional components only
- Hooks for state and side effects
- Props typed with interfaces

**Component Naming:**
- PascalCase: `Login`, `Dashboard`, `MainLayout`
- Hooks: `useAuth`, `useLocalStorage`, `useDebounce`

**State Management:**
- React Query for server state
- useState for component local state
- LocalStorage for authentication tokens

**Service Pattern:**
```typescript
export const authApi = {
  login: (params: LoginParams) => api.post<LoginResult>('/auth/login', params),
  logout: () => api.post('/auth/logout'),
  getUserInfo: () => api.get<UserInfo>('/auth/userinfo'),
};
```

## API Protocol

**Request Format:**
- Content-Type: `application/json`
- Authorization header with Bearer token
- JSON body for POST/PUT/PATCH

**Response Format:**
```json
{
  "code": 0,
  "message": "ok",
  "data": { /* response data */ }
}
```

**Error Codes:**
- 200: Success
- 400: Bad request
- 401: Unauthorized
- 403: Forbidden
- 500: Server error

## Backend Module Organization

**Structure:**
```
src/
├── app/              # App router and state
├── common/           # Shared utilities (auth, error, config)
├── features/         # Feature modules
│   ├── auth/
│   │   ├── model.rs   # Domain models
│   │   ├── repo.rs    # Database repository
│   │   ├── service.rs # Business logic
│   │   ├── routes.rs  # HTTP routes
│   │   └── mod.rs     # Module re-exports
│   ├── users/
│   ├── roles/
│   └── ...
├── models/           # Database models
├── repositories/     # Repository pattern
├── services/         # Service layer
├── middleware/       # HTTP middleware
├── api/              # HTTP API handlers
└── main.rs           # Entry point
```

## Security Standards

**TypeScript:**
- All API calls include authentication token from localStorage
- Input validation via axios interceptors
- Token refresh on 401 responses
- No SQL injection possible (API-based)

**Rust:**
- Parameterized SQL queries via SQLx
- Password hashing with bcrypt
- JWT token validation
- CORS middleware configured
- Auth middleware for protected routes

## Conventions Summary

### Must Follow
- TypeScript strict mode enabled
- Biome/linter enabled in CI
- Async functions return `Promise`
- Rust functions return `Result<T, AppError>`
- All errors formatted consistently
- Authentication on all protected routes
- Input validation before database operations

### Recommended
- Use hooks for reusable logic
- Organize Rust modules by feature
- Test database operations
- Log errors with context
- Use DTOs for API boundaries

---

*Convention analysis: 2026-03-26*