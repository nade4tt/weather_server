use std::{env, error::Error};

use crate::models::weather::WeatherResponse;

pub async fn get_weather_from_api() -> Result<WeatherResponse, Box<dyn Error>> {
    let api_key = env::var("API_KEY").unwrap();
    let city = "Maribor";
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, api_key
    );

    let response: WeatherResponse = reqwest::get(url).await?.json().await?;

    Ok(response)
}
