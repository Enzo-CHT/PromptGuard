use crate::controller::controllers::get_foo;
use axum::{Router, routing::get};

pub fn app_routes() -> Router {
    Router::new()
        .route("/", get(root))
        .route("/foo", get(get_foo))
}

async fn root() -> String {
    String::from("Hello World")
}
