use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct User {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub password_hash: String,  // Hashed password (never exposed directly)
    pub full_name: String,
    pub phone_number: Option<String>,
    pub active: bool,
    pub role: String,  // Role as string to simplify database interaction
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub email: String,
    pub password: String,  // Plain text password (only used for creation)
    pub full_name: String,
    pub phone_number: Option<String>,
    pub role: Option<String>,  // Optional role - if not provided, default to CLIENT
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateUserRequest {
    pub username: Option<String>,
    pub email: Option<String>,
    pub password: Option<String>,  // Optional password update
    pub full_name: Option<String>,
    pub phone_number: Option<String>,
    pub active: Option<bool>,
    pub role: Option<String>,  // Optional role update
}

#[derive(Debug, Serialize)]
pub struct UserResponse {
    pub id: Uuid,
    pub username: String,
    pub email: String,
    pub full_name: String,
    pub phone_number: Option<String>,
    pub active: bool,
    pub role: String,  // Included in response
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    // Nie zwracamy password_hash w odpowiedzi API
}

impl From<User> for UserResponse {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            username: user.username,
            email: user.email,
            full_name: user.full_name,
            phone_number: user.phone_number,
            active: user.active,
            role: user.role,
            created_at: user.created_at,
            updated_at: user.updated_at,
        }
    }
}