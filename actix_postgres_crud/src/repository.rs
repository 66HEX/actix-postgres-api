use crate::error::AppError;
use crate::models::{CreateUserRequest, UpdateUserRequest, User};
use sqlx::{postgres::PgPool, types::Uuid};

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_all(&self) -> Result<Vec<User>, AppError> {
        let users = sqlx::query_as::<_, User>(
            "SELECT * FROM users ORDER BY created_at DESC"
        )
        .fetch_all(&self.pool)
        .await
        .map_err(AppError::DatabaseError)?;

        Ok(users)
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<User, AppError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE id = $1"
        )
        .bind(id)
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::DatabaseError)?;

        user.ok_or_else(|| AppError::NotFoundError(format!("User with id {} not found", id)))
    }

    pub async fn create(&self, user: CreateUserRequest) -> Result<User, AppError> {
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (username, email, full_name)
            VALUES ($1, $2, $3)
            RETURNING *
            "#
        )
        .bind(&user.username)
        .bind(&user.email)
        .bind(&user.full_name)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::DatabaseError)?;

        Ok(user)
    }

    pub async fn update(&self, id: Uuid, user: UpdateUserRequest) -> Result<User, AppError> {
        // Najpierw sprawdzamy, czy użytkownik istnieje
        let existing = self.find_by_id(id).await?;

        // Przygotowanie wartości do aktualizacji
        let username = user.username.unwrap_or(existing.username);
        let email = user.email.unwrap_or(existing.email);
        let full_name = user.full_name.or(existing.full_name);
        let active = user.active.unwrap_or(existing.active);

        let updated_user = sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET username = $1, email = $2, full_name = $3, active = $4, updated_at = NOW()
            WHERE id = $5
            RETURNING *
            "#
        )
        .bind(&username)
        .bind(&email)
        .bind(&full_name)
        .bind(&active)
        .bind(id)
        .fetch_one(&self.pool)
        .await
        .map_err(AppError::DatabaseError)?;

        Ok(updated_user)
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        // Najpierw sprawdzamy, czy użytkownik istnieje
        let _ = self.find_by_id(id).await?;

        sqlx::query("DELETE FROM users WHERE id = $1")
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(AppError::DatabaseError)?;

        Ok(())
    }
}