use chrono::{DateTime, NaiveDate, NaiveTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;

#[derive(Debug, Serialize, Deserialize, sqlx::FromRow)]
pub struct Appointment {
    pub id: Uuid,
    pub client_id: Uuid,
    pub trainer_id: Uuid,
    pub type_: String,
    pub appointment_date: NaiveDate,
    pub start_time: NaiveTime,
    pub duration_minutes: i32,
    pub status: String,
    pub location: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateAppointmentRequest {
    pub trainer_id: Uuid,
    pub type_: String,
    pub appointment_date: NaiveDate,
    pub start_time: NaiveTime,
    pub duration_minutes: i32,
    pub location: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateAppointmentRequest {
    pub type_: Option<String>,
    pub appointment_date: Option<NaiveDate>,
    pub start_time: Option<NaiveTime>,
    pub duration_minutes: Option<i32>,
    pub status: Option<String>,
    pub location: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct AppointmentResponse {
    pub id: Uuid,
    pub client_id: Uuid,
    pub trainer_id: Uuid,
    pub type_: String,
    pub appointment_date: NaiveDate,
    pub start_time: NaiveTime,
    pub duration_minutes: i32,
    pub status: String,
    pub location: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl From<Appointment> for AppointmentResponse {
    fn from(appointment: Appointment) -> Self {
        Self {
            id: appointment.id,
            client_id: appointment.client_id,
            trainer_id: appointment.trainer_id,
            type_: appointment.type_,
            appointment_date: appointment.appointment_date,
            start_time: appointment.start_time,
            duration_minutes: appointment.duration_minutes,
            status: appointment.status,
            location: appointment.location,
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
    Canceled,
    NoShow,
}

impl ToString for AppointmentStatus {
    fn to_string(&self) -> String {
        match self {
            AppointmentStatus::Scheduled => "scheduled".to_string(),
            AppointmentStatus::Completed => "completed".to_string(),
            AppointmentStatus::Canceled => "canceled".to_string(),
            AppointmentStatus::NoShow => "no-show".to_string(),
        }
    }
}

impl From<&str> for AppointmentStatus {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "completed" => AppointmentStatus::Completed,
            "canceled" => AppointmentStatus::Canceled,
            "cancelled" => AppointmentStatus::Canceled, // For backward compatibility
            "no-show" => AppointmentStatus::NoShow,
            _ => AppointmentStatus::Scheduled, // Default is Scheduled
        }
    }
}

// Enum for appointment type
#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum AppointmentType {
    Training,
    CheckIn,
    Consultation,
    Assessment,
}

impl ToString for AppointmentType {
    fn to_string(&self) -> String {
        match self {
            AppointmentType::Training => "training".to_string(),
            AppointmentType::CheckIn => "check-in".to_string(),
            AppointmentType::Consultation => "consultation".to_string(),
            AppointmentType::Assessment => "assessment".to_string(),
        }
    }
}

impl From<&str> for AppointmentType {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "check-in" => AppointmentType::CheckIn,
            "consultation" => AppointmentType::Consultation,
            "assessment" => AppointmentType::Assessment,
            _ => AppointmentType::Training, // Default is Training
        }
    }
}