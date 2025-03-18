# Getting Started

This document provides detailed instructions for setting up and running the User CRUD API.

## Requirements

- Rust (latest stable version)
- PostgreSQL

## Database Configuration

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

## Environment Configuration

Create a `.env` file in the project root directory:

```
# Database Connection
DATABASE_URL=postgres://postgres:admin@localhost/actix_postgres_api?sslmode=prefer
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
ENABLE_HTTPS=false

# CORS Configuration
FRONTEND_ORIGIN=http://localhost:1420
```

Adjust the connection parameters to match your PostgreSQL configuration.

## Running the Application

```bash
cargo run
```

The application will be available at:
- HTTP: `http://127.0.0.1:8080/api/users`
- HTTPS: `https://127.0.0.1:8080/api/users` (when ENABLE_HTTPS=true)
- WSS: `wss://127.0.0.1:8080/api/chat/ws` (for WebSocket chat)

## Running Tests

```bash
# Create test database
psql -U postgres -c "CREATE DATABASE actix_postgres_api_test"

# Create the pgcrypto extension
psql -U postgres -d actix_postgres_api_test -c "CREATE EXTENSION IF NOT EXISTS pgcrypto;"

# Run migrations

# Run tests
cargo test
```

## Project Structure

The project follows a modular architecture with clear separation of concerns:

```
.                                        # Root directory
├── src/                                 # Source code
│   ├── main.rs                          # Application entry point
│   ├── lib.rs                           # Module exports for tests
│   ├── config.rs                        # Application configuration
│   ├── error.rs                         # Error handling
│   ├── handlers/                        # HTTP endpoint handlers
│   ├── models/                          # Data models
│   ├── database/                        # Database access layer
│   ├── auth_utils/                      # Authentication utilities
│   ├── monitoring/                      # Performance monitoring tools
│   ├── logging/                         # Enhanced logging system
│   ├── middleware/                      # Custom middleware components
│   └── services/                        # Business logic layer
└── tests/                               # Integration tests
```

For a more detailed breakdown of the project structure, refer to the main README.md file.