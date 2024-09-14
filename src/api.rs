use serde::{Deserialize, Serialize};

const URL: &str = "http://api.weatherapi.com/v1/";

#[derive(Serialize, Deserialize)]
struct Forecast {
    location: Location,
    current: Current,
    condition: Condition,
}

#[derive(Serialize, Deserialize)]
struct Location {
    name: String,
    region: String,
    country: String,
    localtime: String,
}

#[derive(Serialize, Deserialize)]
struct Current {
    temp_c: i8,
    temp_f: i8,
    is_day: u8,
}

#[derive(Serialize, Deserialize)]
struct Condition {
    text: String,
    code: u16,
}

pub fn get_weather_forecast(location: &str, days: Option<u8>) -> Result<(), reqwest::Error> {
    let params = [("key", std::env::var("KEY"))];
    let response = reqwest::blocking::get(URL)?;
    Ok(())
}
