use regex::Regex;
use crate::error::AppError;

// Nowa funkcja walidacji adresu email
pub fn validate_email(email: &str) -> Result<(), AppError> {
    // Wyrażenie regularne dla walidacji podstawowego formatu email
    let email_regex = Regex::new(r"^[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$")
        .map_err(|_| AppError::InternalServerError("Failed to compile regex".to_string()))?;

    if !email_regex.is_match(email) {
        return Err(AppError::ValidationError("Invalid email format".to_string()));
    }

    // Dodatkowe sprawdzenia
    if email.len() > 100 {
        return Err(AppError::ValidationError("Email is too long (max 100 characters)".to_string()));
    }

    if !email.contains('@') {
        return Err(AppError::ValidationError("Email must contain @ character".to_string()));
    }

    Ok(())
}

// Funkcja walidacji numeru telefonu
pub fn validate_phone_number(phone: &str) -> Result<(), AppError> {
    // Akceptujemy cyfry, spacje, myślniki i znak +
    let phone_regex = Regex::new(r"^[+]?[\d\s-]{6,20}$")
        .map_err(|_| AppError::InternalServerError("Failed to compile regex".to_string()))?;

    if !phone_regex.is_match(phone) {
        return Err(AppError::ValidationError(
            "Invalid phone number format. Use only digits, spaces, hyphens, and optionally a + prefix".to_string()
        ));
    }

    // Sprawdź, czy numer zawiera wystarczającą liczbę cyfr
    let digit_count = phone.chars().filter(|c| c.is_digit(10)).count();
    if digit_count < 6 {
        return Err(AppError::ValidationError(
            "Phone number must contain at least 6 digits".to_string()
        ));
    }

    Ok(())
}

// Funkcja walidacji nazwy użytkownika
pub fn validate_username(username: &str) -> Result<(), AppError> {
    if username.len() < 3 {
        return Err(AppError::ValidationError("Username must be at least 3 characters long".to_string()));
    }

    if username.len() > 50 {
        return Err(AppError::ValidationError("Username is too long (max 50 characters)".to_string()));
    }

    // Dozwolone znaki: litery, cyfry, podkreślniki i kropki
    let username_regex = Regex::new(r"^[a-zA-Z0-9_\.]+$")
        .map_err(|_| AppError::InternalServerError("Failed to compile regex".to_string()))?;

    if !username_regex.is_match(username) {
        return Err(AppError::ValidationError(
            "Username can only contain letters, numbers, underscores and dots".to_string()
        ));
    }

    Ok(())
}

// Funkcja walidacji pełnego imienia i nazwiska
pub fn validate_full_name(full_name: &str) -> Result<(), AppError> {
    if full_name.len() < 2 {
        return Err(AppError::ValidationError("Full name must be at least 2 characters long".to_string()));
    }

    if full_name.len() > 100 {
        return Err(AppError::ValidationError("Full name is too long (max 100 characters)".to_string()));
    }

    // Sprawdź, czy pełne imię zawiera co najmniej dwa człony (imię i nazwisko)
    let name_parts: Vec<&str> = full_name.split_whitespace().collect();
    if name_parts.len() < 2 {
        return Err(AppError::ValidationError("Full name must include both first and last name".to_string()));
    }

    // Dozwolone znaki: litery, spacje, myślniki i apostrofy (np. dla nazwisk typu O'Connor)
    let name_regex = Regex::new(r"^[a-zA-ZąćęłńóśźżĄĆĘŁŃÓŚŹŻ \-\']+$")
        .map_err(|_| AppError::InternalServerError("Failed to compile regex".to_string()))?;

    if !name_regex.is_match(full_name) {
        return Err(AppError::ValidationError(
            "Full name can only contain letters, spaces, hyphens and apostrophes".to_string()
        ));
    }

    Ok(())
}