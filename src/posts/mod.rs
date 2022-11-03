use axum::{routing::post, Router};

pub mod controller;
pub mod dtos;
pub mod errors;
pub mod models;
pub mod service;

pub fn router() -> Router {
    Router::new().route("/posts", post(controller::create_post))
}
