use axum::Json;

use crate::models::weather::{MainStats, WeatherResponse};

pub fn get_weather() -> &'static str {
    "weather"
}
