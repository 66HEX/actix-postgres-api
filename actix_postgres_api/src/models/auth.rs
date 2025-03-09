use serde::{Deserialize, Serialize};
use super::user::UserResponse;

// Model uwierzytelniania
#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct LoginResponse {
    pub user: UserResponse,
    pub message: String,
}