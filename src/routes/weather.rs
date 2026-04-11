use axum::Router;
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

use crate::handlers::weather::get_weather;

pub fn routes() -> Router {
    Router::new()
        .route("/", axum::routing::get(|| async { "Hello World!" }))
        .route("/api/weather", axum::routing::get(get_weather))
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http()))
}
