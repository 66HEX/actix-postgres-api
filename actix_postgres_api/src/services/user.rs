use sqlx::postgres::PgPool;
use uuid::Uuid as UuidTrait;

use crate::error::AppError;
use crate::models::{CreateUserRequest, UpdateUserRequest, User};
use crate::database::user::UserRepository;
use crate::auth_utils::{validate_password, validate_email, validate_phone_number, validate_username, validate_full_name, validate_role};

pub struct UserService {
    repo: UserRepository,
}

impl UserService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            repo: UserRepository::new(pool),
        }
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, AppError> {
        self.repo.find_all().await
    }

    pub async fn get_user_by_id(&self, id_str: &str) -> Result<User, AppError> {
        let user_id = UuidTrait::parse_str(id_str)
            .map_err(|_| AppError::ValidationError("Invalid UUID format".to_string()))?;
        
        self.repo.find_by_id(user_id).await
    }

    pub async fn create_user(&self, user: CreateUserRequest, pool: &PgPool) -> Result<User, AppError> {
        // Walidacja nazwy użytkownika
        validate_username(&user.username)?;
        
        // Walidacja adresu email
        validate_email(&user.email)?;
        
        // Walidacja pełnego imienia i nazwiska
        validate_full_name(&user.full_name)?;
        
        // Walidacja hasła
        validate_password(&user.password)?;
        
        // Validate phone number if provided
        if let Some(ref phone) = user.phone_number {
            validate_phone_number(phone)?;
        }
        
        // Walidacja roli, jeśli podano
        let role = match &user.role {
            Some(role) => Some(validate_role(role)?),
            None => None
        };
        
        // Sprawdź, czy email już istnieje
        let email_exists = sqlx::query!("SELECT COUNT(*) as count FROM users WHERE email = $1", user.email)
            .fetch_one(pool)
            .await
            .map_err(AppError::DatabaseError)?
            .count
            .unwrap_or(0) > 0;
            
        if email_exists {
            return Err(AppError::ValidationError("Email is already in use".to_string()));
        }
        
        // Sprawdź, czy nazwa użytkownika już istnieje
        let username_exists = sqlx::query!("SELECT COUNT(*) as count FROM users WHERE username = $1", user.username)
            .fetch_one(pool)
            .await
            .map_err(AppError::DatabaseError)?
            .count
            .unwrap_or(0) > 0;
            
        if username_exists {
            return Err(AppError::ValidationError("Username is already in use".to_string()));
        }
        
        // Jeśli walidacja roli zmieniła jej wartość, stwórz nowy obiekt z zaktualizowaną rolą
        let mut user_data = user;
        if let Some(validated_role) = role {
            user_data.role = Some(validated_role);
        }
        
        self.repo.create(user_data).await
    }

    pub async fn update_user(&self, id_str: &str, user: UpdateUserRequest, pool: &PgPool) -> Result<User, AppError> {
        let user_id = UuidTrait::parse_str(id_str)
            .map_err(|_| AppError::ValidationError("Invalid UUID format".to_string()))?;
        
        // Walidacja danych wejściowych
        if let Some(ref email) = user.email {
            validate_email(email)?;
            
            // Sprawdź, czy nowy email nie koliduje z istniejącym
            let email_exists = sqlx::query!(
                "SELECT COUNT(*) as count FROM users WHERE email = $1 AND id != $2", 
                email, user_id
            )
            .fetch_one(pool)
            .await
            .map_err(AppError::DatabaseError)?
            .count
            .unwrap_or(0) > 0;
                
            if email_exists {
                return Err(AppError::ValidationError("Email is already in use".to_string()));
            }
        }
        
        if let Some(ref username) = user.username {
            validate_username(username)?;
            
            // Sprawdź, czy nowa nazwa użytkownika nie koliduje z istniejącą
            let username_exists = sqlx::query!(
                "SELECT COUNT(*) as count FROM users WHERE username = $1 AND id != $2", 
                username, user_id
            )
            .fetch_one(pool)
            .await
            .map_err(AppError::DatabaseError)?
            .count
            .unwrap_or(0) > 0;
                
            if username_exists {
                return Err(AppError::ValidationError("Username is already in use".to_string()));
            }
        }
        
        if let Some(ref full_name) = user.full_name {
            validate_full_name(full_name)?;
        }
        
        // Walidacja hasła, jeśli jest aktualizowane
        if let Some(ref password) = user.password {
            validate_password(password)?;
        }
        
        // Validate phone number if provided
        if let Some(ref phone) = user.phone_number {
            validate_phone_number(phone)?;
        }
        
        // Walidacja roli, jeśli jest aktualizowana
        let mut user_data = user;
        if let Some(ref role) = user_data.role {
            user_data.role = Some(validate_role(role)?); 
        }
        
        self.repo.update(user_id, user_data).await
    }

    pub async fn delete_user(&self, id_str: &str) -> Result<(), AppError> {
        let user_id = UuidTrait::parse_str(id_str)
            .map_err(|_| AppError::ValidationError("Invalid UUID format".to_string()))?;
        
        self.repo.delete(user_id).await
    }

    pub async fn get_users_by_role(&self, role: &str) -> Result<Vec<User>, AppError> {
        self.repo.find_by_role(role).await
    }
    
    // Dodatkowe metody pomocnicze
    pub async fn count_users_by_role(&self) -> Result<Vec<(String, i64)>, AppError> {
        self.repo.count_users_by_role().await
    }
    
    pub async fn count_inactive_users(&self) -> Result<i64, AppError> {
        self.repo.count_inactive_users().await
    }
}