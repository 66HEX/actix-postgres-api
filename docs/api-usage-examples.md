# API Usage Examples

This document provides detailed examples of how to use the API endpoints with sample requests and responses.

## User Management

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

## Appointment Scheduling

<details>
<summary><strong>Creating an Appointment</strong></summary>

```bash
curl -X POST http://localhost:8080/api/appointments \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..." \
  -d '{
    "trainer_id": "t1r2a3i4-n5e6r7-abcd-ef1234567890",
    "title": "Personal Training Session",
    "appointment_date": "2025-04-15",
    "start_time": "14:30:00",
    "duration_minutes": 60,
    "notes": "Focus on strength training"
  }'
```

Response:
```json
{
    "id": "b2c3d4e5-f6a7-8901-bcde-f1234567890a",
    "client_id": "c1d2e3f4-a5b6-7890-abcd-ef1234567890",
    "trainer_id": "t1r2a3i4-n5e6r7-abcd-ef1234567890",
    "client_name": "Jane Doe",
    "trainer_name": "James Doe",
    "type_": "consultation",
    "appointment_date": "2025-04-20",
    "start_time": "10:00:00",
    "duration_minutes": 45,
    "status": "scheduled",
    "location": "Gym Room 3",
    "created_at": "2025-04-02T09:30:00Z",
    "updated_at": "2025-04-02T09:30:00Z"
  }
```
</details>

<details>
<summary><strong>Retrieving Client Appointments</strong></summary>

```bash
curl http://localhost:8080/api/appointments/client/{id} \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
```

Response:
```json
[
  {
    "id": "b2c3d4e5-f6a7-8901-bcde-f1234567890a",
    "client_id": "c1d2e3f4-a5b6-7890-abcd-ef1234567890",
    "trainer_id": "t1r2a3i4-n5e6r7-abcd-ef1234567890",
    "client_name": "Jane Doe",
    "trainer_name": "James Doe",
    "type_": "consultation",
    "appointment_date": "2025-04-20",
    "start_time": "10:00:00",
    "duration_minutes": 45,
    "status": "scheduled",
    "location": "Gym Room 3",
    "created_at": "2025-04-02T09:30:00Z",
    "updated_at": "2025-04-02T09:30:00Z"
  }
  {
    "id": "b2c3d4e5-f6a7-8901-bcde-f1234567890a",
    "client_id": "c1d2e3f4-a5b6-7890-abcd-ef1234567890",
    "trainer_id": "t1r2a3i4-n5e6r7-abcd-ef1234567890",
    "client_name": "Jane Doe",
    "trainer_name": "James Doe",
    "type_": "consultation",
    "appointment_date": "2025-04-20",
    "start_time": "10:00:00",
    "duration_minutes": 45,
    "status": "scheduled",
    "location": "Gym Room 3",
    "created_at": "2025-04-02T09:30:00Z",
    "updated_at": "2025-04-02T09:30:00Z"
  }
]
```
</details>

<details>
<summary><strong>Retrieving Trainer Appointments</strong></summary>

```bash
curl http://localhost:8080/api/appointments/trainer/{id} \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
```

Response is similar to client appointments, showing all appointments for the specified trainer.
</details>

<details>
<summary><strong>Updating an Appointment</strong></summary>

```bash
curl -X PUT http://localhost:8080/api/appointments/{id} \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..." \
  -d '{
    "title": "Updated Training Session",
    "appointment_date": "2025-04-16",
    "start_time": "15:00:00",
    "notes": "Focus on core strength"
  }'
```

Response will be the updated appointment object.
</details>

<details>
<summary><strong>Marking an Appointment as Completed</strong></summary>

```bash
curl -X PUT http://localhost:8080/api/appointments/{id} \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..." \
  -d '{
    "status": "completed"
  }'
```

Note: Only trainers or admins can mark appointments as completed.
</details>

<details>
<summary><strong>Cancelling an Appointment</strong></summary>

```bash
curl -X PUT http://localhost:8080/api/appointments/{id} \
  -H "Content-Type: application/json" \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..." \
  -d '{
    "status": "cancelled"
  }'
```
</details>

<details>
<summary><strong>Deleting an Appointment</strong></summary>

```bash
curl -X DELETE http://localhost:8080/api/appointments/{id} \
  -H "Authorization: Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9..."
```

Note: Only the client who created the appointment or an admin can delete appointments.
</details>