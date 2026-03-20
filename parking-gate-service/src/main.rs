mod app_state;
mod dto;
mod middleware {
    pub mod auth;
}
mod models {
    pub mod gate;
    pub mod parking_session;
}
mod repository {
    pub mod gate_repository;
    pub mod parking_session_repository;
}
mod service {
    pub mod reservation_client;
    pub mod parking_session_service;
}
mod handlers {
    pub mod parking_session_handler;
}
mod routes {
    pub mod parking_session_routes;
}

use axum::Router;
use dotenvy::dotenv;
use sqlx::postgres::PgPoolOptions;
use std::{env, net::SocketAddr};
use tower_http::cors::CorsLayer;

use routes::parking_session_routes::parking_session_routes;
use app_state::AppState;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL nije postavljen");
    let reservation_service_url =
        env::var("RESERVATION_SERVICE_URL").expect("RESERVATION_SERVICE_URL nije postavljen");
    let port: u16 = env::var("PORT")
        .unwrap_or_else(|_| "8082".to_string())
        .parse()
        .expect("PORT nije validan");

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&database_url)
        .await
        .expect("Neuspešno povezivanje na bazu");

    let state = AppState {
        db: pool,
        reservation_service_url,
    };

    let app = Router::new()
        .merge(parking_session_routes())
        .with_state(state)
        .layer(CorsLayer::permissive());

    let addr: SocketAddr = SocketAddr::from(([127, 0, 0, 1], port));
    println!("Parking/Gate Service running on {}", addr);
    
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}