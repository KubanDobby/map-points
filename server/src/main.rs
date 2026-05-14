mod handlers;

use axum::{
    routing::{get, post},
    Router,
};

use tower_http::cors::CorsLayer;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/points", post(handlers::create_point))
        .route("/points", get(handlers::get_points))
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:3000")
        .await
        .unwrap();

    println!("Server started on :3000");

    axum::serve(listener, app)
        .await
        .unwrap();
}