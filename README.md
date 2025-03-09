# User CRUD API in Rust

A complete RESTful API backend for user management and authentication, built with Rust using:
- **Actix Web** as the web framework
- **PostgreSQL** as the database
- **SQLx** as the asynchronous database access library
- **bcrypt** for secure password hashing
- **regex** for enhanced input validation
- **JWT** for secure token-based authentication
- **OAuth 2.0** for third-party authentication (Google, Facebook, GitHub)

## Functionality

The API supports standard CRUD (Create, Read, Update, Delete) operations on the `User` entity:

- **Create users** - `POST /api/users`
- **Retrieve list of users** - `GET /api/users`
- **Retrieve users by role** - `GET /api/users/role/{role}`
- **Retrieve user statistics** - `GET /api/users/statistics`
- **Retrieve a single user** - `GET /api/users/{id}`
- **Update a user** - `PUT /api/users/{id}`
- **Delete a user** - `DELETE /api/users/{id}`

Authentication endpoints:
- **Login** - `POST /api/auth/login`
- **OAuth Login** - `GET /api/auth/oauth/{provider}` - initiates OAuth flow with specified provider
- **OAuth Callback** - `GET /api/auth/oauth/callback` - handles OAuth provider callback

Monitoring endpoints:
- **Health check** - `GET /health` - provides basic information about the application status
- **Metrics** - `GET /metrics` - exposes Prometheus metrics for monitoring application performance

## Project Structure

```
.
├── .env                                   # Environment variables
├── Cargo.toml                             # Project configuration and dependencies
├── Cargo.lock                             # Locked dependency versions
├── migrations/                            # Database migrations
│   ├── 20250306220539_create_users_table.sql
│   ├── 20250307212522_add_phone_number_and_required_full_name.sql
│   ├── 20250307215355_add_password_support.sql
│   └── 20250308143333_add_user_role.sql
│   └── 20250309000000_add_admin_role.sql
├── src/
│   ├── main.rs                            # Application entry point
│   ├── lib.rs                             # Module exports for tests
│   ├── config.rs                          # Application configuration
│   ├── error.rs                           # Error handling
│   ├── handlers/                          # HTTP endpoint handlers
│   │   ├── mod.rs                         # Module exports
│   │   ├── user.rs                        # User handlers
│   │   ├── auth.rs                        # Authentication handlers
│   │   └── statistics.rs                  # Statistics handlers
│   ├── models/                            # Data models
│   │   ├── mod.rs                         # Module exports
│   │   ├── user.rs                        # User model
│   │   ├── auth.rs                        # Authentication models
│   │   ├── role.rs                        # Role models
│   │   └── statistics.rs                  # Statistics models
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

## Requirements

- Rust (latest stable version)
- PostgreSQL

## Local Setup

### 1. Database Configuration

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

### 2. Environment Configuration

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
```

Adjust the connection parameters to match your PostgreSQL configuration.

### 3. Running the Application

```bash
cargo run
```

The application will be available at `http://127.0.0.1:8080/api/users`.

### 4. Running Tests

```bash
# Create test database
psql -U postgres -c "CREATE DATABASE actix_postgres_api_test"

# Create the pgcrypto extension
psql -U postgres -d actix_postgres_api_test -c "CREATE EXTENSION IF NOT EXISTS pgcrypto;"

# Run migrations

# Run tests
cargo test
```

## API Usage Examples

### Creating a User

```bash
curl -X POST http://localhost:8080/api/users \
  -H "Content-Type: application/json" \
  -d '{"username":"jsmith","email":"john.smith@example.com","password":"SecurePass123","full_name":"John Smith","phone_number":"+1 234 567 890","role":"client"}'
```

### Creating a Trainer User

```bash
curl -X POST http://localhost:8080/api/users \
  -H "Content-Type: application/json" \
  -d '{"username":"mcoach","email":"mike.coach@example.com","password":"SecurePass123","full_name":"Mike Coach","phone_number":"+1 234 567 891","role":"trainer"}'
```

### Retrieving All Users

```bash
curl http://localhost:8080/api/users
```

### Retrieving Users by Role

```bash
curl http://localhost:8080/api/users/role/trainer
```

### Retrieving User Statistics

```bash
curl http://localhost:8080/api/users/statistics
```

### Retrieving a User by ID

```bash
curl http://localhost:8080/api/users/{id}
```

### Updating a User

```bash
curl -X PUT http://localhost:8080/api/users/{id} \
  -H "Content-Type: application/json" \
  -d '{"email":"new.email@example.com","active":false,"phone_number":"+1 987 654 321"}'
```

### Updating User Role

```bash
curl -X PUT http://localhost:8080/api/users/{id} \
  -H "Content-Type: application/json" \
  -d '{"role":"trainer"}'
```

### Updating User Password

```bash
curl -X PUT http://localhost:8080/api/users/{id} \
  -H "Content-Type: application/json" \
  -d '{"password":"NewSecurePass456"}'
```

### Deleting a User

```bash
curl -X DELETE http://localhost:8080/api/users/{id}
```

### User Login

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

### OAuth Login

To initiate OAuth login with a provider (Google, Facebook, or GitHub):

```bash
# Replace {provider} with google, facebook, or github
curl -L http://localhost:8080/api/auth/oauth/{provider}
```

This will redirect the user to the provider's authentication page. After successful authentication, the provider will redirect back to the callback URL with an authorization code.
```

## Data Model

The `User` entity contains the following fields:

- `id` - unique UUID identifier
- `username` - unique username
- `email` - unique email address
- `password_hash` - bcrypt hashed password (not exposed via API)
- `full_name` - user's full name (required)
- `phone_number` - optional phone number
- `role` - user role: "client" (default), "trainer", or "admin"
- `active` - user activity status (default `true`)
- `created_at` - record creation timestamp
- `updated_at` - record last update timestamp

## User Roles

The API supports two user roles:
- `client` - regular gym client/member (default)
- `trainer` - gym trainer/coach
- `admin` - administrative role with extended API access privileges

When creating or updating a user, the role can be specified. If not provided during user creation, the default role is "client". The admin role grants access to additional API endpoints and operations not available to other roles.

## Statistics

The API provides statistics about users through the `/api/users/statistics` endpoint, which returns:

- Count of users by role
- Count of inactive users

## Password Requirements

Passwords must meet the following security requirements:
- At least 8 characters long
- At least one digit
- At least one uppercase letter
- At least one lowercase letter

## Error Handling

The API returns appropriate HTTP status codes and error messages in JSON format:

- `400 Bad Request` - invalid input data or authentication failure
- `404 Not Found` - resource not found
- `500 Internal Server Error` - server-side error

## Monitoring and Logging

The application includes advanced performance monitoring and extended logging capabilities:

### Performance Monitoring:
- Prometheus metrics accessible at `/metrics` endpoint
- HTTP request timing and throughput metrics
- Database query performance tracking
- Memory usage monitoring
- Active connections counter
- Request/response status code tracking

### Extended Logging:
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

### Available Metrics:
- `api_http_requests_total` - Count of HTTP requests by method, path, and status
- `api_http_request_duration_seconds` - HTTP request duration histograms
- `api_db_queries_total` - Count of database operations by type and table
- `api_db_query_duration_seconds` - Database operation duration histograms
- `api_active_connections` - Current number of active HTTP connections
- `api_memory_usage_bytes` - Current memory usage of the application


### Health Check:
A health check endpoint is available at `/health`, providing basic information about the application status.

## Performance and Scalability

- Asynchronous request processing powered by Actix Web
- Database connection pool for optimal resource utilization
- Secure password storage using bcrypt with cost factor
- Designed with performance and scalability in mind

## Architecture

The application follows a layered architecture:

1. **Handlers Layer** - HTTP request handling and response formatting
2. **Services Layer** - Business logic and validation
3. **Repository Layer** - Data access and persistence
4. **Models Layer** - Data structures and serialization/deserialization

This separation of concerns makes the codebase more maintainable and testable.

## Authentication and Authorization

The API implements two authentication methods:

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

## Future Development

- ✅ Authentication (implemented)
- ✅ User roles (implemented)
- ✅ Enhanced input validation (implemented)
- ✅ Performance monitoring (implemented)
- ✅ Extended logging (implemented)
- ✅ User statistics (implemented)
- ✅ OAuth 2.0 authentication (implemented)
- ✅ JWT token-based authorization (implemented)
- Data pagination
- API documentation integration (e.g., Swagger)