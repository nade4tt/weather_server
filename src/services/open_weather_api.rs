use dotenv::dotenv;
use std::env;

use axum::Json;

use crate::models::weather::WeatherResponse;

pub async fn get_weather_from_api() -> Json<WeatherResponse> {
    let api_key = env::var("API_KEY").unwrap();
    let city = "Maribor";
    let url = format!(
        "https://api.openweathermap.org/data/2.5/weather?q={}&appid={}&units=metric",
        city, api_key
    );

    println!("{}", url);

    let response: WeatherResponse = reqwest::get(url).await.unwrap().json().await.unwrap();
    println!("response {:?}", response);

    Json(response)
}
