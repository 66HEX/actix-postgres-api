use crate::error::AppError;
use crate::models::{Appointment, CreateAppointmentRequest, UpdateAppointmentRequest, AppointmentStatus, AppointmentType};
use crate::monitoring::DbMetrics;
use crate::logging::create_db_span;
use sqlx::{postgres::PgPool, types::Uuid};
use tracing::Instrument;
use chrono::Utc;

pub struct AppointmentRepository {
    pool: PgPool,
}

impl AppointmentRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_all(&self) -> Result<Vec<Appointment>, AppError> {
        let span = create_db_span(
            "find_all_appointments",
            "SELECT * FROM appointments ORDER BY appointment_date ASC, start_time ASC",
            "None",
        );
        
        DbMetrics::track("SELECT", "appointments", || async {
            let appointments = sqlx::query_as::<_, Appointment>(
                "SELECT * FROM appointments ORDER BY appointment_date ASC, start_time ASC"
            )
            .fetch_all(&self.pool)
            .await
            .map_err(AppError::DatabaseError)?;

            Ok(appointments)
        }).instrument(span).await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<Appointment, AppError> {
        let params = format!("id={}", id);
        let span = create_db_span(
            "find_appointment_by_id",
            "SELECT * FROM appointments WHERE id = $1",
            &params,
        );
        
        DbMetrics::track("SELECT", "appointments", || async {
            let appointment = sqlx::query_as::<_, Appointment>(
                "SELECT * FROM appointments WHERE id = $1"
            )
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(AppError::DatabaseError)?;
            
            appointment.ok_or_else(|| AppError::NotFound(format!("Appointment with id {} not found", id)))
        }).instrument(span).await
    }

    pub async fn find_by_client_id(&self, client_id: Uuid) -> Result<Vec<Appointment>, AppError> {
        let params = format!("client_id={}", client_id);
        let span = create_db_span(
            "find_appointments_by_client_id",
            "SELECT * FROM appointments WHERE client_id = $1 ORDER BY appointment_date ASC, start_time ASC",
            &params,
        );
        
        DbMetrics::track("SELECT", "appointments", || async {
            let appointments = sqlx::query_as::<_, Appointment>(
                "SELECT * FROM appointments WHERE client_id = $1 ORDER BY appointment_date ASC, start_time ASC"
            )
            .bind(client_id)
            .fetch_all(&self.pool)
            .await
            .map_err(AppError::DatabaseError)?;
            
            Ok(appointments)
        }).instrument(span).await
    }

    pub async fn find_by_trainer_id(&self, trainer_id: Uuid) -> Result<Vec<Appointment>, AppError> {
        let params = format!("trainer_id={}", trainer_id);
        let span = create_db_span(
            "find_appointments_by_trainer_id",
            "SELECT * FROM appointments WHERE trainer_id = $1 ORDER BY appointment_date ASC, start_time ASC",
            &params,
        );
        
        DbMetrics::track("SELECT", "appointments", || async {
            let appointments = sqlx::query_as::<_, Appointment>(
                "SELECT * FROM appointments WHERE trainer_id = $1 ORDER BY appointment_date ASC, start_time ASC"
            )
            .bind(trainer_id)
            .fetch_all(&self.pool)
            .await
            .map_err(AppError::DatabaseError)?;
            
            Ok(appointments)
        }).instrument(span).await
    }

    pub async fn create(&self, client_id: Uuid, appointment: CreateAppointmentRequest) -> Result<Appointment, AppError> {
        let span = create_db_span(
            "create_appointment",
            "INSERT INTO appointments (client_id, trainer_id, \"type\", appointment_date, start_time, duration_minutes, location) VALUES ($1, $2, $3, $4, $5, $6, $7) RETURNING *",
            "appointment data",
        );
        
        DbMetrics::track("INSERT", "appointments", || async {
            let new_appointment = sqlx::query_as::<_, Appointment>(
                r#"INSERT INTO appointments 
                   (client_id, trainer_id, "type", appointment_date, start_time, duration_minutes, location) 
                   VALUES ($1, $2, $3, $4, $5, $6, $7) 
                   RETURNING *"#
            )
            .bind(client_id)
            .bind(appointment.trainer_id)
            .bind(&appointment.type_)
            .bind(appointment.appointment_date)
            .bind(appointment.start_time)
            .bind(appointment.duration_minutes)
            .bind(appointment.location)
            .fetch_one(&self.pool)
            .await
            .map_err(AppError::DatabaseError)?;
            
            Ok(new_appointment)
        }).instrument(span).await
    }

    pub async fn update(&self, id: Uuid, appointment: UpdateAppointmentRequest) -> Result<Appointment, AppError> {
        let params = format!("id={}", id);
        let span = create_db_span(
            "update_appointment",
            "UPDATE appointments SET ... WHERE id = $1 RETURNING *",
            &params,
        );
        
        DbMetrics::track("UPDATE", "appointments", || async {
            // First check if appointment exists
            let existing = self.find_by_id(id).await?;
            
            // Build dynamic query based on provided fields
            let mut query_builder = sqlx::QueryBuilder::new(
                "UPDATE appointments SET updated_at = NOW()"
            );
            
            let mut needs_comma = false;
            
            if let Some(type_) = &appointment.type_ {
                // Validate type
                let valid_type = AppointmentType::from(type_.as_str()).to_string();
                
                query_builder.push(", \"type\" = ");
                query_builder.push_bind(valid_type);
                needs_comma = true;
            }
            
            if let Some(date) = appointment.appointment_date {
                if needs_comma { query_builder.push(", "); }
                query_builder.push("appointment_date = ");
                query_builder.push_bind(date);
                needs_comma = true;
            }
            
            if let Some(time) = appointment.start_time {
                if needs_comma { query_builder.push(", "); }
                query_builder.push("start_time = ");
                query_builder.push_bind(time);
                needs_comma = true;
            }
            
            if let Some(duration) = appointment.duration_minutes {
                if needs_comma { query_builder.push(", "); }
                query_builder.push("duration_minutes = ");
                query_builder.push_bind(duration);
                needs_comma = true;
            }
            
            if let Some(status) = &appointment.status {
                // Validate status
                let valid_status = AppointmentStatus::from(status.as_str()).to_string();
                
                if needs_comma { query_builder.push(", "); }
                query_builder.push("status = ");
                query_builder.push_bind(valid_status);
                needs_comma = true;
            }
            
            if let Some(location) = &appointment.location {
                if needs_comma { query_builder.push(", "); }
                query_builder.push("location = ");
                query_builder.push_bind(location);
            }
            
            query_builder.push(" WHERE id = ");
            query_builder.push_bind(id);
            query_builder.push(" RETURNING *");
            
            let query = query_builder.build_query_as::<Appointment>();
            
            let updated_appointment = query
                .fetch_one(&self.pool)
                .await
                .map_err(AppError::DatabaseError)?;
            
            Ok(updated_appointment)
        }).instrument(span).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let params = format!("id={}", id);
        let span = create_db_span(
            "delete_appointment",
            "DELETE FROM appointments WHERE id = $1",
            &params,
        );
        
        DbMetrics::track("DELETE", "appointments", || async {
            // First check if appointment exists
            let _ = self.find_by_id(id).await?;
            
            sqlx::query("DELETE FROM appointments WHERE id = $1")
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(AppError::DatabaseError)?;
            
            Ok(())
        }).instrument(span).await
    }
}