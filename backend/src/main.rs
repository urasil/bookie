use axum::{
    routing::{get, post},
    Router,
};

use crate::db::setup::setup_database;
use crate::routes::{places::get_matches, places::get_places, places::like_place};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

pub mod app_state;
mod db;
mod google_places;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let google_places = google_places::GooglePlaces::new(
        std::env::var("GOOGLE_API_KEY").expect("GOOGLE_API_KEY must be set"),
    );

    let state = std::sync::Arc::new(app_state::AppState {
        gp: std::sync::Arc::new(google_places),
        pool: setup_database().await.expect("Failed to connect to db"),
    });

    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/places", get(get_places))
        .route("/like/:id", post(like_place))
        .route("/matches", get(get_matches))
        .with_state(state)
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    axum::serve(tokio::net::TcpListener::bind(&addr).await.unwrap(), app)
        .await
        .unwrap();
}
