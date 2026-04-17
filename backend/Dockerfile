# Build stage
FROM rust:1.94 AS builder

WORKDIR /app

# Install build dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    pkg-config \
    libpq-dev \
    libssl-dev \
    && rm -rf /var/lib/apt/lists/*

# Configure cargo rsproxy mirror for reliable downloads in Docker
ENV CARGO_HOME=/app/.cargo
RUN mkdir -p /app/.cargo && \
    echo '[source.crates-io]' > /app/.cargo/config.toml && \
    echo 'replace-with = "ustc"' >> /app/.cargo/config.toml && \
    echo '' >> /app/.cargo/config.toml && \
    echo '[source.ustc]' >> /app/.cargo/config.toml && \
    echo 'registry = "sparse+https://mirrors.ustc.edu.cn/crates.io-index/"' >> /app/.cargo/config.toml && \
    echo '' >> /app/.cargo/config.toml && \
    echo '[net]' >> /app/.cargo/config.toml && \
    echo 'git-fetch-with-cli = true' >> /app/.cargo/config.toml && \
    echo 'retry = 5' >> /app/.cargo/config.toml

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
FROM ubuntu:22.04 AS runtime

WORKDIR /app

# Install runtime dependencies
RUN apt-get update && apt-get install -y --no-install-recommends \
    ca-certificates \
    libpq5 \
    wget \
    && rm -rf /var/lib/apt/lists/* \
    && useradd -m -u 1000 appuser

# Copy binary from builder
COPY --from=builder /app/target/release/kao-backend /usr/local/bin/kao-backend

# Copy entrypoint and migrate script
COPY entrypoint.sh /app/entrypoint.sh
COPY migrate.sh /app/migrate.sh
COPY .env.example /app/.env.example
RUN chmod +x /app/entrypoint.sh /app/migrate.sh

# Copy migrations directory for sqlx
COPY migrations/ /app/migrations/

# Switch to non-root user
USER appuser

# Expose port
EXPOSE 8080

# Health check
HEALTHCHECK --interval=30s --timeout=10s --start-period=15s --retries=3 \
    CMD wget --no-verbose --tries=1 --spider http://localhost:8080/health || exit 1

# Entry point
ENTRYPOINT ["/app/entrypoint.sh"]
CMD ["kao-backend"]
