mod handlers;
mod models;
mod routes;

use handlers::AppState;
use routes::create_routes;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();

    let database_url =
        std::env::var("DATABASE_URL").expect("DATABASE_URL mora biti postavljen u .env fajlu");

    let jwt_secret =
        std::env::var("JWT_SECRET").expect("JWT_SECRET mora biti postavljen u .env fajlu");

    let pool: PgPool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Ne mogu da se povezem na PostgreSQL");

    println!("Uspesno povezano sa bazom!");

    let state = AppState {
        db: pool,
        jwt_secret,
    };

    let app = create_routes(state);

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    let listener = tokio::net::TcpListener::bind(addr)
        .await
        .expect("Ne mogu da bindujem port");

    println!("Server pokrenut na http://127.0.0.1:3000");

    axum::serve(listener, app)
        .await
        .expect("Greska pri pokretanju servera");
}