mod config;
mod error;
mod handlers;
mod models;
mod repository;
mod auth_utils;
mod monitoring;  // New monitoring module
mod logging;     // New logging module
mod middleware;  // New middleware module

use actix_web::{middleware::Logger, web, App, HttpServer, HttpResponse};
use actix_web_prom::{PrometheusMetricsBuilder, PrometheusMetrics};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use tokio::task;
use tokio::time;
use tracing_actix_web::TracingLogger;

use crate::config::Config;
use crate::handlers::{create_user, delete_user, get_all_users, get_user_by_id, update_user, login, get_users_by_role};
use crate::repository::UserRepository;
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
    
    // Create database connection pool
    let pool = PgPoolOptions::new()
        .max_connections(config.db_max_connections)
        .connect(&config.database_url)
        .await
        .expect("Failed to create database connection pool");
    
    tracing::info!("Database connection pool created successfully");
    
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
    
    tracing::info!("Starting server at http://{}:{}", config.host, config.port);
    
    // Start HTTP server
    HttpServer::new(move || {
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
                            .route("/{id}", web::get().to(get_user_by_id))
                            .route("/{id}", web::put().to(update_user))
                            .route("/{id}", web::delete().to(delete_user))
                    )
                    .service(
                        web::scope("/auth")
                            .route("/login", web::post().to(login))
                    )
            )
    })
    .bind(format!("{}:{}", config.host, config.port))?
    .run()
    .await
}