use crate::app_state::AppState;
use crate::models::structs::{Place, PlaceDetails};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use sqlx::SqlitePool;
use std::sync::Arc;

#[derive(serde::Deserialize)]
pub struct SearchQuery {
    pub query: String,
    pub lat: f64,
    pub lng: f64,
    pub radius: i32,
}

#[derive(serde::Deserialize)]
pub struct LikeParams {
    pub user_id: String,
    pub liked: bool,
}

#[derive(serde::Deserialize)]
pub struct MatchesQuery {
    user_id: String,
}

pub async fn get_places(
    State(state): State<Arc<AppState>>,
    Query(params): Query<SearchQuery>,
) -> Result<Json<Vec<Place>>, StatusCode> {
    let gp = &state.gp;
    let places = gp
        .search_places(&params.query, params.lat, params.lng, params.radius)
        .await;

    match places {
        Ok(places) => Ok(Json(places)),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn like_place(
    Path(place_id): Path<String>,
    State(state): State<Arc<AppState>>,
    Json(payload): Json<LikeParams>,
) -> Result<StatusCode, StatusCode> {
    let result = sqlx::query("INSERT OR REPLACE INTO user_likes (user_id, place_id) VALUES (?, ?)")
        .bind(payload.user_id)
        .bind(place_id)
        .execute(&state.pool)
        .await;

    match result {
        Ok(_) => Ok(StatusCode::OK),
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}

pub async fn get_matches(
    State(state): State<Arc<AppState>>,
    Query(params): Query<MatchesQuery>,
) -> Result<Json<Vec<PlaceDetails>>, StatusCode> {
    let gp = &state.gp;
    let liked_places: Result<Vec<String>, sqlx::Error> =
        sqlx::query_scalar("SELECT place_id FROM user_likes WHERE user_id = ?")
            .bind(params.user_id)
            .fetch_all(&state.pool)
            .await;
    match liked_places {
        Ok(rows) => {
            let mut results = vec![];
            for place_id in rows {
                if let Ok(details) = gp.get_place_details(&place_id).await {
                    results.push(details);
                }
            }
            Ok(Json(results))
        }
        Err(_) => Err(StatusCode::INTERNAL_SERVER_ERROR),
    }
}
