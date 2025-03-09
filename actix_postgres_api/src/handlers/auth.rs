use actix_web::{web, HttpResponse};
use sqlx::postgres::PgPool;

use crate::error::AppError;
use crate::models::{LoginRequest, LoginResponse};
use crate::services::AuthService;

pub async fn login(
    login: web::Json<LoginRequest>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    let service = AuthService::new(db_pool.get_ref().clone());
    
    // Authenticate user
    let (user, token) = service.login(login.into_inner()).await?;
    
    // Create success response
    let response = LoginResponse {
        user: user,
        token: token,
        message: "Login successful".to_string(),
    };
    
    Ok(HttpResponse::Ok().json(response))
}