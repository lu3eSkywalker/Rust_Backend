mod routes;
mod handlers;
mod models;
mod db;
mod state;

use dotenv::dotenv;
use std::sync::Arc;
use tokio::net::TcpListener;
use crate::db::init_db;
use crate::state::AppState;


#[tokio::main]
async fn main() {
    dotenv().ok();
    
    let db_pool = init_db().await;

        // Create shared application state
        let state = AppState {
            db: Arc::new(db_pool),
        };
    
        // Set up routes and pass application state
        let app = routes::create_router(state);
    
        let listener = TcpListener::bind("127.0.0.1:3000").await.unwrap();
        println!("Server running on http://127.0.0.1:3000");
        axum::serve(listener, app).await.unwrap();

}