// Re-export all model components for easier imports
pub use self::user::{User, CreateUserRequest, UpdateUserRequest, UserResponse};
pub use self::auth::{LoginRequest, LoginResponse};
// UserRole is exported for use in API documentation and future role-based features
pub use self::role::UserRole;
pub use self::statistics::{UserStatistics, UserRoleStatistics};

// Define submodules
mod user;
mod auth;
mod role;
mod statistics;