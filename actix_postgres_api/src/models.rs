use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

// Enum reprezentujący role użytkowników
#[derive(Debug, Serialize, Deserialize, sqlx::Type, PartialEq)]
#[sqlx(type_name = "varchar", rename_all = "lowercase")]
pub enum UserRole {
    Client,
    Trainer,
}

impl Default for UserRole {
    fn default() -> Self {
        UserRole::Client
    }
}

// Implementacja konwersji z i do stringa dla UserRole
impl ToString for UserRole {
    fn to_string(&self) -> String {
        match self {
            UserRole::Client => "client".to_string(),
            UserRole::Trainer => "trainer".to_string(),
        }
    }
}

impl From<&str> for UserRole {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "trainer" => UserRole::Trainer,
            _ => UserRole::Client, // domyślnie ustawiamy Client
        }
    }
}

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