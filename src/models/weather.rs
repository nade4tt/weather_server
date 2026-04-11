use chrono::Utc;
use serde::{Deserialize, Serialize};

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherResponse {
    name: String,
    weather: Vec<Description>,
    main: MainStats,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct MainStats {
    temp: f32,
    temp_min: f32,
    temp_max: f32,
    humidity: f32,
    pressure: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Description {
    main: String,
    description: String,
    icon: String,
}

#[allow(dead_code)]
#[derive(Serialize, Deserialize, Debug)]
pub struct WeatherData {
    temperature: f32,
    humidity: f32,
    city: String,
    description: String,
    last_update_time: u64,
}

#[allow(dead_code)]
impl WeatherData {
    pub fn from(response: &WeatherResponse) -> Self {
        let now = Utc::now().timestamp_millis();

        WeatherData {
            temperature: response.main.temp,
            humidity: response.main.humidity,
            city: response.name.clone(),
            description: response
                .weather
                .first()
                .map(|weather| weather.description.clone())
                .unwrap_or("No description".into()),
            last_update_time: now as u64,
        }
    }
}
