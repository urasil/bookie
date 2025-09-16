use serde::{Deserialize, Serialize};

#[derive(Debug, serde::Serialize, serde::Deserialize, sqlx::FromRow, Clone)]
pub struct Location {
    pub lat: f64,
    pub lng: f64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Place {
    #[serde(rename = "placeId")]
    pub place_id: String,

    #[serde(rename = "displayName")]
    pub display_name: Option<DisplayName>,

    #[serde(rename = "formattedAddress")]
    pub formatted_address: Option<String>,

    pub rating: Option<f64>,

    #[serde(rename = "userRatingCount")]
    pub user_rating_count: Option<i32>,

    pub types: Option<Vec<String>>,

    pub photos: Option<Vec<Photo>>,
    pub location: Option<Location>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayName {
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Photo {
    pub name: String,
    #[serde(rename = "widthPx")]
    pub width_px: Option<i32>,
    #[serde(rename = "heightPx")]
    pub height_px: Option<i32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlaceDetails {
    pub id: Option<String>,

    #[serde(rename = "displayName")]
    pub display_name: Option<DisplayName>,

    #[serde(rename = "formattedAddress")]
    pub formatted_address: Option<String>,

    pub rating: Option<f64>,

    #[serde(rename = "userRatingCount")]
    pub user_rating_count: Option<i32>,

    #[serde(rename = "currentOpeningHours")]
    pub current_opening_hours: Option<OpeningHours>,

    #[serde(rename = "regularOpeningHours")]
    pub regular_opening_hours: Option<OpeningHours>,

    pub types: Option<Vec<String>>,

    pub photos: Option<Vec<Photo>>,

    #[serde(rename = "editorialSummary")]
    pub editorial_summary: Option<EditorialSummary>,

    #[serde(rename = "reviewSummary")]
    pub review_summary: Option<ReviewSummary>,

    #[serde(rename = "websiteUri")]
    pub website_uri: Option<String>,

    pub reviews: Option<Vec<Review>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OpeningHours {
    pub open_now: Option<bool>,
    #[serde(rename = "weekdayDescriptions")]
    pub weekday_descriptions: Option<Vec<String>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EditorialSummary {
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReviewSummary {
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Review {
    pub name: Option<String>,
    pub rating: Option<i32>,
    pub text: Option<String>,
    #[serde(rename = "relativePublishTimeDescription")]
    pub relative_publish_time_description: Option<String>,
}

impl PlaceDetails {
    /// Select up to 2 representative reviews per rating class: high (>=4), mid (=3), low (<=2)
    pub fn representative_reviews(&self) -> Vec<&Review> {
        if let Some(ref reviews) = self.reviews {
            let mut selected: Vec<&Review> = vec![];

            let high: Vec<&Review> = reviews
                .iter()
                .filter(|r| r.rating.unwrap_or(0) >= 4)
                .take(2)
                .collect();
            let mid: Vec<&Review> = reviews
                .iter()
                .filter(|r| r.rating.unwrap_or(0) == 3)
                .take(2)
                .collect();
            let low: Vec<&Review> = reviews
                .iter()
                .filter(|r| r.rating.unwrap_or(0) <= 2)
                .take(2)
                .collect();

            selected.extend(high);
            selected.extend(mid);
            selected.extend(low);

            return selected;
        }
        vec![]
    }
}
