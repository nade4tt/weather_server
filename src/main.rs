mod models;

use std::net::SocketAddr;

use dotenv::dotenv;
use weather_server::routes::weather::routes;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let app = routes();
    let addr = SocketAddr::from(([127, 0, 0, 1], 5000));
    println!("Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Failed to bind to address");
    axum::serve(listener, app)
        .await
        .expect("Failed to serve application");
}
