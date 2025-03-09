use sqlx::postgres::PgPool;

use crate::error::AppError;
use crate::models::{LoginRequest, UserResponse};
use crate::database::user::UserRepository;
use crate::auth_utils::validate_email;
use crate::auth_utils::jwt::generate_token;

pub struct AuthService {
    repo: UserRepository,
}

impl AuthService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repo: UserRepository::new(pool),
        }
    }

    pub async fn login(&self, login: LoginRequest) -> Result<(UserResponse, String), AppError> {
        // Walidacja danych logowania
        validate_email(&login.email)?;
        
        if login.password.is_empty() {
            return Err(AppError::ValidationError("Password cannot be empty".to_string()));
        }
        
        // Authenticate user
        let user = self.repo.authenticate(login).await?;
        
        // Generate JWT token
        let token = generate_token(user.id, &user.username, &user.email, &user.role)?;
        
        // Return user response and token
        Ok((UserResponse::from(user), token))
    }
}