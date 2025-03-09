// Export all monitoring components
pub mod metrics;
pub mod memory;

// Re-export commonly used items
// HTTP metrics used in middleware and handlers
pub use metrics::{HTTP_REQUEST_COUNTER, HTTP_REQUEST_DURATION, ACTIVE_CONNECTIONS};
// Database metrics for future query monitoring
#[allow(unused_imports)]
pub use metrics::{DB_QUERY_COUNTER, DB_QUERY_DURATION};
pub use metrics::{Timer, DbMetrics};
// Memory metrics for system monitoring
#[allow(unused_imports)]
pub use memory::MEMORY_USAGE;
pub use memory::update_memory_usage;