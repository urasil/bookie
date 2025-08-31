use crate::models::structs;
use axum::{
    extract::{Path, State},
    http::StatusCode,
    Json,
};
use sqlx::SqlitePool;

pub async fn like_place(
    Path(id): Path<String>,
    State(pool): State<SqlitePool>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("UPDATE places SET liked = 1 WHERE id = ?")
        .bind(id)
        .execute(&pool)
        .await;
    match result {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
