use std::sync::Arc;
use sqlx::PgPool;

/// Application state containing shared resources (like database connection).
#[derive(Clone)]
pub struct AppState {
    pub db: Arc<PgPool>,
}