use tracing::{event, Level, instrument};
use tracing_subscriber::fmt;


mod controller;

#[tokio::main]
#[instrument]
async fn main() {

    fmt().init();
    // run our app with hyper, listening globally on port 3000
    let listener = tokio::net::TcpListener::bind("127.0.0.1:9990").await.unwrap();
    event!(Level::INFO, "Le serveur est disponible à l'adresse : http://localhost:9990");
    axum::serve(listener, controller::routes::app_routes()).await.unwrap();
}