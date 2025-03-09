// Re-export all model components for easier imports
pub use self::user::{User, CreateUserRequest, UpdateUserRequest, UserResponse};
pub use self::auth::{LoginRequest, LoginResponse};
pub use self::statistics::{UserStatistics, UserRoleStatistics};
pub use self::role::UserRole;

// Define submodules
pub mod user;
pub mod auth;
pub mod role;
pub mod statistics;