# API Endpoints

This document provides detailed information about all available API endpoints in the User CRUD API.

## User Management Endpoints

| Endpoint | Method | Description | Authentication |
|----------|--------|-------------|---------------|
| `/api/users` | POST | Create a new user | No |
| `/api/users` | GET | Retrieve list of all users | Yes |
| `/api/users/role/{role}` | GET | Retrieve users by role | Yes |
| `/api/users/statistics` | GET | Retrieve user statistics | Yes (Admin only) |
| `/api/users/{id}` | GET | Retrieve a single user | Yes |
| `/api/users/{id}` | PUT | Update a user | Yes |
| `/api/users/{id}` | DELETE | Delete a user | Yes |

## Appointment Endpoints

| Endpoint | Method | Description | Authentication |
|----------|--------|-------------|---------------|
| `/api/appointments` | GET | Retrieve all appointments | Yes (Admin only) |
| `/api/appointments/{id}` | GET | Retrieve a specific appointment | Yes |
| `/api/appointments/client/{id}` | GET | Retrieve appointments for a specific client | Yes |
| `/api/appointments/trainer/{id}` | GET | Retrieve appointments for a specific trainer | Yes |
| `/api/appointments` | POST | Create a new appointment | Yes |
| `/api/appointments/{id}` | PUT | Update an existing appointment | Yes |
| `/api/appointments/{id}` | DELETE | Delete an appointment | Yes |

## Authentication Endpoints

| Endpoint | Method | Description | Authentication |
|----------|--------|-------------|---------------|
| `/api/auth/login` | POST | User login | No |
| `/api/auth/oauth/{provider}` | GET | Initiate OAuth flow with specified provider | No |
| `/api/auth/oauth/callback` | GET | Handle OAuth provider callback | No |

## Chat Endpoints

| Endpoint | Method | Description | Authentication |
|----------|--------|-------------|---------------|
| `/api/chat/ws` | GET | WebSocket endpoint for real-time chat | Yes (via WebSocket) |
| `/api/chat/rooms` | GET | Retrieve available chat rooms | Yes |
| `/api/chat/rooms/{room_id}/messages` | GET | Retrieve messages for a specific room | Yes |

## Monitoring Endpoints

| Endpoint | Method | Description | Authentication |
|----------|--------|-------------|---------------|
| `/health` | GET | Health check endpoint | No |
| `/metrics` | GET | Prometheus metrics endpoint | Yes (Admin only) |

## Request and Response Formats

Detailed request and response formats for each endpoint can be found in the [API Usage Examples](./api-usage-examples.md) documentation.