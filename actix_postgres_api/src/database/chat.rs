use sqlx::{PgPool, Error, postgres::PgQueryResult};
use uuid::Uuid;
use chrono::{DateTime, Utc};

use crate::models::chat::{ChatMessage, ChatRoom};

pub struct ChatRepository;

impl ChatRepository {
    // Get all chat rooms
    pub async fn get_all_rooms(pool: &PgPool) -> Result<Vec<ChatRoom>, Error> {
        sqlx::query_as!(ChatRoom,
            r#"
            SELECT id, name, created_at
            FROM chat_rooms
            ORDER BY name
            "#
        )
        .fetch_all(pool)
        .await
    }

    // Get a specific chat room by ID
    pub async fn get_room_by_id(pool: &PgPool, room_id: &str) -> Result<Option<ChatRoom>, Error> {
        sqlx::query_as!(ChatRoom,
            r#"
            SELECT id, name, created_at
            FROM chat_rooms
            WHERE id = $1
            "#,
            room_id
        )
        .fetch_optional(pool)
        .await
    }

    // Save a chat message to the database
    pub async fn save_message(
        pool: &PgPool,
        sender_id: Uuid,
        sender_name: &str,
        content: &str,
        room_id: &str,
    ) -> Result<ChatMessage, Error> {
        sqlx::query_as!(ChatMessage,
            r#"
            INSERT INTO chat_messages (sender_id, sender_name, content, room_id)
            VALUES ($1, $2, $3, $4)
            RETURNING id, sender_id, sender_name, content, room_id, created_at
            "#,
            sender_id,
            sender_name,
            content,
            room_id
        )
        .fetch_one(pool)
        .await
    }

    // Get recent messages for a specific room
    pub async fn get_recent_messages(
        pool: &PgPool,
        room_id: &str,
        limit: i64
    ) -> Result<Vec<ChatMessage>, Error> {
        sqlx::query_as!(ChatMessage,
            r#"
            SELECT id, sender_id, sender_name, content, room_id, created_at
            FROM chat_messages
            WHERE room_id = $1
            ORDER BY created_at DESC
            LIMIT $2
            "#,
            room_id,
            limit
        )
        .fetch_all(pool)
        .await
    }

    // Create a new chat room
    pub async fn create_room(
        pool: &PgPool,
        id: &str,
        name: &str
    ) -> Result<ChatRoom, Error> {
        sqlx::query_as!(ChatRoom,
            r#"
            INSERT INTO chat_rooms (id, name)
            VALUES ($1, $2)
            RETURNING id, name, created_at
            "#,
            id,
            name
        )
        .fetch_one(pool)
        .await
    }
}