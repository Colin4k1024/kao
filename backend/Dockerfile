# Build stage
FROM rust:1.70-alpine AS builder

WORKDIR /app

# Install build dependencies
RUN apk add --no-cache \
    postgresql-dev \
    openssl-dev \
    pkgconfig \
    cargo

# Copy only Cargo.toml and lockfile for caching
COPY Cargo.toml .
COPY Cargo.lock .

# Create a dummy src to allow dependency caching
RUN mkdir -p src && echo "fn main() {}" > src/main.rs

# Download dependencies and build them
RUN cargo build --release 2>/dev/null || true

# Remove dummy src
RUN rm -rf src

# Copy actual source code
COPY src/ src/

# Build with optimizations
RUN cargo build --release --bin kao-backend

# Runtime stage
FROM alpine:latest AS runtime

WORKDIR /app

# Install runtime dependencies
RUN apk add --no-cache \
    ca-certificates \
    postgresql-libs \
    openssl

# Create non-root user
RUN addgroup -g 1000 app && \
    adduser -u 1000 -G app -s /bin/sh -D appuser

# Copy binary from builder
COPY --from=builder /app/target/release/kao-backend /usr/local/bin/kao-backend

# Copy .env.example
COPY --from=builder /app/.env.example /app/.env.example

# Copy entrypoint and migrate script
COPY --from=builder /app/entrypoint.sh /app/entrypoint.sh
COPY --from=builder /app/migrate.sh /app/migrate.sh
RUN chmod +x /app/entrypoint.sh /app/migrate.sh

# Update ownership
RUN chown -R appuser:app /app

# Switch to non-root user
USER appuser

# Expose port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=5s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:8080/api/health || exit 1

# Entry point
ENTRYPOINT ["/app/entrypoint.sh"]
CMD ["kao-backend"]
