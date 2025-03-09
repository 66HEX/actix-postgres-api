use prometheus::IntGauge;
use prometheus::register_int_gauge;

// Lazy static to hold memory metrics
lazy_static::lazy_static! {
    pub static ref MEMORY_USAGE: IntGauge = register_int_gauge!(
        "api_memory_usage_bytes",
        "Current memory usage in bytes"
    ).unwrap();
}

// Memory usage updater
pub fn update_memory_usage() {
    let mut sys = sysinfo::System::new_all();
    sys.refresh_all();
    let total_memory_used = sys.used_memory() * 1024; // Convert to bytes
    MEMORY_USAGE.set(total_memory_used as i64);
}