use std::env;
use axum::http::StatusCode;
use crate::middleware::auth::AuthUser;

pub fn ensure_admin(user: &AuthUser) -> Result<(), (StatusCode, String)> {
    let admin_email = env::var("ADMIN_EMAIL").unwrap_or_default();

    if user.email != admin_email {
        return Err((
            StatusCode::FORBIDDEN,
            "Nemate dozvolu za ovu akciju.".to_string(),
        ));
    }

    Ok(())
}