use actix_web::{test, web, App};
use sqlx::postgres::PgPoolOptions;
use user_crud_api::config::Config;
use user_crud_api::handlers::{create_user, delete_user, get_all_users, get_user_by_id, update_user};
use user_crud_api::models::{CreateUserRequest, UpdateUserRequest, User};
use uuid::Uuid;

async fn setup_test_app() -> impl actix_web::dev::Service<
    actix_http::Request,
    Response = actix_web::dev::ServiceResponse,
    Error = actix_web::Error,
> {
    // Do testów używamy testowej bazy danych
    std::env::set_var("DATABASE_URL", "postgres://postgres:admin@localhost/user_crud_test?sslmode=prefer");
    
    let config = Config::from_env().expect("Failed to load configuration");
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(&config.database_url)
        .await
        .expect("Failed to create database connection pool");
    
    // Czyścimy tabelę przed testami
    sqlx::query("TRUNCATE TABLE users")
        .execute(&pool)
        .await
        .expect("Failed to clean test database");
    
    test::init_service(
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .service(
                web::scope("/api")
                    .service(
                        web::scope("/users")
                            .route("", web::get().to(get_all_users))
                            .route("", web::post().to(create_user))
                            .route("/{id}", web::get().to(get_user_by_id))
                            .route("/{id}", web::put().to(update_user))
                            .route("/{id}", web::delete().to(delete_user))
                    )
            )
    ).await
}

#[actix_web::test]
async fn test_create_and_get_user() {
    let app = setup_test_app().await;
    
    // Tworzenie użytkownika
    let create_req = CreateUserRequest {
        username: "testuser".to_string(),
        email: "test@example.com".to_string(),
        full_name: Some("Test User".to_string()),
    };
    
    let resp = test::TestRequest::post()
        .uri("/api/users")
        .set_json(&create_req)
        .send_request(&app)
        .await;
    
    assert!(resp.status().is_success());
    
    let created_user: serde_json::Value = test::read_body_json(resp).await;
    let user_id = created_user["id"].as_str().unwrap();
    
    // Pobieranie utworzonego użytkownika
    let resp = test::TestRequest::get()
        .uri(&format!("/api/users/{}", user_id))
        .send_request(&app)
        .await;
    
    assert!(resp.status().is_success());
    
    let user: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(user["username"], "testuser");
    assert_eq!(user["email"], "test@example.com");
    assert_eq!(user["full_name"], "Test User");
}

#[actix_web::test]
async fn test_update_user() {
    let app = setup_test_app().await;
    
    // Tworzenie użytkownika
    let create_req = CreateUserRequest {
        username: "updateuser".to_string(),
        email: "update@example.com".to_string(),
        full_name: Some("Update User".to_string()),
    };
    
    let resp = test::TestRequest::post()
        .uri("/api/users")
        .set_json(&create_req)
        .send_request(&app)
        .await;
    
    let created_user: serde_json::Value = test::read_body_json(resp).await;
    let user_id = created_user["id"].as_str().unwrap();
    
    // Aktualizacja użytkownika
    let update_req = UpdateUserRequest {
        username: Some("updateduser".to_string()),
        email: None,
        full_name: Some("Updated User".to_string()),
        active: Some(false),
    };
    
    let resp = test::TestRequest::put()
        .uri(&format!("/api/users/{}", user_id))
        .set_json(&update_req)
        .send_request(&app)
        .await;
    
    assert!(resp.status().is_success());
    
    let updated_user: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(updated_user["username"], "updateduser");
    assert_eq!(updated_user["email"], "update@example.com"); // Nie zmieniono
    assert_eq!(updated_user["full_name"], "Updated User");
    assert_eq!(updated_user["active"], false);
}

#[actix_web::test]
async fn test_delete_user() {
    let app = setup_test_app().await;
    
    // Tworzenie użytkownika
    let create_req = CreateUserRequest {
        username: "deleteuser".to_string(),
        email: "delete@example.com".to_string(),
        full_name: None,
    };
    
    let resp = test::TestRequest::post()
        .uri("/api/users")
        .set_json(&create_req)
        .send_request(&app)
        .await;
    
    let created_user: serde_json::Value = test::read_body_json(resp).await;
    let user_id = created_user["id"].as_str().unwrap();
    
    // Usuwanie użytkownika
    let resp = test::TestRequest::delete()
        .uri(&format!("/api/users/{}", user_id))
        .send_request(&app)
        .await;
    
    assert_eq!(resp.status().as_u16(), 204); // No Content
    
    // Próba pobrania usuniętego użytkownika
    let resp = test::TestRequest::get()
        .uri(&format!("/api/users/{}", user_id))
        .send_request(&app)
        .await;
    
    assert_eq!(resp.status().as_u16(), 404); // Not Found
}

#[actix_web::test]
async fn test_get_all_users() {
    let app = setup_test_app().await;
    
    // Pobieramy początkową liczbę użytkowników
    let initial_resp = test::TestRequest::get()
        .uri("/api/users")
        .send_request(&app)
        .await;
    
    assert!(initial_resp.status().is_success());
    let initial_users: Vec<serde_json::Value> = test::read_body_json(initial_resp).await;
    let initial_count = initial_users.len();
    
    // Generate a truly unique prefix using a full UUID
    let unique_prefix = format!("getall_{}", uuid::Uuid::new_v4().to_string());
    
    // Tworzenie kilku użytkowników z unikalnym prefiksem
    for i in 1..=3 {
        let create_req = CreateUserRequest {
            username: format!("{}_user{}", unique_prefix, i),
            email: format!("{}_user{}@example.com", unique_prefix, i),
            full_name: Some(format!("User {}", i)),
        };
        
        let resp = test::TestRequest::post()
            .uri("/api/users")
            .set_json(&create_req)
            .send_request(&app)
            .await;
            
        assert!(resp.status().is_success());
    }
    
    // Pobieranie wszystkich użytkowników po dodaniu nowych
    let resp = test::TestRequest::get()
        .uri("/api/users")
        .send_request(&app)
        .await;
    
    assert!(resp.status().is_success());
    
    let users: Vec<serde_json::Value> = test::read_body_json(resp).await;
    
    // Sprawdzamy, czy liczba użytkowników zwiększyła się dokładnie o 3
    assert_eq!(users.len(), initial_count + 3);
    
    // Dodatkowo sprawdzamy, czy utworzeni przez nas użytkownicy są w wynikach
    // Use the exact full prefix to avoid any chance of collision
    let created_users: Vec<_> = users.into_iter()
        .filter(|user| {
            let username = user["username"].as_str().unwrap_or("");
            username.starts_with(&unique_prefix)
        })
        .collect();
    
    assert_eq!(created_users.len(), 3);
}