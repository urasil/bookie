use axum::{
    routing::{get, post},
    Router,
};

use crate::db::setup::setup_database;
use crate::routes::{like::like_place, matches::get_matches, places::get_places};
use std::net::SocketAddr;
use tower_http::cors::{Any, CorsLayer};

mod db;
mod models;
mod routes;

#[tokio::main]
async fn main() {
    let pool = setup_database().await.expect("Failed to connect to db");

    let cors = CorsLayer::new()
        .allow_methods(Any)
        .allow_origin(Any)
        .allow_headers(Any);

    let app = Router::new()
        .route("/places", get(get_places))
        .route("/like/:id", post(like_place))
        .route("/matches", get(get_matches))
        .with_state(pool)
        .layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    println!("listening on {}", addr);
    axum::serve(tokio::net::TcpListener::bind(&addr).await.unwrap(), app)
        .await
        .unwrap();
}
