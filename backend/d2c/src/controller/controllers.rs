use axum::{Json, http::StatusCode};

pub async fn is_alive() -> StatusCode {
    StatusCode::OK
}

pub async fn handle_text() -> String {
    String::from("ManageText")
}
