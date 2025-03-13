use jsonwebtoken::{encode, decode, Header, Validation, EncodingKey, DecodingKey};
use serde::{Deserialize, Serialize};
use chrono::{Utc, Duration};
use sqlx::types::Uuid;
use std::env;
use crate::error::AppError;

// Struktura przechowująca dane w tokenie JWT
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,         // Subject (user ID)
    pub name: String,        // User's name
    pub email: String,       // User's email
    pub role: String,        // User's role
    pub exp: i64,            // Expiration time (Unix timestamp)
    pub iat: i64,            // Issued at (Unix timestamp)
}

// Konfiguracja JWT
pub struct JwtConfig {
    secret: String,
    expiration: i64, // czas ważności tokenu w sekundach
}

impl JwtConfig {
    pub fn from_env() -> Self {
        Self {
            secret: env::var("JWT_SECRET").unwrap_or_else(|_| "default_secret_change_in_production".to_string()),
            expiration: env::var("JWT_EXPIRATION")
                .unwrap_or_else(|_| "86400".to_string()) // 24 godziny domyślnie
                .parse()
                .unwrap_or(86400),
        }
    }
}

// Funkcja generująca token JWT
pub fn generate_token(user_id: Uuid, username: &str, email: &str, role: &str) -> Result<String, AppError> {
    let config = JwtConfig::from_env();
    
    let now = Utc::now();
    let expiration = now + Duration::seconds(config.expiration);
    
    let claims = Claims {
        sub: user_id.to_string(),
        name: username.to_string(),
        email: email.to_string(),
        role: role.to_string(),
        exp: expiration.timestamp(),
        iat: now.timestamp(),
    };
    
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(config.secret.as_bytes())
    ).map_err(|e| AppError::InternalServerError(format!("Token generation error: {}", e)))
}

// Funkcja weryfikująca token JWT
pub fn verify_token(token: &str) -> Result<Claims, AppError> {
    let config = JwtConfig::from_env();
    
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(config.secret.as_bytes()),
        &Validation::default()
    )
    .map(|data| data.claims)
    .map_err(|e| match e.kind() {
        jsonwebtoken::errors::ErrorKind::ExpiredSignature => AppError::ValidationError("Token expired".to_string()),
        jsonwebtoken::errors::ErrorKind::InvalidToken => AppError::ValidationError("Invalid token".to_string()),
        _ => AppError::InternalServerError(format!("Token validation error: {}", e)),
    })
}

// Funkcja do wyodrębnienia tokenu z nagłówka Authorization
pub fn extract_token_from_header(auth_header: &str) -> Result<&str, AppError> {
    if !auth_header.starts_with("Bearer ") {
        return Err(AppError::ValidationError("Invalid authorization header format".to_string()));
    }
    
    Ok(&auth_header[7..]) // Pomijamy "Bearer " (7 znaków)
}