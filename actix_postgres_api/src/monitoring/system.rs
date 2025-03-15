use std::sync::atomic::{AtomicI64, Ordering};
use std::time::{SystemTime, UNIX_EPOCH};
use lazy_static::lazy_static;
use sysinfo::System;

// Store application start time as a timestamp
lazy_static! {
    static ref APP_START_TIME: AtomicI64 = {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_secs() as i64;
        AtomicI64::new(now)
    };
}

/// Get the application uptime in seconds
pub fn get_uptime() -> u64 {
    let start_time = APP_START_TIME.load(Ordering::Relaxed) as u64;
    let current_time = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards")
        .as_secs();
    
    current_time - start_time
}

/// Get the application start time as a Unix timestamp
pub fn get_start_time() -> i64 {
    APP_START_TIME.load(Ordering::Relaxed)
}

/// Get system information including CPU usage, total memory, etc.
pub fn get_system_info() -> SystemInfo {
    let mut sys = System::new_all();
    sys.refresh_all();
    
    let cpu_usage = sys.global_cpu_info().cpu_usage();
    let total_memory = sys.total_memory() * 1024; // Convert to bytes
    let used_memory = sys.used_memory() * 1024; // Convert to bytes
    let total_swap = sys.total_swap() * 1024; // Convert to bytes
    let used_swap = sys.used_swap() * 1024; // Convert to bytes
    
    SystemInfo {
        cpu_usage,
        total_memory,
        used_memory,
        memory_usage_percent: if total_memory > 0 {
            (used_memory as f64 / total_memory as f64) * 100.0
        } else {
            0.0
        },
        total_swap,
        used_swap,
        hostname: System::host_name().unwrap_or_else(|| String::from("unknown")),
        os_name: System::name().unwrap_or_else(|| String::from("unknown")),
        os_version: System::os_version().unwrap_or_else(|| String::from("unknown")),
        kernel_version: System::kernel_version().unwrap_or_else(|| String::from("unknown")),
    }
}

/// System information structure
#[derive(Debug, Clone, serde::Serialize)]
pub struct SystemInfo {
    pub cpu_usage: f32,
    pub total_memory: u64,
    pub used_memory: u64,
    pub memory_usage_percent: f64,
    pub total_swap: u64,
    pub used_swap: u64,
    pub hostname: String,
    pub os_name: String,
    pub os_version: String,
    pub kernel_version: String,
}