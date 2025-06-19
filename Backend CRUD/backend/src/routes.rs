use axum::{
    routing::{get, post, put, delete},
    Router,
};
use crate::{handlers, state::AppState};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/users", post(handlers::create_user))
        .with_state(state)
}