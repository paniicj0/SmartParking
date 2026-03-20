mod app_state;
mod config;
mod error;
mod models;
mod dto;
mod handlers;
mod services;
mod repositories;
mod routes;
mod middleware;

use axum::Router;
use dotenvy::dotenv;
use sqlx::PgPool;
use std::net::SocketAddr;
use tower_http::cors::CorsLayer;
use tracing_subscriber::FmtSubscriber;

use crate::app_state::AppState;
use crate::config::Config;
use crate::routes::create_router;

use chrono::Local;
use tokio::time::{sleep, Duration};

fn start_expire_old_job(pool: PgPool) {
    tokio::spawn(async move {
        loop {
            let now = Local::now().naive_local();

            match crate::services::reservation_service::expire_old(&pool, now).await {
                Ok(count) => {
                    println!("Expired reservations updated: {}", count);
                }
                Err(err) => {
                    eprintln!("Error while expiring reservations: {}", err);
                }
            }

            sleep(Duration::from_secs(60)).await;
        }
    });
}

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

    start_expire_old_job(db_pool.clone());

    let app_state = AppState { db: db_pool };

    let app: Router = create_router(app_state).layer(CorsLayer::permissive());

    let addr = SocketAddr::from(([127, 0, 0, 1], config.port));
    println!("Reservation service running on {}", addr);

    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("failed to bind listener");

    axum::serve(listener, app)
        .await
        .expect("server failed");
}