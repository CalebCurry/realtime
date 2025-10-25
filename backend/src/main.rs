use axum::{body::Bytes, extract::{ws::WebSocket, State, WebSocketUpgrade}, http::StatusCode, response::IntoResponse, routing::{get, post}, Router};
use serde_json::json;
use tokio::sync::broadcast;
use std::{net::SocketAddr, sync::Arc};

pub mod proto {
    pub mod temperature;
}
use proto::temperature::SensorReading;

struct AppState {
    tx: broadcast::Sender<String>,
}


#[tokio::main]
async fn main() {
    let (tx, _) = broadcast::channel::<String>(100);
    let state = Arc::new(AppState { tx });
    // Build the router
    let app = Router::new()
    .route("/api/sensor", post(handle_sensor))
    .route("/ws", get(ws_handler))
    .with_state(state);

    // Run the server
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    println!("Listening on http://{}", addr);
    axum::serve(tokio::net::TcpListener::bind(addr).await.unwrap(), app)
        .await
        .unwrap();
}

async fn ws_handler(ws: WebSocketUpgrade, State(state): State<Arc<AppState>>) -> impl IntoResponse {
    println!("ws_handler");
    ws.on_upgrade(move |socket| handle_ws(socket, state))
}

async fn handle_ws(mut socket: WebSocket, state: Arc<AppState>){
    println!("Handing websocket");

    let mut rx = state.tx.subscribe();

    while let Ok(msg) = rx.recv().await {
        if socket.send(msg.into()).await.is_err() {
            break;
        }
    }
}

async fn handle_sensor(State(state): State<Arc<AppState>>, body: Bytes) -> StatusCode {
    match prost::Message::decode(body) {
        Ok(reading) => {
            let SensorReading {location, sensor_id, temperature, timestamp} = reading;
            let msg = json!({
                "sensor_id": sensor_id,
                "temperature": temperature,
                "location": location,
                "timestamp": timestamp
            }).to_string();
            println!("Received SensorReading: {}", msg);

            let _ = state.tx.send(msg);

            StatusCode::OK
        }
        Err(e) => {
            eprintln!("Error decoding proto {}", e);
            StatusCode::BAD_REQUEST
        }
    }
}