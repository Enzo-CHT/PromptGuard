use std::sync::{Arc, Mutex};

use crate::{controller::controllers::analyze, service::recognizer_service::RecognizerService};
use axum::{Router, handler::Handler, routing::get, routing::post};

pub fn drs_routes() -> Router<Arc<Mutex<RecognizerService>>> {
    Router::new()
        .route("/drs/", get(root))
        .route("/drs/analyze", post(analyze))
}

async fn root() -> String {
    String::from("Hello World")
}
