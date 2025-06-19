use sqlx::{postgres::PgPoolOptions, PgPool};
use std::env;

/// Asynchronously initializes the database connection pool
pub async fn init_db() -> PgPool {
    // Load DATABASE_URL from the .env file
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    
    // Create and return a connection pool
    PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Failed to create database Pool")
}