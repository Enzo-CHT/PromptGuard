use std::sync::{Arc, Mutex};
use tracing::{Level, event, instrument};
use tracing_subscriber::fmt;

mod controller;
mod service;

#[tokio::main]
#[instrument]
async fn main() -> anyhow::Result<()> {
    fmt().init();

    let service = Arc::new(Mutex::new(
        tokio::task::spawn_blocking(|| service::recognizer_service::RecognizerService::new())
            .await??,
    ));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:9990")
        .await
        .unwrap();

    event!(
        Level::INFO,
        "Le serveur est disponible à l'adresse : http://localhost:9990"
    );

    axum::serve(
        listener,
        controller::routes::drs_routes().with_state(service),
    )
    .await
    .unwrap();

    Ok(())
}
