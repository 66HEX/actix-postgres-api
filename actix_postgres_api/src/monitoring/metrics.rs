use prometheus::{
    register_histogram_vec, register_int_counter_vec, register_int_gauge,
    HistogramVec, IntCounterVec, IntGauge,
};
use std::time::Instant;

// Lazy static to hold our metrics
lazy_static::lazy_static! {
    pub static ref HTTP_REQUEST_COUNTER: IntCounterVec = register_int_counter_vec!(
        "api_http_requests_total",
        "Total number of HTTP requests",
        &["method", "path", "status"]
    ).unwrap();

    pub static ref HTTP_REQUEST_DURATION: HistogramVec = register_histogram_vec!(
        "api_http_request_duration_seconds",
        "HTTP request duration in seconds",
        &["method", "path", "status"],
        vec![0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5, 5.0, 10.0]
    ).unwrap();

    pub static ref DB_QUERY_COUNTER: IntCounterVec = register_int_counter_vec!(
        "api_db_queries_total",
        "Total number of database queries",
        &["operation", "table"]
    ).unwrap();

    pub static ref DB_QUERY_DURATION: HistogramVec = register_histogram_vec!(
        "api_db_query_duration_seconds",
        "Database query duration in seconds",
        &["operation", "table"],
        vec![0.001, 0.005, 0.01, 0.025, 0.05, 0.1, 0.25, 0.5, 1.0, 2.5]
    ).unwrap();

    pub static ref ACTIVE_CONNECTIONS: IntGauge = register_int_gauge!(
        "api_active_connections",
        "Number of active connections"
    ).unwrap();
}

// Timer utility for tracking durations
pub struct Timer {
    start: Instant,
}

impl Timer {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
        }
    }

    pub fn elapsed_seconds(&self) -> f64 {
        self.start.elapsed().as_secs_f64()
    }
}

impl Default for Timer {
    fn default() -> Self {
        Self::new()
    }
}

// Database operations tracker
pub struct DbMetrics;

impl DbMetrics {
    pub fn track<F, T>(operation: &str, table: &str, f: F) -> T
    where
        F: FnOnce() -> T,
    {
        DB_QUERY_COUNTER.with_label_values(&[operation, table]).inc();
        let timer = Timer::new();
        let result = f();
        let duration = timer.elapsed_seconds();
        DB_QUERY_DURATION
            .with_label_values(&[operation, table])
            .observe(duration);
        result
    }
}