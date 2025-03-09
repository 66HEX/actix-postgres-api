use actix_web::{web, HttpResponse, HttpRequest};
use sqlx::postgres::PgPool;

use crate::error::AppError;
use crate::models::LoginResponse;
use crate::models::user::UserResponse;
use crate::auth_utils::oauth::{OAuthService, OAuthProvider};

// Handler to initiate OAuth login - redirects to provider's authorization page
pub async fn oauth_login(
    provider_name: web::Path<String>,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    // Convert provider name to OAuthProvider enum
    let provider = OAuthProvider::from(provider_name.as_str());
    
    // Create OAuth service
    let oauth_service = OAuthService::new(db_pool.get_ref().clone());
    
    // Get authorization URL
    let auth_url = oauth_service.get_authorization_url(provider);
    
    // Redirect to provider's authorization page
    Ok(HttpResponse::Found()
        .append_header(("Location", auth_url))
        .finish())
}

// Handler for OAuth callback - processes the authorization code
pub async fn oauth_callback(
    req: HttpRequest,
    db_pool: web::Data<PgPool>,
) -> Result<HttpResponse, AppError> {
    // Extract query parameters
    let query_string = req.query_string();
    let query_params: Vec<(String, String)> = url::form_urlencoded::parse(query_string.as_bytes())
        .into_owned()
        .collect();
    
    // Extract code and provider from query parameters
    let code = query_params.iter()
        .find(|(key, _)| key == "code")
        .map(|(_, value)| value.clone())
        .ok_or_else(|| AppError::ValidationError("Missing authorization code".to_string()))?;
    
    let provider_name = query_params.iter()
        .find(|(key, _)| key == "provider")
        .map(|(_, value)| value.clone())
        .ok_or_else(|| AppError::ValidationError("Missing provider".to_string()))?;
    
    let provider = OAuthProvider::from(provider_name.as_str());
    
    // Create OAuth service
    let oauth_service = OAuthService::new(db_pool.get_ref().clone());
    
    // Process OAuth login
    let (user, token) = oauth_service.process_oauth_login(provider, &code).await?;
    
    // Create success response
    let response = LoginResponse {
        user: UserResponse::from(user),
        token,
        message: "OAuth login successful".to_string(),
    };
    
    // Return success response
    Ok(HttpResponse::Ok().json(response))
}