use serde::{Deserialize, Serialize};
use sqlx::types::Uuid;
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ChatMessage {
    pub id: Uuid,
    pub sender_id: Uuid,
    pub sender_name: String,
    pub content: String,
    pub room_id: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateChatMessageRequest {
    pub content: String,
    pub room_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatMessageResponse {
    pub id: Uuid,
    pub sender_id: Uuid,
    pub sender_name: String,
    pub content: String,
    pub room_id: String,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ChatRoom {
    pub id: String,
    pub name: String,
    pub created_at: Option<DateTime<Utc>>,
}

// WebSocket message types
#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WsMessage {
    #[serde(rename = "connect")]
    Connect { user_id: Uuid, room_id: String },
    #[serde(rename = "message")]
    Message { content: String, room_id: String },
    #[serde(rename = "disconnect")]
    Disconnect { user_id: Uuid, room_id: String },
}