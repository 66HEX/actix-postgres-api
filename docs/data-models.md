# Data Models

This document provides detailed information about the data models used in the User CRUD API.

## User Entity

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

## User Roles

The API supports three user roles:

| Role | Description |
|------|-------------|
| `client` | Regular gym client/member (default) |
| `trainer` | Gym trainer/coach |
| `admin` | Administrative role with extended API access privileges |

When creating or updating a user, the role can be specified. If not provided during user creation, the default role is "client". The admin role grants access to additional API endpoints and operations not available to other roles.

## Statistics

The API provides statistics about users through the `/api/users/statistics` endpoint. The statistics include detailed information about user roles, inactive users, and registration trends.

| Field | Type | Description |
|-------|------|-------------|
| `roles` | Array of Objects | List of user role statistics |
| `roles[].role` | String | User role name ("client", "trainer", "admin") |
| `roles[].count` | Integer | Number of users with this role |
| `inactive_count` | Integer | Total number of inactive users |
| `registration_stats` | Object | Registration statistics over different time periods |
| `registration_stats.last_24h` | Integer | Number of users registered in the last 24 hours |
| `registration_stats.last_7d` | Integer | Number of users registered in the last 7 days |
| `registration_stats.last_30d` | Integer | Number of users registered in the last 30 days |

### Statistics Response Example

```json
{
  "roles": [
    { "role": "client", "count": 100 },
    { "role": "trainer", "count": 45 },
    { "role": "admin", "count": 5 }
  ],
  "inactive_count": 30,
  "registration_stats": {
    "last_24h": 5,
    "last_7d": 25,
    "last_30d": 45
  }
}
```

## Appointment Entity

The `Appointment` entity represents scheduled training sessions between clients and trainers:

| Field | Type | Description |
|--|--|--|
| `id` | UUID | Unique identifier |
| `client_id` | UUID | ID of the client who booked the appointment |
| `trainer_id` | UUID | ID of the trainer conducting the session |
| `title` | String | Title/name of the appointment |
| `appointment_date` | Date | Date of the appointment (YYYY-MM-DD) |
| `start_time` | Time | Start time of the appointment (HH:MM:SS) |
| `duration_minutes` | Integer | Duration of the appointment in minutes |
| `status` | String | Current status: "scheduled", "completed", or "cancelled" |
| `notes` | String | Optional notes about the appointment |
| `created_at` | DateTime | Record creation timestamp |
| `updated_at` | DateTime | Record last update timestamp |

### Appointment Status Values

| Status | Description |
|--|--|
| `scheduled` | Default status for new appointments |
| `completed` | Marked by trainer or admin when the appointment is completed |
| `cancelled` | Marked by client, trainer, or admin when the appointment is cancelled |

### Access Control
- Clients can create, view, update, and cancel their own appointments
- Trainers can view and update appointments where they are the assigned trainer
- Only trainers and admins can mark appointments as "completed"
- Only clients who created the appointment or admins can delete appointments