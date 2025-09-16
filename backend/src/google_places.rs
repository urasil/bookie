use crate::models::structs::{Place, PlaceDetails};
use reqwest::Client;
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::RwLock;

const GOOGLE_PLACES_URL: &str = "https://places.googleapis.com/v1";
const CACHE_TTL: Duration = Duration::from_secs(60 * 5); // 5 minutes

#[derive(Clone)]
pub struct GooglePlaces {
    api_key: String,
    client: Client,
    search_cache: Arc<RwLock<HashMap<String, (Instant, Vec<Place>)>>>,
    details_cache: Arc<RwLock<HashMap<String, (Instant, PlaceDetails)>>>,
    photo_cache: Arc<RwLock<HashMap<String, (Instant, Vec<u8>)>>>,
}

impl GooglePlaces {
    pub fn new(api_key: String) -> Self {
        GooglePlaces {
            api_key,
            client: Client::new(),
            search_cache: Arc::new(RwLock::new(HashMap::new())),
            details_cache: Arc::new(RwLock::new(HashMap::new())),
            photo_cache: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    fn is_expired(ts: Instant) -> bool {
        ts.elapsed() > CACHE_TTL
    }

    pub async fn search_places(
        &self,
        query: &str,
        lat: f64,
        lng: f64,
        radius_meters: i32,
    ) -> anyhow::Result<Vec<Place>> {
        let cache_key = format!("{}:{}:{}:{}", query, lat, lng, radius_meters);
        if let Some((ts, places)) = self.search_cache.read().await.get(&cache_key) {
            if !Self::is_expired(*ts) {
                return Ok(places.clone());
            }
        }

        let url = format!("{}/places/searchText", GOOGLE_PLACES_URL);
        let body = serde_json::json!({
            "textQuery": query,
            "locationBias": {
                "circle": {
                    "center": { "latitude": lat, "longitude": lng },
                    "radius": radius_meters
                }
            }
        });

        let res = self
            .client
            .post(&url)
            .header("X-Goog-Api-Key", &self.api_key)
            .header("X-Goog-FieldMask", "places.placeId,places.displayName,places.formattedAddress,places.rating,places.userRatingCount,places.photos,places.types")
            .json(&body)
            .send()
            .await?;

        let json: Value = res.json().await?;
        let mut places: Vec<Place> = vec![];

        if let Some(arr) = json.get("places").and_then(|v| v.as_array()) {
            for item in arr {
                if let Ok(place) = serde_json::from_value::<Place>(item.clone()) {
                    places.push(place);
                }
            }
        }

        self.search_cache
            .write()
            .await
            .insert(cache_key, (Instant::now(), places.clone()));

        Ok(places)
    }

    pub async fn get_place_details(&self, place_id: &str) -> anyhow::Result<PlaceDetails> {
        if let Some((ts, details)) = self.details_cache.read().await.get(place_id) {
            if !Self::is_expired(*ts) {
                return Ok(details.clone());
            }
        }

        let url = format!("{}/places/{}", GOOGLE_PLACES_URL, place_id);
        let res = self
            .client
            .get(&url)
            .header("X-Goog-Api-Key", &self.api_key)
            .header("X-Goog-FieldMask", "id,displayName,formattedAddress,location,rating,userRatingCount,currentOpeningHours,regularOpeningHours,types,photos,editorialSummary,reviewSummary,websiteUri,goodForChildren,goodForGroups,priceLevel,reservable,servesBeer,servesWine,servesCoffee,servesVegetarianFood,reviews")
            .send()
            .await?;

        let json: Value = res.json().await?;
        let details: PlaceDetails = serde_json::from_value(json)?;
        self.details_cache
            .write()
            .await
            .insert(place_id.to_string(), (Instant::now(), details.clone()));
        Ok(details)
    }

    pub async fn fetch_photo(&self, photo_name: &str, max_width: i32) -> anyhow::Result<Vec<u8>> {
        if let Some((ts, data)) = self.photo_cache.read().await.get(photo_name) {
            if !Self::is_expired(*ts) {
                return Ok(data.clone());
            }
        }

        let url = format!(
            "{}/{}/media?maxWidthPx={}",
            GOOGLE_PLACES_URL, photo_name, max_width
        );
        let res = self
            .client
            .get(&url)
            .header("X-Goog-Api-Key", &self.api_key)
            .send()
            .await?;

        let bytes = res.bytes().await?.to_vec();
        self.photo_cache
            .write()
            .await
            .insert(photo_name.to_string(), (Instant::now(), bytes.clone()));
        Ok(bytes)
    }
}
