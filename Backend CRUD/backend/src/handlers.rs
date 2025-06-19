use axum::{extract::{Path, State}, http::StatusCode, response::Json};
use crate::{models::User, state::AppState};
use serde::Deserialize;
use sqlx::query_as;
use uuid::Uuid;


// Input DTO
#[derive(Deserialize)]
pub struct CreateUser {
    pub name: String,
    pub email: String
}

// Create a New Users
pub async fn create_user(
    State(state): State<AppState>,
    Json(input): Json<CreateUser>,
) -> Result<Json<User>, StatusCode> {
    let user = query_as!(
        User,
        "INSERT INTO users (name, email) VALUES ($1, $2) RETURNING id, name, email",
        input.name,
        input.email
    )
    .fetch_one(&*state.db)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;
    
    Ok(Json(user))
}


// Delete a Users
pub async fn delete_user(Path(id): Path<Uuid>, State(state): State<AppState>) -> Result<StatusCode, StatusCode> {
    let rows_affected = sqlx::query!("DELETE FROM users WHERE id = $1", id)
        .execute(&*state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?
        .rows_affected();

    if rows_affected == 0 {
        return Err(StatusCode::NOT_FOUND);
    }

    Ok(StatusCode::NO_CONTENT)
}
// Update existing Users


// Fetch all Users



// Fetch Users by Id