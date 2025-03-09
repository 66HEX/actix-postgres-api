use bcrypt::{hash, verify, DEFAULT_COST};
use crate::error::AppError;

/// Generuje hash hasła z użyciem bcrypt
pub fn hash_password(password: &str) -> Result<String, AppError> {
    hash(password, DEFAULT_COST)
        .map_err(|e| AppError::InternalServerError(format!("Hashing error: {}", e)))
}

/// Weryfikuje, czy podane hasło odpowiada hashowi
pub fn verify_password(password: &str, hash: &str) -> Result<bool, AppError> {
    verify(password, hash)
        .map_err(|e| AppError::InternalServerError(format!("Verification error: {}", e)))
}

// Funkcja pomocnicza do walidacji siły hasła
pub fn validate_password(password: &str) -> Result<(), AppError> {
    if password.len() < 8 {
        return Err(AppError::ValidationError("Password must be at least 8 characters long".to_string()));
    }
    
    // Sprawdź, czy hasło zawiera cyfrę
    if !password.chars().any(|c| c.is_digit(10)) {
        return Err(AppError::ValidationError("Password must contain at least one digit".to_string()));
    }
    
    // Sprawdź, czy hasło zawiera dużą literę
    if !password.chars().any(|c| c.is_uppercase()) {
        return Err(AppError::ValidationError("Password must contain at least one uppercase letter".to_string()));
    }
    
    // Sprawdź, czy hasło zawiera małą literę
    if !password.chars().any(|c| c.is_lowercase()) {
        return Err(AppError::ValidationError("Password must contain at least one lowercase letter".to_string()));
    }
    
    Ok(())
}