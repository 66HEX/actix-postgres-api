use actix_web::{web, HttpResponse, HttpRequest, get, post, put, delete};
use crate::middleware::auth_middleware::Auth;
use crate::models::role::UserRole;
use crate::error::AppError;
use sqlx::postgres::PgPool;

use crate::models::{CreateAppointmentRequest, UpdateAppointmentRequest, AppointmentResponse};
use crate::services::AppointmentService;

#[get("/appointments")]
pub async fn get_all_appointments(
    req: HttpRequest,
    db_pool: web::Data<PgPool>
) -> Result<HttpResponse, AppError> {
    // Only admins can see all appointments
    Auth::validate_request(&req, UserRole::Admin)?;
    
    let service = AppointmentService::new(db_pool.get_ref().clone());
    let appointments = service.get_all_appointments().await?;
    let response: Vec<AppointmentResponse> = appointments.into_iter().map(AppointmentResponse::from).collect();
    
    Ok(HttpResponse::Ok().json(response))
}

#[get("/appointments/{id}")]
pub async fn get_appointment_by_id(
    req: HttpRequest,
    id: web::Path<String>,
    db_pool: web::Data<PgPool>
) -> Result<HttpResponse, AppError> {
    // Get the user ID from the token
    let user_id = Auth::extract_user_id(&req)?;
    
    // Get the appointment
    let service = AppointmentService::new(db_pool.get_ref().clone());
    let appointment = service.get_appointment_by_id(&id).await?;
    
    // Check if the user is authorized to view this appointment
    // Allow if user is the client, the trainer, or an admin
    let user_role = Auth::extract_role(&req)?;
    let is_client = appointment.client_id.to_string() == user_id;
    let is_trainer = appointment.trainer_id.to_string() == user_id;
    
    if !is_client && !is_trainer && user_role != UserRole::Admin {
        return Err(AppError::Forbidden("You are not authorized to view this appointment".to_string()));
    }
    
    Ok(HttpResponse::Ok().json(AppointmentResponse::from(appointment)))
}

#[get("/users/{id}/client-appointments")]
pub async fn get_client_appointments(
    req: HttpRequest,
    id: web::Path<String>,
    db_pool: web::Data<PgPool>
) -> Result<HttpResponse, AppError> {
    // Get the user ID from the token
    let user_id = Auth::extract_user_id(&req)?;
    let requested_id = id.to_string();
    
    // If not the same user, require admin role
    if user_id != requested_id {
        Auth::validate_request(&req, UserRole::Admin)?;
    }
    
    let service = AppointmentService::new(db_pool.get_ref().clone());
    let appointments = service.get_client_appointments(&requested_id).await?;
    let response: Vec<AppointmentResponse> = appointments.into_iter().map(AppointmentResponse::from).collect();
    
    Ok(HttpResponse::Ok().json(response))
}

#[get("/users/{id}/trainer-appointments")]
pub async fn get_trainer_appointments(
    req: HttpRequest,
    id: web::Path<String>,
    db_pool: web::Data<PgPool>
) -> Result<HttpResponse, AppError> {
    // Get the user ID from the token
    let user_id = Auth::extract_user_id(&req)?;
    let requested_id = id.to_string();
    
    // If not the same user, require admin role
    if user_id != requested_id {
        Auth::validate_request(&req, UserRole::Admin)?;
    }
    
    let service = AppointmentService::new(db_pool.get_ref().clone());
    let appointments = service.get_trainer_appointments(&requested_id).await?;
    let response: Vec<AppointmentResponse> = appointments.into_iter().map(AppointmentResponse::from).collect();
    
    Ok(HttpResponse::Ok().json(response))
}

#[post("/appointments")]
pub async fn create_appointment(
    req: HttpRequest,
    appointment: web::Json<CreateAppointmentRequest>,
    db_pool: web::Data<PgPool>
) -> Result<HttpResponse, AppError> {
    // Only clients can create appointments
    Auth::validate_request(&req, UserRole::Client)?;
    
    // Get the client ID from the token
    let client_id = Auth::extract_user_id(&req)?;
    
    let service = AppointmentService::new(db_pool.get_ref().clone());
    let created_appointment = service.create_appointment(
        &client_id, 
        appointment.into_inner(),
        db_pool.get_ref()
    ).await?;
    
    Ok(HttpResponse::Created().json(AppointmentResponse::from(created_appointment)))
}

#[put("/appointments/{id}")]
pub async fn update_appointment(
    req: HttpRequest,
    id: web::Path<String>,
    appointment: web::Json<UpdateAppointmentRequest>,
    db_pool: web::Data<PgPool>
) -> Result<HttpResponse, AppError> {
    // Get the user ID from the token
    let user_id = Auth::extract_user_id(&req)?;
    let user_role = Auth::extract_role(&req)?;
    
    // Get the appointment to check ownership
    let service = AppointmentService::new(db_pool.get_ref().clone());
    let existing_appointment = service.get_appointment_by_id(&id).await?;
    
    // Check if the user is authorized to update this appointment
    // Allow if user is the client, the trainer, or an admin
    let is_client = existing_appointment.client_id.to_string() == user_id;
    let is_trainer = existing_appointment.trainer_id.to_string() == user_id;
    
    if !is_client && !is_trainer && user_role != UserRole::Admin {
        return Err(AppError::Forbidden("You are not authorized to update this appointment".to_string()));
    }
    
    // If trying to change status to completed, only trainer or admin can do that
    if let Some(status) = &appointment.status {
        if status == "completed" && !is_trainer && user_role != UserRole::Admin {
            return Err(AppError::Forbidden("Only trainers or admins can mark appointments as completed".to_string()));
        }
    }
    
    let updated_appointment = service.update_appointment(&id, appointment.into_inner()).await?;
    
    Ok(HttpResponse::Ok().json(AppointmentResponse::from(updated_appointment)))
}

#[delete("/appointments/{id}")]
pub async fn delete_appointment(
    req: HttpRequest,
    id: web::Path<String>,
    db_pool: web::Data<PgPool>
) -> Result<HttpResponse, AppError> {
    // Get the user ID from the token
    let user_id = Auth::extract_user_id(&req)?;
    let user_role = Auth::extract_role(&req)?;
    
    // Get the appointment to check ownership
    let service = AppointmentService::new(db_pool.get_ref().clone());
    let existing_appointment = service.get_appointment_by_id(&id).await?;
    
    // Check if the user is authorized to delete this appointment
    // Allow if user is the client, or an admin
    let is_client = existing_appointment.client_id.to_string() == user_id;
    
    if !is_client && user_role != UserRole::Admin {
        return Err(AppError::Forbidden("You are not authorized to delete this appointment".to_string()));
    }
    
    service.delete_appointment(&id).await?;
    
    Ok(HttpResponse::NoContent().finish())
}

// Function to configure and register all appointment routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        get_all_appointments
    )
    .service(get_appointment_by_id)
    .service(get_client_appointments)
    .service(get_trainer_appointments)
    .service(create_appointment)
    .service(update_appointment)
    .service(delete_appointment);
}