use actix_web::{test, web, App};
use sqlx::postgres::PgPoolOptions;
use actix_postgres_api::config::Config;
use actix_postgres_api::handlers::{create_user, delete_user, get_all_users, get_user_by_id, update_user, login};
use actix_postgres_api::models::{CreateUserRequest, UpdateUserRequest, LoginRequest};

// Przygotowanie środowiska testowego
async fn setup_test_app() -> impl actix_web::dev::Service<
    actix_http::Request,
    Response = actix_web::dev::ServiceResponse,
    Error = actix_web::Error,
> {
    // Ustawienie URL do bazy testowej
    std::env::set_var("DATABASE_URL", "postgres://postgres:admin@localhost/actix_postgres_api_test?sslmode=prefer");
    
    let config = Config::from_env().expect("Failed to load configuration");
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(&config.database_url)
        .await
        .expect("Failed to create database connection pool");
    
    // Przed testami czyścimy tabelę users
    sqlx::query("TRUNCATE TABLE users CASCADE")
        .execute(&pool)
        .await
        .expect("Failed to clean test database");
    
    // Inicjalizacja aplikacji testowej
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
                    .service(
                        web::scope("/auth")
                            .route("/login", web::post().to(login))
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
        password: "Test1234".to_string(),
        full_name: "Test User".to_string(),
        phone_number: Some("+48 123 456 789".to_string()),
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
    assert_eq!(user["phone_number"], "+48 123 456 789");
}

#[actix_web::test]
async fn test_update_user() {
    let app = setup_test_app().await;
    
    // Tworzenie użytkownika
    let create_req = CreateUserRequest {
        username: "updateuser".to_string(),
        email: "update@example.com".to_string(),
        password: "Update1234".to_string(),
        full_name: "Update User".to_string(),
        phone_number: None,
    };
    
    let resp = test::TestRequest::post()
        .uri("/api/users")
        .set_json(&create_req)
        .send_request(&app)
        .await;
    
    assert!(resp.status().is_success());
    
    let created_user: serde_json::Value = test::read_body_json(resp).await;
    let user_id = created_user["id"].as_str().unwrap();
    
    // Aktualizacja użytkownika
    let update_req = UpdateUserRequest {
        username: Some("updateduser".to_string()),
        email: None,
        password: Some("NewPassword1234".to_string()),
        full_name: Some("Updated User".to_string()),
        phone_number: Some("+1 987 654 321".to_string()),
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
    assert_eq!(updated_user["phone_number"], "+1 987 654 321");
    assert_eq!(updated_user["active"], false);
}

#[actix_web::test]
async fn test_delete_user() {
    let app = setup_test_app().await;
    
    // Tworzenie użytkownika
    let create_req = CreateUserRequest {
        username: "deleteuser".to_string(),
        email: "delete@example.com".to_string(),
        password: "Delete1234".to_string(),
        full_name: "Delete User".to_string(),
        phone_number: None,
    };
    
    let resp = test::TestRequest::post()
        .uri("/api/users")
        .set_json(&create_req)
        .send_request(&app)
        .await;
    
    assert!(resp.status().is_success());
    
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
async fn test_login() {
    let app = setup_test_app().await;
    
    // Tworzenie użytkownika
    let create_req = CreateUserRequest {
        username: "loginuser".to_string(),
        email: "login@example.com".to_string(),
        password: "Login1234".to_string(),
        full_name: "Login User".to_string(),
        phone_number: None,
    };
    
    let resp = test::TestRequest::post()
        .uri("/api/users")
        .set_json(&create_req)
        .send_request(&app)
        .await;
    
    assert!(resp.status().is_success());
    
    // Próba logowania z poprawnymi danymi
    let login_req = LoginRequest {
        email: "login@example.com".to_string(),
        password: "Login1234".to_string(),
    };
    
    let resp = test::TestRequest::post()
        .uri("/api/auth/login")
        .set_json(&login_req)
        .send_request(&app)
        .await;
    
    assert!(resp.status().is_success());
    
    let login_resp: serde_json::Value = test::read_body_json(resp).await;
    assert_eq!(login_resp["user"]["username"], "loginuser");
    assert_eq!(login_resp["message"], "Login successful");
}

#[actix_web::test]
async fn test_weak_password_rejection() {
    let app = setup_test_app().await;
    
    // Próba utworzenia użytkownika ze zbyt słabym hasłem (brak dużej litery)
    let create_req = CreateUserRequest {
        username: "weakpassuser".to_string(),
        email: "weak@example.com".to_string(),
        password: "weak1234".to_string(), // brak dużej litery
        full_name: "Weak Password User".to_string(),
        phone_number: None,
    };
    
    let resp = test::TestRequest::post()
        .uri("/api/users")
        .set_json(&create_req)
        .send_request(&app)
        .await;
    
    assert_eq!(resp.status().as_u16(), 400); // Bad Request
}