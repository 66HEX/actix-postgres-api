# User CRUD API in Rust

A complete RESTful API backend for user management, built with Rust using:
- **Actix Web** as the web framework
- **PostgreSQL** as the database
- **SQLx** as the asynchronous database access library

## Functionality

The API supports standard CRUD (Create, Read, Update, Delete) operations on the `User` entity:

- **Create users** - `POST /api/users`
- **Retrieve list of users** - `GET /api/users`
- **Retrieve a single user** - `GET /api/users/{id}`
- **Update a user** - `PUT /api/users/{id}`
- **Delete a user** - `DELETE /api/users/{id}`

## Project Structure

```
.
├── .env                                   # Environment variables
├── Cargo.toml                             # Project configuration and dependencies
├── Cargo.lock                             # Locked dependency versions
├── migrations/                            # Database migrations
│   └── 20230101000000_create_users_table.sql
├── src/
│   ├── config.rs                          # Application configuration
│   ├── error.rs                           # Error handling
│   ├── handlers.rs                        # HTTP endpoint handlers
│   ├── lib.rs                             # Module exports for tests
│   ├── main.rs                            # Application entry point
│   ├── models.rs                          # Data models
│   └── repository.rs                      # Data access layer
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
psql -U postgres -c "CREATE DATABASE user_crud"

# Create the users table
psql -U postgres -d user_crud -c "CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    full_name VARCHAR(255),
    active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);"
```

### 2. Environment Configuration

Create a `.env` file in the project root directory:

```
DATABASE_URL=postgres://postgres:password@localhost/user_crud?sslmode=prefer
HOST=127.0.0.1
PORT=8080
DB_MAX_CONNECTIONS=5
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
psql -U postgres -c "CREATE DATABASE user_crud_test"

# Create the table in the test database
psql -U postgres -d user_crud_test -c "CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(255) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    full_name VARCHAR(255),
    active BOOLEAN NOT NULL DEFAULT TRUE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);"

# Run tests
cargo test
```

## API Usage Examples

### Creating a User

```bash
curl -X POST http://localhost:8080/api/users \
  -H "Content-Type: application/json" \
  -d '{"username":"jsmith","email":"john.smith@example.com","full_name":"John Smith"}'
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
  -d '{"email":"new.email@example.com","active":false}'
```

### Deleting a User

```bash
curl -X DELETE http://localhost:8080/api/users/{id}
```

## Data Model

The `User` entity contains the following fields:

- `id` - unique UUID identifier
- `username` - unique username
- `email` - unique email address
- `full_name` - optional full name
- `active` - user activity status (default `true`)
- `created_at` - record creation timestamp
- `updated_at` - record last update timestamp

## Error Handling

The API returns appropriate HTTP status codes and error messages in JSON format:

- `400 Bad Request` - invalid input data
- `404 Not Found` - resource not found
- `500 Internal Server Error` - server-side error

## Performance and Scalability

- Asynchronous request processing powered by Actix Web
- Database connection pool for optimal resource utilization
- Designed with performance and scalability in mind

## Future Development

- Authentication and authorization
- Data pagination
- Enhanced input validation
- Logging and monitoring
- API documentation integration (e.g., Swagger)
