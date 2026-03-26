# Kao Monitoring Dashboard Documentation

## Overview

This document describes the Grafana dashboards and monitoring configuration for the Kao admin management system.

## Available Dashboards

### 1. Backend Performance Dashboard

**Dashboard ID:** `kao-backend-performance`

**Metrics:**
- Request Rate (requests/sec)
- Error Rate (%)
- P50/P95/P99 Latency
- Active Connections

**Prometheus Queries:**

```promql
# Request Rate
rate(http_requests_total[5m])

# Error Rate
sum(rate(http_errors_total[5m])) / sum(rate(http_requests_total[5m])) * 100

# P50 Latency
histogram_quantile(0.50, sum(rate(http_request_duration_seconds_bucket[5m])) by (le))

# P95 Latency
histogram_quantile(0.95, sum(rate(http_request_duration_seconds_bucket[5m])) by (le))

# P99 Latency
histogram_quantile(0.99, sum(rate(http_request_duration_seconds_bucket[5m])) by (le))

# Active Connections
active_connections
```

**Alert Thresholds:**
- Error Rate: > 5% (Critical)
- P95 Latency: > 500ms (Warning)
- Active Connections: > 20 (Critical)

### 2. Database Metrics Dashboard

**Panel 1: Query Count**
```promql
rate(db_query_duration_seconds_count[5m])
```

**Panel 2: Query Latency**
```promql
histogram_quantile(0.95, sum(rate(db_query_duration_seconds_bucket[5m])) by (le))
```

**Panel 3: Connection Pool Usage**
```promql
database_connections_active / database_connections_total * 100
```

**Panel 4: Slow Queries**
```promql
increase(db_query_duration_seconds_bucket{le="+Inf"}[5m])
```

### 3. Cache Metrics Dashboard

**Panel 1: Cache Hit/Miss Ratio**
```promql
rate(cache_operations_total{type="hit"}[5m]) / rate(cache_operations_total[5m]) * 100
```

**Panel 2: Cache Operations Rate**
```promql
rate(cache_operations_total[5m])
```

### 4. Business Metrics Dashboard

**Panel 1: Login Attempts**
```promql
rate(password_validation_failures_total[5m])
```

**Panel 2: Audit Log Write Latency**
```promql
histogram_quantile(0.95, sum(rate(audit_log_write_latency_seconds_bucket[5m])) by (le))
```

## Alerting Rules

### Critical Alerts

| Alert Name | Metric | Threshold | Duration | Severity |
|-----------|--------|-----------|----------|----------|
| High Error Rate | http_error_rate | > 5% | 5m | Critical |
| Database Pool Exhausted | database_connections_active | == 20 | 1m | Critical |
| High CPU Usage | cpu_usage_percent | > 90% | 1m | Critical |

### Warning Alerts

| Alert Name | Metric | Threshold | Duration | Severity |
|-----------|--------|-----------|----------|----------|
| High Latency | http_request_duration_p95 | > 500ms | 5m | Warning |
| Cache Miss Rate | cache_miss_rate | > 90% | 5m | Warning |
| Password Validation Spike | password_validation_failures_total | > 100 | 5m | Warning |
| High Memory Usage | memory_usage_percent | > 80% | 1m | Warning |

### Info Alerts

| Alert Name | Metric | Threshold | Duration | Severity |
|-----------|--------|-----------|----------|----------|
| Audit Log Write Latency | audit_log_write_latency_p95 | > 1s | 1m | Info |

## Setup Instructions

### 1. Import Dashboard

1. Log in to Grafana
2. Go to **Dashboards** -> **New** -> **Import**
3. Upload `grafana-dashboards.json`
4. Select Prometheus as data source

### 2. Configure Prometheus Scraping

Add to your `prometheus.yml`:

```yaml
scrape_configs:
  - job_name: 'kao-backend'
    static_configs:
      - targets: ['localhost:8080']
    metrics_path: '/system/monitor/metrics'
    scrape_interval: 15s
    scrape_timeout: 10s
```

### 3. Configure Alerting

1. Go to **Settings** -> **Alerting**
2. Add Alertmanager configuration
3. Configure notification channels (email, Slack, etc.)

## Dashboard Customization

### Adding New Panels

1. Go to dashboard **Edit** mode
2. Click **Add Panel**
3. Select metric and configure query
4. Set thresholds for alerting
5. Save dashboard

### Modifying Thresholds

Edit the alert thresholds in the dashboard JSON:

```json
"thresholds": {
  "mode": "absolute",
  "steps": [
    {"color": "green", "value": null},
    {"color": "yellow", "value": 200},
    {"color": "red", "value": 500}
  ]
}
```

## Monitoring Best Practices

1. **Set up alerting on:**
   - Error rate spikes
   - Latency degradation
   - Resource utilization (CPU, memory)
   - Database connection pool exhaustion

2. ** regularly:**
   - Review alert history
   - Adjust thresholds based on traffic patterns
   - Review slow queries

3. **Dashboard maintenance:**
   - Update dashboards when adding new metrics
   - Remove unused panels
   - Add annotations for important events

## Troubleshooting

### Missing Metrics

1. Check Prometheus is scraping the correct endpoint
2. Verify metrics endpoint is accessible: `curl http://localhost:8080/system/monitor/metrics`
3. Check Prometheus logs for scrape errors

### High Latency

1. Check query performance in database dashboard
2. Review cache hit/miss ratio
3. Analyze slow query logs

### Alert Fatigue

1. Review alert thresholds
2. Increase cooldown periods
3. Add alert suppression rules
4. Group similar alerts
