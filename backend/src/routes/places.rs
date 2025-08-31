use crate::models::structs;
use axum::{extract::State, http::StatusCode, Json};
use sqlx::SqlitePool;

pub async fn get_places(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<structs::Place>>, StatusCode> {
    let places = sqlx::query_as::<_, structs::Place>(
        "SELECT id, name, image, description, price, location, liked FROM places WHERE liked = 0",
    )
    .fetch_all(&pool)
    .await;

    match places {
        Ok(places) => Ok(Json(places)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
