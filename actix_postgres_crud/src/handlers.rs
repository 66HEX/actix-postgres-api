use actix_web::{web, HttpResponse};
use sqlx::{postgres::PgPool, types::Uuid};
use uuid::Uuid as UuidTrait;

use crate::error::AppError;
use crate::models::{CreateUserRequest, UpdateUserRequest, UserResponse};
use crate::repository::UserRepository;

pub async fn get_all_users(db_pool: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    let repo = UserRepository::new(db_pool.get_ref().clone());
    let users = repo.find_all().await?;
    let response: Vec<UserResponse> = users.into_iter().map(UserResponse::from).collect();
    
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_user_by_id(
    id: web::Path<String>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    let user_id = UuidTrait::parse_str(&id)
        .map_err(|_| AppError::ValidationError("Invalid UUID format".to_string()))?;
    
    let repo = UserRepository::new(db_pool.get_ref().clone());
    let user = repo.find_by_id(user_id).await?;
    
    Ok(HttpResponse::Ok().json(UserResponse::from(user)))
}

pub async fn create_user(
    user: web::Json<CreateUserRequest>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    // Walidacja danych wejściowych
    if user.username.is_empty() {
        return Err(AppError::ValidationError("Username cannot be empty".to_string()));
    }
    
    if user.email.is_empty() || !user.email.contains('@') {
        return Err(AppError::ValidationError("Invalid email address".to_string()));
    }
    
    let repo = UserRepository::new(db_pool.get_ref().clone());
    let created_user = repo.create(user.into_inner()).await?;
    
    Ok(HttpResponse::Created().json(UserResponse::from(created_user)))
}

pub async fn update_user(
    id: web::Path<String>,
    user: web::Json<UpdateUserRequest>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    let user_id = UuidTrait::parse_str(&id)
        .map_err(|_| AppError::ValidationError("Invalid UUID format".to_string()))?;
    
    // Walidacja danych wejściowych
    if let Some(ref email) = user.email {
        if email.is_empty() || !email.contains('@') {
            return Err(AppError::ValidationError("Invalid email address".to_string()));
        }
    }
    
    if let Some(ref username) = user.username {
        if username.is_empty() {
            return Err(AppError::ValidationError("Username cannot be empty".to_string()));
        }
    }
    
    let repo = UserRepository::new(db_pool.get_ref().clone());
    let updated_user = repo.update(user_id, user.into_inner()).await?;
    
    Ok(HttpResponse::Ok().json(UserResponse::from(updated_user)))
}

pub async fn delete_user(
    id: web::Path<String>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    let user_id = UuidTrait::parse_str(&id)
        .map_err(|_| AppError::ValidationError("Invalid UUID format".to_string()))?;
    
    let repo = UserRepository::new(db_pool.get_ref().clone());
    repo.delete(user_id).await?;
    
    Ok(HttpResponse::NoContent().finish())
}