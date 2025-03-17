use actix_cors::Cors;
use actix_web::http::header;
use std::env;

/// Creates a CORS middleware configuration for the application
/// 
/// This allows cross-origin requests from specified origins (like the Tauri app)
/// and configures the allowed methods, headers, and other CORS settings.
pub fn cors_middleware() -> Cors {
    // Get the allowed origin from environment or use a default for development
    let frontend_origin = env::var("FRONTEND_ORIGIN").unwrap_or_else(|_| "http://localhost:1420".to_string());
    
    Cors::default()
        // Allow the specific origin of your Tauri app
        .allowed_origin(&frontend_origin)
        // For development, you might want to allow localhost with different ports
        .allowed_origin("http://localhost:1420")
        // Allow credentials (cookies, authorization headers)
        .allow_any_method()
        .allow_any_header()
        .expose_headers(&[
            header::CONTENT_DISPOSITION,
            header::CONTENT_TYPE,
            header::CONTENT_LENGTH,
        ])
        .max_age(3600) // Cache preflight requests for 1 hour
        .supports_credentials()
}