use std::net::SocketAddr;

use axum::{Extension, Router};
use dotenv::dotenv;
use sqlx::postgres::PgPoolOptions;
use tower_http::cors::{Any, CorsLayer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

mod app;
mod auth;
mod devices;
mod posts;
mod users;

#[tokio::main]
async fn main() {
    // environment
    dotenv().ok();
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

    // app
    let app = Router::new()
        // routers
        .merge(app::router())
        .merge(auth::router())
        .merge(users::router())
        .merge(posts::router())
        // layers
        .layer(cors)
        .layer(Extension(pool));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .expect("failed to start server");
}
