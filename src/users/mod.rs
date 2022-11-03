use axum::{routing::get, Router};

pub mod controller;
pub mod dtos;
pub mod errors;
pub mod models;
pub mod service;

pub fn router() -> Router {
    Router::new()
        .route("/users", get(controller::get_users))
        .route("/users/me", get(controller::get_user_from_request))
        .route("/users/:id", get(controller::get_user_by_id))
}
