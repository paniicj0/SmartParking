mod old_handlers;
mod models;
mod routes;
mod dto;
mod handlers;
mod state;

use state::AppState;
use routes::create_routes;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::net::SocketAddr;

use tower_http::cors::{Any, CorsLayer};
use axum::http::{Method, HeaderValue};

use dotenvy::dotenv;
use std::env;

#[tokio::main]
async fn main() {
    dotenv().ok();

    let database_url =
        env::var("DATABASE_URL").expect("DATABASE_URL mora biti postavljen u .env fajlu");

    let pool: PgPool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Ne mogu da se povezem na PostgreSQL");

    println!("Uspesno povezano sa bazom!");

    let cors = CorsLayer::new()
        .allow_origin("http://localhost:4200".parse::<HeaderValue>().unwrap())
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE, Method::OPTIONS])
        .allow_headers(Any);

    let state = AppState {
        db: pool,
        jwt_secret: env::var("JWT_SECRET").expect("JWT_SECRET mora biti postavljen"),
        mail_username: env::var("MAIL_USERNAME").expect("MAIL_USERNAME mora biti postavljen"),
        mail_password: env::var("MAIL_PASSWORD").expect("MAIL_PASSWORD mora biti postavljen"),
        mail_from: env::var("MAIL_FROM").expect("MAIL_FROM mora biti postavljen"),
    };

    let app = create_routes(state).layer(cors);

    let addr = SocketAddr::from(([0, 0, 0, 0], 3000));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Ne mogu da bindujem port");


    axum::serve(listener, app)
        .await
        .expect("Greska pri pokretanju servera");
}