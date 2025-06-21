use axum::{
    routing::{get, put},
    Router,
};
use crate::{handlers, state::AppState};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        .route("/users", get(handlers::get_users).post(handlers::create_user))
        .route(
            "/users/{id}",
                get(handlers::get_user)
                .put(handlers::update_user)
                .delete(handlers::delete_user))
        .with_state(state)
}