# Security

This document provides detailed information about the security features and authentication mechanisms used in the User CRUD API.

## Password Requirements

Passwords must meet the following security requirements:
- At least 8 characters long
- At least one digit
- At least one uppercase letter
- At least one lowercase letter

## JWT Authentication

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

## OAuth 2.0 Authentication

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

## HTTPS and WSS Support

The API supports secure HTTPS and WSS (WebSocket Secure) connections. To enable HTTPS, set the `ENABLE_HTTPS` environment variable to `true` and provide valid SSL certificate and key files.

## Role-Based Access Control

The API implements role-based access control to restrict access to certain endpoints and operations based on the user's role:

- **Client**: Can manage their own profile and appointments
- **Trainer**: Can manage their profile and view/update appointments where they are the assigned trainer
- **Admin**: Has full access to all endpoints and operations

Endpoints that require specific roles will return a `403 Forbidden` error if accessed by a user with insufficient privileges.