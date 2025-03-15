// Export database components
pub mod user;
pub mod connection;
pub mod chat;
pub mod appointment;

// Re-export database components for easier imports
// These are exported to provide a cleaner API for other modules
#[allow(unused_imports)]
pub use connection::DatabasePool;
#[allow(unused_imports)]
pub use user::UserRepository;
#[allow(unused_imports)]
pub use chat::ChatRepository;

// Re-export repositories
pub use appointment::AppointmentRepository;