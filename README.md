# User CRUD API in Rust

[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org/)
[![Actix Web](https://img.shields.io/badge/actix--web-4.1-blue.svg)](https://actix.rs/)
[![PostgreSQL](https://img.shields.io/badge/PostgreSQL-17.4-blue.svg)](https://www.postgresql.org/)

A complete RESTful API backend for user management and authentication, built with Rust.

## Table of Contents

- [Features](#features)
- [Technology Stack](#technology-stack)
- [API Endpoints](#api-endpoints)
- [Project Structure](#project-structure)
- [Getting Started](#getting-started)
  - [Requirements](#requirements)
  - [Database Configuration](#database-configuration)
  - [Environment Configuration](#environment-configuration)
  - [Running the Application](#running-the-application)
  - [Running Tests](#running-tests)
- [API Usage Examples](#api-usage-examples)
  - [User Management](#user-management)
  - [Authentication](#authentication)
  - [Chat API](#chat-api)
- [Data Models](#data-models)
  - [User Entity](#user-entity)
  - [User Roles](#user-roles)
  - [Statistics](#statistics)
- [Security](#security)
  - [Password Requirements](#password-requirements)
  - [JWT Authentication](#jwt-authentication)
  - [OAuth 2.0 Authentication](#oauth-20-authentication)
- [Error Handling](#error-handling)
- [Monitoring and Logging](#monitoring-and-logging)
- [Architecture](#architecture)
- [WebSocket Chat API](#websocket-chat-api)
- [Future Development](#future-development)

## Features

- Complete user management system with CRUD operations
- Role-based access control (client, trainer, admin)
- Secure authentication with JWT and OAuth 2.0
- Real-time chat functionality using WebSockets
- Structured logging with tracing
- Health check endpoint with system information
- Comprehensive error handling
- Database migrations for version control
- Secure HTTPS and WSS protocol support

## Technology Stack

- **Actix Web** - Fast, pragmatic web framework
- **PostgreSQL** - Robust, scalable database
- **SQLx** - Asynchronous database access library
- **bcrypt** - Secure password hashing
- **regex** - Enhanced input validation
- **JWT** - Secure token-based authentication
- **OAuth 2.0** - Third-party authentication (Google, Facebook, GitHub)
- **WebSockets** - Real-time chat functionality
- **WSS Protocol** - Secure WebSocket connections

## API Endpoints

<details>
<summary><strong>User Management Endpoints</strong></summary>

- **Create users** - `POST /api/users`
- **Retrieve list of users** - `GET /api/users`
- **Retrieve users by role** - `GET /api/users/role/{role}`
- **Retrieve user statistics** - `GET /api/users/statistics`
- **Retrieve a single user** - `GET /api/users/{id}`
- **Update a user** - `PUT /api/users/{id}`
- **Delete a user** - `DELETE /api/users/{id}`
</details>

<details>
<summary><strong>Authentication Endpoints</strong></summary>

- **Login** - `POST /api/auth/login`
- **OAuth Login** - `GET /api/auth/oauth/{provider}` - initiates OAuth flow with specified provider
- **OAuth Callback** - `GET /api/auth/oauth/callback` - handles OAuth provider callback
</details>

<details>
<summary><strong>Chat Endpoints</strong></summary>

- **WebSocket Chat** - `GET /api/chat/ws` - WebSocket endpoint for real-time chat
- **Get Chat Rooms** - `GET /api/chat/rooms` - retrieves available chat rooms
- **Get Room Messages** - `GET /api/chat/rooms/{room_id}/messages` - retrieves messages for a specific room
</details>

<details>
<summary><strong>Monitoring Endpoints</strong></summary>

- **Health check** - `GET /health` - provides basic information about the application status
- **Metrics** - `GET /metrics` - exposes Prometheus metrics for monitoring application performance
</details>

## Project Structure

<details>
<summary>Click to expand project directory structure</summary>

```
.
├── .env                                   # Environment variables
├── Cargo.toml                             # Project configuration and dependencies
├── Cargo.lock                             # Locked dependency versions
├── certs/                                 # SSL/TLS certificates for HTTPS and WSS
│   ├── cert.pem                           # SSL certificate
│   └── key.pem                            # SSL private key
├── migrations/                            # Database migrations
│   ├── 20250306220539_create_users_table.sql
│   ├── 20250307212522_add_phone_number_and_required_full_name.sql
│   ├── 20250307215355_add_password_support.sql
│   ├── 20250308143333_add_user_role.sql
│   ├── 20250309000000_add_admin_role.sql
│   └── 20250310000000_create_chat_tables.sql
├── src/
│   ├── main.rs                            # Application entry point
│   ├── lib.rs                             # Module exports for tests
│   ├── config.rs                          # Application configuration
│   ├── error.rs                           # Error handling
│   ├── handlers/                          # HTTP endpoint handlers
│   │   ├── mod.rs                         # Module exports
│   │   ├── user.rs                        # User handlers
│   │   ├── auth.rs                        # Authentication handlers
│   │   ├── statistics.rs                  # Statistics handlers
│   │   └── chat.rs                        # WebSocket chat handlers
│   ├── models/                            # Data models
│   │   ├── mod.rs                         # Module exports
│   │   ├── user.rs                        # User model
│   │   ├── auth.rs                        # Authentication models
│   │   ├── role.rs                        # Role models
│   │   ├── statistics.rs                  # Statistics models
│   │   └── chat.rs                        # Chat models
│   ├── database/                          # Database access layer
│   │   ├── mod.rs                         # Module exports
│   │   ├── user.rs                        # User repository
│   │   └── connection.rs                  # Database connection pool
│   ├── auth_utils/                        # Authentication utilities
│   │   ├── mod.rs                         # Module exports
│   │   ├── password.rs                    # Password hashing and verification
│   │   ├── validation.rs                  # Input validation
│   │   └── roles.rs                       # Role management
│   ├── monitoring/                        # Performance monitoring tools
│   │   ├── mod.rs                         # Module exports
│   │   ├── metrics.rs                     # Prometheus metrics
│   │   └── memory.rs                      # Memory usage monitoring
│   ├── logging/                           # Enhanced logging system
│   │   ├── mod.rs                         # Module exports
│   │   └── tracing.rs                     # Tracing configuration
│   ├── middleware/                        # Custom middleware components
│   │   ├── mod.rs                         # Module exports
│   │   ├── performance_metrics.rs         # Performance metrics middleware
│   │   └── tracing.rs                     # Tracing middleware
│   └── services/                          # Business logic layer
│       ├── mod.rs                         # Module exports
│       ├── user.rs                        # User service
│       └── auth.rs                        # Authentication service
└── tests/
    └── api_tests.rs                       # API integration tests
```
</details>

## Getting Started

### Requirements

- Rust (latest stable version)
- PostgreSQL

### Database Configuration

<details>
<summary>Database setup instructions</summary>

```bash
# Create the database
psql -U postgres -c "CREATE DATABASE actix_postgres_api"

# Create the pgcrypto extension (needed for gen_random_uuid())
psql -U postgres -d actix_postgres_api -c "CREATE EXTENSION IF NOT EXISTS pgcrypto;"

# Run migrations
# You can use sqlx-cli to run migrations:
# cargo install sqlx-cli
# sqlx migrate run
```
</details>

### Environment Configuration

<details>
<summary>Environment variables (.env file)</summary>

Create a `.env` file in the project root directory:

```
DATABASE_URL=postgres://postgres:password@localhost/actix_postgres_api?sslmode=prefer
HOST=127.0.0.1
PORT=8080
DB_MAX_CONNECTIONS=5
RUST_LOG=actix_postgres_api=info,actix_web=info,sqlx=warn

# JWT Configuration
JWT_SECRET=your_jwt_secret_key_here
JWT_EXPIRATION=86400

# OAuth Configuration
GOOGLE_CLIENT_ID=your_google_client_id
GOOGLE_CLIENT_SECRET=your_google_client_secret
FACEBOOK_CLIENT_ID=your_facebook_client_id
FACEBOOK_CLIENT_SECRET=your_facebook_client_secret
GITHUB_CLIENT_ID=your_github_client_id
GITHUB_CLIENT_SECRET=your_github_client_secret
OAUTH_REDIRECT_URL=http://localhost:8080/api/auth/oauth/callback

# SSL/TLS Configuration for HTTPS and WSS
SSL_CERT_PATH=./certs/cert.pem
SSL_KEY_PATH=./certs/key.pem
ENABLE_HTTPS=true
```

Adjust the connection parameters to match your PostgreSQL configuration.
</details>

### Running the Application

```bash
cargo run
```

The application will be available at:
- HTTP: `http://127.0.0.1:8080/api/users`
- HTTPS: `https://127.0.0.1:8080/api/users` (when ENABLE_HTTPS=true)
- WSS: `wss://127.0.0.1:8080/api/chat/ws` (for WebSocket chat)

### Running Tests

<details>
<summary>Test setup and execution</summary>

```bash
# Create test database
psql -U postgres -c "CREATE DATABASE actix_postgres_api_test"

# Create the pgcrypto extension
psql -U postgres -d actix_postgres_api_test -c "CREATE EXTENSION IF NOT EXISTS pgcrypto;"

# Run migrations

# Run tests
cargo test
```
</details>

## API Usage Examples

### User Management

<details>
<summary><strong>Creating a User</strong></summary>

```bash
curl -X POST http://localhost:8080/api/users \
  -H "Content-Type: application/json" \
  -d '{"username":"jsmith","email":"john.smith@example.com","password":"SecurePass123","full_name":"John Smith","phone_number":"+1 234 567 890","role":"client"}'
```
</details>

<details>
<summary><strong>Creating a Trainer User</strong></summary>

```bash
curl -X POST http://localhost:8080/api/users \
  -H "Content-Type: application/json" \
  -d '{"username":"mcoach","email":"mike.coach@example.com","password":"SecurePass123","full_name":"Mike Coach","phone_number":"+1 234 567 891","role":"trainer"}'
```
</details>

<details>
<summary><strong>Retrieving All Users</strong></summary>

```bash
curl http://localhost:8080/api/users
```
</details>

<details>
<summary><strong>Retrieving Users by Role</strong></summary>

```bash
curl http://localhost:8080/api/users/role/trainer
```
</details>

<details>
<summary><strong>Retrieving User Statistics</strong></summary>

```bash
curl http://localhost:8080/api/users/statistics
```
</details>

<details>
<summary><strong>Retrieving a User by ID</strong></summary>

```bash
curl http://localhost:8080/api/users/{id}
```
</details>

<details>
<summary><strong>Updating a User</strong></summary>

```bash
curl -X PUT http://localhost:8080/api/users/{id} \
  -H "Content-Type: application/json" \
  -d '{"email":"new.email@example.com","active":false,"phone_number":"+1 987 654 321"}'
```
</details>

<details>
<summary><strong>Updating User Role</strong></summary>

```bash
curl -X PUT http://localhost:8080/api/users/{id} \
  -H "Content-Type: application/json" \
  -d '{"role":"trainer"}'
```
</details>

<details>
<summary><strong>Updating User Password</strong></summary>

```bash
curl -X PUT http://localhost:8080/api/users/{id} \
  -H "Content-Type: application/json" \
  -d '{"password":"NewSecurePass456"}'
```
</details>

<details>
<summary><strong>Deleting a User</strong></summary>

```bash
curl -X DELETE http://localhost:8080/api/users/{id}
```
</details>

### Authentication

<details>
<summary><strong>User Login</strong></summary>

```bash
curl -X POST http://localhost:8080/api/auth/login \
  -H "Content-Type: application/json" \
  -d '{"email":"john.smith@example.com","password":"SecurePass123"}'
```

The login response includes a JWT token that should be used for authenticated requests:

```json
{
  "user": {
    "id": "a1b2c3d4-e5f6-7890-abcd-ef1234567890",
    "username": "jsmith",
    "email": "john.smith@example.com",
    "full_name": "John Smith",
    "phone_number": "+1 234 567 890",
    "role": "client",
    "active": true,
    "created_at": "2023-01-01T12:00:00Z",
    "updated_at": "2023-01-01T12:00:00Z"
  },
  "token": "eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9...",
  "message": "Login successful"
}
```
</details>

<details>
<summary><strong>OAuth Login</strong></summary>

To initiate OAuth login with a provider (Google, Facebook, or GitHub):

```bash
# Replace {provider} with google, facebook, or github
curl -L http://localhost:8080/api/auth/oauth/{provider}
```

This will redirect the user to the provider's authentication page. After successful authentication, the provider will redirect back to the callback URL with an authorization code.
</details>

### Chat API

<details>
<summary><strong>WebSocket Chat Connection</strong></summary>

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
  
  // Check if we're authenticated successfully
  if (message.type === "authenticated") {
    console.log("Authentication successful!");
  }
};
```
</details>

<details>
<summary><strong>WebSocket Message Types</strong></summary>

#### Client to Server Messages

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

#### Server to Client Messages

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

5. **Disconnected Message**
   - Sent by server after successfully leaving a room
   ```json
   {
     "type": "disconnected",
     "room_id": "general"
   }
   ```

6. **Error Message**
   - Sent by server when an error occurs
   ```json
   {
     "error": "Not connected to this room"
   }
   ```
</details>

<details>
<summary><strong>Example WebSocket Client Implementation</strong></summary>

```javascript
// Connect to secure WebSocket (WSS)
const token = "your_jwt_token"; // JWT token from login
const ws = new WebSocket("wss://localhost:8080/api/ws/chat?token=" + token);

// Handle connection open
ws.onopen = () => {
  console.log("Connected to chat");
  
  // Join a chat room
  ws.send(JSON.stringify({
    type: "connect",
    user_id: "your_user_id",
    room_id: "general"
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
</details>

<details>
<summary><strong>Retrieving Chat History</strong></summary>

```bash
# Get all available chat rooms
curl https://localhost:8080/api/chat/rooms \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."

# Get messages from a specific room
curl https://localhost:8080/api/chat/rooms/general/messages \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
```
</details>

## Data Models

### User Entity

The `User` entity contains the following fields:

| Field | Type | Description |
|-------|------|-------------|
| `id` | UUID | Unique identifier |
| `username` | String | Unique username |
| `email` | String | Unique email address |
| `password_hash` | String | Bcrypt hashed password (not exposed via API) |
| `full_name` | String | User's full name (required) |
| `phone_number` | String | Optional phone number |
| `role` | String | User role: "client", "trainer", or "admin" |
| `active` | Boolean | User activity status (default `true`) |
| `created_at` | DateTime | Record creation timestamp |
| `updated_at` | DateTime | Record last update timestamp |

### User Roles

The API supports three user roles:

| Role | Description |
|------|-------------|
| `client` | Regular gym client/member (default) |
| `trainer` | Gym trainer/coach |
| `admin` | Administrative role with extended API access privileges |

When creating or updating a user, the role can be specified. If not provided during user creation, the default role is "client". The admin role grants access to additional API endpoints and operations not available to other roles.

### Statistics

The API provides statistics about users through the `/api/users/statistics` endpoint, which returns:

- Count of users by role
- Count of inactive users

## Security

### Password Requirements

Passwords must meet the following security requirements:
- At least 8 characters long
- At least one digit
- At least one uppercase letter
- At least one lowercase letter

### JWT Authentication

JSON Web Tokens (JWT) are used for secure authentication. When a user logs in successfully, the server returns a JWT token that should be included in subsequent requests.

To access protected endpoints, include the JWT token in the Authorization header:

```bash
curl http://localhost:8080/api/users \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
```

JWT tokens contain the following claims:
- `sub`: User ID
- `name`: Username
- `email`: User email
- `role`: User role
- `exp`: Expiration timestamp
- `iat`: Issued at timestamp

### OAuth 2.0 Authentication

<details>
<summary>OAuth 2.0 Flow details</summary>

The API supports OAuth 2.0 authentication with the following providers:
- Google
- Facebook
- GitHub

The OAuth flow works as follows:
1. User is redirected to the provider's authentication page
2. After successful authentication, the provider redirects back to the application with an authorization code
3. The application exchanges the code for an access token
4. The access token is used to fetch user information from the provider
5. If the user exists in the database, they are logged in; otherwise, a new user account is created
6. A JWT token is generated and returned to the client
</details>

## Error Handling

The API returns appropriate HTTP status codes and error messages in JSON format:

| Status Code | Description |
|-------------|-------------|
| `400 Bad Request` | Invalid input data or authentication failure |
| `404 Not Found` | Resource not found |
| `500 Internal Server Error` | Server-side error |

Example error response:

```json
{
  "error": true,
  "message": "User with this email already exists",
  "status": 400
}
```

## Monitoring and Logging

<details>
<summary><strong>Performance Monitoring</strong></summary>

- Prometheus metrics accessible at `/metrics` endpoint
- HTTP request timing and throughput metrics
- Database query performance tracking
- Memory usage monitoring
- Active connections counter
- Request/response status code tracking

### Available Metrics:
- `api_http_requests_total` - Count of HTTP requests by method, path, and status
- `api_http_request_duration_seconds` - HTTP request duration histograms
- `api_db_queries_total` - Count of database operations by type and table
- `api_db_query_duration_seconds` - Database operation duration histograms
- `api_active_connections` - Current number of active HTTP connections
- `api_memory_usage_bytes` - Current memory usage of the application
</details>

<details>
<summary><strong>Extended Logging</strong></summary>

- Structured JSON logging
- Request lifecycle tracing with unique IDs
- Database operation detailed logging
- Error context enrichment
- Configurable log levels via environment variables

### Configuration:
Log levels can be set via environment variables:
```
RUST_LOG=actix_postgres_api=debug,actix_web=info,sqlx=warn
```
</details>

<details>
<summary><strong>Health Check</strong></summary>

A health check endpoint is available at `/health`, providing information about the application and system status. It's useful for monitoring and debugging purposes.

```json
{
  "status": "up",
  "version": "0.1.0",
  "uptime": 3600,
  "start_time": 1678901234,
  "system_info": {
    "cpu_usage": 5.2,
    "total_memory": 16777216000,
    "used_memory": 8388608000,
    "memory_usage_percent": 50.0,
    "total_swap": 4294967296,
    "used_swap": 1073741824,
    "hostname": "server-name",
    "os_name": "Windows",
    "os_version": "10.0.19045",
    "kernel_version": "10.0.19045.1"
  }
}
```

#### Response Fields

| Field | Type | Description |
|-------|------|-------------|
| status | string | Current application status ("up" when running) |
| version | string | Application version from Cargo.toml |
| uptime | number | Application uptime in seconds |
| start_time | number | Unix timestamp when the application started |
| system_info | object | Detailed system information |
| system_info.cpu_usage | number | Current CPU usage percentage |
| system_info.total_memory | number | Total system memory in bytes |
| system_info.used_memory | number | Used system memory in bytes |
| system_info.memory_usage_percent | number | Memory usage as percentage |
| system_info.total_swap | number | Total swap space in bytes |
| system_info.used_swap | number | Used swap space in bytes |
| system_info.hostname | string | System hostname |
| system_info.os_name | string | Operating system name |
| system_info.os_version | string | Operating system version |
| system_info.kernel_version | string | Kernel version |

</details>

## Architecture

The application follows a layered architecture:

<details>
<summary>Architecture diagram</summary>

```
┌───────────────────┐
│   Client Request  │
└─────────┬─────────┘
          │
┌─────────▼─────────┐
│  Middleware Layer │ ◄─── Performance metrics, Request tracing, Auth validation
└─────────┬─────────┘
          │
┌─────────▼─────────┐
│   Handlers Layer  │ ◄─── HTTP request handling and response formatting
└─────────┬─────────┘
          │
┌─────────▼─────────┐
│   Services Layer  │ ◄─── Business logic and validation
└─────────┬─────────┘
          │
┌─────────▼─────────┐
│ Repository Layer  │ ◄─── Data access and persistence
└─────────┬─────────┘
          │
┌─────────▼─────────┐
│    Database       │
└───────────────────┘
```
</details>

This separation of concerns makes the codebase more maintainable and testable, with each layer having a distinct responsibility:

1. **Handlers Layer** - HTTP request handling and response formatting
2. **Services Layer** - Business logic and validation
3. **Repository Layer** - Data access and persistence
4. **Models Layer** - Data structures and serialization/deserialization

## WebSocket Chat API

The application includes a real-time chat system using WebSockets with secure WSS protocol support:

### Chat Data Model

<details>
<summary>Chat data models</summary>

- **Chat Rooms** - Predefined chat rooms where users can join and exchange messages
- **Chat Messages** - Messages sent by users in specific chat rooms

The database schema includes:
- `chat_rooms` table - Stores available chat rooms
- `chat_messages` table - Stores all messages with references to rooms and users

Message format:
```json
{
  "id": "uuid",
  "user_id": "uuid",
  "room_id": "string",
  "content": "string",
  "created_at": "timestamp",
  "user": {
    "username": "string",
    "full_name": "string"
  }
}
```
</details>

### WebSocket Protocol

<details>
<summary>WebSocket message types</summary>

The WebSocket API supports the following message types:

| Type | Direction | Description |
|------|-----------|-------------|
| `connect` | Client → Server | Join a chat room |
| `message` | Client → Server | Send a message to a room |
| `message` | Server → Client | Receive a message from a room |
| `user_joined` | Server → Client | Notification that a user joined the room |
| `user_left` | Server → Client | Notification that a user left the room |
| `error` | Server → Client | Error message |

Example message format:
```json
{
  "type": "message",
  "content": "Hello everyone!",
  "room_id": "general",
  "user": {
    "id": "a1b2c3d4-...",
    "username": "jsmith",
    "full_name": "John Smith"
  },
  "timestamp": "2023-01-01T12:00:00Z"
}
```
</details>

## Future Development

The roadmap for future development includes:

✅ Implemented features:
- ✅ Authentication with JWT
- ✅ User roles and permissions
- ✅ Enhanced input validation
- ✅ Performance monitoring
- ✅ Extended logging
- ✅ User statistics
- ✅ OAuth 2.0 authentication
- ✅ JWT token-based authorization
- ✅ WebSocket chat functionality
- ✅ WSS protocol support

🔜 Planned features:
- 🔜 Data pagination for large result sets
- 🔜 API documentation integration (e.g., Swagger/OpenAPI)
- 🔜 Rate limiting for API endpoints
- 🔜 Docker containerization
- 🔜 Email notification service
- 🔜 File upload functionality
- 🔜 Two-factor authentication (2FA)