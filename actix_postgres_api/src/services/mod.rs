// Export all service modules
pub mod user;
pub mod auth;
pub mod appointment;

// Re-export all services for easier imports
pub use user::UserService;
pub use auth::AuthService;
pub use appointment::AppointmentService;