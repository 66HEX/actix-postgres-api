use actix_web::{web, HttpResponse};
use sqlx::postgres::PgPool;

use crate::error::AppError;
use crate::models::{LoginRequest, LoginResponse, UserResponse};
use crate::repository::UserRepository;
use crate::auth_utils::validate_email;

pub async fn login(
    login: web::Json<LoginRequest>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    // Walidacja danych logowania
    validate_email(&login.email)?;
    
    if login.password.is_empty() {
        return Err(AppError::ValidationError("Password cannot be empty".to_string()));
    }
    
    let repo = UserRepository::new(db_pool.get_ref().clone());
    
    // Authenticate user
    let user = repo.authenticate(login.into_inner()).await?;
    
    // Create success response
    let response = LoginResponse {
        user: UserResponse::from(user),
        message: "Login successful".to_string(),
    };
    
    Ok(HttpResponse::Ok().json(response))
}