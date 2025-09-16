use crate::google_places::GooglePlaces;
use sqlx::SqlitePool;
use std::sync::Arc;

pub struct AppState {
    pub gp: Arc<GooglePlaces>,
    pub pool: SqlitePool,
}
