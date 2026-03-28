# Phase 3: Production Readiness - Discussion Log

> **Audit trail only.** Do not use as input to planning, research, or execution agents.
> Decisions are captured in CONTEXT.md — this log preserves the alternatives considered.

**Date:** 2026-03-28
**Phase:** 03-production-readiness
**Areas discussed:** Alert Delivery Channels, Metrics Collection Strategy, Log Aggregation Backend, Dashboard Approach

---

## Alert Delivery Channels

| Option | Description | Selected |
|--------|-------------|----------|
| Webhook only (Recommended) | Simple integration point. User configures webhook URL, which can forward to anything (Slack, email, PagerDuty). Minimum complexity. | ✓ |
| Multiple channels | Direct integration with Slack, email, and webhook. More complex but more flexible. | |
| Alertmanager integration | Forward to Prometheus Alertmanager which handles routing. Most powerful but requires Alertmanager setup. | |

**User's choice:** Webhook only (Recommended)
**Notes:** Simple is best for initial implementation. User can configure webhook to forward anywhere.

---

## Metrics Collection Strategy

| Option | Description | Selected |
|--------|-------------|----------|
| Middleware integration (Recommended) | Integrate metrics middleware into app.rs to automatically collect real request metrics (counters, histograms, timing). Real metrics without manual instrumentation. | ✓ |
| Instrumentation per handler | Manually add record_request(), record_request_duration() calls in each handler. More control but more work. | |
| Hybrid approach | Middleware for HTTP metrics, manual for business metrics (DB queries, cache). Balanced approach. | |

**User's choice:** Middleware integration (Recommended)
**Notes:** Automatic collection is most practical. Real metrics out of the box.

---

## Log Aggregation Backend

| Option | Description | Selected |
|--------|-------------|----------|
| Elasticsearch (Recommended) | Full-text search, structured documents, mature ecosystem. Plan says this. Kibana for visualization. | ✓ |
| Loki + file-based | Loki is lighter weight, designed for logs. Use with Promtail agent. Simpler than Elasticsearch. | |
| File-based aggregation only | Just write structured JSON logs to files. Simplest option, query with jq/grep. No centralized search. | |

**User's choice:** Elasticsearch (Recommended)
**Notes:** Matches plan. Kibana integration is valuable for log analysis.

---

## Dashboard Approach

| Option | Description | Selected |
|--------|-------------|----------|
| JSON import (Recommended) | Import the existing JSON into Grafana. Fast, reproducible, version-controlled. User can tweak after import. | ✓ |
| Manual creation | Create dashboards manually in Grafana UI. More interactive but not reproducible. | |
| Both approaches | Provide JSON for reproducibility but also guide for manual creation. Most flexible. | |

**User's choice:** JSON import (Recommended)
**Notes:** Reproducible and fast. Version control is important.

---

## Claude's Discretion

- Alert thresholds: Conservative defaults (5% error rate, 500ms p95 latency)
- Metrics retention: In-memory for Prometheus scrapes
- Dashboard panels: Backend Performance, Database Metrics, Cache Metrics, Business Metrics

## Deferred Ideas

None — discussion stayed within phase scope.

