use crate::models::structs::Place;
use reqwest::{Client, Error};
use serde_json::Value;

pub async fn search_places(
    api_key: &str,
    location: &str,
    place_type: &str,
) -> Result<Vec<Place>, Error> {
    let url = format!(
        "https://maps.googleapis.com/maps/api/place/nearbysearch/json?location={}&radius=1500&type={}&key={}",
        location, place_type, api_key
    );
    let client = Client::new();
    let response = client.get(url).send().await?.json::<Value>().await?;
    let places = response["results"]
        .as_array()
        .unwrap_or(&vec![])
        .iter()
        .map(|place| Place {
            id: place["place_id"].as_str().unwrap_or("").to_string(),
            name: place["name"].as_str().unwrap_or("").to_string(),
            image: "".to_string(), // TODO: Fetch image if available)
            description: place["vicinity"].as_str().unwrap_or("").to_string(),
            price: place["price_level"].as_f64().unwrap_or(0.0),
            location: format!(
                "{}, {}",
                place["geometry"]["location"]["lat"].as_f64().unwrap_or(0.0),
                place["geometry"]["location"]["lng"].as_f64().unwrap_or(0.0)
            ),
            liked: false,
        })
        .collect();
    Ok(places)
}
