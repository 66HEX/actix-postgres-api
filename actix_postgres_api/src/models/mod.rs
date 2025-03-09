// Re-export all model components for easier imports
pub use self::user::{User, CreateUserRequest, UpdateUserRequest, UserResponse};
pub use self::auth::{LoginRequest, LoginResponse};
pub use self::role::UserRole;

// Define submodules
mod user;
mod auth;
mod role;