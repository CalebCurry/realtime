use axum::{Json, Router, routing::get};
use std::net::SocketAddr;

pub mod proto {
    pub mod temperature;
}
use proto::temperature::SensorReading;

#[tokio::main]
async fn main() {
    // Build the router
    let app = Router::new().route("/example", get(example_handler));

    // Run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

#[axum::debug_handler]
async fn example_handler() -> Json<SensorReading> {
    let example = SensorReading {
        sensor_id: "ABC123".to_string(),
        temperature: 72.5,
        timestamp: 1_000_000_000,
        location: "Living Room".to_string(),
    };

    Json(example)
}