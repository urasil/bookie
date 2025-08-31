use crate::models::structs;
use axum::{extract::State, http::StatusCode, Json};
use sqlx::SqlitePool;

pub async fn get_matches(
    State(pool): State<SqlitePool>,
) -> Result<Json<Vec<structs::Place>>, StatusCode> {
    let matches = sqlx::query_as::<_, structs::Place>(
        "SELECT id, name, image, description, price, location, liked FROM places WHERE liked = 1",
    )
    .fetch_all(&pool)
    .await;

    match matches {
        Ok(matches) => Ok(Json(matches)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
