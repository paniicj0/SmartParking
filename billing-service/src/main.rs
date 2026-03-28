mod app_state;
mod config;
mod error;
mod models;
mod dto;
mod handlers;
mod service;
mod repository;
mod routes;

use axum::Router;
use dotenvy::dotenv;
use sqlx::PgPool;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing_subscriber::FmtSubscriber;

use crate::app_state::AppState;
use crate::config::Config;
use crate::routes::create_router;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let subscriber = FmtSubscriber::new();
    tracing::subscriber::set_global_default(subscriber)
        .expect("failed to set tracing subscriber");

    let config = Config::from_env();

    let db_pool = PgPool::connect(&config.database_url)
        .await
        .expect("failed to connect to database");

    let app_state = AppState { db: db_pool };

    let app = create_router()
    .with_state(app_state)
    .layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([0, 0, 0, 0], config.port));
    println!("Billing service running on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind listener");

    axum::serve(listener, app)
        .await
        .expect("server failed");
}