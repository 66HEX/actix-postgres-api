use crate::error::AppError;
use crate::models::{CreateUserRequest, UpdateUserRequest, User, LoginRequest};
use crate::auth_utils::{hash_password, verify_password};
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

    pub async fn find_by_email(&self, email: &str) -> Result<User, AppError> {
        let user = sqlx::query_as::<_, User>(
            "SELECT * FROM users WHERE email = $1"
        )
        .bind(email)
        .fetch_optional(&self.pool)
        .await
        .map_err(AppError::DatabaseError)?;

        user.ok_or_else(|| AppError::NotFoundError(format!("User with email {} not found", email)))
    }

    pub async fn create(&self, user: CreateUserRequest) -> Result<User, AppError> {
        // Hashuj hasło przed zapisaniem
        let password_hash = hash_password(&user.password)?;
        
        let user = sqlx::query_as::<_, User>(
            r#"
            INSERT INTO users (username, email, password_hash, full_name, phone_number)
            VALUES ($1, $2, $3, $4, $5)
            RETURNING *
            "#
        )
        .bind(&user.username)
        .bind(&user.email)
        .bind(&password_hash)
        .bind(&user.full_name)
        .bind(&user.phone_number)
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
        let full_name = user.full_name.unwrap_or(existing.full_name);
        let phone_number = user.phone_number.or(existing.phone_number);
        let active = user.active.unwrap_or(existing.active);
        
        // Aktualizacja hasła tylko jeśli podano nowe
        let password_hash = match user.password {
            Some(new_password) => hash_password(&new_password)?,
            None => existing.password_hash
        };

        let updated_user = sqlx::query_as::<_, User>(
            r#"
            UPDATE users
            SET username = $1, email = $2, password_hash = $3, full_name = $4, 
                phone_number = $5, active = $6, updated_at = NOW()
            WHERE id = $7
            RETURNING *
            "#
        )
        .bind(&username)
        .bind(&email)
        .bind(&password_hash)
        .bind(&full_name)
        .bind(&phone_number)
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
    
    pub async fn authenticate(&self, login: LoginRequest) -> Result<User, AppError> {
        // Znajdź użytkownika po emailu
        let user = self.find_by_email(&login.email).await?;
        
        // Zweryfikuj hasło
        let is_valid = verify_password(&login.password, &user.password_hash)?;
        
        if !is_valid {
            return Err(AppError::ValidationError("Invalid credentials".to_string()));
        }
        
        // Jeśli konto jest nieaktywne, zwróć błąd
        if !user.active {
            return Err(AppError::ValidationError("Account is inactive".to_string()));
        }
        
        Ok(user)
    }
}