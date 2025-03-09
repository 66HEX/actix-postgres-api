use actix_web::{dev::ServiceRequest, Error, HttpMessage, HttpRequest};
use actix_web::error::ErrorUnauthorized;
use actix_web_httpauth::extractors::bearer::BearerAuth;
use futures::future::{ok, Ready};

use crate::auth_utils::jwt::{verify_token, extract_token_from_header};
use crate::error::AppError;
use crate::models::role::UserRole;

pub struct Auth;

impl Auth {
    // Validate request based on JWT token and required role
    pub fn validate_request(req: &HttpRequest, required_role: UserRole) -> Result<String, AppError> {
        // Extract the Authorization header
        let auth_header = req.headers().get("Authorization")
            .ok_or_else(|| AppError::ValidationError("Missing authorization header".to_string()))?;
        
        // Convert header to string
        let auth_str = auth_header.to_str()
            .map_err(|_| AppError::ValidationError("Invalid authorization header format".to_string()))?;
        
        // Extract token from header
        let token = extract_token_from_header(auth_str)?;
        
        // Verify token
        let claims = verify_token(token)?;
        
        // Check if user has required role
        let user_role = UserRole::from(claims.role.as_str());
        
        if user_role != required_role {
            return Err(AppError::ValidationError("Insufficient permissions".to_string()));
        }
        
        // Return user ID if validation successful
        Ok(claims.sub)
    }
    
    // Helper method to extract user ID from token without role check
    pub fn extract_user_id(req: &HttpRequest) -> Result<String, AppError> {
        // Extract the Authorization header
        let auth_header = req.headers().get("Authorization")
            .ok_or_else(|| AppError::ValidationError("Missing authorization header".to_string()))?;
        
        // Convert header to string
        let auth_str = auth_header.to_str()
            .map_err(|_| AppError::ValidationError("Invalid authorization header format".to_string()))?;
        
        // Extract token from header
        let token = extract_token_from_header(auth_str)?;
        
        // Verify token
        let claims = verify_token(token)?;
        
        // Return user ID
        Ok(claims.sub)
    }
}