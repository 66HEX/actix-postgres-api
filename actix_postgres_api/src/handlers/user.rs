use actix_web::{web, HttpResponse};
use sqlx::{postgres::PgPool, types::Uuid};
use uuid::Uuid as UuidTrait;

use crate::error::AppError;
use crate::models::{CreateUserRequest, UpdateUserRequest, UserResponse};
use crate::repository::UserRepository;
use crate::auth_utils::{validate_password, validate_email, validate_phone_number, validate_username, validate_full_name, validate_role};

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
    // Walidacja nazwy użytkownika
    validate_username(&user.username)?;
    
    // Walidacja adresu email
    validate_email(&user.email)?;
    
    // Walidacja pełnego imienia i nazwiska
    validate_full_name(&user.full_name)?;
    
    // Walidacja hasła
    validate_password(&user.password)?;
    
    // Validate phone number if provided
    if let Some(ref phone) = user.phone_number {
        validate_phone_number(phone)?;
    }
    
    // Walidacja roli, jeśli podano
    let role = match &user.role {
        Some(role) => Some(validate_role(role)?),
        None => None
    };
    
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
    
    // Jeśli walidacja roli zmieniła jej wartość, stwórz nowy obiekt z zaktualizowaną rolą
    let mut user_data = user.into_inner();
    if let Some(validated_role) = role {
        user_data.role = Some(validated_role);
    }
    
    let created_user = repo.create(user_data).await?;
    
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
        validate_email(email)?;
        
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
        validate_username(username)?;
        
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
        validate_full_name(full_name)?;
    }
    
    // Walidacja hasła, jeśli jest aktualizowane
    if let Some(ref password) = user.password {
        validate_password(password)?;
    }
    
    // Validate phone number if provided
    if let Some(ref phone) = user.phone_number {
        validate_phone_number(phone)?;
    }
    
    // Walidacja roli, jeśli jest aktualizowana
    let mut user_data = user.into_inner();
    if let Some(ref role) = user_data.role {
        user_data.role = Some(validate_role(role)?); 
    }
    
    let repo = UserRepository::new(db_pool.get_ref().clone());
    let updated_user = repo.update(user_id, user_data).await?;
    
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

// Handler do filtrowania użytkowników wg roli
pub async fn get_users_by_role(
    role: web::Path<String>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    let repo = UserRepository::new(db_pool.get_ref().clone());
    let users = repo.find_by_role(&role).await?;
    
    let response: Vec<UserResponse> = 
        users.into_iter().map(UserResponse::from).collect();
    
    Ok(HttpResponse::Ok().json(response))
}