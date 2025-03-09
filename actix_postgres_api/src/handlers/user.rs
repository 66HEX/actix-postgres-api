use actix_web::{web, HttpResponse};
use sqlx::postgres::PgPool;

use crate::error::AppError;
use crate::models::{CreateUserRequest, UpdateUserRequest, UserResponse};
use crate::services::UserService;

pub async fn get_all_users(db_pool: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    let service = UserService::new(db_pool.get_ref().clone());
    let users = service.get_all_users().await?;
    let response: Vec<UserResponse> = users.into_iter().map(UserResponse::from).collect();
    
    Ok(HttpResponse::Ok().json(response))
}

pub async fn get_user_by_id(
    id: web::Path<String>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    let service = UserService::new(db_pool.get_ref().clone());
    let user = service.get_user_by_id(&id).await?;
    
    Ok(HttpResponse::Ok().json(UserResponse::from(user)))
}

pub async fn create_user(
    user: web::Json<CreateUserRequest>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    let service = UserService::new(db_pool.get_ref().clone());
    let created_user = service.create_user(user.into_inner(), db_pool.get_ref()).await?;
    
    Ok(HttpResponse::Created().json(UserResponse::from(created_user)))
}

pub async fn update_user(
    id: web::Path<String>,
    user: web::Json<UpdateUserRequest>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    let service = UserService::new(db_pool.get_ref().clone());
    let updated_user = service.update_user(&id, user.into_inner(), db_pool.get_ref()).await?;
    
    Ok(HttpResponse::Ok().json(UserResponse::from(updated_user)))
}

pub async fn delete_user(
    id: web::Path<String>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    let service = UserService::new(db_pool.get_ref().clone());
    service.delete_user(&id).await?;
    
    Ok(HttpResponse::NoContent().finish())
}

// Handler do filtrowania użytkowników wg roli
pub async fn get_users_by_role(
    role: web::Path<String>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    let service = UserService::new(db_pool.get_ref().clone());
    let users = service.get_users_by_role(&role).await?;
    
    let response: Vec<UserResponse> = 
        users.into_iter().map(UserResponse::from).collect();
    
    Ok(HttpResponse::Ok().json(response))
}
