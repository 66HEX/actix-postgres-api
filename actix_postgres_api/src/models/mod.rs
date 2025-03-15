// Re-export all model components for easier imports
pub use self::user::{User, CreateUserRequest, UpdateUserRequest, UserResponse};
pub use self::auth::{LoginRequest, LoginResponse};
pub use self::statistics::{UserStatistics, UserRoleStatistics};
pub use self::chat::{ChatMessage, ChatMessageResponse, CreateChatMessageRequest, ChatRoom, WsMessage};

// Define submodules
pub mod user;
pub mod auth;
pub mod role;
pub mod chat;
pub mod statistics;
pub mod appointment;

// Re-export models
pub use appointment::{Appointment, CreateAppointmentRequest, UpdateAppointmentRequest, AppointmentResponse, AppointmentStatus};