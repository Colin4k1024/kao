# Testing Patterns

**Analysis Date:** 2026-03-26

## Test Framework Matrix

### Frontend

**Unit Testing:**
- Framework: Jest via Umi Max
- Config: `frontend/jest.config.ts`
- Test environment: Browser with localStorage mock

**E2E Testing:**
- Framework: Playwright
- Config: `playwright.config.ts`
- Target: Local development server

### Backend

**Unit/Integration Testing:**
- Framework: tokio-test + axum test utilities
- Location: `backend/tests/` directory
- Runtime: tokio async runtime

## Test File Organization

### Frontend

**Location Patterns:**
- Co-located with source files: `xxx.test.tsx`
- Test snapshots: `xxx.__snapshots__/xxx.test.tsx.snap`
- E2E tests: `tests/` directory

**Examples:**
```
frontend/src/pages/user/login/
├── login.test.tsx
└── __snapshots__/login.test.tsx.snap

frontend/tests/
└── e2e.spec.ts

tests/
├── login.spec.ts
├── basic.spec.ts
└── home.spec.ts
```

### Backend

**Location:** `backend/tests/*.rs`

**Examples:**
- `backend/tests/auth_login.rs`
- `backend/tests/admin_domains.rs`
- `backend/tests/auth_session.rs`
- `backend/tests/backend_infrastructure.rs`

## Run Commands

### Frontend

```bash
# Run unit tests (Jest)
npm test

# Run E2E tests (Playwright)
npx playwright test

# Run all tests
npm test && npx playwright test

# Watch mode
npm test -- --watch
```

### Backend

```bash
# Run all tests
cargo test

# Run specific test
cargo test auth_login

# Run with coverage (requires external tooling)
cargo tarpaulin
```

## Frontend Test Structure

### Jest Test Pattern

```typescript
import { fireEvent, render } from '@testing-library/react';
import { startMock } from '@@/requestRecordMock';
import { TestBrowser } from '@@/testBrowser';

describe('Login Page', () => {
  let server: { close: () => void };

  beforeAll(async () => {
    server = await startMock({
      port: 8000,
      scene: 'login',
    });
  });

  afterAll(() => {
    server?.close();
  });

  it('should show login form', async () => {
    const historyRef = React.createRef<any>();
    const rootContainer = render(
      <TestBrowser
        historyRef={historyRef}
        location={{ pathname: '/user/login' }}
      />,
    );

    await rootContainer.findAllByText('Ant Design');
    expect(rootContainer.asFragment()).toMatchSnapshot();
    rootContainer.unmount();
  });

  it('should login success', async () => {
    // Test implementation
  });
});
```

**Configuration Highlights:**
- Mock server setup in `beforeAll`
- Snapshot testing enabled
- Custom `TestBrowser` wrapper for routing
- LocalStorage mocked as null

### Playwright Test Pattern

```typescript
import { test, expect } from '@playwright/test';

test.describe('Login Page E2E', () => {
  let authToken: string;

  test.beforeAll(async ({ request }) => {
    const response = await request.post(`${API_URL}/auth/login`, {
      data: { username: 'admin', password: 'Admin123!' },
    });
    const body = await response.json();
    authToken = body.data.access_token;
  });

  test('1. Login page renders', async ({ page }) => {
    await page.goto(`${BASE_URL}/user/login`);
    await expect(page.locator('text=RBAC Admin').first()).toBeVisible({ timeout: 10000 });
  });

  test('2. API - Profile returns correct data', async ({ request }) => {
    const response = await request.get(`${API_URL}/auth/profile`, {
      headers: { Authorization: `Bearer ${authToken}` },
    });
    expect(response.ok()).toBeTruthy();
    const body = await response.json();
    expect(body.code).toBe(0);
    expect(body.data.username).toBe('admin');
  });
});
```

### Backend Test Pattern

```rust
use axum::{body::Body, http::{Request, StatusCode}};
use tower::ServiceExt;
use ai_coding_backend::app::{router::create_router, state::AppState};
use ai_coding_backend::common::{config::Config, db::create_db_pool};
use std::sync::Arc;

#[tokio::test]
async fn login_returns_access_token_and_profile() {
    // Skip if DATABASE_URL not set
    if std::env::var("DATABASE_URL").is_err() {
        println!("Skipping test: DATABASE_URL not set");
        return;
    }

    let config = Arc::new(Config {
        port: 3000,
        database_url: std::env::var("DATABASE_URL").unwrap(),
        jwt_secret: "test_secret_for_testing".to_string(),
    });

    let db_pool = create_db_pool(&config.database_url).await.unwrap();
    let app_state = AppState { db: db_pool, config: config.clone() };
    let app = create_router(app_state);

    let login_payload = LoginRequest {
        username: "admin".to_string(),
        password: "Admin123!".to_string(),
    };

    let response = app.oneshot(
        Request::builder()
            .method("POST")
            .uri("/api/v1/auth/login")
            .header("content-type", "application/json")
            .body(Body::from(serde_json::to_vec(&login_payload).unwrap()))
            .unwrap(),
    ).await.unwrap();

    assert_eq!(response.status(), StatusCode::OK);
    
    let body = hyper::body::to_bytes(response.into_body()).await.unwrap();
    let response_json: serde_json::Value = serde_json::from_str(&String::from_utf8(body.to_vec()).unwrap()).unwrap();
    assert_eq!(response_json["code"], 0);
    assert!(response_json["data"]["access_token"].is_string());
}
```

## Test Coverage Areas

### Frontend Tests

**Unit Tests:**
- Login form component rendering
- UI interactions (input changes, button clicks)
- Snapshot testing for UI consistency
- Local storage interactions

**E2E Tests:**
1. Login page renders (`tests/login.spec.ts`)
2. Login form populated and submitted (`tests/login.spec.ts`)
3. API endpoints (`frontend/tests/e2e.spec.ts`):
   - Authentication (`/api/v1/auth/login`)
   - Profile (`/api/v1/auth/profile`)
   - Users list (`/api/v1/users`)
   - Roles list (`/api/v1/roles`)
   - Departments list (`/api/v1/departments`)
   - Menus list (`/api/v1/menus`)
   - Permissions (`/api/v1/auth/permissions`)
4. Frontend dashboard loads with token cookies
5. Navigation menu visibility

### Backend Tests

**Test Files:**
- `backend/tests/auth_login.rs` - Authentication login endpoint
- `backend/tests/auth_session.rs` - Session management
- `backend/tests/backend_infrastructure.rs` - Health check and infrastructure
- `backend/tests/admin_domains.rs` - Admin domain operations

**Coverage:**
- HTTP endpoint responses
- JWT token generation and validation
- Database operations
- Error handling paths
- Middleware functionality

## Mocking Strategy

### Frontend

**Request Mocking:**
- `@/requestRecordMock` for API request mocking
- `startMock({ port: 8000, scene: 'login' })` patterns

**DOM Testing:**
- React Testing Library for component rendering
- `fireEvent` for simulating user actions
- `findByPlaceholderText`, `findByText` for element queries

**Authentication Mocking:**
- LocalStorage mocked as null in globals
- Mock server intercepts API calls

### Backend

**HTTP requests:**
- Axum `ServiceExt::oneshot()` for router testing
- `Request::builder()` for constructing test requests
- Hyper body utilities for response inspection

**Database:**
- Real PostgreSQL via `create_db_pool()`
- Test database URL from environment
- Skip tests if `DATABASE_URL` not set

## Fixtures and Data

### Frontend

**Test Setup:**
```typescript
const waitTime = (time: number = 100) => {
  return new Promise((resolve) => {
    setTimeout(() => resolve(true), time);
  });
};
```

**Mock Data:**
- Hardcoded in tests: `{ username: 'admin', password: 'Admin123!' }`
- User data: `{ username: 'admin', roles: ['ADMIN'] }`

### Backend

**Test Data:**
```rust
let login_payload = LoginRequest {
    username: "admin".to_string(),
    password: "Admin123!".to_string(),
};
```

**Configuration:**
```rust
let config = Arc::new(Config {
    port: 3000,
    database_url: std::env::var("DATABASE_URL").unwrap(),
    jwt_secret: "test_secret_for_testing".to_string(),
});
```

## CI/CD Quality Gates

### Current Implementation

**Lint-staged:**
- Config: `frontend/.lintstagedrc`
- Runs Biome on all staged files
- Auto-fix enabled

```json
{
  "**/*.{js,jsx,tsx,ts,md,css,less,json}": [
    "npx @biomejs/biome check --write"
  ]
}
```

### Missing CI/CD

- No GitHub Actions workflows detected
- No automated test execution in CI
- No coverage reporting configured
- No pre-commit hooks (beyond lint-staged)

### Recommended CI/CD Pipeline

```yaml
# .github/workflows/ci.yml (recommended)
name: CI

on:
  push:
    branches: [main, develop]
  pull_request:
    branches: [main]

jobs:
  test:
    runs-on: ubuntu-latest
    services:
      postgres:
        image: postgres:15
        env:
          POSTGRES_PASSWORD: postgres
        ports: ["5432:5432"]
        options: >-
          --health-cmd "pg_isready -U postgres"
          --health-interval 10s
          --health-timeout 5s
          --health-retries 5
    
    steps:
      - uses: actions/checkout@v4
      
      - name: Setup Node.js
        uses: actions/setup-node@v4
        with:
          node-version: '20'
          cache: 'npm'
      
      - name: Setup Rust
        uses: dtolnay/rust-toolchain@stable
        with:
          components: clippy, rustfmt
      
      - name: Install dependencies
        run: npm ci
      
      - name: Lint TypeScript
        run: npm run lint
      
      - name: Run TypeScript tests
        run: npm test
      
      - name: Run Rust tests
        run: cargo test
      
      - name: Run Playwright tests
        run: npx playwright test
        env:
          DATABASE_URL: postgresql://postgres:postgres@localhost:5432/test_db
```

## Test Quality Metrics

### Frontend

**Coverage Areas:**
- Authentication flows
- API service layer
- Component rendering
- Route navigation
- LocalStorage interactions

**Coverage Gaps:**
- No coverage reports generated
- Limited form validation testing
- No memory leak testing
- No performance testing

### Backend

**Coverage Areas:**
- HTTP endpoint responses
- Authentication middleware
- Database queries
- Error responses
- JWT token handling

**Coverage Gaps:**
- No coverage reporting
- Limited negative test cases (errors, edge cases)
- No integration tests for complex workflows
- No load testing

## Best Practices Observed

### What's Well Done
1. ✅ Separate test directories for E2E and unit tests
2. ✅ Test database setup with proper isolation
3. ✅ JWT token reuse in E2E tests (beforeAll hooks)
4. ✅ Snapshot testing for UI components
5. ✅ Consistent error status code assertions
6. ✅ Skip tests when dependencies not configured
7. ✅ Playwright webServer auto-start

### What Needs Improvement
1. ❌ No code coverage reports generated
2. ❌ No automated CI/CD pipeline defined
3. ❌ Limited negative test cases (errors, edge cases)
4. ❌ Test data duplication across test files
5. ❌ No test fixtures directory
6. ❌ Missing performance regression tests
7. ❌ No accessibility testing

## Test File Templates

### Frontend Unit Test
```typescript
import { render, fireEvent } from '@testing-library/react';
import { test, expect } from '@jest/globals';

describe('ComponentName', () => {
  it('should render correctly', () => {
    const { container } = render(<Component />);
    expect(container).toMatchSnapshot();
  });

  it('should handle user interaction', () => {
    const { getByPlaceholderText } = render(<Component />);
    const input = getByPlaceholderText('Enter text');
    fireEvent.change(input, { target: { value: 'test' } });
    expect(input).toHaveValue('test');
  });
});
```

### Backend Integration Test
```rust
#[tokio::test]
async fn test_name() {
    // Skip if DB not configured
    if std::env::var("DATABASE_URL").is_err() {
        println!("Skipping: DATABASE_URL not set");
        return;
    }

    // Setup
    let config = Arc::new(Config { /* ... */ });
    let db_pool = create_db_pool(&config.database_url).await.unwrap();
    let app_state = AppState { db: db_pool, config };
    let app = create_router(app_state);

    // Execute
    let response = app.oneshot(request).await.unwrap();

    // Assert
    assert_eq!(response.status(), StatusCode::OK);
}
```

---

*Testing analysis: 2026-03-26*