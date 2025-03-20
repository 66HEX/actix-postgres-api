use crate::error::AppError;
use crate::models::appointment::{Appointment, AppointmentWithNames, CreateAppointmentRequest, UpdateAppointmentRequest};
use crate::models::role::UserRole;
use crate::database::AppointmentRepository;
use sqlx::{postgres::PgPool, types::Uuid};

pub struct AppointmentService {
    repository: AppointmentRepository,
}

impl AppointmentService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repository: AppointmentRepository::new(pool),
        }
    }
    
    pub async fn get_all_appointments(&self) -> Result<Vec<AppointmentWithNames>, AppError> {
        self.repository.find_all_with_names().await
    }
    
    pub async fn get_appointment_by_id(&self, id: &str) -> Result<AppointmentWithNames, AppError> {
        let appointment_id = Uuid::parse_str(id)
            .map_err(|_| AppError::BadRequest("Invalid appointment ID format".to_string()))?;
            
        self.repository.find_by_id_with_names(appointment_id).await
    }
    
    pub async fn get_client_appointments(&self, client_id: &str) -> Result<Vec<AppointmentWithNames>, AppError> {
        let user_id = Uuid::parse_str(client_id)
            .map_err(|_| AppError::BadRequest("Invalid user ID format".to_string()))?;
            
        self.repository.find_by_client_id_with_names(user_id).await
    }
    
    pub async fn get_trainer_appointments(&self, trainer_id: &str) -> Result<Vec<Appointment>, AppError> {
        let user_id = Uuid::parse_str(trainer_id)
            .map_err(|_| AppError::BadRequest("Invalid user ID format".to_string()))?;
            
        self.repository.find_by_trainer_id(user_id).await
    }
    
    pub async fn create_appointment(
        &self, 
        client_id: &str, 
        appointment: CreateAppointmentRequest,
        db_pool: &PgPool
    ) -> Result<Appointment, AppError> {
        // Validate client ID
        let client_uuid = Uuid::parse_str(client_id)
            .map_err(|_| AppError::BadRequest("Invalid client ID format".to_string()))?;
        
        // Verify that the trainer exists and has the trainer role
        let trainer_id = appointment.trainer_id;
        let user_repo = crate::database::UserRepository::new(db_pool.clone());
        let trainer = user_repo.find_by_id(trainer_id).await?;
        
        // Check if the trainer has the correct role
        if trainer.role != UserRole::Trainer.to_string() {
            return Err(AppError::BadRequest("Selected user is not a trainer".to_string()));
        }
        
        // Create the appointment
        self.repository.create(client_uuid, appointment).await
    }
    
    pub async fn create_appointment_by_trainer(
        &self, 
        trainer_id: &str, 
        client_id: &str,
        appointment: CreateAppointmentRequest,
        db_pool: &PgPool
    ) -> Result<Appointment, AppError> {
        // Validate trainer ID
        let trainer_uuid = Uuid::parse_str(trainer_id)
            .map_err(|_| AppError::BadRequest("Invalid trainer ID format".to_string()))?;
        
        // Validate client ID
        let client_uuid = Uuid::parse_str(client_id)
            .map_err(|_| AppError::BadRequest("Invalid client ID format".to_string()))?;
        
        // Create a modified appointment with the correct trainer ID
        let modified_appointment = CreateAppointmentRequest {
            trainer_id: trainer_uuid,
            type_: appointment.type_,
            appointment_date: appointment.appointment_date,
            start_time: appointment.start_time,
            duration_minutes: appointment.duration_minutes,
            location: appointment.location,
        };
        
        // Create the appointment
        self.repository.create(client_uuid, modified_appointment).await
    }
    
    pub async fn update_appointment(
        &self,
        id: &str,
        appointment: UpdateAppointmentRequest
    ) -> Result<Appointment, AppError> {
        let appointment_id = Uuid::parse_str(id)
            .map_err(|_| AppError::BadRequest("Invalid appointment ID format".to_string()))?;
            
        self.repository.update(appointment_id, appointment).await
    }
    
    pub async fn delete_appointment(&self, id: &str) -> Result<(), AppError> {
        let appointment_id = Uuid::parse_str(id)
            .map_err(|_| AppError::BadRequest("Invalid appointment ID format".to_string()))?;
            
        self.repository.delete(appointment_id).await
    }
}