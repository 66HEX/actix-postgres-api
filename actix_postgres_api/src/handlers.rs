use actix_web::{web, HttpResponse};
use sqlx::{postgres::PgPool, types::Uuid};
use uuid::Uuid as UuidTrait;

use crate::error::AppError;
use crate::models::{CreateUserRequest, UpdateUserRequest, UserResponse, LoginRequest, LoginResponse};
use crate::repository::UserRepository;
use crate::auth_utils::validate_password;

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
    
    if user.full_name.is_empty() {
        return Err(AppError::ValidationError("Full name cannot be empty".to_string()));
    }
    
    // Walidacja hasła
    validate_password(&user.password)?;
    
    // Validate phone number if provided (simple check)
    if let Some(ref phone) = user.phone_number {
        if !phone.chars().all(|c| c.is_digit(10) || c == '+' || c == ' ' || c == '-') {
            return Err(AppError::ValidationError("Invalid phone number format".to_string()));
        }
    }
    
    let repo = UserRepository::new(db_pool.get_ref().clone());
    
    // Sprawdź, czy email już istnieje
    let email_exists = sqlx::query!("SELECT COUNT(*) as count FROM users WHERE email = $1", user.email)
        .fetch_one(db_pool.get_ref())
        .await
        .map_err(AppError::DatabaseError)?
        .count
        .unwrap_or(0) > 0;
        
    if email_exists {
        return Err(AppError::ValidationError("Email is already in use".to_string()));
    }
    
    // Sprawdź, czy nazwa użytkownika już istnieje
    let username_exists = sqlx::query!("SELECT COUNT(*) as count FROM users WHERE username = $1", user.username)
        .fetch_one(db_pool.get_ref())
        .await
        .map_err(AppError::DatabaseError)?
        .count
        .unwrap_or(0) > 0;
        
    if username_exists {
        return Err(AppError::ValidationError("Username is already in use".to_string()));
    }
    
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
        
        // Sprawdź, czy nowy email nie koliduje z istniejącym
        let email_exists = sqlx::query!(
            "SELECT COUNT(*) as count FROM users WHERE email = $1 AND id != $2", 
            email, user_id
        )
        .fetch_one(db_pool.get_ref())
        .await
        .map_err(AppError::DatabaseError)?
        .count
        .unwrap_or(0) > 0;
            
        if email_exists {
            return Err(AppError::ValidationError("Email is already in use".to_string()));
        }
    }
    
    if let Some(ref username) = user.username {
        if username.is_empty() {
            return Err(AppError::ValidationError("Username cannot be empty".to_string()));
        }
        
        // Sprawdź, czy nowa nazwa użytkownika nie koliduje z istniejącą
        let username_exists = sqlx::query!(
            "SELECT COUNT(*) as count FROM users WHERE username = $1 AND id != $2", 
            username, user_id
        )
        .fetch_one(db_pool.get_ref())
        .await
        .map_err(AppError::DatabaseError)?
        .count
        .unwrap_or(0) > 0;
            
        if username_exists {
            return Err(AppError::ValidationError("Username is already in use".to_string()));
        }
    }
    
    if let Some(ref full_name) = user.full_name {
        if full_name.is_empty() {
            return Err(AppError::ValidationError("Full name cannot be empty".to_string()));
        }
    }
    
    // Walidacja hasła, jeśli jest aktualizowane
    if let Some(ref password) = user.password {
        validate_password(password)?;
    }
    
    // Validate phone number if provided
    if let Some(ref phone) = user.phone_number {
        if !phone.chars().all(|c| c.is_digit(10) || c == '+' || c == ' ' || c == '-') {
            return Err(AppError::ValidationError("Invalid phone number format".to_string()));
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

pub async fn login(
    login: web::Json<LoginRequest>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
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