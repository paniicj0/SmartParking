use sqlx::PgPool;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub jwt_secret: String,
    pub mail_username: String,
    pub mail_password: String,
    pub mail_from: String,
}