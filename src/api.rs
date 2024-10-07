use crate::weather::Weather;

const URL: &str = "http://api.weatherapi.com/v1/forecast.json";

pub fn get_current_weather(location: &str, days: Option<u8>) -> Option<Weather> {
    match api_fetch(location, days) {
        Ok(result) => match Some(result) {
            Some(result) => result,
            None => None,
        },
        Err(_) => None,
    }
}

fn api_fetch(location: &str, days: Option<u8>) -> Result<Option<Weather>, reqwest::Error> {
    let api_key: &str = &std::env::var("KEY").unwrap();
    let params = [
        ("key", api_key),
        ("q", location),
        ("days", &days.unwrap_or(1).to_string()),
    ];

    let url = reqwest::Url::parse_with_params(URL, &params).unwrap();
    let response = reqwest::blocking::get(url)?;
    let body = response.text().unwrap();
    let response: Option<Weather> = serde_json::from_str(&body).unwrap_or_default();

    Ok(response)
}
