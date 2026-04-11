use axum::Json;

use crate::{
    models::weather::WeatherData, services::error::WeatherError,
    services::open_weather_api::get_weather_from_api,
};

pub async fn get_weather() -> Result<Json<WeatherData>, WeatherError> {
    let weather = get_weather_from_api().await?;

    let data: WeatherData = WeatherData::from(&weather);

    Ok(Json(data))
}
