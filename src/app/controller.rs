use axum::{http::StatusCode, response::IntoResponse};

pub async fn get_root() -> impl IntoResponse {
    StatusCode::ACCEPTED
}
