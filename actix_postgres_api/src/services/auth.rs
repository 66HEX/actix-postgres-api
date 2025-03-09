use sqlx::postgres::PgPool;

use crate::error::AppError;
use crate::models::{LoginRequest, UserResponse};
use crate::database::user::UserRepository;
use crate::auth_utils::validate_email;

pub struct AuthService {
    repo: UserRepository,
}

impl AuthService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repo: UserRepository::new(pool),
        }
    }

    pub async fn login(&self, login: LoginRequest) -> Result<UserResponse, AppError> {
        // Walidacja danych logowania
        validate_email(&login.email)?;
        
        if login.password.is_empty() {
            return Err(AppError::ValidationError("Password cannot be empty".to_string()));
        }
        
        // Authenticate user
        let user = self.repo.authenticate(login).await?;
        
        // Return user response
        Ok(UserResponse::from(user))
    }
}