use axum::{routing::get, Router};

pub mod controller;
pub mod errors;
pub mod models;
pub mod util;

pub fn router() -> Router {
    Router::new().route("/", get(controller::get_root))
}
