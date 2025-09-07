use crate::{google_places, models::structs};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use sqlx::SqlitePool;
use std::env;

#[derive(serde::Deserialize)]
pub struct SearchQuery {
    location: String,
    #[serde(rename = "type")]
    place_type: String,
}

#[derive(serde::Deserialize)]
pub struct LikeParams {
    user_id: String,
    place: structs::Place,
}

#[derive(serde::Deserialize)]
pub struct MatchesQuery {
    user_id: String,
}

pub async fn get_places(
    State(pool): State<SqlitePool>,
    Query(params): Query<SearchQuery>,
) -> Result<Json<Vec<structs::Place>>, StatusCode> {
    let api_key = env::var("GOOGLE_PLACES_API_KEY").expect("missing api key");
    let places = google_places::search_places(&api_key, &params.location, &params.place_type).await;

    match places {
        Ok(places) => {
            for place in &places {
                let _ = sqlx::query("INSERT OR IGNORE INTO places (id, name, image, description, price, location) VALUES (?, ?, ?, ?, ?, ?)")
                    .bind(&place.id)
                    .bind(&place.name)
                    .bind(&place.image)
                    .bind(&place.description)
                    .bind(place.price)
                    .bind(&place.location)
                    .execute(&pool)
                    .await;
            }
            Ok(Json(places))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn like_place(
    Path(id): Path<String>,
    State(pool): State<SqlitePool>,
    Json(payload): Json<LikeParams>,
) -> Result<StatusCode, StatusCode> {
    let _ = sqlx::query("INSER OR IGNORE INTO places (id, name, image, description, price, location) VALUES (?, ?, ?, ?, ?, ?)")
        .bind(&payload.place.id)
        .bind(&payload.place.name)
        .bind(&payload.place.image)
        .bind(&payload.place.description)
        .bind(payload.place.price)
        .bind(&payload.place.location)
        .execute(&pool)
        .await;

    let result = sqlx::query("INSERT OR IGNORE INTO user_likes (user_id, place_id) VALUES (?, ?)")
        .bind(payload.user_id)
        .bind(id)
        .execute(&pool)
        .await;

    match result {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_matches(
    State(pool): State<SqlitePool>,
    Query(params): Query<MatchesQuery>,
) -> Result<Json<Vec<structs::Place>>, StatusCode> {
    let matches =
        sqlx::query_as::<_, structs::Place>("SELECT p.id, p.name, p.image, p.description, p.price, p.location, 1 as liked FROM places p INNER JOIN user_likes ul ON p.id = ul.place_id WHERE ul.user_id = ?",)
            .bind(params.user_id)
            .fetch_all(&pool)
            .await;

    match matches {
        Ok(matches) => Ok(Json(matches)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
