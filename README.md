# User CRUD API in Rust

[![Rust](https://img.shields.io/badge/rust-stable-brightgreen.svg)](https://www.rust-lang.org/)
[![Actix Web](https://img.shields.io/badge/actix--web-4.1-blue.svg)](https://actix.rs/)
[![PostgreSQL](https://img.shields.io/badge/PostgreSQL-17.4-blue.svg)](https://www.postgresql.org/)

A complete RESTful API backend for user management and authentication, built with Rust.

## Overview

This project is a comprehensive backend API built with Rust, Actix Web, and PostgreSQL. It provides a complete solution for user management, authentication, appointment scheduling, and real-time chat functionality. The API is designed with security, performance, and scalability in mind.

Detailed documentation is available in the [docs](./docs) directory:

- [API Endpoints](./api-endpoints.md) - Detailed information about all available API endpoints
- [API Usage Examples](./api-usage-examples.md) - Examples of how to use the API with sample requests and responses
- [Getting Started](./getting-started.md) - Setup and configuration instructions
- [Data Models](./data-models.md) - Information about data structures and models
- [Security](./security.md) - Authentication and security details
- [WebSocket Chat API](./websocket-chat.md) - Real-time chat functionality documentation
- [Monitoring](./monitoring.md) - Performance monitoring and logging information

## Table of Contents

- [Features](#features)
- [Technology Stack](#technology-stack)
- [Project Structure](#project-structure)
- [Architecture](#architecture)
- [Future Development](#future-development)

## Features

- Complete user management system with CRUD operations
- Appointment scheduling system
- Role-based access control (client, trainer, admin)
- Secure authentication with JWT and OAuth 2.0
- Real-time chat functionality using WebSockets
- Structured logging with tracing
- Health check endpoint with system information
- Comprehensive error handling
- Database migrations for version control
- Secure HTTPS and WSS protocol support
- Cross-Origin Resource Sharing (CORS) support for frontend integration

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
- **Actix CORS** - Cross-Origin Resource Sharing middleware

## Project Structure

```
.
├── .env                                   # Environment variables
├── Cargo.toml                             # Project configuration and dependencies
├── Cargo.lock                             # Locked dependency versions
├── certs/                                 # SSL/TLS certificates for HTTPS and WSS
│   ├── cert.pem                           # SSL certificate
│   └── key.pem                            # SSL private key
├── migrations/                            # Database migrations
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
│   │   ├── chat.rs                        # WebSocket chat handlers
│   │   └── appointment.rs                 # Appointment handlers
│   ├── models/                            # Data models
│   │   ├── mod.rs                         # Module exports
│   │   ├── user.rs                        # User model
│   │   ├── auth.rs                        # Authentication models
│   │   ├── role.rs                        # Role models
│   │   ├── statistics.rs                  # Statistics models
│   │   ├── chat.rs                        # Chat models
│   │   └── appointment.rs                 # Appointment model
│   ├── database/                          # Database access layer
│   │   ├── mod.rs                         # Module exports
│   │   ├── user.rs                        # User repository
│   │   ├── appointment.rs                 # Appointment repository
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
│   │   ├── cors.rs                        # CORS configuration middleware
│   │   ├── performance_metrics.rs         # Performance metrics middleware
│   │   └── tracing.rs                     # Tracing middleware
│   └── services/                          # Business logic layer
│       ├── mod.rs                         # Module exports
│       ├── user.rs                        # User service
│       ├── appointment.rs                 # Appointment service
│       └── auth.rs                        # Authentication service
└── tests/
    └── api_tests.rs                       # API integration tests
```

## Architecture

The application follows a layered architecture:

```
┌───────────────────┐
│   Client Request  │
└─────────┬─────────┘
          │
┌─────────▼─────────┐
│  Middleware Layer │ ◄─── Performance metrics, Request tracing, Auth validation, CORS
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

This separation of concerns makes the codebase more maintainable and testable, with each layer having a distinct responsibility:

1. **Handlers Layer** - HTTP request handling and response formatting
2. **Services Layer** - Business logic and validation
3. **Repository Layer** - Data access and persistence
4. **Models Layer** - Data structures and serialization/deserialization

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
- ✅ Appointment scheduling system
- ✅ CORS middleware

🔜 Planned features:
- 🔜 Data pagination for large result sets
- 🔜 API documentation integration (e.g., Swagger/OpenAPI)
- 🔜 Rate limiting for API endpoints
- 🔜 Docker containerization
- 🔜 Email notification service
- 🔜 File upload functionality
- 🔜 Two-factor authentication (2FA)