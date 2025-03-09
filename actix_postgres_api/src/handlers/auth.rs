use actix_web::{web, HttpResponse};
use sqlx::postgres::PgPool;

use crate::error::AppError;
use crate::models::{LoginRequest, LoginResponse, UserResponse};
use crate::services::AuthService;

pub async fn login(
    login: web::Json<LoginRequest>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    let service = AuthService::new(db_pool.get_ref().clone());
    
    // Authenticate user
    let user = service.login(login.into_inner()).await?;
    
    // Create success response
    let response = LoginResponse {
        user: user,
        message: "Login successful".to_string(),
    };
    
    Ok(HttpResponse::Ok().json(response))
}