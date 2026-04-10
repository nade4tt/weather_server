mod models;

use std::{error::Error, net::SocketAddr};

use dotenv::dotenv;
use weather_server::routes::weather::routes;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    dotenv().ok();

    let app = routes();
    let addr = SocketAddr::from(([192, 168, 8, 101], 5000));
    println!("Server listening on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();

    Ok(())
}
