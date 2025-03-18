# WebSocket Chat API

This document provides detailed information about the real-time chat functionality implemented using WebSockets in the User CRUD API.

## WebSocket Connection

To connect to the chat system, establish a WebSocket connection to the following endpoint:

```
ws://localhost:8080/api/chat/ws  # For non-secure connections
wss://localhost:8080/api/chat/ws  # For secure connections (when HTTPS is enabled)
```

## Authentication

After establishing the WebSocket connection, you must authenticate by sending an authorization message with your JWT token:

```json
{
  "type": "authorization",
  "token": "your_jwt_token"
}
```

If authentication is successful, the server will respond with an authenticated message:

```json
{
  "type": "authenticated",
  "user_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890"
}
```

## Message Types

### Client to Server Messages

1. **Authorization Message**
   - Used to authenticate the WebSocket connection with a JWT token
   - Must be sent as the first message after establishing the WebSocket connection
   ```json
   {
     "type": "authorization",
     "token": "your_jwt_token"
   }
   ```

2. **Connect Message**
   - Used to join a specific chat room
   - Must be sent after successful authentication
   ```json
   {
     "type": "connect",
     "user_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
     "room_id": "general"
   }
   ```

3. **Message**
   - Used to send a message to the current room
   - Must be connected to a room first
   ```json
   {
     "type": "message",
     "content": "Hello everyone!",
     "room_id": "general"
   }
   ```

4. **Disconnect Message**
   - Used to leave a chat room
   ```json
   {
     "type": "disconnect",
     "user_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
     "room_id": "general"
   }
   ```

### Server to Client Messages

1. **Authenticated Message**
   - Sent by server after successful authentication
   ```json
   {
     "type": "authenticated",
     "user_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890"
   }
   ```

2. **Connected Message**
   - Sent by server after successfully joining a room
   ```json
   {
     "type": "connected",
     "room_id": "general"
   }
   ```

3. **Message**
   - Sent by server when a message is received in the room
   ```json
   {
     "id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
     "sender_id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
     "sender_name": "John Smith",
     "content": "Hello everyone!",
     "room_id": "general",
     "created_at": "2023-01-01T12:00:00Z"
   }
   ```

4. **System Message**
   - Sent by server for system notifications (user joined/left)
   ```json
   {
     "id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
     "sender_id": "00000000-0000-0000-0000-000000000000",
     "sender_name": "System",
     "content": "John Smith has joined the chat",
     "room_id": "general",
     "created_at": "2023-01-01T12:00:00Z"
   }
   ```

5. **Error Message**
   - Sent by server when an error occurs
   ```json
   {
     "error": "Not connected to this room"
   }
   ```

## Chat Rooms

The API provides endpoints to retrieve available chat rooms and messages:

```bash
# Get all available chat rooms
curl https://localhost:8080/api/chat/rooms \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."

# Get messages from a specific room
curl https://localhost:8080/api/chat/rooms/general/messages \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
```

## Example WebSocket Client Implementation

```javascript
// Connect to secure WebSocket (WSS)
const token = "your_jwt_token"; // JWT token from login
const ws = new WebSocket("wss://localhost:8080/api/ws/chat");

// Handle connection open
ws.onopen = () => {
  console.log("Connected to chat");
  
  // Send authorization message with JWT token
  ws.send(JSON.stringify({
    type: "authorization",
    token: token
  }));
};

// Handle incoming messages
ws.onmessage = (event) => {
  const message = JSON.parse(event.data);
  console.log("Received message:", message);
  
  // Handle different message types
  if (message.type === "connected") {
    console.log("Successfully joined room:", message.room_id);
  } else if (message.type === "authenticated") {
    console.log("Successfully authenticated as:", message.user_id);
    
    // After authentication, join a chat room
    ws.send(JSON.stringify({
      type: "connect",
      user_id: message.user_id,
      room_id: "general"
    }));
  } else if (message.error) {
    console.error("Error:", message.error);
  } else if (message.sender_name === "System") {
    console.log("System message:", message.content);
  } else {
    console.log(`${message.sender_name}: ${message.content}`);
  }
};

// Send a message
function sendMessage(content) {
  ws.send(JSON.stringify({
    type: "message",
    content: content,
    room_id: "general"
  }));
}

// Leave a chat room
function leaveRoom(userId, roomId) {
  ws.send(JSON.stringify({
    type: "disconnect",
    user_id: userId,
    room_id: roomId
  }));
}
```