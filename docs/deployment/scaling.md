# Horizontal Scaling Guide

## Overview

This guide explains how to horizontally scale the Kao admin management system from a single instance to multiple backend instances behind a load balancer.

---

## Architecture

### Single Instance (Current)

```
┌─────────────────────────────────────────────────────────────────┐
│                        Frontend (SPA)                          │
│                   React 18 + Vite + Ant Design                  │
│                   Port: 80 (Production)                         │
└─────────────────────────────────────────────────────────────────┘
                                 │
                                 ▼
┌─────────────────────────────────────────────────────────────────┐
│                      Backend (API)                              │
│                   Rust + Axum + Tokio                           │
│                   Port: 8080 (Production)                       │
└─────────────────────────────────────────────────────────────────┘
                                 │
                                 ▼
┌─────────────────────────────────────────────────────────────────┐
│                       PostgreSQL Database                        │
│                     Port: 5432 (Internal)                       │
└─────────────────────────────────────────────────────────────────┘
```

### Horizontal Scaling (Multiple Instances)

```
┌─────────────────────────────────────────────────────────────────┐
│                        Frontend (SPA)                          │
│                   React 18 + Vite + Ant Design                  │
│                   Port: 80 (Production)                         │
└─────────────────────────────────────────────────────────────────┘
                                 │
                                 ▼
┌─────────────────────────────────────────────────────────────────┐
│                   Load Balancer                                 │
│              (Nginx / AWS ELB / GCP LB)                        │
│              Sticky Sessions: Enabled                          │
└─────────────────────────────────────────────────────────────────┘
                                 │
        ┌────────────────────────┼────────────────────────┐
        ▼                        ▼                        ▼
┌─────────────────────┐  ┌─────────────────────┐  ┌─────────────────────┐
│  Backend Instance 1 │  │  Backend Instance 2 │  │  Backend Instance 3 │
│    Rust + Axum      │  │    Rust + Axum      │  │    Rust + Axum      │
│    Port: 8081       │  │    Port: 8082       │  │    Port: 8083       │
└─────────────────────┘  └─────────────────────┘  └─────────────────────┘
        │                        │                        │
        └────────────────────────┼────────────────────────┘
                                 ▼
┌─────────────────────────────────────────────────────────────────┐
│                       PostgreSQL Database                        │
│                     Port: 5432 (Internal)                       │
│                         Connection Pool                         │
└─────────────────────────────────────────────────────────────────┘
```

---

## Stateless Authentication

### JWT Token Distribution

All backend instances share the same JWT secret and can validate tokens independently without database lookup.

```rust
// All instances can validate tokens using only:
// 1. JWT secret (shared via environment variable)
// 2. Token revocation list stored in Redis (optional)

let claims = validate_token_without_db(token, &jwt_secret)?;
```

### Token Refresh

Tokens can be refreshed without database lookup:

```rust
// Refresh token generates new access token from JWT claims only
let new_token = refresh_token(refresh_token, &jwt_secret, access_expires_in)?;
```

### Token Revocation (Optional)

For token revocation, use Redis-backed revocation list:

```rust
// Configure Redis for token revocation
let validator = TokenValidator::new(
    Some("redis://localhost:6379"),
    jwt_secret
)?;

// Validate token with revocation check
let claims = validator.validate_token(&token).await?;

// Revoke token
validator.revoke_token(&token, Duration::from_secs(3600)).await?;
```

---

## Load Balancer Configuration

### Nginx Configuration

Create `/etc/nginx/sites-available/kao`:

```nginx
upstream kao_backend {
    server 127.0.0.1:8081;
    server 127.0.0.1:8082;
    server 127.0.0.1:8083;
}

server {
    listen 80;
    server_name your-domain.com;
    
    # Enable sticky sessions
    set $sticky_session "";
    if ($http_cookie ~* "lb_session=") {
        set $sticky_session $cookie_lb_session;
    }
    
    location /api {
        # Sticky session support
        set_by_lua_block $backend {
            local cookie = ngx.var.cookie_lb_session
            if cookie then
                local _, _, instance_id = string.find(cookie, '"instance_id":"([^"]+)"')
                if instance_id then
                    if instance_id == "instance-1" then
                        return "127.0.0.1:8081"
                    elseif instance_id == "instance-2" then
                        return "127.0.0.1:8082"
                    elseif instance_id == "instance-3" then
                        return "127.0.0.1:8083"
                    end
                end
            end
            return "kao_backend"
        }
        
        proxy_pass http://$backend;
        proxy_set_header Host $host;
        proxy_set_header X-Real-IP $remote_addr;
        proxy_set_header X-Forwarded-For $proxy_add_x_forwarded_for;
        proxy_set_header X-Forwarded-Proto $scheme;
        proxy_set_header X-Request-ID $request_id;
        
        # Add load balancer header
        add_header X-Backend-Instance "instance-1" always;
    }
    
    location / {
        proxy_pass http://localhost:3000;
        proxy_set_header Host $host;
    }
}
```

### AWS Elastic Load Balancer

For AWS ELB, configure:

1. **Listener**: Port 80/443 → Target Group
2. **Target Group**: Health check on `/api/health`
3. **Sticky Sessions**: Enable with duration (e.g., 24 hours)

### Google Cloud Load Balancer

For GCP HTTP(S) Load Balancer:

1. **Backend Service**: Create with multiple instances
2. **Health Check**: `/api/health` endpoint
3. **Session Affinity**: CLIENT_IP or HTTP_COOKIE

---

## Database Connection Pool

### Connection Pool Configuration

Each backend instance should configure connection pooling:

```rust
// config/settings.rs
pub struct DatabaseSettings {
    pub url: String,
    pub max_connections: u32,      // Max connections per instance
    pub min_connections: u32,      // Min connections per instance
    pub connect_timeout: u64,      // Connection timeout in seconds
    pub idle_timeout: u64,         // Idle connection timeout
}

// Recommended settings for N instances:
// - max_connections: 10 * N (total max connections across all instances)
// - min_connections: 5 * N (minimum connections maintained)
// - connect_timeout: 30
// - idle_timeout: 600
```

### Scaling Formula

```
Total Max Connections = max_connections_per_instance * number_of_instances

Example with 3 instances:
- max_connections: 10 * 3 = 30
- min_connections: 5 * 3 = 15
```

---

## Cache Strategy

### Redis Configuration

For distributed caching across multiple instances:

```env
REDIS_URL=redis://localhost:6379
CACHE_ENABLED=true
CACHE_TTL=3600
```

### Cache Keys

```rust
// User cache
cache_user(&user_id, &user_data).await?;

// Menu tree cache
cache_menu_tree(&menu_tree).await?;

// Role list cache
cache_role_list(&roles).await?;
```

---

## Health Check

Each instance should expose a health check endpoint:

```rust
// routes/health.rs
pub async fn health_check() -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "status": "healthy",
        "version": env!("CARGO_PKG_VERSION"),
        "timestamp": chrono::Utc::now().to_rfc3339(),
        "dependencies": {
            "database": "healthy",
            "cache": "healthy"
        }
    }))
}
```

---

## Monitoring

### Metrics Endpoint

Each instance exposes metrics at `/metrics`:

```rust
// routes/metrics.rs
pub async fn get_metrics() -> Response {
    let metrics = collect_metrics();
    let prometheus_format = metrics.to_prometheus_format();
    // Return in Prometheus format
}
```

### Prometheus Configuration

```yaml
scrape_configs:
  - job_name: 'kao-backend'
    static_configs:
      - targets: ['localhost:8081', 'localhost:8082', 'localhost:8083']
    metrics_path: /metrics
```

---

## Scaling Checklist

### Before Scaling

- [ ] JWT secret is consistent across all instances
- [ ] Database connection pool is configured
- [ ] Redis is configured for caching (optional)
- [ ] Health check endpoint is available
- [ ] Logging is centralized

### Scaling Out

1. Deploy additional backend instances
2. Configure load balancer with sticky sessions
3. Update database connection pool settings
4. Monitor health and performance
5. Adjust instance count based on load

### Scaling In

1. Remove instances from load balancer
2. Wait for connections to drain
3. Verify remaining instances can handle load
4. Remove instance

---

## Load Testing

Run load tests to validate scaling:

```bash
# Run load test for 60 seconds, 100 requests/second, 1000 total requests
bash scripts/load-test.sh -t 60 -r 100 -n 1000
```

### Load Test Scenarios

1. **Authentication Load**
   - Login rate: 100 requests/second
   - Expected response time: < 100ms

2. **API Load**
   -GET /api/system/users: 200 requests/second
   - Expected response time: < 50ms

3. **JWT Validation Load**
   - Token validation rate: 500 requests/second
   - Expected response time: < 10ms

---

## Troubleshooting

### Common Issues

#### 1. Token Validation Failures

**Problem**: Tokens created on one instance are not valid on another.

**Solution**: Ensure all instances use the same JWT secret.

```env
# All instances must have identical JWT_SECRET
JWT_SECRET=your-super-secret-key
```

#### 2. Sticky Session Failures

**Problem**: Users get logged out randomly.

**Solution**: Increase sticky session duration or disable sticky sessions.

```nginx
# Increase sticky session duration
add_header Set-Cookie "lb_session=...; Max-Age=86400";
```

#### 3. Database Connection Exhaustion

**Problem**: "Too many connections" errors.

**Solution**: Increase database pool size or add more instances.

```rust
// Increase pool size per instance
pub struct DatabaseSettings {
    pub max_connections: 20, // Increase from 10
    // ...
}
```

---

## Deployment

### Docker Compose for Multiple Instances

```yaml
version: '3.8'

services:
  backend-1:
    build: ./backend
    environment:
      - INSTANCE_ID=instance-1
      - APP_PORT=8081
    ports:
      - "8081:8081"
    depends_on:
      - postgres
    deploy:
      replicas: 1

  backend-2:
    build: ./backend
    environment:
      - INSTANCE_ID=instance-2
      - APP_PORT=8082
    ports:
      - "8082:8082"
    depends_on:
      - postgres
    deploy:
      replicas: 1

  backend-3:
    build: ./backend
    environment:
      - INSTANCE_ID=instance-3
      - APP_PORT=8083
    ports:
      - "8083:8083"
    depends_on:
      - postgres
    deploy:
      replicas: 1

  nginx:
    image: nginx:alpine
    ports:
      - "80:80"
    volumes:
      - ./nginx.conf:/etc/nginx/nginx.conf:ro
    depends_on:
      - backend-1
      - backend-2
      - backend-3
```

---

## Next Steps

1. Deploy at least 2 backend instances
2. Configure load balancer
3. Test with load testing script
4. Monitor metrics and adjust
