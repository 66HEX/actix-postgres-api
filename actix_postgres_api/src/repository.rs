use crate::error::AppError;
use crate::models::{CreateUserRequest, UpdateUserRequest, User, LoginRequest};
use crate::auth_utils::{hash_password, verify_password, validate_role};
use crate::monitoring::DbMetrics;
use crate::logging::create_db_span;
use sqlx::{postgres::PgPool, types::Uuid};
use tracing::Instrument;

pub struct UserRepository {
    pool: PgPool,
}

impl UserRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub async fn find_all(&self) -> Result<Vec<User>, AppError> {
        // Create a span for this database operation
        let span = create_db_span(
            "find_all_users",
            "SELECT * FROM users ORDER BY created_at DESC",
            "None",
        );
        
        // Track DB metrics and wrap in a span
        DbMetrics::track("SELECT", "users", || async {
            let users = sqlx::query_as::<_, User>(
                "SELECT * FROM users ORDER BY created_at DESC"
            )
            .fetch_all(&self.pool)
            .await
            .map_err(AppError::DatabaseError)?;

            Ok(users)
        }).instrument(span).await
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<User, AppError> {
        // Unikamy używania format! jako tymczasowej wartości
        let params = format!("id={}", id);
        let span = create_db_span(
            "find_user_by_id",
            "SELECT * FROM users WHERE id = $1",
            &params,
        );
        
        DbMetrics::track("SELECT", "users", || async {
            let user = sqlx::query_as::<_, User>(
                "SELECT * FROM users WHERE id = $1"
            )
            .bind(id)
            .fetch_optional(&self.pool)
            .await
            .map_err(AppError::DatabaseError)?;

            user.ok_or_else(|| AppError::NotFoundError(format!("User with id {} not found", id)))
        }).instrument(span).await
    }

    pub async fn find_by_email(&self, email: &str) -> Result<User, AppError> {
        let params = format!("email={}", email);
        let span = create_db_span(
            "find_user_by_email",
            "SELECT * FROM users WHERE email = $1",
            &params,
        );
        
        DbMetrics::track("SELECT", "users", || async {
            let user = sqlx::query_as::<_, User>(
                "SELECT * FROM users WHERE email = $1"
            )
            .bind(email)
            .fetch_optional(&self.pool)
            .await
            .map_err(AppError::DatabaseError)?;

            user.ok_or_else(|| AppError::NotFoundError(format!("User with email {} not found", email)))
        }).instrument(span).await
    }

    pub async fn create(&self, user: CreateUserRequest) -> Result<User, AppError> {
        let params = format!("username={}, email={}", user.username, user.email);
        let span = create_db_span(
            "create_user",
            "INSERT INTO users (username, email, password_hash, full_name, phone_number, role) VALUES ($1, $2, $3, $4, $5, $6)",
            &params,
        );
        
        DbMetrics::track("INSERT", "users", || async {
            // Hashuj hasło przed zapisaniem
            let password_hash = hash_password(&user.password)?;
            
            // Ustaw domyślną rolę client, jeśli nie podano
            let role = match user.role {
                Some(role) => role,
                None => "client".to_string()
            };
            
            let user = sqlx::query_as::<_, User>(
                r#"
                INSERT INTO users (username, email, password_hash, full_name, phone_number, role)
                VALUES ($1, $2, $3, $4, $5, $6)
                RETURNING *
                "#
            )
            .bind(&user.username)
            .bind(&user.email)
            .bind(&password_hash)
            .bind(&user.full_name)
            .bind(&user.phone_number)
            .bind(&role)
            .fetch_one(&self.pool)
            .await
            .map_err(AppError::DatabaseError)?;

            Ok(user)
        }).instrument(span).await
    }

    pub async fn update(&self, id: Uuid, user: UpdateUserRequest) -> Result<User, AppError> {
        let params = format!("id={}", id);
        let span = create_db_span(
            "update_user",
            "UPDATE users SET ... WHERE id = $1",
            &params,
        );
        
        DbMetrics::track("UPDATE", "users", || async {
            // Najpierw sprawdzamy, czy użytkownik istnieje
            let existing = self.find_by_id(id).await?;

            // Przygotowanie wartości do aktualizacji
            let username = user.username.unwrap_or(existing.username);
            let email = user.email.unwrap_or(existing.email);
            let full_name = user.full_name.unwrap_or(existing.full_name);
            let phone_number = user.phone_number.or(existing.phone_number);
            let active = user.active.unwrap_or(existing.active);
            let role = user.role.unwrap_or(existing.role);
            
            // Aktualizacja hasła tylko jeśli podano nowe
            let password_hash = match user.password {
                Some(new_password) => hash_password(&new_password)?,
                None => existing.password_hash
            };

            tracing::debug!(
                "Updating user: id={}, username={}, email={}, active={}, role={}",
                id, username, email, active, role
            );

            let updated_user = sqlx::query_as::<_, User>(
                r#"
                UPDATE users
                SET username = $1, email = $2, password_hash = $3, full_name = $4, 
                    phone_number = $5, active = $6, role = $7, updated_at = NOW()
                WHERE id = $8
                RETURNING *
                "#
            )
            .bind(&username)
            .bind(&email)
            .bind(&password_hash)
            .bind(&full_name)
            .bind(&phone_number)
            .bind(&active)
            .bind(&role)
            .bind(id)
            .fetch_one(&self.pool)
            .await
            .map_err(AppError::DatabaseError)?;

            Ok(updated_user)
        }).instrument(span).await
    }

    pub async fn delete(&self, id: Uuid) -> Result<(), AppError> {
        let params = format!("id={}", id);
        let span = create_db_span(
            "delete_user",
            "DELETE FROM users WHERE id = $1",
            &params,
        );
        
        DbMetrics::track("DELETE", "users", || async {
            // Najpierw sprawdzamy, czy użytkownik istnieje
            let _ = self.find_by_id(id).await?;

            tracing::info!("Deleting user with id={}", id);

            sqlx::query("DELETE FROM users WHERE id = $1")
                .bind(id)
                .execute(&self.pool)
                .await
                .map_err(AppError::DatabaseError)?;

            Ok(())
        }).instrument(span).await
    }
    
    pub async fn authenticate(&self, login: LoginRequest) -> Result<User, AppError> {
        let params = format!("email={}", login.email);
        let span = create_db_span(
            "authenticate_user",
            "SELECT * FROM users WHERE email = $1",
            &params,
        );
        
        DbMetrics::track("SELECT", "users", || async {
            // Znajdź użytkownika po emailu
            let user = self.find_by_email(&login.email).await?;
            
            // Zweryfikuj hasło
            let is_valid = verify_password(&login.password, &user.password_hash)?;
            
            if !is_valid {
                tracing::warn!("Authentication failed for user: {}", login.email);
                return Err(AppError::ValidationError("Invalid credentials".to_string()));
            }
            
            // Jeśli konto jest nieaktywne, zwróć błąd
            if !user.active {
                tracing::warn!("Attempt to log in to inactive account: {}", login.email);
                return Err(AppError::ValidationError("Account is inactive".to_string()));
            }
            
            tracing::info!("User authenticated successfully: {}", login.email);
            Ok(user)
        }).instrument(span).await
    }
    
    pub async fn find_by_role(&self, role: &str) -> Result<Vec<User>, AppError> {
        let params = format!("role={}", role);
        let span = create_db_span(
            "find_users_by_role",
            "SELECT * FROM users WHERE role = $1 ORDER BY created_at DESC",
            &params,
        );
        
        DbMetrics::track("SELECT", "users", || async {
            // Waliduj rolę
            let valid_role = validate_role(role)?;
            
            tracing::debug!("Finding users with role: {}", valid_role);

            let users = sqlx::query_as::<_, User>(
                "SELECT * FROM users WHERE role = $1 ORDER BY created_at DESC"
            )
            .bind(&valid_role)
            .fetch_all(&self.pool)
            .await
            .map_err(AppError::DatabaseError)?;
            
            tracing::debug!("Found {} users with role '{}'", users.len(), valid_role);
            Ok(users)
        }).instrument(span).await
    }
    
    // Dodatkowa metoda pomocnicza do statystyk
    pub async fn count_users_by_role(&self) -> Result<Vec<(String, i64)>, AppError> {
        let span = create_db_span(
            "count_users_by_role",
            "SELECT role, COUNT(*) FROM users GROUP BY role",
            "None",
        );
        
        DbMetrics::track("SELECT", "users", || async {
            let counts = sqlx::query!(
                "SELECT role, COUNT(*) as count FROM users GROUP BY role"
            )
            .fetch_all(&self.pool)
            .await
            .map_err(AppError::DatabaseError)?;
            
            let result = counts
                .into_iter()
                .map(|row| (row.role, row.count.unwrap_or(0)))
                .collect();
                
            Ok(result)
        }).instrument(span).await
    }
    
    // Metoda do monitorowania nieaktywnych użytkowników
    pub async fn count_inactive_users(&self) -> Result<i64, AppError> {
        let span = create_db_span(
            "count_inactive_users",
            "SELECT COUNT(*) FROM users WHERE active = false",
            "None",
        );
        
        DbMetrics::track("SELECT", "users", || async {
            let result = sqlx::query!(
                "SELECT COUNT(*) as count FROM users WHERE active = false"
            )
            .fetch_one(&self.pool)
            .await
            .map_err(AppError::DatabaseError)?;
            
            Ok(result.count.unwrap_or(0))
        }).instrument(span).await
    }
}