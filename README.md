# User CRUD API in Rust

A complete RESTful API backend for user management and authentication, built with Rust using:
- **Actix Web** as the web framework
- **PostgreSQL** as the database
- **SQLx** as the asynchronous database access library
- **bcrypt** for secure password hashing
- **regex** for enhanced input validation

## Functionality

The API supports standard CRUD (Create, Read, Update, Delete) operations on the `User` entity:

- **Create users** - `POST /api/users`
- **Retrieve list of users** - `GET /api/users`
- **Retrieve users by role** - `GET /api/users/role/{role}`
- **Retrieve a single user** - `GET /api/users/{id}`
- **Update a user** - `PUT /api/users/{id}`
- **Delete a user** - `DELETE /api/users/{id}`

Authentication endpoints:
- **Login** - `POST /api/auth/login`

## Project Structure

```
.
├── .env                                   # Environment variables
├── Cargo.toml                             # Project configuration and dependencies
├── Cargo.lock                             # Locked dependency versions
├── migrations/                            # Database migrations
│   ├── 20250306220539_create_users_table.sql
│   └── 20250307212522_add_phone_number_and_required_full_name.sql
│   └── 20250307215355_add_password_support.sql
│   └── 20250308143333_add_user_role.sql.sql
├── src/
│   ├── config.rs                          # Application configuration
│   ├── error.rs                           # Error handling
│   ├── handlers.rs                        # HTTP endpoint handlers
│   ├── lib.rs                             # Module exports for tests
│   ├── main.rs                            # Application entry point
│   ├── models.rs                          # Data models
│   ├── repository.rs                      # Data access layer
│   ├── auth_utils.rs                      # Authentication utilities
│   ├── monitoring.rs                      # Performance monitoring tools
│   ├── logging.rs                         # Enhanced logging system
│   └── middleware.rs                      # Custom middleware components
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

# Create the users table
psql -U postgres -d actix_postgres_api -c "CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) NOT NULL UNIQUE,
    email VARCHAR(100) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL DEFAULT '$2a$12$k8Y6Nt5zfQXmGO9VvQH2CehxfMY0lPuqJxzAkrxoHSJRZz8obzg4W',
    full_name VARCHAR(100) NOT NULL,
    phone_number VARCHAR(20),
    role VARCHAR(20) NOT NULL DEFAULT 'client',
    active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT check_valid_role CHECK (role IN ('client', 'trainer'))
);"
```

### 2. Environment Configuration

Create a `.env` file in the project root directory:

```
DATABASE_URL=postgres://postgres:password@localhost/user_crud?sslmode=prefer
HOST=127.0.0.1
PORT=8080
DB_MAX_CONNECTIONS=5
RUST_LOG=actix_postgres_api=info,actix_web=info,sqlx=warn
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

# Create the table in the test database
psql -U postgres -d actix_postgres_api_test -c "CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) NOT NULL UNIQUE,
    email VARCHAR(100) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL DEFAULT '$2a$12$k8Y6Nt5zfQXmGO9VvQH2CehxfMY0lPuqJxzAkrxoHSJRZz8obzg4W',
    full_name VARCHAR(100) NOT NULL,
    phone_number VARCHAR(20),
    role VARCHAR(20) NOT NULL DEFAULT 'client',
    active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT check_valid_role CHECK (role IN ('client', 'trainer'))
);"

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

## Data Model

The `User` entity contains the following fields:

- `id` - unique UUID identifier
- `username` - unique username
- `email` - unique email address
- `password_hash` - bcrypt hashed password (not exposed via API)
- `full_name` - user's full name (required)
- `phone_number` - optional phone number
- `role` - user role: "client" (default) or "trainer"
- `active` - user activity status (default `true`)
- `created_at` - record creation timestamp
- `updated_at` - record last update timestamp

## User Roles

The API supports two user roles:
- `client` - regular gym client/member (default)
- `trainer` - gym trainer/coach

When creating or updating a user, the role can be specified. If not provided during user creation, the default role is "client".

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

### Testing:
The monitoring and logging features are covered by integration tests in `api_tests.rs`, which verify the correctness of metrics collection and health endpoint functionality.

## Performance and Scalability

- Asynchronous request processing powered by Actix Web
- Database connection pool for optimal resource utilization
- Secure password storage using bcrypt with cost factor
- Designed with performance and scalability in mind

## Future Development

- ✅ Authentication (implemented)
- ✅ User roles (implemented)
- ✅ Enhanced input validation (implemented)
- ✅ Performance monitoring (implemented)
- ✅ Extended logging (implemented)
- Authorization with role-based access control
- Data pagination
- API documentation integration (e.g., Swagger)