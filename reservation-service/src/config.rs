use std::env;

pub struct Config {
    pub database_url: String,
    pub port: u16,
    pub jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            database_url: env::var("DATABASE_URL")
                .expect("DATABASE_URL must be set"),
            port: env::var("PORT")
                .unwrap_or_else(|_| "8082".to_string())
                .parse()
                .expect("PORT must be a valid number"),
            jwt_secret: env::var("JWT_SECRET")
                .expect("JWT_SECRET must be set"),
        }
    }
}