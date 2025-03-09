use sqlx::postgres::{PgPool, PgPoolOptions};
use anyhow::Result;
use crate::config::Config;

#[derive(Clone)]
pub struct DatabasePool {
    pool: PgPool,
}

impl DatabasePool {
    pub async fn new(config: &Config) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(config.db_max_connections)
            .connect(&config.database_url)
            .await?
;
        
        tracing::info!("Database connection pool created successfully");
        
        Ok(Self { pool })
    }
    
    pub fn get_pool(&self) -> PgPool {
        self.pool.clone()
    }
}