use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Weather {
    location: Location,
    current: Current,
    forecast: Forecast,
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

#[derive(Debug, Serialize, Deserialize)]
struct Forecast {
    forecastday: Vec<Day>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Day {
    hour: Vec<Hour>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Hour {
    time: String,
    temp_c: f32,
    temp_f: f32,
    is_day: u8,
    condition: Condition,
}

impl Weather {
    pub fn to_string(self) -> String {
        format!(
            "{:#?}\n{:#?}\n{:#?}",
            &self.location, &self.current, &self.forecast
        )
    }
}
