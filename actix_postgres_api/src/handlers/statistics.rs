use actix_web::{web, HttpResponse};
use sqlx::postgres::PgPool;

use crate::error::AppError;
use crate::models::{UserStatistics, UserRoleStatistics};
use crate::models::statistics::RegistrationStatistics;
use crate::services::UserService;

pub async fn get_user_statistics(db_pool: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    let service = UserService::new(db_pool.get_ref().clone());
    
    // Get role statistics
    let role_counts = service.count_users_by_role().await?;
    
    // Get inactive users count
    let inactive_count = service.count_inactive_users().await?;
    
    // Get registration statistics
    let last_24h = service.count_registrations_last_24h().await?;
    let last_7d = service.count_registrations_last_7d().await?;
    let last_30d = service.count_registrations_last_30d().await?;
    
    // Transform role counts into the expected format
    let roles = role_counts
        .into_iter()
        .map(|(role, count)| UserRoleStatistics { role, count })
        .collect();
    
    // Create the registration statistics
    let registration_stats = RegistrationStatistics {
        last_24h,
        last_7d,
        last_30d,
    };
    
    // Create the statistics response
    let statistics = UserStatistics {
        roles,
        inactive_count,
        registration_stats,
    };
    
    Ok(HttpResponse::Ok().json(statistics))
}