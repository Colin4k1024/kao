# Connection Pool Sizing Guide

## Overview

This guide explains how to size PostgreSQL connection pools for horizontal scaling.

---

## Connection Pool Parameters

### Key Parameters

| Parameter | Default | Description |
|-----------|---------|-------------|
| `max_connections` | 10 | Maximum number of connections in the pool |
| `min_connections` | 2 | Minimum number of connections maintained |
| `connect_timeout` | 30 | Maximum time to establish a connection (seconds) |
| `idle_timeout` | 600 | Maximum time for idle connections (seconds) |

### Configuration Example

```rust
pub struct DatabaseSettings {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout: u64,
    pub idle_timeout: u64,
}
```

---

## Sizing Formulas

### Formula 1: Simple Rule

```
max_connections = number_of_instances * connections_per_instance

Example with 3 instances:
- connections_per_instance: 10
- max_connections: 3 * 10 = 30
```

### Formula 2: PostgreSQL Limit

```
 PostgreSQL max_connections: 100-200 (depends on RAM)
 
 per_instance_max = min(10, PostgreSQL_max / number_of_instances)
```

### Formula 3: Thread-Based

```
max_connections = threads_per_instance * connections_per_thread

For Axum (tokio):
- threads_per_instance: number of CPU cores
- connections_per_thread: 2-4
```

---

## Scaling Calculator

### Single Instance

| Scenario | max_connections | min_connections | Use Case |
|----------|----------------|-----------------|----------|
| Development | 5 | 1 | Local development |
| Small Project | 10 | 2 | Small production |
| Medium Project | 20 | 5 | Medium production |
| Large Project | 50 | 10 | Large production |

### Multiple Instances

| Instances | max_connections_per_instance | Total max | Use Case |
|-----------|------------------------------|-----------|----------|
| 1 | 10 | 10 | Single instance |
| 2 | 10 | 20 | High availability |
| 3 | 10 | 30 | Multi-region |
| 5 | 10 | 50 | Large scale |
| 10 | 10 | 100 | Very large scale |

---

## Database Configuration

### PostgreSQL Settings

```sql
-- Check current connections
SELECT count(*) FROM pg_stat_connection;

-- Check connection limits
SHOW max_connections;
SHOW superuser_reserved_connections;

-- Adjust if needed
ALTER SYSTEM SET max_connections = 200;
ALTER SYSTEM SET superuser_reserved_connections = 3;
```

### pg_hba.conf

```conf
# Allow connections from all instances
host    all             all             0.0.0.0/0               md5
```

---

## Connection Pool Metrics

### Active Connections

```rust
pub async fn get_pool_stats(pool: &PgPool) -> PoolStats {
    let state = pool.state();
    
    PoolStats {
        active: state.connections() - state.idle(),
        idle: state.idle(),
        total: state.connections(),
    }
}
```

### Connection Wait Time

```rust
use std::time::Instant;

pub async fn execute_with_timing<F, T>(pool: &PgPool, query: F) -> Result<T, sqlx::Error>
where
    F: FnOnce(&mut sqlx::PgConnection) -> futures::Future<Output = Result<T, sqlx::Error>> + Send + Sync,
{
    let start = Instant::now();
    let result = query(&mut pool.acquire().await?).await?;
    let duration = start.elapsed();
    
    tracing::info!("Query executed in {:?}", duration);
    
    Ok(result)
}
```

### Connection Pool Hit Rate

```rust
pub async fn get_pool_hit_rate(pool: &PgPool, queries_executed: u64) -> f64 {
    if queries_executed == 0 {
        return 100.0;
    }
    
    let total_acquires = pool.state().connections();
    let hit_rate = (queries_executed as f64 / total_acquires as f64) * 100.0;
    
    hit_rate.min(100.0)
}
```

---

## Monitoring Dashboard

### Prometheus Metrics

```rust
#[derive(Serialize)]
pub struct PoolMetrics {
    pub active_connections: u32,
    pub idle_connections: u32,
    pub total_connections: u32,
    pub wait_time_seconds: f64,
    pub connection_rate: f64,
    pub hit_rate: f64,
}

pub async fn collect_pool_metrics(pool: &PgPool) -> PoolMetrics {
    let state = pool.state();
    let now = Utc::now();
    
    PoolMetrics {
        active_connections: state.connections() - state.idle(),
        idle_connections: state.idle(),
        total_connections: state.connections(),
        wait_time_seconds: 0.0, // Track actual wait time
        connection_rate: 0.0,   // Track connection rate
        hit_rate: 100.0,        // Track hit rate
    }
}
```

### Grafana Dashboard

```json
{
  "panels": [
    {
      "title": "Active Connections",
      "type": "graph",
      "query": "pool_active_connections"
    },
    {
      "title": "Idle Connections",
      "type": "graph",
      "query": "pool_idle_connections"
    },
    {
      "title": "Connection Wait Time",
      "type": "graph",
      "query": "pool_wait_time_seconds"
    },
    {
      "title": "Connection Pool Hit Rate",
      "type": "graph",
      "query": "pool_hit_rate"
    }
  ]
}
```

---

## Connection Pool Health Check

### Health Check Endpoint

```rust
pub async fn pool_health_check(pool: &PgPool) -> Result<bool, String> {
    // Try to acquire a connection
    let conn = pool.acquire().await;
    
    match conn {
        Ok(_) => Ok(true),
        Err(e) => Err(format!("Failed to acquire connection: {}", e)),
    }
}
```

### Liveness Probe

```rust
#[derive(Serialize)]
pub struct LivenessResponse {
    pub status: String,
    pub timestamp: String,
}

pub async fn liveness() -> Json<LivenessResponse> {
    Json(LivenessResponse {
        status: "alive".to_string(),
        timestamp: Utc::now().to_rfc3339(),
    })
}
```

### Readiness Probe

```rust
#[derive(Serialize)]
pub struct ReadinessResponse {
    pub status: String,
    pub timestamp: String,
    pub database: String,
    pub pool_size: u32,
}

pub async fn readiness(pool: web::Data<PgPool>) -> Json<ReadinessResponse> {
    let pool_stats = pool.state();
    let total_connections = pool_stats.connections();
    
    Json(ReadinessResponse {
        status: if total_connections > 0 { "ready".to_string() } else { "not_ready".to_string() },
        timestamp: Utc::now().to_rfc3339(),
        database: "connected".to_string(),
        pool_size: total_connections,
    })
}
```

---

## Troubleshooting

### Common Issues

#### 1. Connection Pool Exhausted

**Symptoms**: "Too many connections" errors

**Solution**:
```rust
// Increase max connections
pub struct DatabaseSettings {
    pub max_connections: 50, // Increase from 10
    // ...
}

// Or add more instances
// reduce connections_per_instance
```

#### 2. Connection Timeout

**Symptoms**: "Connection timeout" errors

**Solution**:
```rust
// Increase timeout
pub struct DatabaseSettings {
    pub connect_timeout: 60, // Increase from 30
    // ...
}
```

#### 3. Idle Connection Timeout

**Symptoms**: "Connection closed" errors after inactivity

**Solution**:
```rust
// Decrease idle timeout
pub struct DatabaseSettings {
    pub idle_timeout: 300, // Decrease from 600
    // ...
}
```

#### 4. Slow Queries

**Symptoms**: High wait times

**Solution**:
```rust
// Add indexes
CREATE INDEX idx_user_username ON sys_users(username);
CREATE INDEX idx_user_email ON sys_users(email);

// Optimize queries
// Use EXPLAIN ANALYZE to identify slow queries
```

---

## Checklist

### Before Scaling

- [ ] Connection pool size is configured
- [ ] Database max_connections is sufficient
- [ ] Connection timeout is reasonable
- [ ] Idle timeout is set correctly
- [ ] Health check is working

### During Scaling

- [ ] Monitor connection pool metrics
- [ ] Check for connection leaks
- [ ] Verify connection reuse
- [ ] Track query performance

### After Scaling

- [ ] Verify performance targets met
- [ ] Check connection pool hit rate
- [ ] Review and tune settings
- [ ] Document scaling configuration

---

## References

- [SQLx Connection Pooling](https://docs.rs/sqlx/latest/sqlx/pg/struct.PgPool.html)
- [PostgreSQL Connection Limits](https://www.postgresql.org/docs/current/runtime-config-connection.html)
- [Performance Tuning](https://www.postgresql.org/docs/current/monitoring.html)
