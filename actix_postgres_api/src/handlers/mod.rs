pub mod user;
pub mod auth;
pub mod statistics;
pub mod oauth;
pub mod chat;
pub mod appointment;

pub use oauth::*;
pub use user::{get_all_users, get_user_by_id, create_user, update_user, delete_user, get_users_by_role};
pub use chat::{ws_connect, get_chat_rooms, get_room_messages, create_chat_room};
pub use auth::login;
pub use statistics::get_user_statistics;

// Re-export handler configuration functions
pub use appointment::configure_routes as configure_appointment_routes;