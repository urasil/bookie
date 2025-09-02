use serde::{Deserialize, Serialize};

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
    #[allow(dead_code)]
    pub liked: bool,
    #[allow(dead_code)]
    pub place_id: String,
}
