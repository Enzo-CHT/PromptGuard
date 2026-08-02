use crate::controller::controllers::{handle_text, is_alive};
use axum::{
    Router,
    routing::{get, post},
};

pub fn app_routes() -> Router {
    Router::new()
        .route("/d2c/", get(root))
        .route("/d2c/isalive", get(is_alive))
        .route("/d2c/text", post(handle_text))
}

async fn root() -> String {
    String::from("Hello World")
}
