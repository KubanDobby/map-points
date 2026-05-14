mod db;
mod errors;
mod handlers;
mod models;

use axum::{
    Router,
    routing::{get, post},
};

use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let pool = db::create_pool().await;

    let app = Router::new()
        .route("/points", post(handlers::create_point))
        .route("/points", get(handlers::get_points))
        .with_state(pool)
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();

    println!("Server started on :3000");

    axum::serve(listener, app).await.unwrap();
}
