use thiserror::Error;

#[derive(Debug, Error)]
pub enum WeatherError {
    #[error("Failed to fetch weather data: {0}")]
    ApiError(#[from] reqwest::Error),

    #[error("Missing required environment variable: {0}")]
    ConfigError(String),

    #[error("Invalid API response: {0}")]
    ParseError(String),
}

impl axum::response::IntoResponse for WeatherError {
    fn into_response(self) -> axum::response::Response {
        axum::response::Json(serde_json::json!({
            "error": self.to_string()
        }))
        .into_response()
    }
}
