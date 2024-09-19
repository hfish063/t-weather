use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Weather {
    pub location: Location,
    pub current: Current,
    pub forecast: Forecast,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Location {
    pub name: String,
    pub region: String,
    pub country: String,
    pub localtime: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Current {
    pub temp_c: f32,
    pub temp_f: f32,
    pub is_day: u8,
    pub condition: Condition,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Condition {
    pub text: String,
    pub code: u16,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Forecast {
    pub forecastday: Vec<Day>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Day {
    pub hour: Vec<Hour>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Hour {
    pub time: String,
    pub temp_c: f32,
    pub temp_f: f32,
    pub is_day: u8,
    pub condition: Condition,
}

impl Weather {
    pub fn to_string(&self) -> String {
        format!(
            "{:#?}\n{:#?}\n{:#?}",
            &self.location, &self.current, &self.forecast
        )
    }
}
