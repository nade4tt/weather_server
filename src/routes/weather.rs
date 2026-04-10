use axum::Router;

use crate::handlers::weather::get_weather;

pub fn routes() -> Router {
    Router::new()
        .route("/", axum::routing::get("Hello World!"))
        .route("/api/weather", axum::routing::get(get_weather))
}
