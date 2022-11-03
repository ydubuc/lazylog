use axum::{routing::post, Router};

pub mod controller;
pub mod dtos;
pub mod jwt;
pub mod models;
pub mod service;

pub fn router() -> Router {
    Router::new()
        .route("/auth/register", post(controller::register))
        .route("/auth/login", post(controller::login))
        .route("/auth/refresh", post(controller::refresh))
}
