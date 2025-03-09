// Export all handlers modules
pub mod user;
pub mod auth;
pub mod statistics;

// Re-export all handlers for backward compatibility
pub use user::{get_all_users, get_user_by_id, create_user, update_user, delete_user, get_users_by_role};
pub use auth::login;
pub use statistics::get_user_statistics;