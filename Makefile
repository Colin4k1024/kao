# Kao Development Makefile
# Enterprise Admin Management System

.PHONY: help build build-back build-front run-back run-front test clean migrate lint format

# Default target
help:
	@echo "Kao Development Makefile"
	@echo "========================"
	@echo ""
	@echo "Available commands:"
	@echo "  make build        - Build backend application"
	@echo "  make build-back   - Build backend application"
	@echo "  make build-front  - Build frontend application"
	@echo "  make run          - Run both backend and frontend"
	@echo "  make run-back     - Run backend server"
	@echo "  make run-front    - Run frontend dev server"
	@echo "  make test         - Run all tests"
	@echo "  make test-back    - Run backend tests"
	@echo "  make test-front   - Run frontend tests"
	@echo "  make lint         - Run linter on all code"
	@echo "  make lint-back    - Run backend linter"
	@echo "  make lint-front   - Run frontend linter"
	@echo "  make format       - Format all code"
	@echo "  make doc-back     - Generate backend documentation"
	@echo "  make doc-front    - Generate frontend documentation"
	@echo "  make migrate      - Run database migrations"
	@echo "  make clean        - Clean build artifacts"
	@echo "  make docker       - Build Docker images"
	@echo "  make docker-up    - Start Docker containers"
	@echo "  make docker-down  - Stop Docker containers"
	@echo "  make dev          - Run backend and frontend in dev mode"

# Build targets
build: build-back
	@echo "✅ Backend build complete"

build-back:
	@echo "🔨 Building backend..."
	@cd backend && cargo build --release
	@echo "✅ Backend build complete"

build-front:
	@echo "🔨 Building frontend..."
	@cd frontend && npm run build
	@echo "✅ Frontend build complete"

# Run targets
run: run-back
	@echo "🚀 Kao is running!"

run-back:
	@echo "🚀 Starting backend server..."
	@cd backend && cargo run

run-front:
	@echo "🚀 Starting frontend dev server..."
	@cd frontend && npm run dev

# Test targets
test: test-back test-front
	@echo "✅ All tests passed"

test-back:
	@echo "🧪 Running backend tests..."
	@cd backend && cargo test

test-front:
	@echo "🧪 Running frontend tests..."
	@cd frontend && npm test

# Lint targets
lint: lint-back lint-front
	@echo "✅ Linting complete"

lint-back:
	@echo "Linting backend..."
	@cd backend && cargo clippy --all-targets --all-features -- -D warnings

lint-front:
	@echo "Linting frontend..."
	@cd frontend && npm run lint

# Format targets
format:
	@echo "Formatting code..."
	@cd backend && cargo fmt
	@cd frontend && npm run format
	@echo "✅ Formatting complete"

# Documentation targets
doc-back:
	@echo "📄 Generating backend documentation..."
	@cd backend && cargo doc --open

doc-front:
	@echo "📄 Generating frontend documentation..."
	@cd frontend && npm run doc

# Database targets
migrate:
	@echo "Migration database..."
	@cd backend && psql -U postgres -d kao_db -f migrations/0099_init_data.sql
	@echo "✅ Migration complete"

clean:
	@echo "🧹 Cleaning build artifacts..."
	@cd backend && cargo clean
	@cd frontend && rm -rf node_modules dist
	@echo "✅ Clean complete"

# Docker targets
docker:
	@echo "🐳 Building Docker images..."
	@docker-compose build
	@echo "✅ Docker images built"

docker-up:
	@echo "🐳 Starting Docker containers..."
	@docker-compose up -d
	@echo "✅ Docker containers started"

docker-down:
	@echo "🐳 Stopping Docker containers..."
	@docker-compose down
	@echo "✅ Docker containers stopped"

# Development targets
dev:
	@echo "🚀 Starting development servers..."
	@echo "Backend: http://localhost:8080"
	@echo "Frontend: http://localhost:3000"
	@echo "API Docs: http://localhost:8080/api-docs"
	@make run-back &
	@make run-front

# Health check
health:
	@echo "🏥 Checking system health..."
	@curl -f http://localhost:8080/health && echo "Backend: OK" || echo "Backend: FAIL"
	@node -e "fetch('http://localhost:3000').then(() => console.log('Frontend: OK')).catch(() => console.log('Frontend: FAIL'))"

# Release
release:
	@echo "📦 Creating release..."
	@echo "Please update version numbers first:"
	@echo "  - backend/Cargo.toml"
	@echo "  - frontend/package.json"
	@echo ""
	@echo "Then run:"
	@echo "  git add -A && git commit -m 'chore: release x.x.x'"
	@echo "  git tag -a vx.x.x -m 'Version x.x.x'"
	@echo "  git push && git push --tags"
