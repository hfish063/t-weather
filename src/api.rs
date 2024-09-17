use serde::{Deserialize, Serialize};

const URL: &str = "http://api.weatherapi.com/v1/current.json";

#[derive(Debug, Serialize, Deserialize)]
pub struct Forecast {
    location: Location,
    current: Current,
}

#[derive(Debug, Serialize, Deserialize)]
struct Location {
    name: String,
    region: String,
    country: String,
    localtime: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Current {
    temp_c: f32,
    temp_f: f32,
    is_day: u8,
    condition: Condition,
}

#[derive(Debug, Serialize, Deserialize)]
struct Condition {
    text: String,
    code: u16,
}

impl Forecast {
    pub fn to_string(self) -> String {
        format!("{:#?}\n{:#?}", &self.location, &self.current)
    }
}

pub fn get_current_weather(location: &str, days: Option<u8>) -> Option<Forecast> {
    match api_fetch(location, days) {
        Ok(result) => Some(result),
        Err(_) => None,
    }
}

fn api_fetch(location: &str, days: Option<u8>) -> Result<Forecast, reqwest::Error> {
    let api_key: &str = &std::env::var("KEY").unwrap();
    let params = [
        ("key", api_key),
        ("q", location),
        ("days", &days.unwrap_or(1).to_string()),
    ];

    let url = reqwest::Url::parse_with_params(URL, &params).unwrap();
    let response = reqwest::blocking::get(url)?;
    let body = response.text().unwrap();
    let response: Forecast = serde_json::from_str(&body).unwrap();

    Ok(response)
}
