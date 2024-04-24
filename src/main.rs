mod state;

use axum::routing::get;

use socketioxide::{
    extract::{Data, SocketRef, State},
    SocketIo,
};
use tracing::info;
use tracing_subscriber::FmtSubscriber;
use tower::ServiceBuilder;
use tower_http::cors::CorsLayer;

#[derive(Debug, serde::Deserialize)]
struct MessageIn {
    room: String,
    text: String,
}

#[derive(serde::Serialize)]
struct Messages {
    messages: Vec<state::Message>,
}

async fn handler(axum::extract::State(io): axum::extract::State<SocketIo>) {
    info!("handler called");
    let _ = io.emit("hello", "world");
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing::subscriber::set_global_default(FmtSubscriber::default())?;

    let app = axum::Router::new()
        .route("/", get(|| async { "Hello, World!" }))
        .route("/hello", get(handler))
        .with_state(io)
        .layer(
            ServiceBuilder::new()
                .layer(CorsLayer::permissive())
                .layer(layer),
        );

    info!("Starting server");


    Ok(())
}
