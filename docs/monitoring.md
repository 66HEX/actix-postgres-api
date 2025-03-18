# Monitoring and Logging

This document provides detailed information about the monitoring and logging features implemented in the User CRUD API.

## Performance Monitoring

The API includes comprehensive performance monitoring capabilities using Prometheus metrics:

- Prometheus metrics accessible at `/metrics` endpoint (admin access only)
- HTTP request timing and throughput metrics
- Database query performance tracking
- Memory usage monitoring
- Active connections counter
- Request/response status code tracking

### Available Metrics

| Metric | Type | Description |
|--------|------|-------------|
| `api_http_requests_total` | Counter | Count of HTTP requests by method, path, and status |
| `api_http_request_duration_seconds` | Histogram | HTTP request duration histograms |
| `api_db_queries_total` | Counter | Count of database operations by type and table |
| `api_db_query_duration_seconds` | Histogram | Database operation duration histograms |
| `api_active_connections` | Gauge | Current number of active HTTP connections |
| `api_memory_usage_bytes` | Gauge | Current memory usage of the application |

## Logging System

The API uses a structured logging system based on the `tracing` crate:

- Log levels configurable via environment variables
- JSON-formatted logs for machine readability
- Request ID tracking across log entries
- Performance impact logging for slow operations
- Error cause chain logging

### Log Configuration

Logging can be configured using the `RUST_LOG` environment variable:

```
RUST_LOG=actix_postgres_api=info,actix_web=info,sqlx=warn
```

### Log Format

Logs are output in a structured JSON format that includes:

- Timestamp
- Log level
- Module path
- Message
- Request ID (for HTTP requests)
- Additional context fields

### Health Check Endpoint

The API provides a health check endpoint at `/health` that returns basic information about the application status:

```json
{
  "status": "up",
  "version": "0.1.0",
  "uptime": 3600,
  "start_time": 1678901234,
  "system_info": {
    "cpu_usage": 5.2,
    "total_memory": 16777216000,
    "used_memory": 8388608000,
    "memory_usage_percent": 50.0,
    "total_swap": 4294967296,
    "used_swap": 1073741824,
    "hostname": "server-name",
    "os_name": "Windows",
    "os_version": "10.0.19045",
    "kernel_version": "10.0.19045.1"
  }
}
```

## Performance Middleware

The API includes custom middleware components for performance monitoring:

- Request timing middleware that logs slow requests
- Database query timing middleware
- Memory usage tracking middleware

These middleware components automatically collect performance metrics and make them available through the `/metrics` endpoint.