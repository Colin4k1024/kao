# Kao Deployment Guide

Enterprise Admin Management System Deployment Documentation

## Table of Contents
- [Overview](#overview)
- [Prerequisites](#prerequisites)
- [Development Environment](#development-environment)
- [Production Environment](#production-environment)
- [Docker Deployment](#docker-deployment)
- [Environment Configuration](#environment-configuration)
- [Database Migration](#database-migration)
- [Health Check](#health-check)
- [Troubleshooting](#troubleshooting)

---

## Overview

Kao is an enterprise-grade admin management system built with Rust backend and React frontend. This guide covers deployment options for development, production, and containerized environments.

### Architecture

```
┌─────────────────────────────────────────────────────────────────┐
│                        External Network                        │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                        Frontend (SPA)                          │
│                   React 18 + Vite + Ant Design                  │
│                   Port: 80 (Production)                         │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                       Backend (API)                             │
│                     Rust + Axum + Tokio                         │
│                   Port: 8080 (Production)                       │
└─────────────────────────────────────────────────────────────────┘
                                │
                                ▼
┌─────────────────────────────────────────────────────────────────┐
│                       PostgreSQL Database                        │
│                     Port: 5432 (Internal)                       │
└─────────────────────────────────────────────────────────────────┘
```

---

## Prerequisites

### Required Software

- **Rust**: 1.70 or later
  ```bash
  curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
  rustc --version  # Should be 1.70 or later
  ```

- **Node.js**: 18 or later
  ```bash
  curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
  sudo apt-get install -y nodejs
  node --version  # Should be 18 or later
  ```

- **PostgreSQL**: 14 or later
  ```bash
  sudo apt-get update
  sudo apt-get install -y postgresql postgresql-contrib
  psql --version  # Should be 14 or later
  ```

- **Docker** (Optional, for containerized deployment)
  ```bash
  curl -fsSL https://get.docker.com -o get-docker.sh
  sudo sh get-docker.sh
  docker --version
  docker-compose --version
  ```

### Development Tools

- **Git**: Version control
  ```bash
  git --version
  ```

- **Make**: Build automation
  ```bash
  make --version
  ```

---

## Development Environment

### Quick Start

1. **Clone the repository**
   ```bash
   git clone https://github.com/kao-admin/kao.git
   cd kao
   ```

2. **Configure environment**
   ```bash
   # Copy environment template
   cp .env.example .env
   
   # Edit .env with your configuration
   vim .env
   ```

3. **Set up database**
   ```bash
   # Create database
   createdb kao_db
   
   # Run migrations
   psql -U postgres -d kao_db -f backend/migrations/0001_create_sys_department.sql
   psql -U postgres -d kao_db -f backend/migrations/0002_create_sys_post.sql
   psql -U postgres -d kao_db -f backend/migrations/0003_create_sys_user.sql
   psql -U postgres -d kao_db -f backend/migrations/0004_create_sys_role.sql
   psql -U postgres -d kao_db -f backend/migrations/0005_create_sys_menu.sql
   psql -U postgres -d kao_db -f backend/migrations/0006_create_sys_user_role.sql
   psql -U postgres -d kao_db -f backend/migrations/0007_create_sys_role_menu.sql
   psql -U postgres -d kao_db -f backend/migrations/0008_create_sys_role_department.sql
   psql -U postgres -d kao_db -f backend/migrations/0099_init_data.sql
   ```

4. **Start backend**
   ```bash
   cd backend
   cargo run
   ```

5. **Start frontend**
   ```bash
   cd frontend
   npm install
   npm run dev
   ```

6. **Access application**
   - Frontend: http://localhost:3000
   - Backend: http://localhost:8080
   - API Docs: http://localhost:8080/api-docs

### Environment Variables (.env)

```env
# Database Configuration
DATABASE_URL=postgres://postgres:password@localhost:5432/kao_db
DB_PASSWORD=your-password

# JWT Configuration
JWT_SECRET=your-super-secret-key-change-in-production
JWT_ACCESS_TOKEN_EXPIRES_IN=3600
JWT_REFRESH_TOKEN_EXPIRES_IN=604800

# Application Configuration
APP_HOST=0.0.0.0
APP_PORT=8080
RUST_LOG=info

# Frontend
VITE_API_URL=http://localhost:8080
```

---

## Production Environment

### Backend Deployment

#### 1. Build for Release

```bash
# Build optimized backend
cd backend
cargo build --release

#或使用Makefile
make build
```

#### 2. Production Configuration

Create a production `.env`:

```env
# Database Configuration
DATABASE_URL=postgres://kao_user:secure-password@prod-db.example.com:5432/kao_db
DB_PASSWORD=secure-password

# JWT Configuration
JWT_SECRET=super-secure-production-key-min-32-chars
JWT_ACCESS_TOKEN_EXPIRES_IN=3600
JWT_REFRESH_TOKEN_EXPIRES_IN=604800

# Application Configuration
APP_HOST=0.0.0.0
APP_PORT=8080
RUST_LOG=warn

# Security Settings
ALLOWED_ORIGINS=https://your-domain.com,https://www.your-domain.com
```

#### 3. Running as a Service

Create systemd service file `/etc/systemd/system/kao-backend.service`:

```ini
[Unit]
Description=Kao Backend API Server
After=network.target postgresql.service
Documentation=https://github.com/kao-admin/kao

[Service]
Type=simple
User=kao
WorkingDirectory=/opt/kao/backend
ExecStart=/opt/kao/backend/target/release/kao-backend
EnvironmentFile=/opt/kao/backend/.env
Restart=always
RestartSec=10
StandardOutput=journal
StandardError=journal
SyslogIdentifier=kao-backend

[Install]
WantedBy=multi-user.target
```

Enable and start the service:

```bash
sudo systemctl daemon-reexec
sudo systemctl enable kao-backend
sudo systemctl start kao-backend
sudo systemctl status kao-backend
```

#### 4. Viewing Logs

```bash
# View logs
journalctl -u kao-backend -f

# View logs with timestamp
journalctl -u kao-backend --since "1 hour ago" -f
```

### Frontend Deployment

#### 1. Build for Production

```bash
cd frontend
npm install
npm run build

#或使用Makefile
make build-front
```

Build output will be in `frontend/dist/`

#### 2. Configure Nginx

Create Nginx configuration `/etc/nginx/sites-available/kao`:

```nginx
server {
    listen 80;
    server_name your-domain.com;
    return 301 https://$server_name;
}

server {
    listen 443 ssl;
    server_name your-domain.com;
    
    ssl_certificate /etc/ssl/certs/kao.crt;
    ssl_certificate_key /etc/ssl/private/kao.key;
    
    root /opt/kao/frontend/dist;
    index index.html;
    
    # Serve static files
    location / {
        try_files $uri $uri/ /index.html;
    }
    
    # API proxy
    location /api {
        proxy_pass http://localhost:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_connect_timeout 300;
        proxy_send_timeout 300;
        proxy_read_timeout 300;
    }
    
    # Swagger UI
    location /api-docs {
        proxy_pass http://localhost:8080;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
    }
}
```

Enable the configuration:

```bash
sudo ln -s /etc/nginx/sites-available/kao /etc/nginx/sites-enabled/kao
sudo nginx -t
sudo systemctl reload nginx
```

#### 3. Access Application

- Frontend: https://your-domain.com
- Backend API: https://your-domain.com/api
- API Docs: https://your-domain.com/api-docs

---

## Docker Deployment

### Prerequisites

- Docker: 20 or later
- Docker Compose: 2.0 or later

### Quick Start with Docker Compose

1. **Clone and configure**
   ```bash
   git clone https://github.com/kao-admin/kao.git
   cd kao
   cp .env.example .env
   vim .env  # Configure as needed
   ```

2. **Build and start**
   ```bash
   docker-compose up -d
   ```

3. **Check status**
   ```bash
   docker-compose ps
   docker-compose logs -f
   ```

4. **Access applications**
   - Frontend: http://localhost (port 80)
   - Backend API: http://localhost:8080
   - API Docs: http://localhost:8080/api-docs
   - PostgreSQL: localhost:5432

### Common Docker Commands

```bash
# Start services
docker-compose up -d

# Stop services
docker-compose down

# View logs
docker-compose logs -f

# Rebuild images
docker-compose build --no-cache

# Run one-off command
docker-compose run --rm backend bash

# Restart specific service
docker-compose restart backend
docker-compose restart frontend
docker-compose restart postgres
```

### Production Docker Deployment

Create `docker-compose.production.yml`:

```yaml
version: '3.8'

services:
  postgres:
    image: postgres:15-alpine
    container_name: kao-postgres
    environment:
      POSTGRES_DB: kao_db
      POSTGRES_USER: ${POSTGRES_USER}
      POSTGRES_PASSWORD: ${POSTGRES_PASSWORD}
    ports:
      - "5432:5432"
    volumes:
      - postgres_data:/var/lib/postgresql/data
      - ./backend/migrations:/docker-entrypoint-initdb.d
    networks:
      - kao-network
    healthcheck:
      test: ["CMD-SHELL", "pg_isready -U ${POSTGRES_USER}"]
      interval: 10s
      timeout: 5s
      retries: 5

  backend:
    build:
      context: ./backend
      dockerfile: Dockerfile
    container_name: kao-backend
    environment:
      DATABASE_URL: postgres://${POSTGRES_USER}:${POSTGRES_PASSWORD}@postgres:5432/kao_db
      JWT_SECRET: ${JWT_SECRET}
      JWT_ACCESS_TOKEN_EXPIRES_IN: 3600
      JWT_REFRESH_TOKEN_EXPIRES_IN: 604800
      RUST_LOG: warn
      APP_HOST: 0.0.0.0
      APP_PORT: 8080
    ports:
      - "8080:8080"
    depends_on:
      postgres:
        condition: service_healthy
    networks:
      - kao-network
    healthcheck:
      test: ["CMD", "curl", "-f", "http://localhost:8080/health"]
      interval: 30s
      timeout: 3s
      retries: 3

  frontend:
    build:
      context: ./frontend
      dockerfile: Dockerfile
    container_name: kao-frontend
    ports:
      - "80:80"
    depends_on:
      - backend
    networks:
      - kao-network

volumes:
  postgres_data:

networks:
  kao-network:
    driver: bridge
```

Deploy:

```bash
docker-compose -f docker-compose.production.yml up -d
```

---

## Environment Configuration

### Database Configuration

```env
# Database connection string
DATABASE_URL=postgres://user:password@host:port/database

# Alternative configuration
DB_HOST=localhost
DB_PORT=5432
DB_NAME=kao_db
DB_USER=postgres
DB_PASSWORD=your-password
```

### JWT Configuration

```env
# Secret key (minimum 32 characters for production)
JWT_SECRET=your-super-secret-key-change-in-production

# Token expiration times (in seconds)
JWT_ACCESS_TOKEN_EXPIRES_IN=3600           # 1 hour
JWT_REFRESH_TOKEN_EXPIRES_IN=604800        # 7 days
```

### Security Configuration

```env
# CORS allowed origins (comma-separated)
ALLOWED_ORIGINS=https://your-domain.com,https://www.your-domain.com

# Rate limiting
RATE_LIMIT_WINDOW=60
RATE_LIMIT_MAX=100

# Request size limit (bytes)
MAXIMUM_REQUEST_SIZE=10485760  # 10MB
```

### Logging Configuration

```env
# Log level: error, warn, info, debug, trace
RUST_LOG=warn

# Log format: json or text
LOG_FORMAT=json
```

### Frontend Configuration

```env
# Vite environment variables (prefix with VITE_)
VITE_API_URL=http://localhost:8080
VITE_APP_NAME="Kao Admin"
VITE_APP_URL=http://localhost:3000
```

---

## Database Migration

### Running Migrations Manually

```bash
# Run all pending migrations
psql -U postgres -d kao_db -f backend/migrations/0099_init_data.sql

# Or use SQLx migrate tool
sqlx migrate run --database-url postgres://postgres:password@localhost:5432/kao_db
```

### Checking Migration Status

```bash
# Show migration status
sqlx migrate status --database-url postgres://postgres:password@localhost:5432/kao_db

# Revert last migration
sqlx migrate revert --database-url postgres://postgres:password@localhost:5432/kao_db
```

### Creating New Migrations

```bash
# Create new migration
sqlx migrate add --database-url postgres://postgres:password@localhost:5432/kao_db add_new_column_to_users

# This creates backend/migrations/YYYYMMDDHHMMSS_add_new_column_to_users.sql
```

### Verification

After migrations run, verify:

```bash
# Check table structure
psql -U postgres -d kao_db -c "\dt"

# Check data
psql -U postgres -d kao_db -c "SELECT COUNT(*) FROM sys_user;"
```

---

## Health Check

### Backend Health Endpoint

```bash
# Basic health check
curl http://localhost:8080/health

# Detailed health check (with dependency status)
curl http://localhost:8080/health/check
```

Expected response:

```json
{
  "status": "healthy",
  "version": "1.0.0",
  "timestamp": "2024-01-01T00:00:00Z",
  "dependencies": {
    "database": "healthy",
    "cache": "healthy"
  }
}
```

### Docker Health Check

```bash
# Check container health
docker-compose ps

# View health check logs
docker-compose logs backend | grep healthcheck
```

### Monitoring Health

```bash
# Check all services
for service in backend frontend postgres; do
  docker-compose exec $service curl -fs http://localhost:8080/health || echo "$service: UNHEALTHY"
done
```

### Health Check Script

Create `scripts/health-check.sh`:

```bash
#!/bin/bash

echo "Checking backend health..."
curl -fs http://localhost:8080/health
echo ""

echo "Checking database health..."
 docker-compose exec postgres pg_isready -U postgres
echo ""

echo "Checking frontend health..."
curl -fs http://localhost -o /dev/null && echo "Frontend: OK" || echo "Frontend: FAIL"
```

Make executable and run:

```bash
chmod +x scripts/health-check.sh
./scripts/health-check.sh
```

---

## Troubleshooting

### Common Issues

#### 1. Port Already in Use

```bash
# Check what's using the port
lsof -i :8080
# or
netstat -tlnp | grep :8080

# Kill the process
kill -9 <PID>

# Or change port in .env
APP_PORT=8081
```

#### 2. Database Connection Failed

```bash
# Check PostgreSQL is running
systemctl status postgresql

# Or in Docker
docker-compose ps postgres

# Test connection
psql -U postgres -d kao_db -h localhost -c "SELECT 1"

# Restart PostgreSQL
systemctl restart postgresql
# or
docker-compose restart postgres
```

#### 3. JWT Token Validation Failed

```bash
# Check JWT_SECRET is set
grep JWT_SECRET .env

# Check token expiration settings
grep JWT.*EXPIRES .env

# Verify token format
jwt-decode <token>
```

#### 4. Migration Failed

```bash
# Check migration status
sqlx migrate status --database-url $DATABASE_URL

# Revert failed migration
sqlx migrate revert --database-url $DATABASE_URL

# Manually fix and re-run
psql -U postgres -d kao_db -f backend/migrations/<migration-file>.sql
```

#### 5. Frontend Build Failed

```bash
# Clear node_modules
rm -rf frontend/node_modules

# Reinstall dependencies
npm install

# Clean build
npm run clean
npm run build
```

#### 6. Backend Compilation Failed

```bash
# Clean build
cargo clean

# Rebuild
cargo build

# Check Rust version
rustc --version
```

### Logs

```bash
# Backend logs
journalctl -u kao-backend -f

# Docker logs
docker-compose logs -f

# Database logs
docker-compose logs postgres

# Frontend logs
npm run dev 2>&1 | tee frontend.log
```

### Debug Mode

```bash
# Enable debug logging
RUST_LOG=debug cargo run

# Docker debug
docker-compose -e RUST_LOG=debug up
```

### Support

For additional support:

- GitHub Issues: https://github.com/kao-admin/kao/issues
- Documentation: https://kao-admin.com/docs
- Email: support@kao-admin.com

---

## Next Steps

After successful deployment:

1. [Configure SSL/HTTPS](#ssl-configuration)
2. [Set up backup](#backup-configuration)
3. [Configure monitoring](#monitoring-configuration)
4. [Set up CI/CD](#continuous-integration)

### SSL Configuration

See `docs/deployment/SSL.md`

### Backup Configuration

See `docs/deployment/Backup.md`

### Monitoring Configuration

See `docs/deployment/Monitoring.md`

### Continuous Integration

See `docs/deployment/CI-CD.md`
