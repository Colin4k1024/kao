# Kao Development Guide

Enterprise Admin Management System Development Documentation

## Table of Contents
- [Overview](#overview)
- [Project Structure](#project-structure)
- [Development Environment Setup](#development-environment-setup)
- [Coding Standards](#coding-standards)
- [Git Workflow](#git-workflow)
- [Pull Request Process](#pull-request-process)
- [Code Review Guidelines](#code-review-guidelines)
- [Release Process](#release-process)

---

## Overview

Kao is an enterprise-grade admin management system built with Rust backend and React frontend. This guide covers development practices, coding standards, and workflows.

### Technology Stack

#### Backend
- **Language**: Rust 1.70+
- **Web Framework**: Axum 0.7
- **Async Runtime**: Tokio
- **Database ORM**: SQLx 0.8
- **Database**: PostgreSQL 14+
- **Authentication**: JWT (jsonwebtoken 9) + bcrypt 0.15
- **Logging**: tracing 0.1
- **Validation**:Validator
- **Error Handling**: thiserror, anyhow

#### Frontend
- **Framework**: React 18.2
- **Language**: TypeScript 5.6
- **Build Tool**: Vite 5.4
- **UI Library**: Ant Design 5.21
- **Routing**: React Router DOM 6.20
- **State Management**: React Query 5.60, React Hook Form 7.53
- **HTTP Client**: Axios 1.7
- **Validation**: Zod 3.23

### Project Goals
- Highly scalable and secure admin management system
- Enterprise-grade RBAC (Role-Based Access Control)
- Clean architecture with separation of concerns
- Comprehensive documentation and test coverage

---

## Project Structure

### Backend Structure

```
backend/
├── src/
│   ├── main.rs                 # Application entry point
│   ├── app/                    # Application setup and routing
│   │   ├── mod.rs
│   │   └── routes.rs
│   ├── config/                 # Configuration management
│   │   ├── mod.rs
│   │   └── settings.rs
│   ├── db/                     # Database connection and pool
│   │   ├── mod.rs
│   │   └── pool.rs
│   ├── middleware/             # Axum middleware
│   │   ├── mod.rs
│   │   ├── auth.rs            # Authentication middleware
│   │   ├── cors.rs            # CORS configuration
│   │   ├── logger.rs          # Request logging
│   │   └── openapi.rs         # OpenAPI documentation
│   ├── common/                 # Common utilities
│   │   ├── mod.rs
│   │   ├── response.rs        # Response formatting
│   │   ├── error.rs           # Error types
│   │   ├── permissions.rs     # Permission validation
│   │   └── auth.rs            # Auth utilities
│   ├── features/               # Feature modules
│   │   ├── mod.rs
│   │   ├── auth/              # Authentication feature
│   │   │   ├── mod.rs
│   │   │   ├── handlers.rs
│   │   │   ├── service.rs
│   │   │   ├── repo.rs
│   │   │   └── routes.rs
│   │   ├── users/             # User management feature
│   │   │   └── ...
│   │   ├── roles/             # Role management feature
│   │   │   └── ...
│   │   ├── menus/             # Menu management feature
│   │   │   └── ...
│   │   ├── departments/       # Department management feature
│   │   │   └── ...
│   │   ├── config/            # Configuration feature
│   │   │   └── ...
│   │   ├── dictionary/        # Dictionary feature
│   │   │   └── ...
│   │   ├── notice/            # Notice feature
│   │   │   └── ...
│   │   ├── job/               # Job management feature
│   │   │   └── ...
│   │   └── monitoring/        # Monitoring feature
│   │       └── ...
│   ├── models/                 # Data models
│   │   ├── mod.rs
│   │   └── user.rs
│   ├── repositories/           # Repository layer
│   │   ├── mod.rs
│   │   └── user_repo.rs
│   ├── services/               # Business logic
│   │   ├── mod.rs
│   │   └── user_service.rs
│   └── utils/                  # Utility functions
│       ├── mod.rs
│       └── validators.rs
├── migrations/                 # Database migrations
│   ├── 0001_create_sys_department.sql
│   ├── 0002_create_sys_post.sql
│   ├── 0003_create_sys_user.sql
│   ├── 0004_create_sys_role.sql
│   ├── 0005_create_sys_menu.sql
│   ├── 0006_create_sys_user_role.sql
│   ├── 0007_create_sys_role_menu.sql
│   ├── 0008_create_sys_role_department.sql
│   └── 0099_init_data.sql
├── tests/                      # Integration tests
│   ├── common/
│   │   └── mod.rs
│   ├── features/
│   │   ├── auth/
│   │   └── users/
│   └── mod.rs
├── scripts/                    # Utility scripts
│   ├── setup.sh
│   └── health-check.sh
├── Cargo.toml                  # Project dependencies
└── Cargo.lock                  # Dependency lock file
```

### Frontend Structure

```
frontend/
├── src/
│   ├── main.tsx                # Application entry point
│   ├── App.tsx                 # Root component
│   ├── assets/                 # Static assets
│   │   ├── styles/
│   │   │   ├── main.css
│   │   │   └── variables.css
│   │   └── images/
│   ├── components/             # Reusable components
│   │   ├── common/            # Common components
│   │   │   ├── Button.tsx
│   │   │   ├── Card.tsx
│   │   │   ├── Input.tsx
│   │   │   ├── Select.tsx
│   │   │   └── Table.tsx
│   │   ├── layout/            # Layout components
│   │   │   ├── Header.tsx
│   │   │   ├── Sidebar.tsx
│   │   │   └── Footer.tsx
│   │   └── .../
│   ├── pages/                  # Page components
│   │   ├── Login.tsx
│   │   ├── Dashboard.tsx
│   │   ├── system/            # System management pages
│   │   │   ├── users/        # User management
│   │   │   │   ├── UserList.tsx
│   │   │   │   ├── UserForm.tsx
│   │   │   │   └── ...
│   │   │   ├── roles/        # Role management
│   │   │   │   ├── RoleList.tsx
│   │   │   │   └── ...
│   │   │   ├── menus/        # Menu management
│   │   │   │   └── ...
│   │   │   └── departments/  # Department management
│   │   │       └── ...
│   │   ├── config/            # Configuration pages
│   │   │   ├── ConfigList.tsx
│   │   │   └── ...
│   │   ├── dictionary/        # Dictionary pages
│   │   │   └── ...
│   │   ├── notice/            # Notice pages
│   │   │   └── ...
│   │   ├── job/               # Job management pages
│   │   │   └── ...
│   │   ├── monitoring/        # Monitoring pages
│   │   │   └── ...
│   │   └── error/             # Error pages
│   │       ├── 403.tsx
│   │       ├── 404.tsx
│   │       └── 500.tsx
│   ├── services/               # API services
│   │   ├── api/              # API clients
│   │   │   ├── auth.ts
│   │   │   ├── user.ts
│   │   │   ├── role.ts
│   │   │   ├── menu.ts
│   │   │   ├── department.ts
│   │   │   ├── config.ts
│   │   │   ├── dictionary.ts
│   │   │   ├── notice.ts
│   │   │   ├── job.ts
│   │   │   └── monitoring.ts
│   │   └── request.ts         # HTTP client setup
│   ├── hooks/                  # Custom hooks
│   │   ├── useAuth.ts
│   │   ├── usePermission.ts
│   │   └── useValidation.ts
│   ├── lib/                    # Utility libraries
│   │   ├── utils.ts
│   │   ├── validator.ts
│   │   └── constants.ts
│   ├── routes/                 # Route configuration
│   │   ├── index.ts
│   │   └── guards.ts
│   ├── store/                  # Global state
│   │   ├── index.ts
│   │   └── reducer.ts
│   └── types/                  # TypeScript types
│       ├── api.d.ts
│       ├── auth.d.ts
│       └── common.d.ts
├── public/
│   ├── index.html
│   └── favicon.ico
├── tests/                      # Test files
│   ├── components/
│   ├── pages/
│   └── utils/
├── vite.config.ts              # Vite configuration
├── tailwind.config.js          # Tailwind configuration
├── postcss.config.js           # PostCSS configuration
├── tsconfig.json               # TypeScript configuration
├── eslintrc.js                 # ESLint configuration
├── prettier.config.js          # Prettier configuration
└── package.json                # Project dependencies
```

### API Layer Structure

```
API Layer Pattern:
┌──────────────────────────────────────────────────────────┐
│                   API Request                            │
└──────────────────────────────────────────────────────────┘
                         │
                         ▼
┌──────────────────────────────────────────────────────────┐
│              Controllers (routes.rs)                     │
│  - Route definitions                                     │
│  - Request validation                                    │
│  - Response formatting                                   │
└──────────────────────────────────────────────────────────┘
                         │
                         ▼
┌──────────────────────────────────────────────────────────┐
│               Services (service.rs)                      │
│  - Business logic                                        │
│  - Data transformation                                   │
│  - Cross-cutting concerns                                │
└──────────────────────────────────────────────────────────┘
                         │
                         ▼
┌──────────────────────────────────────────────────────────┐
│              Repositories (repo.rs)                      │
│  - Database queries                                      │
│  - Query construction                                    │
│  - Transaction management                                │
└──────────────────────────────────────────────────────────┘
                         │
                         ▼
┌──────────────────────────────────────────────────────────┐
│               Database (SQLx)                            │
│  - Query execution                                       │
│  - Result mapping                                        │
└──────────────────────────────────────────────────────────┘
```

---

## Development Environment Setup

### Prerequisites

```bash
# Check installed versions
rustc --version      # Should be 1.70 or later
cargo --version      # Should be 1.70 or later
node --version       # Should be 18 or later
npm --version        # Should be 9 or later
psql --version       # Should be 14 or later
```

### Linux/Ubuntu Setup

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
source "$HOME/.cargo/env"

# Install Node.js
curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
sudo apt-get install -y nodejs

# Install PostgreSQL
sudo apt-get update
sudo apt-get install -y postgresql postgresql-contrib

# Install Make (build-essential includes make)
sudo apt-get install -y build-essential

# Install SQLx CLI (optional, for migrations)
cargo install sqlx-cli --features postgres
```

### macOS Setup

```bash
# Install Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Install Node.js (using Homebrew)
brew install node

# Install PostgreSQL
brew install postgresql
brew services start postgresql

# Install SQLx CLI (optional)
cargo install sqlx-cli --features postgres
```

### Windows Setup

1. Install Rust from https://rustup.rs/
2. Install Node.js from https://nodejs.org/
3. Install PostgreSQL from https://www.postgresql.org/download/windows/
4. Ensure PostgreSQL is in PATH
5. Install Make from https://gnuwin32.sourceforge.net/packages/make.htm

---

### Repository Setup

```bash
# Clone repository
git clone https://github.com/kao-admin/kao.git
cd kao

# Install backend dependencies
cd backend
cargo build

# Install frontend dependencies
cd ../frontend
npm install

# Setup database
cd ../backend
cp .env.example .env
# Edit .env with your database credentials

# Run migrations
make migrate

# Start backend
cargo run

# In another terminal, start frontend
cd ../frontend
npm run dev
```

### Environment Variables

Create `.env` file in backend directory:

```env
# Database Configuration
DATABASE_URL=postgres://postgres:password@localhost:5432/kao_db
DB_PASSWORD=your-password

# JWT Configuration
JWT_SECRET=your-super-secret-key-change-in-production-min-32-chars
JWT_ACCESS_TOKEN_EXPIRES_IN=3600
JWT_REFRESH_TOKEN_EXPIRES_IN=604800

# Application Configuration
APP_HOST=0.0.0.0
APP_PORT=8080
RUST_LOG=info

# CORS Configuration (comma-separated)
ALLOWED_ORIGINS=http://localhost:3000,http://localhost:5173
```

Create `.env` file in frontend directory:

```env
# Vite environment variables (must start with VITE_)
VITE_API_URL=http://localhost:8080
VITE_APP_NAME=Kao Admin
VITE_APP_URL=http://localhost:3000
```

### Development Commands

```bash
# Backend
cargo build        # Build backend
cargo run          # Run backend
cargo test         # Run backend tests
cargo clippy       # Run clippy linter
cargo fmt          # Format code

# Frontend
npm install        # Install dependencies
npm run dev        # Run dev server
npm run build      # Build for production
npm run lint       # Run linter
npm run format     # Format code
npm test           # Run tests

# Common Tasks
make build          # Build backend
make build-front    # Build frontend
make test           # Run all tests
make migrate        # Run database migrations
make dev            # Run both backend and frontend
```

---

## Coding Standards

### Backend Standards (Rust)

#### 1. Code Style

Follow [The Rust Programming Language](https://doc.rust-lang.org/book/) and [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/).

#### 2. Naming Conventions

```rust
// Types (PascalCase)
struct UserService;
enum UserRole { Active, Disabled }

// Variables (snake_case)
let user_name: String;
let max_retries: u32;

// Functions (camelCase)
fn get_user_by_id(id: &Uuid) -> Result<User> {
    // ...
}

// Constants (SCREAMING_SNAKE_CASE)
const MAX_LOGIN_ATTEMPTS: usize = 3;
const TOKEN_EXPIRY_SECONDS: u64 = 3600;

// Modules (snake_case)
mod user_service;
mod auth_middleware;

// Traits (PascalCase + 'e' suffix for traits)
trait Repository {
    fn find(&self, id: &Uuid) -> Result<Option<Self::Item>>;
}
```

#### 3. Error Handling

```rust
// Use thiserror for custom errors
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),
    
    #[error("Authentication error: {0}")]
    Authentication(String),
    
    #[error("Not found: {0}")]
    NotFound(String),
}

// Convert errors using ? operator
fn find_user(id: Uuid) -> Result<User, AppError> {
    let user = sqlx::query_as!(User, "SELECT * FROM users WHERE id = $1", id)
        .fetch_one(&pool)
        .await?;
    Ok(user)
}
```

#### 4. Response Format

All API responses should follow this format:

```rust
{
    "code": 200,
    "message": "success",
    "data": { ... }
}
```

Error responses:

```rust
{
    "code": 400,
    "message": "Invalid input",
    "data": null
}
```

#### 5. Logging

Use `tracing` crate for structured logging:

```rust
use tracing::{info, error, debug};

info!("User logged in: {}", user_id);
error!("Failed to process request: {}", error);
debug!("Request payload: {:?}", payload);
```

#### 6. Validation

Use Validator for input validation:

```rust
use validator::{Validate, ValidationErrors};

#[derive(Validate)]
pub struct LoginRequest {
    #[validate(length(min = 3, max = 50))]
    pub username: String,
    
    #[validate(length(min = 8))]
    pub password: String,
}
```

### Frontend Standards (TypeScript/React)

#### 1. Code Style

Follow [TypeScript Deep Dive](https://basarat.gitbooks.io/typescript/) and [React Best Practices](https://react.dev/).

#### 2. Component Naming

```tsx
// Component names (PascalCase)
const UserProfile: React.FC = () => { ... };

// Hook names (useXxx)
const useAuth = () => { ... };
const useValidation = () => { ... };

// Event handler names (handleXxx)
const handleClick = () => { ... };
const handleChange = (e: React.ChangeEvent<HTMLInputElement>) => { ... };

// Variable names (camelCase)
const userId: string = '123';
const maxRetryAttempts: number = 3;

// Constant names (UPPER_SNAKE_CASE)
const API_BASE_URL = 'http://localhost:8080';
const TOKEN_EXPIRY = 3600;
```

#### 3. File Organization

```tsx
// Feature-based structure
src/
├── pages/
│   └── system/
│       └── users/
│           ├── components/
│           │   ├── UserList.tsx
│           │   └── UserForm.tsx
│           ├── hooks/
│           │   ├── useUsers.ts
│           │   └── useUserForm.ts
│           ├── services/
│           │   └── userApi.ts
│           ├── mocks/
│           │   └── userMock.ts
│           ├── tests/
│           │   └── UserList.test.tsx
│           ├── types/
│           │   └── user.d.ts
│           └── UserPage.tsx
```

#### 4. Component Structure

```tsx
import React from 'react';
import { useQuery, useMutation } from '@tanstack/react-query';
import { Button, Table, Form } from 'antd';

// 1. Import dependencies
// 2. Define types/interfaces
// 3. Define component
// 4. Define helper functions
// 5. Export component

export interface User {
  id: string;
  username: string;
  email: string;
  status: 'active' | 'disabled';
}

interface UserFormData {
  username: string;
  email: string;
}

export const UserList: React.FC = () => {
  // 6. Define hooks
  const [form] = Form.useForm();
  
  // 7. Define queries
  const { data, isLoading } = useQuery(['users'], fetchUsers);
  
  // 8. Define mutations
  const { mutate: createUser } = useMutation(createUserMutation);
  
  // 9. Define event handlers
  const handleClick = () => { ... };
  
  // 10. Define render
  return (
    <div>
      {/* JSX content */}
    </div>
  );
};
```

#### 5. API Service Pattern

```typescript
// src/services/api/user.ts
import { request } from '../request';

export interface User {
  id: string;
  username: string;
  email: string;
  status: 'active' | 'disabled';
}

export interface CreateUserRequest {
  username: string;
  email: string;
  password: string;
}

export const userApi = {
  // Get user list
  getUsers: (params?: { page?: number; pageSize?: number }) => 
    request.get<User[]>('/api/system/users', { params }),
  
  // Get user by ID
  getUser: (id: string) => 
    request.get<User>(`/api/system/users/${id}`),
  
  // Create user
  createUser: (data: CreateUserRequest) => 
    request.post<User>('/api/system/users', data),
  
  // Update user
  updateUser: (id: string, data: Partial<User>) => 
    request.put<User>(`/api/system/users/${id}`, data),
  
  // Delete user
  deleteUser: (id: string) => 
    request.delete(`/api/system/users/${id}`),
  
  // Reset password
  resetPassword: (id: string, newPassword: string) => 
    request.put(`/api/system/users/${id}/reset-password`, { 
      newPassword 
    }),
};
```

#### 6. Form Validation

```typescript
import { z } from 'zod';

// Define schema
const createUserSchema = z.object({
  username: z.string().min(3).max(50),
  email: z.string().email(),
  password: z.string().min(8),
  confirmPassword: z.string().min(8),
});

// Infer type
export type CreateUserFormData = z.infer<typeof createUserSchema>;

// In component
const formSchema = z
  .object({
    password: z.string().min(8),
    confirmPassword: z.string().min(8),
  })
  .superRefine((data, ctx) => {
    if (data.password !== data.confirmPassword) {
      ctx.addIssue({
        path: ['confirmPassword'],
        code: 'custom',
        message: 'Passwords do not match',
      });
    }
  });
```

---

## Git Workflow

### Branching Strategy

```
main
├── release/1.0.0
│   └── hotfix/fix-login-bug
├── develop
│   ├── feature/user-management
│   ├── feature/role-management
│   └── feature/menu-management
```

### Branch Types

| Branch Type | Purpose | Naming Convention |
|-------------|---------|-------------------|
| `main` | Production-ready code | `main` |
| `develop` | Integration branch | `develop` |
| `feature` | New features | `feature/<feature-name>` |
| `hotfix` | Production bug fixes | `hotfix/<issue-number>` |
| `release` | Release preparation | `release/<version>` |

### Commit Message Convention

```
<type>(<scope>): <subject>

<body>

<footer>
```

#### Types

- `feat`: New feature
- `fix`: Bug fix
- `docs`: Documentation changes
- `style`: Code style changes (formatting, etc.)
- `refactor`: Code refactoring
- `test`: Adding or updating tests
- `chore`: Build process or auxiliary tool changes

#### Examples

```bash
# Feature
git commit -m "feat(users): add user creation endpoint

- Add POST /api/system/users endpoint
- Implement user validation
- Add response formatting"

# Bug fix
git commit -m "fix(auth): resolve JWT token refresh issue

- Fix token refresh endpoint
- Update refresh token validation
- Add expiration check"

# Documentation
git commit -m "docs(readme): update installation instructions

- Add missing dependency installation steps
- Include environment variable examples"

# Refactoring
git commit -m "refactor(db): simplify database connection pool

- Extract connection logic
- Add connection state tracking
- Remove unnecessary error handling"
```

### Git Workflow

```bash
# 1. Update main branch
git checkout main
git pull origin main

# 2. Create branch
git checkout -b feature/user-management

# 3. Make changes, commit
git add .
git commit -m "feat(users): add user CRUD operations"

# 4. Push branch
git push origin feature/user-management

# 5. Create Pull Request (PR)
# Go to GitHub and create PR from feature branch

# 6. After PR approval and merge
git checkout main
git pull origin main

# 7. Delete feature branch (optional)
git branch -d feature/user-management
git push origin --delete feature/user-management
```

### Pull Request Checklist

Before creating a PR:

- [ ] Code follows project conventions
- [ ] Code is properly formatted
- [ ] Tests are added/updated
- [ ] Documentation is updated
- [ ] Changes are tested locally
- [ ] No hardcoded secrets
- [ ] Error handling is appropriate
- [ ] Logging is added for important operations

---

## Pull Request Process

### 1. Create Pull Request

```bash
# Push your branch
git push origin feature/user-management

# Go to GitHub and create PR
# Or use GitHub CLI
gh pr create --title "feat(users): add user CRUD operations" \
             --body "Implement user CRUD operations with validation"
```

### 2. PR Template

```markdown
## Description
Brief description of changes

## Type of Change
- [ ] Bug fix
- [ ] New feature
- [ ] Breaking change
- [ ] Documentation update

## Checklist
- [ ] Code follows project conventions
- [ ] Tests are added/updated
- [ ] Documentation is updated
- [ ] Code is formatted
- [ ] Linting passes

## Related Issues
Fixes #123
Related to #456

## Testing
Describe how this was tested
```

### 3. Review Process

#### Reviewer Checklist

- [ ] Code is readable and maintainable
- [ ]Follows project conventions
- [ ] Tests are comprehensive
- [ ] Error handling is appropriate
- [ ] Performance implications considered
- [ ] Security concerns addressed

#### Review Comments

```markdown
# Review Comments

## LGTM 🎉
This looks good! Just a couple of suggestions:

- Add more tests for edge cases
- Update API documentation

## Changes Requested 🔄
Please address the following:

- [ ] Add input validation for user input
- [ ] Handle edge case where user doesn't exist
- [ ] Add logging for audit trail
```

### 4. Merge Strategy

- **Squash Merge**: For feature branches (preferred)
- **Merge Commit**: For hotfixes
- **Rebase Merge**: For cleanup commits

```bash
# Squash merge via GitHub UI
# Or via CLI
gh pr merge --squash
```

---

## Code Review Guidelines

### 1. Review Focus

| Priority | Focus Area |
|----------|-----------|
| 1 | **Correctness** |
| 2 | **Security** |
| 3 | **Performance** |
| 4 | **Readability** |
| 5 | **Consistency** |

### 2. Review Scope

#### Backend Review

- [ ] Code follows Rust API guidelines
- [ ] Error handling is comprehensive
- [ ] Logging is appropriate
- [ ] Database queries are optimized
- [ ] Middleware is properly configured
- [ ] Authentication/Authorization is correct
- [ ] Input validation is in place

#### Frontend Review

- [ ] Component is reusable
- [ ] State management is correct
- [ ] API integration is proper
- [ ] Types are well-defined
- [ ] Testing coverage is adequate
- [ ] Performance is acceptable

### 3. Review Timeframes

| Severity | Review Timeframe |
|----------|-----------------|
| Critical | < 4 hours |
| High | < 24 hours |
| Medium | < 48 hours |
| Low | < 72 hours |

### 4. Approval Requirements

- **Bug fixes**: 1 reviewer approval
- **Features**: 2 reviewer approvals
- **Breaking changes**: 3 reviewer approvals
- **Security fixes**: 2 reviewer approvals (one must be senior)

---

## Release Process

### 1. Release Branch

```bash
# Create release branch
git checkout -b release/1.0.0

# Update version in Cargo.toml and package.json
# Update CHANGELOG.md
# Update documentation

# Push release branch
git push origin release/1.0.0
```

### 2. Release Testing

```bash
# Run all tests
cargo test
npm test

# Build backend
cargo build --release

# Build frontend
npm run build

# Run integration tests
# Test all API endpoints
# Test UI functionality
```

### 3. Version Bumping

#### Backend (Cargo.toml)

```toml
[package]
name = "kao-backend"
version = "1.0.0"  # Update this
edition = "2021"
```

#### Frontend (package.json)

```json
{
  "name": "kao-frontend",
  "version": "1.0.0",
  ...
}
```

### 4. Release Tag

```bash
# Create tag
git tag -a v1.0.0 -m "Version 1.0.0"

# Push tag
git push origin v1.0.0
```

### 5. Release Notes

```markdown
# Version 1.0.0 (2024-01-01)

## Features
- User management CRUD operations
- Role management with permissions
- Menu management with tree structure
- Department management with hierarchy
- JWT authentication
- RBAC access control

## Bug Fixes
- Fixed JWT token validation issue
- Fixed CORS configuration
- Fixed database connection pooling

## Breaking Changes
- None

## Migration Guide
See MIGRATION.md for upgrade instructions.
```

### 6. Post-Release

```bash
# Merge release to main
git checkout main
git merge release/1.0.0

# Merge release to develop
git checkout develop
git merge release/1.0.0

# Delete release branch
git branch -d release/1.0.0
```

---

## Next Steps

After completing this guide:

1. Read [Architecture Documentation](./architecture.md)
2. Set up your development environment
3. Run the project locally
4.熟悉 the codebase structure
5. Start contributing!

---

## Additional Resources

- [Rust Book](https://doc.rust-lang.org/book/)
- [Rust API Guidelines](https://rust-lang.github.io/api-guidelines/)
- [React Documentation](https://react.dev/)
- [TypeScript Handbook](https://www.typescriptlang.org/docs/handbook/)
- [Axum Documentation](https://docs.rs/axum/)
- [SQLx Documentation](https://docs.rs/sqlx/)
