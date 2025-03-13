use actix::{Actor, ActorContext, Addr, AsyncContext, Handler, Message, StreamHandler};
use actix_web::{web, Error, HttpRequest, HttpResponse};
use actix_web_actors::ws;
use serde_json::json;
use sqlx::PgPool;
use std::collections::{HashMap, HashSet};
use std::sync::{Arc, Mutex};
use uuid::Uuid;

use crate::auth_utils::jwt::{extract_token_from_header, verify_token};
use crate::database::ChatRepository;
use crate::error::AppError;
use crate::models::chat::{ChatMessage, ChatMessageResponse, ChatRoom, WsMessage};

// Define the WebSocket connection actor
pub struct ChatWebSocket {
    // User ID of the connected client
    user_id: Uuid,
    // User's display name
    user_name: String,
    // Current chat room
    room_id: Option<String>,
    // Database connection pool
    db_pool: web::Data<PgPool>,
    // Server state with all active connections
    server: Arc<Mutex<ChatServer>>,
}

// Server state to track all active connections
pub struct ChatServer {
    // Map of room_id -> set of session addresses
    rooms: HashMap<String, HashSet<Uuid>>,
    // Map of user_id -> session address
    sessions: HashMap<Uuid, Addr<ChatWebSocket>>,
}

impl ChatServer {
    pub fn new() -> Self {
        Self {
            rooms: HashMap::new(),
            sessions: HashMap::new(),
        }
    }

    // Add a user to a room
    pub fn join_room(&mut self, room_id: &str, user_id: Uuid, addr: Addr<ChatWebSocket>) {
        // Add user to the room
        self.rooms
            .entry(room_id.to_owned())
            .or_insert_with(HashSet::new)
            .insert(user_id);

        // Store the user's session address
        self.sessions.insert(user_id, addr);
    }

    // Remove a user from a room
    pub fn leave_room(&mut self, room_id: &str, user_id: &Uuid) {
        if let Some(room) = self.rooms.get_mut(room_id) {
            room.remove(user_id);
            if room.is_empty() {
                self.rooms.remove(room_id);
            }
        }
    }

    // Send a message to all users in a room
    pub fn send_message(&self, room_id: &str, message: &ChatMessage, skip_user_id: Option<Uuid>) {
        if let Some(room) = self.rooms.get(room_id) {
            for &user_id in room {
                if let Some(skip_id) = skip_user_id {
                    if user_id == skip_id {
                        continue;
                    }
                }
                if let Some(addr) = self.sessions.get(&user_id) {
                    let msg = ChatMessageBroadcast(message.clone());
                    addr.do_send(msg);
                }
            }
        }
    }

    // Get all users in a room
    pub fn get_room_users(&self, room_id: &str) -> Vec<Uuid> {
        if let Some(room) = self.rooms.get(room_id) {
            return room.iter().copied().collect();
        }
        vec![]
    }
}

// Message to broadcast to clients
#[derive(Message)]
#[rtype(result = "()")]
struct ChatMessageBroadcast(ChatMessage);

impl Actor for ChatWebSocket {
    type Context = ws::WebsocketContext<Self>;

    fn started(&mut self, _ctx: &mut Self::Context) {
        tracing::info!("WebSocket connection started for user: {}", self.user_id);
    }

    fn stopped(&mut self, _ctx: &mut Self::Context) {
        tracing::info!("WebSocket connection closed for user: {}", self.user_id);
        
        // Remove user from room when connection closes
        if let Some(room_id) = &self.room_id {
            let mut server = self.server.lock().unwrap();
            server.leave_room(room_id, &self.user_id);
            
            // Notify other users that this user has disconnected
            let message = format!("{} has left the chat", self.user_name);
            
            // Create system message
            let system_message = ChatMessage {
                id: Uuid::new_v4(),
                sender_id: Uuid::nil(), // System message
                sender_name: "System".to_string(),
                content: message,
                room_id: room_id.clone(),
                created_at: chrono::Utc::now(),
            };
            
            server.send_message(room_id, &system_message, None);
        }
    }
}

// Handle WebSocket messages
impl StreamHandler<Result<ws::Message, ws::ProtocolError>> for ChatWebSocket {
    fn handle(&mut self, msg: Result<ws::Message, ws::ProtocolError>, ctx: &mut Self::Context) {
        match msg {
            Ok(ws::Message::Text(text)) => {
                // Parse the incoming message
                match serde_json::from_str::<WsMessage>(&text) {
                    Ok(ws_message) => match ws_message {
                        WsMessage::Connect { user_id, room_id } => {
                            // Validate that the user_id matches the authenticated user
                            if user_id != self.user_id {
                                ctx.text(json!({
                                    "error": "User ID mismatch"
                                }).to_string());
                                return;
                            }
                            
                            // Join the room
                            self.room_id = Some(room_id.clone());
                            
                            // Add user to the room in the server state
                            let mut server = self.server.lock().unwrap();
                            server.join_room(&room_id, self.user_id, ctx.address());
                            
                            // Notify other users that a new user has joined
                            let message = format!("{} has joined the chat", self.user_name);
                            
                            // Create system message
                            let system_message = ChatMessage {
                                id: Uuid::new_v4(),
                                sender_id: Uuid::nil(), // System message
                                sender_name: "System".to_string(),
                                content: message,
                                room_id: room_id.clone(),
                                created_at: chrono::Utc::now(),
                            };
                            
                            server.send_message(&room_id, &system_message, None);
                            
                            // Send confirmation to the client
                            ctx.text(json!({
                                "type": "connected",
                                "room_id": room_id
                            }).to_string());
                        },
                        WsMessage::Message { content, room_id } => {
                            // Check if user is in the specified room
                            if self.room_id.as_deref() != Some(&room_id) {
                                ctx.text(json!({
                                    "error": "Not connected to this room"
                                }).to_string());
                                return;
                            }
                            
                            // Create and save the message
                            let db_pool = self.db_pool.clone();
                            let user_id = self.user_id;
                            let user_name = self.user_name.clone();
                            let server_clone = Arc::clone(&self.server);
                            let room_id_clone = room_id.clone();
                            
                            // Process message asynchronously
                            actix::spawn(async move {
                                match ChatRepository::save_message(
                                    &db_pool,
                                    user_id,
                                    &user_name,
                                    &content,
                                    &room_id_clone,
                                ).await {
                                    Ok(message) => {
                                        // Broadcast the message to all users in the room
                                        let server = server_clone.lock().unwrap();
                                        server.send_message(&room_id_clone, &message, None);
                                    },
                                    Err(e) => {
                                        tracing::error!("Failed to save message: {}", e);
                                    }
                                }
                            });
                        },
                        WsMessage::Disconnect { user_id, room_id } => {
                            // Validate that the user_id matches the authenticated user
                            if user_id != self.user_id {
                                ctx.text(json!({
                                    "error": "User ID mismatch"
                                }).to_string());
                                return;
                            }
                            
                            // Leave the room
                            if let Some(current_room) = &self.room_id {
                                if current_room == &room_id {
                                    let mut server = self.server.lock().unwrap();
                                    server.leave_room(&room_id, &self.user_id);
                                    self.room_id = None;
                                    
                                    // Notify other users
                                    let message = format!("{} has left the chat", self.user_name);
                                    
                                    // Create system message
                                    let system_message = ChatMessage {
                                        id: Uuid::new_v4(),
                                        sender_id: Uuid::nil(), // System message
                                        sender_name: "System".to_string(),
                                        content: message,
                                        room_id: room_id.clone(),
                                        created_at: chrono::Utc::now(),
                                    };
                                    
                                    server.send_message(&room_id, &system_message, None);
                                    
                                    // Send confirmation to the client
                                    ctx.text(json!({
                                        "type": "disconnected",
                                        "room_id": room_id
                                    }).to_string());
                                }
                            }
                        }
                    },
                    Err(e) => {
                        tracing::error!("Failed to parse WebSocket message: {}", e);
                        ctx.text(json!({
                            "error": "Invalid message format"
                        }).to_string());
                    }
                }
            },
            Ok(ws::Message::Ping(msg)) => {
                ctx.pong(&msg);
            },
            Ok(ws::Message::Pong(_)) => {
                // Client responded to ping
            },
            Ok(ws::Message::Binary(_)) => {
                ctx.text(json!({
                    "error": "Binary messages are not supported"
                }).to_string());
            },
            Ok(ws::Message::Close(reason)) => {
                ctx.close(reason);
                ctx.stop();
            },
            _ => {
                ctx.stop();
            }
        }
    }
}

// Handle broadcast messages
impl Handler<ChatMessageBroadcast> for ChatWebSocket {
    type Result = ();

    fn handle(&mut self, msg: ChatMessageBroadcast, ctx: &mut Self::Context) {
        // Convert the message to a response and send it to the client
        let response = ChatMessageResponse {
            id: msg.0.id,
            sender_id: msg.0.sender_id,
            sender_name: msg.0.sender_name,
            content: msg.0.content,
            room_id: msg.0.room_id,
            created_at: msg.0.created_at,
        };
        
        ctx.text(serde_json::to_string(&response).unwrap());
    }
}

// Create a global chat server state
lazy_static::lazy_static! {
    static ref CHAT_SERVER: Arc<Mutex<ChatServer>> = Arc::new(Mutex::new(ChatServer::new()));
}

// WebSocket connection handler
pub async fn ws_connect(req: HttpRequest, stream: web::Payload, db_pool: web::Data<PgPool>) -> Result<HttpResponse, Error> {
    // Extract and validate JWT token from query parameters
    let query = req.query_string();
    let token = query.split('&')
        .find(|s| s.starts_with("token="))
        .map(|s| s.trim_start_matches("token="))
        .ok_or_else(|| {
            tracing::error!("Missing token in WebSocket connection");
            actix_web::error::ErrorUnauthorized("Missing authentication token")
        })?;
    
    // Verify the token
    let claims = match verify_token(token) {
        Ok(claims) => claims,
        Err(e) => {
            tracing::error!("Invalid token in WebSocket connection: {}", e);
            return Err(actix_web::error::ErrorUnauthorized("Invalid authentication token").into());
        }
    };
    
    // Extract user information from claims
    let user_id = Uuid::parse_str(&claims.sub).map_err(|_| {
        actix_web::error::ErrorInternalServerError("Invalid user ID in token")
    })?;
    
    let user_name = claims.name;
    
    // Create the WebSocket actor
    let ws = ChatWebSocket {
        user_id,
        user_name,
        room_id: None,
        db_pool,
        server: CHAT_SERVER.clone(),
    };
    
    // Start the WebSocket connection
    let resp = ws::start(ws, &req, stream)?;
    Ok(resp)
}

// REST endpoint to get all available chat rooms
pub async fn get_chat_rooms(db_pool: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    let rooms = ChatRepository::get_all_rooms(db_pool.get_ref())
        .await
        .map_err(|e| {
            tracing::error!("Failed to get chat rooms: {}", e);
            AppError::DatabaseError(e)
        })?;
    
    Ok(HttpResponse::Ok().json(rooms))
}

// REST endpoint to get recent messages for a room
pub async fn get_room_messages(path: web::Path<String>, db_pool: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    let room_id = path.into_inner();
    
    // Verify that the room exists
    let room = ChatRepository::get_room_by_id(db_pool.get_ref(), &room_id)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get chat room: {}", e);
            AppError::DatabaseError(e)
        })?;
    
    if room.is_none() {
        return Err(AppError::NotFoundError(format!("Chat room with ID {} not found", room_id)));
    }
    
    // Get recent messages (last 50)
    let messages = ChatRepository::get_recent_messages(db_pool.get_ref(), &room_id, 50)
        .await
        .map_err(|e| {
            tracing::error!("Failed to get chat messages: {}", e);
            AppError::DatabaseError(e)
        })?;
    
    Ok(HttpResponse::Ok().json(messages))
}

// REST endpoint to create a new chat room
pub async fn create_chat_room(room: web::Json<ChatRoom>, db_pool: web::Data<PgPool>) -> Result<HttpResponse, AppError> {
    // Create the room
    let new_room = ChatRepository::create_room(
        db_pool.get_ref(),
        &room.id,
        &room.name
    )
    .await
    .map_err(|e| {
        tracing::error!("Failed to create chat room: {}", e);
        AppError::DatabaseError(e)
    })?;
    
    Ok(HttpResponse::Created().json(new_room))
}