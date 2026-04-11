use std::env;

use crate::models::weather::WeatherResponse;
use crate::services::error::WeatherError;

pub async fn get_weather_from_api() -> Result<WeatherResponse, WeatherError> {
    let api_key = env::var("API_KEY").map_err(|_| WeatherError::ConfigError("API_KEY".into()))?;
    let city = "Maribor";
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, api_key
    );

    let response: WeatherResponse = reqwest::get(url).await?.json().await?;
    println!("Response: {:?}", response);

    Ok(response)
}
