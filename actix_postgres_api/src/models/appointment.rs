use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Appointment {
    pub id: Uuid,
    pub client_id: Uuid,
    pub trainer_id: Uuid,
    pub title: String,
    pub appointment_date: NaiveDate,
    pub start_time: NaiveTime,
    pub duration_minutes: i32,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAppointmentRequest {
    pub trainer_id: Uuid,
    pub title: String,
    pub appointment_date: NaiveDate,
    pub start_time: NaiveTime,
    pub duration_minutes: i32,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAppointmentRequest {
    pub title: Option<String>,
    pub appointment_date: Option<NaiveDate>,
    pub start_time: Option<NaiveTime>,
    pub duration_minutes: Option<i32>,
    pub status: Option<String>,
    pub notes: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppointmentResponse {
    pub id: Uuid,
    pub client_id: Uuid,
    pub trainer_id: Uuid,
    pub title: String,
    pub appointment_date: NaiveDate,
    pub start_time: NaiveTime,
    pub duration_minutes: i32,
    pub status: String,
    pub notes: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Appointment> for AppointmentResponse {
    fn from(appointment: Appointment) -> Self {
        Self {
            id: appointment.id,
            client_id: appointment.client_id,
            trainer_id: appointment.trainer_id,
            title: appointment.title,
            appointment_date: appointment.appointment_date,
            start_time: appointment.start_time,
            duration_minutes: appointment.duration_minutes,
            status: appointment.status,
            notes: appointment.notes,
            created_at: appointment.created_at,
            updated_at: appointment.updated_at,
        }
    }
}

// Enum for appointment status
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum AppointmentStatus {
    Scheduled,
    Completed,
    Cancelled,
}

impl ToString for AppointmentStatus {
    fn to_string(&self) -> String {
        match self {
            AppointmentStatus::Scheduled => "scheduled".to_string(),
            AppointmentStatus::Completed => "completed".to_string(),
            AppointmentStatus::Cancelled => "cancelled".to_string(),
        }
    }
}

impl From<&str> for AppointmentStatus {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "completed" => AppointmentStatus::Completed,
            "cancelled" => AppointmentStatus::Cancelled,
            _ => AppointmentStatus::Scheduled, // Default is Scheduled
        }
    }
}