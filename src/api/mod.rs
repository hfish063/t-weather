mod api;
mod weather;

pub fn get_current_weather(location: &str, days: Option<u8>) -> Option<weather::Weather> {
    match api::api_fetch(location, days) {
        Ok(result) => Some(result),
        Err(_) => None,
    }
}
