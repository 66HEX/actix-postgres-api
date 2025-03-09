use actix_web::{web, HttpResponse};
use sqlx::postgres::PgPool;

use crate::error::AppError;
use crate::models::UserStatistics;
use crate::models::UserRoleStatistics;
use crate::services::UserService;

pub async fn get_user_statistics(db_pool: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    let service = UserService::new(db_pool.get_ref().clone());
    
    // Get role statistics
    let role_counts = service.count_users_by_role().await?;
    
    // Get inactive users count
    let inactive_count = service.count_inactive_users().await?;
    
    // Transform role counts into the expected format
    let roles = role_counts
        .into_iter()
        .map(|(role, count)| UserRoleStatistics { role, count })
        .collect();
    
    // Create the statistics response
    let statistics = UserStatistics {
        roles,
        inactive_count,
    };
    
    Ok(HttpResponse::Ok().json(statistics))
}