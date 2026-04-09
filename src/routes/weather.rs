use axum::{Router, routing::get};

use crate::{handlers::weather::get_weather, services::open_weather_api::get_weather_from_api};

pub fn routes() -> Router {
    Router::new()
        .route("/weather", axum::routing::get(get_weather()))
        .route("/api", axum::routing::get(get_weather_from_api))
}
