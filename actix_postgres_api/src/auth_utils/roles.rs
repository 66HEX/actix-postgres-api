use crate::error::AppError;

// Funkcja walidująca rolę użytkownika
pub fn validate_role(role: &str) -> Result<String, AppError> {
    match role.to_lowercase().as_str() {
        "client" | "trainer" | "admin" => Ok(role.to_lowercase()),
        _ => Err(AppError::ValidationError("Invalid role. Must be 'client', 'trainer', or 'admin'".to_string()))
    }
}