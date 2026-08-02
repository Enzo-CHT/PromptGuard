use std::sync::{Arc, Mutex};

use crate::{
    controller::controllers::analyze, controller::controllers::is_alive,
    service::recognizer_service::RecognizerService,
};
use axum::{Router, routing::get, routing::post};

pub fn drs_routes() -> Router<Arc<Mutex<RecognizerService>>> {
    Router::new()
        .route("/drs/", get(root))
        .route("/drs/isalive", get(is_alive))
        .route("/drs/analyze", post(analyze))
}

async fn root() -> String {
    String::from("Hello World")
}
