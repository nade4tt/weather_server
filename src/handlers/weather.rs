use axum::Json;

use crate::{models::weather::WeatherData, services::open_weather_api::get_weather_from_api};

pub async fn get_weather() -> Json<WeatherData> {
    let weather = get_weather_from_api().await.unwrap();

    let data: WeatherData = WeatherData::from(&weather);

    Json(data)
}
