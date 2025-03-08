mod config;
mod error;
mod handlers;
mod models;
mod repository;
mod auth_utils;  // Nowy moduł dla funkcji hashowania haseł

use actix_web::{middleware, web, App, HttpServer, HttpResponse};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use log::info;

use crate::config::Config;
use crate::handlers::{create_user, delete_user, get_all_users, get_user_by_id, update_user, login};
use crate::repository::UserRepository;
use crate::error::AppError;

// Nowy handler do filtrowania użytkowników wg roli
async fn get_users_by_role(
    role: web::Path<String>,
    db_pool: web::Data<sqlx::PgPool>,
) -> Result<HttpResponse, AppError> {
    let repo = UserRepository::new(db_pool.get_ref().clone());
    let users = repo.find_by_role(&role).await?;
    
    let response: Vec<crate::models::UserResponse> = 
        users.into_iter().map(crate::models::UserResponse::from).collect();
    
    Ok(HttpResponse::Ok().json(response))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Inicjalizacja zmiennych środowiskowych i loggera
    dotenv().ok();
    env_logger::init_from_env(env_logger::Env::default().default_filter_or("info"));
    
    // Odczytanie konfiguracji
    let config = Config::from_env().expect("Failed to load configuration");
    
    // Utworzenie puli połączeń do bazy danych
    let pool = PgPoolOptions::new()
        .max_connections(config.db_max_connections)
        .connect(&config.database_url)
        .await
        .expect("Failed to create database connection pool");
    
    info!("Starting server at http://{}:{}", config.host, config.port);
    
    // Uruchomienie serwera HTTP
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/users")
                            .route("", web::get().to(get_all_users))
                            .route("", web::post().to(create_user))
                            .route("/role/{role}", web::get().to(get_users_by_role)) // Nowy endpoint
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