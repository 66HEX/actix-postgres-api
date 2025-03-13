use anyhow::Result;
use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    pub database_url: String,
    pub host: String,
    pub port: u16,
    pub db_max_connections: u32,
    pub ssl_cert_path: Option<String>,
    pub ssl_key_path: Option<String>,
    pub use_ssl: bool,
}

impl Config {
    pub fn from_env() -> Result<Self> {
        Ok(Self {
            database_url: env::var("DATABASE_URL")?,
            host: env::var("HOST").unwrap_or_else(|_| "127.0.0.1".to_string()),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8080".to_string())
                .parse()?,
            db_max_connections: env::var("DB_MAX_CONNECTIONS")
                .unwrap_or_else(|_| "5".to_string())
                .parse()?,
            ssl_cert_path: env::var("SSL_CERT_PATH").ok(),
            ssl_key_path: env::var("SSL_KEY_PATH").ok(),
            use_ssl: env::var("ENABLE_HTTPS")
                .unwrap_or_else(|_| "false".to_string())
                .parse()
                .unwrap_or(false),
        })
    }
}