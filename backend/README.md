# Kao Backend - Rust API Server

Enterprise admin management system backend built with Rust and Axum.

## Features

- ✅ RESTful API with comprehensive documentation
- ✅ JWT authentication with refresh tokens
- ✅ RBAC (Role-Based Access Control)
- ✅ Structured logging with tracing
- ✅ Database migrations with SQLx
- ✅ Error handling with thiserror
- ✅ Input validation with Validator
- ✅ OpenAPI/Swagger documentation

## Tech Stack

- **Language**: Rust 1.70+
- **Web Framework**: Axum 0.7
- **Async Runtime**: Tokio
- **Database ORM**: SQLx 0.8
- **Database**: PostgreSQL 14+
- **Authentication**: JWT (jsonwebtoken) + bcrypt
- **Logging**: tracing
- **Validation**: Validator

## Quick Start

### Prerequisites

```bash
# Rust 1.70 or later
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# PostgreSQL 14 or later
sudo apt-get install -y postgresql postgresql-contrib
```

### Setup

```bash
# Clone repository
git clone https://github.com/kao-admin/kao.git
cd kao/backend

# Create environment file
cp .env.example .env
# Edit .env with your database credentials

# Run database migrations
make migrate

# Start backend
cargo run
```

The server will start on `http://localhost:8080`.

## API Endpoints

### Authentication
- `POST /api/auth/login` - User login
- `POST /api/auth/logout` - User logout
- `POST /api/auth/refresh` - Refresh token
- `POST /api/auth/register` - User registration

### User Management
- `GET /api/system/users` - List users
- `GET /api/system/users/:id` - Get user detail
- `POST /api/system/users` - Create user
- `PUT /api/system/users/:id` - Update user
- `DELETE /api/system/users/:id` - Delete user
- `PUT /api/system/users/:id/reset-password` - Reset password
- `PUT /api/system/users/:id/roles` - Assign roles

### Role Management
- `GET /api/system/roles` - List roles
- `GET /api/system/roles/:id` - Get role detail
- `POST /api/system/roles` - Create role
- `PUT /api/system/roles/:id` - Update role
- `DELETE /api/system/roles/:id` - Delete role
- `PUT /api/system/roles/:id/menus` - Assign menus

### Menu Management
- `GET /api/system/menus` - List menus
- `GET /api/system/menus/:id` - Get menu detail
- `POST /api/system/menus` - Create menu
- `PUT /api/system/menus/:id` - Update menu
- `DELETE /api/system/menus/:id` - Delete menu

### Department Management
- `GET /api/system/departments` - List departments
- `GET /api/system/departments/:id` - Get department detail
- `POST /api/system/departments` - Create department
- `PUT /api/system/departments/:id` - Update department
- `DELETE /api/system/departments/:id` - Delete department

### System Operations
- `GET /api/system/jobs` - List scheduled jobs
- `GET /api/system/jobs/logs` - List job logs
- `GET /api/system/oper/logs` - List operation logs
- `GET /api/system/login/logs` - List login logs
- `GET /api/system/online/users` - List online users

### Health Check
- `GET /health` - Basic health check
- `GET /health/check` - Detailed health check with dependencies
- `GET /api-docs` - OpenAPI/Swagger documentation

## Project Structure

```
backend/
├── src/
│   ├── main.rs              # Entry point
│   ├── app/                 # Application setup
│   │   ├── mod.rs
│   │   └── routes.rs
│   ├── config/              # Configuration
│   │   ├── mod.rs
│   │   └── settings.rs
│   ├── db/                  # Database
│   │   ├── mod.rs
│   │   └── pool.rs
│   ├── middleware/          # Axum middleware
│   │   ├── mod.rs
│   │   ├── auth.rs
│   │   ├── cors.rs
│   │   ├── logger.rs
│   │   └── openapi.rs
│   ├── common/              # Common utilities
│   │   ├── mod.rs
│   │   ├── response.rs
│   │   ├── error.rs
│   │   └── auth.rs
│   ├── features/            # Feature modules
│   │   ├── auth/
│   │   ├── users/
│   │   ├── roles/
│   │   ├── menus/
│   │   ├── departments/
│   │   ├── config/
│   │   ├── dictionary/
│   │   ├── notice/
│   │   ├── job/
│   │   └── monitoring/
│   ├── models/              # Data models
│   │   ├── mod.rs
│   │   └── user.rs
│   ├── repositories/        # Repository layer
│   │   ├── mod.rs
│   │   └── user_repo.rs
│   ├── services/            # Business logic
│   │   ├── mod.rs
│   │   └── user_service.rs
│   └── utils/               # Utility functions
│       ├── mod.rs
│       └── validators.rs
├── migrations/              # Database migrations
│   ├── 0001_create_sys_department.sql
│   ├── 0002_create_sys_post.sql
│   ├── 0003_create_sys_user.sql
│   ├── 0004_create_sys_role.sql
│   ├── 0005_create_sys_menu.sql
│   ├── 0006_create_sys_user_role.sql
│   ├── 0007_create_sys_role_menu.sql
│   ├── 0008_create_sys_role_department.sql
│   └── 0099_init_data.sql
├── Cargo.toml               # Dependencies
├── Cargo.lock               # Lock file
└── README.md                # This file
```

## Development

### Build

```bash
# Debug build (fast)
cargo build

# Release build (optimized)
cargo build --release
```

### Run

```bash
# Run in development mode
cargo run

# Run with specific features
cargo run --features dev
```

### Test

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with coverage
cargo tarpaulin
```

### Lint

```bash
# Run clippy (linting)
cargo clippy

# Format code
cargo fmt
```

### Debug

```bash
# Run with debug logging
RUST_LOG=debug cargo run

# Run with specific module logging
RUST_LOG=auth=debug,user=info cargo run
```

## Docker

### Build Image

```bash
docker build -t kao-backend .
```

### Run Container

```bash
docker run -p 8080:8080 --env-file .env kao-backend
```

### Docker Compose

```bash
docker-compose up -d
```

## Configuration

### Environment Variables

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

# CORS Configuration
ALLOWED_ORIGINS=http://localhost:3000,http://localhost:5173
```

### Configuration File

Create `config/config.toml` for additional configuration:

```toml
[database]
url = "postgres://user:password@localhost:5432/kao_db"

[auth]
token_expiry = 3600
refresh_token_expiry = 604800

[server]
host = "0.0.0.0"
port = 8080

[logging]
level = "info"
format = "json"
```

## API Documentation

### OpenAPI/Swagger

The API documentation is available at:
- Development: http://localhost:8080/api-docs
- Production: https://api.kao-admin.com/api-docs

## Common Tasks

### Add New API Endpoint

1. Add route to `src/app/routes.rs`
2. Add handler to feature module
3. Add service in `src/features/*/service.rs`
4. Add repository in `src/features/*/repo.rs`
5. Add database migration
6. Update OpenAPI documentation

### Add New Model

1. Define model in `src/common/models/mod.rs`
2. Add database column mapping
3. Add validation
4. Add serialization

### Add New Middleware

1. Create middleware in `src/middleware/`
2. Implement `tower::Layer` trait
3. Register in `src/middleware/mod.rs`
4. Use in routes

---

## License

MIT License - See LICENSE file for details.

## Contributing

1. Fork the repository
2. Create feature branch (`git checkout -b feature/amazing-feature`)
3. Commit changes (`git commit -m 'feat: add amazing feature'`)
4. Push branch (`git push origin feature/amazing-feature`)
5. Open Pull Request

## Support

- GitHub Issues: https://github.com/kao-admin/kao/issues
- Documentation: https://kao-admin.com/docs
- Email: support@kao-admin.com

---

**Version**: 1.0.0  
**Last Updated**: 2024-01-01
