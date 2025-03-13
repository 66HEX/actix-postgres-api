mod config;
mod error;
mod handlers;
mod models;
mod database;  // New database module
mod auth_utils;
mod monitoring;  // New monitoring module
mod logging;     // New logging module
mod middleware;  // New middleware module
mod services;    // New services module

use actix_web::{middleware::Logger, web, App, HttpServer, HttpResponse};
use actix_web_prom::PrometheusMetricsBuilder;
use dotenv::dotenv;
use std::time::Duration;
use tokio::task;
use tokio::time;
use tracing_actix_web::TracingLogger;
// Use the specific rustls version that actix-web expects
// Use the specific rustls version that actix-web expects
use rustls::ServerConfig;
use rustls_pemfile::{certs, pkcs8_private_keys};
use std::fs::File;
use std::io::BufReader;


use crate::config::Config;
use crate::handlers::{create_user, delete_user, get_all_users, get_user_by_id, update_user, login, get_users_by_role, get_user_statistics, oauth_login, oauth_callback, ws_connect, get_chat_rooms, get_room_messages, create_chat_room};
// These imports are kept for potential future use
#[allow(unused_imports)]
use crate::database::user::UserRepository;
#[allow(unused_imports)]
use crate::error::AppError;
use crate::logging::init_logging;
use crate::middleware::{CustomRootSpanBuilder, PerformanceMetrics};
use crate::monitoring::update_memory_usage;



// Endpoint to expose application health status
async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "up",
        "version": env!("CARGO_PKG_VERSION")
    }))
}

// Function to load SSL configuration
fn load_rustls_config(cert_path: &str, key_path: &str) -> ServerConfig {
    // Load certificate chain
    let mut cert_file = BufReader::new(File::open(cert_path).expect("Failed to open certificate file"));
    let cert_chain = certs(&mut cert_file).expect("Failed to parse certificate")
        .into_iter()
        .map(|c| rustls::Certificate(c))
        .collect();
    
    // Load private key
    let mut key_file = BufReader::new(File::open(key_path).expect("Failed to open private key file"));
    let mut keys = pkcs8_private_keys(&mut key_file).expect("Failed to parse private key");
    let key = rustls::PrivateKey(keys.remove(0));
    
    // Create server config using rustls 0.20 API which is compatible with actix-web
    let config = ServerConfig::builder()
        .with_safe_defaults()
        .with_no_client_auth()
        .with_single_cert(cert_chain, key)
        .expect("Failed to create server config");
    
    config
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize environment variables
    dotenv().ok();
    
    // Initialize structured logging
    init_logging("actix_postgres_api", "info");
    
    // Log startup information
    tracing::info!("Starting up application");
    tracing::info!("Version: {}", env!("CARGO_PKG_VERSION"));
    
    // Read configuration
    let config = Config::from_env().expect("Failed to load configuration");
    tracing::info!("Configuration loaded: {:?}", config);
    
    // Create database connection pool using our new database module
    let db_pool = database::connection::DatabasePool::new(&config)
        .await
        .expect("Failed to create database connection pool");
    
    let pool = db_pool.get_pool();
    
    // Setup Prometheus metrics
    let prometheus = PrometheusMetricsBuilder::new("api")
        .endpoint("/metrics")
        .build()
        .unwrap();
    
    tracing::info!("Starting memory usage monitoring task");
    
    // Start a background task to update memory usage metric periodically
    task::spawn(async {
        let mut interval = time::interval(Duration::from_secs(5));
        loop {
            interval.tick().await;
            update_memory_usage();
        }
    });
    
    // Create the HTTP server
    let mut server = HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            // Add Prometheus metrics
            .wrap(prometheus.clone())
            // Add tracing logger instead of standard logger
            .wrap(TracingLogger::<CustomRootSpanBuilder>::new())
            // Add performance metrics middleware
            .wrap(PerformanceMetrics)
            // Add standard logger as a fallback
            .wrap(Logger::default())
            // Health check endpoint
            .route("/health", web::get().to(health_check))
            // API routes
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/users")
                            .route("", web::get().to(get_all_users))
                            .route("", web::post().to(create_user))
                            .route("/role/{role}", web::get().to(get_users_by_role))
                            .route("/statistics", web::get().to(get_user_statistics))
                            .route("/{id}", web::get().to(get_user_by_id))
                            .route("/{id}", web::put().to(update_user))
                            .route("/{id}", web::delete().to(delete_user))
                    )
                    .service(
                        web::scope("/auth")
                            .route("/login", web::post().to(login))
                            .route("/oauth/{provider}", web::get().to(oauth_login))
                            .route("/oauth/callback", web::get().to(oauth_callback))
                    )
                    .service(
                        web::scope("/chat")
                            .route("/ws", web::get().to(ws_connect))  // WebSocket endpoint (will be WSS when SSL is enabled)
                            .route("/rooms", web::get().to(get_chat_rooms))
                            .route("/rooms", web::post().to(create_chat_room))
                            .route("/rooms/{room_id}/messages", web::get().to(get_room_messages))
                    )
            )
    });
    
    // Determine if we should use HTTPS or HTTP
    if config.use_ssl {
        if let (Some(cert_path), Some(key_path)) = (&config.ssl_cert_path, &config.ssl_key_path) {
            tracing::info!("Starting HTTPS server with WSS support at https://{}:{}", config.host, config.port);
            let rustls_config = load_rustls_config(cert_path, key_path);
            server = server.bind_rustls(format!("{}:{}", config.host, config.port), rustls_config)?
        } else {
            tracing::warn!("SSL is enabled but certificate or key path is missing. Falling back to HTTP.");
            tracing::info!("Starting HTTP server at http://{}:{}", config.host, config.port);
            server = server.bind(format!("{}:{}", config.host, config.port))?
        }
    } else {
        tracing::info!("Starting HTTP server at http://{}:{}", config.host, config.port);
        server = server.bind(format!("{}:{}", config.host, config.port))?
    }
    
    server.run().await
}