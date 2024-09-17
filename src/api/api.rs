use serde::{Deserialize, Serialize};

use super::weather::Weather;

const URL: &str = "http://api.weatherapi.com/v1/forecast.json";

pub fn api_fetch(location: &str, days: Option<u8>) -> Result<Weather, reqwest::Error> {
    let api_key: &str = &std::env::var("KEY").unwrap();
    let params = [
        ("key", api_key),
        ("q", location),
        ("days", &days.unwrap_or(1).to_string()),
    ];

    let url = reqwest::Url::parse_with_params(URL, &params).unwrap();
    let response = reqwest::blocking::get(url)?;
    let body = response.text().unwrap();
    let response: Weather = serde_json::from_str(&body).unwrap();

    Ok(response)
}
