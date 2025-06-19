use axum::{
    routing::{get, post, put, delete},
};
use serde::{Deserialize, Serialize};
use uuid::Uuid;


// User Model
#[derive(Serialize, Deserialize, Debug)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}