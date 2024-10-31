use serde::{Deserialize, Serialize};

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Weather {
    pub location: Location,
    pub current: Current,
    pub forecast: Forecast,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Location {
    pub name: String,
    pub region: String,
    pub country: String,
    pub lat: f64,
    pub lon: f64,
    #[serde(rename = "tz_id")]
    pub tz_id: String,
    #[serde(rename = "localtime_epoch")]
    pub localtime_epoch: i64,
    pub localtime: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Current {
    #[serde(rename = "last_updated_epoch")]
    pub last_updated_epoch: i64,
    #[serde(rename = "last_updated")]
    pub last_updated: String,
    #[serde(rename = "temp_c")]
    pub temp_c: f64,
    #[serde(rename = "temp_f")]
    pub temp_f: f64,
    #[serde(rename = "is_day")]
    pub is_day: i64,
    pub condition: Condition,
    #[serde(rename = "wind_mph")]
    pub wind_mph: f64,
    #[serde(rename = "wind_kph")]
    pub wind_kph: f64,
    #[serde(rename = "wind_degree")]
    pub wind_degree: i64,
    #[serde(rename = "wind_dir")]
    pub wind_dir: String,
    #[serde(rename = "pressure_mb")]
    pub pressure_mb: f64,
    #[serde(rename = "pressure_in")]
    pub pressure_in: f64,
    #[serde(rename = "precip_mm")]
    pub precip_mm: f64,
    #[serde(rename = "precip_in")]
    pub precip_in: f64,
    pub humidity: i64,
    pub cloud: i64,
    #[serde(rename = "feelslike_c")]
    pub feelslike_c: f64,
    #[serde(rename = "feelslike_f")]
    pub feelslike_f: f64,
    #[serde(rename = "windchill_c")]
    pub windchill_c: f64,
    #[serde(rename = "windchill_f")]
    pub windchill_f: f64,
    #[serde(rename = "heatindex_c")]
    pub heatindex_c: f64,
    #[serde(rename = "heatindex_f")]
    pub heatindex_f: f64,
    #[serde(rename = "dewpoint_c")]
    pub dewpoint_c: f64,
    #[serde(rename = "dewpoint_f")]
    pub dewpoint_f: f64,
    #[serde(rename = "vis_km")]
    pub vis_km: f64,
    #[serde(rename = "vis_miles")]
    pub vis_miles: f64,
    pub uv: f64,
    #[serde(rename = "gust_mph")]
    pub gust_mph: f64,
    #[serde(rename = "gust_kph")]
    pub gust_kph: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Condition {
    pub text: String,
    pub icon: String,
    pub code: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Forecast {
    pub forecastday: Vec<Forecastday>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Forecastday {
    pub date: String,
    #[serde(rename = "date_epoch")]
    pub date_epoch: i64,
    pub day: Day,
    pub astro: Astro,
    pub hour: Vec<Hour>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Day {
    #[serde(rename = "maxtemp_c")]
    pub maxtemp_c: f64,
    #[serde(rename = "maxtemp_f")]
    pub maxtemp_f: f64,
    #[serde(rename = "mintemp_c")]
    pub mintemp_c: f64,
    #[serde(rename = "mintemp_f")]
    pub mintemp_f: f64,
    #[serde(rename = "avgtemp_c")]
    pub avgtemp_c: f64,
    #[serde(rename = "avgtemp_f")]
    pub avgtemp_f: f64,
    #[serde(rename = "maxwind_mph")]
    pub maxwind_mph: f64,
    #[serde(rename = "maxwind_kph")]
    pub maxwind_kph: f64,
    #[serde(rename = "totalprecip_mm")]
    pub totalprecip_mm: f64,
    #[serde(rename = "totalprecip_in")]
    pub totalprecip_in: f64,
    #[serde(rename = "totalsnow_cm")]
    pub totalsnow_cm: f64,
    #[serde(rename = "avgvis_km")]
    pub avgvis_km: f64,
    #[serde(rename = "avgvis_miles")]
    pub avgvis_miles: f64,
    pub avghumidity: i64,
    #[serde(rename = "daily_will_it_rain")]
    pub daily_will_it_rain: i64,
    #[serde(rename = "daily_chance_of_rain")]
    pub daily_chance_of_rain: i64,
    #[serde(rename = "daily_will_it_snow")]
    pub daily_will_it_snow: i64,
    #[serde(rename = "daily_chance_of_snow")]
    pub daily_chance_of_snow: i64,
    pub condition: Condition2,
    pub uv: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Condition2 {
    pub text: String,
    pub icon: String,
    pub code: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Astro {
    pub sunrise: String,
    pub sunset: String,
    pub moonrise: String,
    pub moonset: String,
    #[serde(rename = "moon_phase")]
    pub moon_phase: String,
    #[serde(rename = "moon_illumination")]
    pub moon_illumination: i64,
    #[serde(rename = "is_moon_up")]
    pub is_moon_up: i64,
    #[serde(rename = "is_sun_up")]
    pub is_sun_up: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Hour {
    #[serde(rename = "time_epoch")]
    pub time_epoch: i64,
    pub time: String,
    #[serde(rename = "temp_c")]
    pub temp_c: f64,
    #[serde(rename = "temp_f")]
    pub temp_f: f64,
    #[serde(rename = "is_day")]
    pub is_day: i64,
    pub condition: Condition3,
    #[serde(rename = "wind_mph")]
    pub wind_mph: f64,
    #[serde(rename = "wind_kph")]
    pub wind_kph: f64,
    #[serde(rename = "wind_degree")]
    pub wind_degree: i64,
    #[serde(rename = "wind_dir")]
    pub wind_dir: String,
    #[serde(rename = "pressure_mb")]
    pub pressure_mb: f64,
    #[serde(rename = "pressure_in")]
    pub pressure_in: f64,
    #[serde(rename = "precip_mm")]
    pub precip_mm: f64,
    #[serde(rename = "precip_in")]
    pub precip_in: f64,
    #[serde(rename = "snow_cm")]
    pub snow_cm: f64,
    pub humidity: i64,
    pub cloud: i64,
    #[serde(rename = "feelslike_c")]
    pub feelslike_c: f64,
    #[serde(rename = "feelslike_f")]
    pub feelslike_f: f64,
    #[serde(rename = "windchill_c")]
    pub windchill_c: f64,
    #[serde(rename = "windchill_f")]
    pub windchill_f: f64,
    #[serde(rename = "heatindex_c")]
    pub heatindex_c: f64,
    #[serde(rename = "heatindex_f")]
    pub heatindex_f: f64,
    #[serde(rename = "dewpoint_c")]
    pub dewpoint_c: f64,
    #[serde(rename = "dewpoint_f")]
    pub dewpoint_f: f64,
    #[serde(rename = "will_it_rain")]
    pub will_it_rain: i64,
    #[serde(rename = "chance_of_rain")]
    pub chance_of_rain: i64,
    #[serde(rename = "will_it_snow")]
    pub will_it_snow: i64,
    #[serde(rename = "chance_of_snow")]
    pub chance_of_snow: i64,
    #[serde(rename = "vis_km")]
    pub vis_km: f64,
    #[serde(rename = "vis_miles")]
    pub vis_miles: f64,
    #[serde(rename = "gust_mph")]
    pub gust_mph: f64,
    #[serde(rename = "gust_kph")]
    pub gust_kph: f64,
    pub uv: f64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Condition3 {
    pub text: String,
    pub icon: String,
    pub code: i64,
}

impl Weather {
    pub fn to_string(&self) -> String {
        format!(
            "{:#?}\n{:#?}\n{:#?}C\n{:#?}F",
            &self.location.name,
            &self.current.condition.text,
            &self.current.temp_c,
            &self.current.temp_f
        )
    }

    pub fn get_morning_data(&self) -> Option<&Hour> {
        self.get_data_for_time(&self.forecast.forecastday[0], 6)
    }

    pub fn get_afternoon_data(&self) -> Option<&Hour> {
        self.get_data_for_time(&self.forecast.forecastday[0], 12)
    }

    pub fn get_evening_data(&self) -> Option<&Hour> {
        self.get_data_for_time(&self.forecast.forecastday[0], 18)
    }

    pub fn get_night_data(&self) -> Option<&Hour> {
        self.get_data_for_time(&self.forecast.forecastday[0], 0)
    }

    /// Returns list of weather forecast data for corresponding number of days
    /// Data includes the hour of morning/afternoon/evening/night for each subsequent day
    pub fn get_data_for_days(&self, days: usize) -> Option<Vec<&Forecastday>> {
        let mut result: Vec<&Forecastday> = vec![];
        if days > 0 && days <= 7 {
            for i in 0..days {
                let curr = &self.forecast.forecastday[i];
                result.push(curr);
            }
            Some((result))
        } else {
            None
        }
    }

    fn get_data_for_time<'a>(&'a self, day: &'a Forecastday, time: u32) -> Option<&Hour> {
        for hour in &day.hour {
            let split_hour: Vec<&str> = hour.time.split_whitespace().collect();
            let hour_str = split_hour[1];

            if hour_str == self.time_str(time) {
                return Some(&hour);
            }
        }
        None
    }

    /// Formats hour in the form 0x:00
    fn time_str(&self, hour: u32) -> String {
        let mut hour_str = format!("{}:00", hour);

        if hour < 10 {
            hour_str = format!("0{}:00", hour);
        }

        hour_str
    }
}

/// Returns list of weather forecast data for corresponding number of days
/// Data includes the hour of morning/afternoon/evening/night for each subsequent day
pub fn get_data_for_days(weather: &Weather, days: usize) -> Option<Vec<&Forecastday>> {
    let mut result: Vec<&Forecastday> = vec![];
    if days > 0 && days <= 7 {
        for i in 0..days {
            let curr = &weather.forecast.forecastday[i];
            result.push(curr);
        }
        Some((result))
    } else {
        None
    }
}
