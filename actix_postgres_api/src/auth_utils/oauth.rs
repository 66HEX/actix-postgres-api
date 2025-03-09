use serde::{Deserialize, Serialize};
use std::env;
use reqwest::Client;
use crate::error::AppError;
use crate::models::role::UserRole;
use crate::models::User;
use crate::auth_utils::jwt::generate_token;
use sqlx::postgres::PgPool;
use crate::database::user::UserRepository;
use uuid::Uuid;

// OAuth provider configurations
#[derive(Debug, Clone)]
pub struct OAuthConfig {
    pub google_client_id: String,
    pub google_client_secret: String,
    pub facebook_client_id: String,
    pub facebook_client_secret: String,
    pub github_client_id: String,
    pub github_client_secret: String,
    pub redirect_url: String,
}

impl OAuthConfig {
    pub fn from_env() -> Self {
        Self {
            google_client_id: env::var("GOOGLE_CLIENT_ID").unwrap_or_default(),
            google_client_secret: env::var("GOOGLE_CLIENT_SECRET").unwrap_or_default(),
            facebook_client_id: env::var("FACEBOOK_CLIENT_ID").unwrap_or_default(),
            facebook_client_secret: env::var("FACEBOOK_CLIENT_SECRET").unwrap_or_default(),
            github_client_id: env::var("GITHUB_CLIENT_ID").unwrap_or_default(),
            github_client_secret: env::var("GITHUB_CLIENT_SECRET").unwrap_or_default(),
            redirect_url: env::var("OAUTH_REDIRECT_URL").unwrap_or_else(|_| "http://localhost:8080/auth/callback".to_string()),
        }
    }
}

// OAuth provider types
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum OAuthProvider {
    Google,
    Facebook,
    GitHub,
}

impl ToString for OAuthProvider {
    fn to_string(&self) -> String {
        match self {
            OAuthProvider::Google => "google".to_string(),
            OAuthProvider::Facebook => "facebook".to_string(),
            OAuthProvider::GitHub => "github".to_string(),
        }
    }
}

impl From<&str> for OAuthProvider {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "google" => OAuthProvider::Google,
            "facebook" => OAuthProvider::Facebook,
            "github" => OAuthProvider::GitHub,
            _ => OAuthProvider::Google, // Default to Google
        }
    }
}

// OAuth user info from providers
#[derive(Debug, Serialize, Deserialize)]
pub struct OAuthUserInfo {
    pub id: String,
    pub email: String,
    pub name: String,
    pub provider: String,
}

// OAuth service for handling authentication with providers
pub struct OAuthService {
    config: OAuthConfig,
    client: Client,
    repo: UserRepository,
}

impl OAuthService {
    pub fn new(pool: PgPool) -> Self {
        Self {
            config: OAuthConfig::from_env(),
            client: Client::new(),
            repo: UserRepository::new(pool),
        }
    }

    // Generate authorization URL for the specified provider
    pub fn get_authorization_url(&self, provider: OAuthProvider) -> String {
        match provider {
            OAuthProvider::Google => {
                format!(
                    "https://accounts.google.com/o/oauth2/v2/auth?client_id={}&redirect_uri={}&response_type=code&scope=email%20profile",
                    self.config.google_client_id,
                    self.config.redirect_url
                )
            }
            OAuthProvider::Facebook => {
                format!(
                    "https://www.facebook.com/v12.0/dialog/oauth?client_id={}&redirect_uri={}&response_type=code&scope=email,public_profile",
                    self.config.facebook_client_id,
                    self.config.redirect_url
                )
            }
            OAuthProvider::GitHub => {
                format!(
                    "https://github.com/login/oauth/authorize?client_id={}&redirect_uri={}&scope=user:email",
                    self.config.github_client_id,
                    self.config.redirect_url
                )
            }
        }
    }

    // Exchange authorization code for access token
    pub async fn exchange_code_for_token(
        &self,
        provider: OAuthProvider,
        code: &str,
    ) -> Result<String, AppError> {
        match provider {
            OAuthProvider::Google => {
                let params = [
                    ("client_id", self.config.google_client_id.as_str()),
                    ("client_secret", self.config.google_client_secret.as_str()),
                    ("code", code),
                    ("redirect_uri", self.config.redirect_url.as_str()),
                    ("grant_type", "authorization_code"),
                ];

                let response = self.client
                    .post("https://oauth2.googleapis.com/token")
                    .form(&params)
                    .send()
                    .await
                    .map_err(|e| AppError::InternalServerError(format!("OAuth token exchange error: {}", e)))?;

                let token_response: serde_json::Value = response.json().await
                    .map_err(|e| AppError::InternalServerError(format!("OAuth token parsing error: {}", e)))?;

                token_response.get("access_token")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
                    .ok_or_else(|| AppError::InternalServerError("Failed to get access token".to_string()))
            }
            OAuthProvider::Facebook => {
                let params = [
                    ("client_id", self.config.facebook_client_id.as_str()),
                    ("client_secret", self.config.facebook_client_secret.as_str()),
                    ("code", code),
                    ("redirect_uri", self.config.redirect_url.as_str()),
                ];

                let response = self.client
                    .get("https://graph.facebook.com/v12.0/oauth/access_token")
                    .query(&params)
                    .send()
                    .await
                    .map_err(|e| AppError::InternalServerError(format!("OAuth token exchange error: {}", e)))?;

                let token_response: serde_json::Value = response.json().await
                    .map_err(|e| AppError::InternalServerError(format!("OAuth token parsing error: {}", e)))?;

                token_response.get("access_token")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
                    .ok_or_else(|| AppError::InternalServerError("Failed to get access token".to_string()))
            }
            OAuthProvider::GitHub => {
                let params = [
                    ("client_id", self.config.github_client_id.as_str()),
                    ("client_secret", self.config.github_client_secret.as_str()),
                    ("code", code),
                    ("redirect_uri", self.config.redirect_url.as_str()),
                ];

                let response = self.client
                    .post("https://github.com/login/oauth/access_token")
                    .form(&params)
                    .header("Accept", "application/json")
                    .send()
                    .await
                    .map_err(|e| AppError::InternalServerError(format!("OAuth token exchange error: {}", e)))?;

                let token_response: serde_json::Value = response.json().await
                    .map_err(|e| AppError::InternalServerError(format!("OAuth token parsing error: {}", e)))?;

                token_response.get("access_token")
                    .and_then(|v| v.as_str())
                    .map(|s| s.to_string())
                    .ok_or_else(|| AppError::InternalServerError("Failed to get access token".to_string()))
            }
        }
    }

    // Get user info from provider using access token
    pub async fn get_user_info(
        &self,
        provider: OAuthProvider,
        access_token: &str,
    ) -> Result<OAuthUserInfo, AppError> {
        match provider {
            OAuthProvider::Google => {
                let response = self.client
                    .get("https://www.googleapis.com/oauth2/v2/userinfo")
                    .header("Authorization", format!("Bearer {}", access_token))
                    .send()
                    .await
                    .map_err(|e| AppError::InternalServerError(format!("OAuth user info error: {}", e)))?;

                let user_data: serde_json::Value = response.json().await
                    .map_err(|e| AppError::InternalServerError(format!("OAuth user info parsing error: {}", e)))?;

                let id = user_data.get("id")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| AppError::InternalServerError("Missing user ID".to_string()))?;

                let email = user_data.get("email")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| AppError::InternalServerError("Missing user email".to_string()))?;

                let name = user_data.get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Google User");

                Ok(OAuthUserInfo {
                    id: id.to_string(),
                    email: email.to_string(),
                    name: name.to_string(),
                    provider: "google".to_string(),
                })
            }
            OAuthProvider::Facebook => {
                let response = self.client
                    .get("https://graph.facebook.com/me")
                    .query(&[("fields", "id,name,email")])
                    .header("Authorization", format!("Bearer {}", access_token))
                    .send()
                    .await
                    .map_err(|e| AppError::InternalServerError(format!("OAuth user info error: {}", e)))?;

                let user_data: serde_json::Value = response.json().await
                    .map_err(|e| AppError::InternalServerError(format!("OAuth user info parsing error: {}", e)))?;

                let id = user_data.get("id")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| AppError::InternalServerError("Missing user ID".to_string()))?;

                let email = user_data.get("email")
                    .and_then(|v| v.as_str())
                    .ok_or_else(|| AppError::InternalServerError("Missing user email".to_string()))?;

                let name = user_data.get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("Facebook User");

                Ok(OAuthUserInfo {
                    id: id.to_string(),
                    email: email.to_string(),
                    name: name.to_string(),
                    provider: "facebook".to_string(),
                })
            }
            OAuthProvider::GitHub => {
                // Get user profile
                let response = self.client
                    .get("https://api.github.com/user")
                    .header("Authorization", format!("token {}", access_token))
                    .header("User-Agent", "Actix-Postgres-API")
                    .send()
                    .await
                    .map_err(|e| AppError::InternalServerError(format!("OAuth user info error: {}", e)))?;

                let user_data: serde_json::Value = response.json().await
                    .map_err(|e| AppError::InternalServerError(format!("OAuth user info parsing error: {}", e)))?;

                let id = user_data.get("id")
                    .and_then(|v| v.as_i64().map(|i| i.to_string()))
                    .ok_or_else(|| AppError::InternalServerError("Missing user ID".to_string()))?;

                let name = user_data.get("name")
                    .and_then(|v| v.as_str())
                    .unwrap_or("GitHub User");

                // GitHub requires a separate call to get email
                let email_response = self.client
                    .get("https://api.github.com/user/emails")
                    .header("Authorization", format!("token {}", access_token))
                    .header("User-Agent", "Actix-Postgres-API")
                    .send()
                    .await
                    .map_err(|e| AppError::InternalServerError(format!("OAuth email info error: {}", e)))?;

                let emails: Vec<serde_json::Value> = email_response.json().await
                    .map_err(|e| AppError::InternalServerError(format!("OAuth email info parsing error: {}", e)))?;

                // Find primary email
                let email = emails.iter()
                    .find(|e| e.get("primary").and_then(|v| v.as_bool()).unwrap_or(false))
                    .and_then(|e| e.get("email").and_then(|v| v.as_str()))
                    .ok_or_else(|| AppError::InternalServerError("Missing user email".to_string()))?;

                Ok(OAuthUserInfo {
                    id,
                    email: email.to_string(),
                    name: name.to_string(),
                    provider: "github".to_string(),
                })
            }
        }
    }

    // Process OAuth login - find or create user and generate JWT token
    pub async fn process_oauth_login(
        &self,
        provider: OAuthProvider,
        code: &str,
    ) -> Result<(User, String), AppError> {
        // Exchange authorization code for access token
        let access_token = self.exchange_code_for_token(provider.clone(), code).await?;
        
        // Get user info from provider
        let user_info = self.get_user_info(provider, &access_token).await?;
        
        // Check if user exists by email
        let user_result = self.repo.find_by_email(&user_info.email).await;
        
        let user = match user_result {
            Ok(existing_user) => {
                // User exists, return existing user
                existing_user
            },
            Err(_) => {
                // User doesn't exist, create new user
                // Generate a random password for OAuth users
                let random_password = Uuid::new_v4().to_string();
                
                // Create user with data from OAuth provider
                let new_user = crate::models::CreateUserRequest {
                    username: format!("{}_user", user_info.provider),
                    email: user_info.email,
                    password: random_password,
                    full_name: user_info.name,
                    phone_number: None,
                    role: Some("client".to_string()), // Default role for OAuth users
                };
                
                self.repo.create(new_user).await?
            }
        };
        
        // Generate JWT token
        let token = generate_token(user.id, &user.username, &user.email, &user.role)?;
        
        Ok((user, token))
    }
}