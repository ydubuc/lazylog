use std::net::SocketAddr;

use axum::{
    routing::{delete, get, patch, post},
    Router,
};
use dotenv::dotenv;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod app;
mod auth;
mod devices;
mod posts;
mod users;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
}

#[tokio::main]
async fn main() {
    // environment
    dotenv().ok();
    let port: u16 = match std::env::var("PORT") {
        Ok(port) => port.parse().expect("environment: PORT is not a number"),
        Err(_) => 3000,
    };
    let db_url = std::env::var("DATABASE_URL").expect("environment: DATABASE_URL missing");

    // debug
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "lazylog=debug".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    // properties
    let cors = CorsLayer::new().allow_origin(Any);

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&db_url)
        .await
        .expect("failed to connect to database");

    let state = AppState { pool };

    // app
    let app = Router::with_state(state)
        .route("/", get(app::controller::get_root))
        .route("/auth/register", post(auth::controller::register))
        .route("/auth/login", post(auth::controller::login))
        .route("/auth/refresh", post(auth::controller::refresh))
        .route("/users", get(users::controller::get_users))
        .route("/users/me", get(users::controller::get_user_from_request))
        .route("/users/:id", get(users::controller::get_user_by_id))
        .route("/users/:id", patch(users::controller::edit_user_by_id))
        .route("/posts", post(posts::controller::create_post))
        .route("/posts", get(posts::controller::get_posts))
        .route("/posts/:id", get(posts::controller::get_post_by_id))
        .route("/posts/:id", patch(posts::controller::edit_post_by_id))
        .route("/posts/:id", delete(posts::controller::delete_post_by_id))
        .layer(cors);

    let addr = SocketAddr::from(([127, 0, 0, 1], port));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("failed to start server");
}
