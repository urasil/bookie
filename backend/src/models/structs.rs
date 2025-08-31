use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, sqlx::FromRow)]
pub struct Place {
    pub id: String,
    pub name: String,
    pub image: String,
    pub description: String,
    pub price: f64,
    pub location: String,
    pub liked: bool,
}

#[derive(Deserialize)]
pub struct LikeAction {
    pub liked: bool,
}
