use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};

use axum::{
    extract::State,
    http::{header, HeaderMap, StatusCode},
    Json,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};

use crate::{
    models::{Claims, LoginRequest, LoginResponse, RegisterUserRequest, RegisterUserResponse, User},
    state::AppState,
};

use lettre::message::header::ContentType;
use lettre::transport::smtp::authentication::Credentials;
use lettre::{Message, SmtpTransport, Transport};

use uuid::Uuid;

pub async fn health() -> &'static str {
    "API radi"
}

pub fn extract_claims_from_token(
    headers: &HeaderMap,
    jwt_secret: &str,
) -> Result<Claims, (StatusCode, String)> {
    let auth_header = headers
        .get(header::AUTHORIZATION)
        .and_then(|value| value.to_str().ok())
        .ok_or((
            StatusCode::UNAUTHORIZED,
            "Authorization header nedostaje.".to_string(),
        ))?;

    if !auth_header.starts_with("Bearer ") {
        return Err((
            StatusCode::UNAUTHORIZED,
            "Authorization header mora biti Bearer token.".to_string(),
        ));
    }

    let token = auth_header.trim_start_matches("Bearer ").trim();

    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(jwt_secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|e| {
        (
            StatusCode::UNAUTHORIZED,
            format!("Neispravan ili istekao token: {}", e),
        )
    })?;

    Ok(token_data.claims)
}

pub async fn register_user(
    State(state): State<AppState>,
    Json(payload): Json<RegisterUserRequest>,
) -> Result<(StatusCode, Json<RegisterUserResponse>), (StatusCode, String)> {
    let existing_user = sqlx::query_scalar::<_, i32>(
        "SELECT id FROM users WHERE email = $1"
    )
    .bind(&payload.email)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Greska pri proveri email-a: {}", e),
        )
    })?;

    if existing_user.is_some() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Korisnik sa tim email-om vec postoji.".to_string(),
        ));
    }

    let hashed_password = hash(&payload.password, DEFAULT_COST)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Greska pri hashovanju lozinke: {}", e),
            )
        })?;

    let user_id: i32 = sqlx::query_scalar(
        r#"
        INSERT INTO users (email, password_hash, first_name, last_name, phone_number, is_active)
        VALUES ($1, $2, $3, $4, $5, $6)
        RETURNING id
        "#,
    )
    .bind(&payload.email)
    .bind(&hashed_password)
    .bind(&payload.first_name)
    .bind(&payload.last_name)
    .bind(&payload.phone_number)
    .bind(false)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Greska pri cuvanju korisnika: {}", e),
        )
    })?;

    let activation_token = Uuid::new_v4().to_string();
    let expires_at = (Utc::now() + Duration::hours(24)).naive_utc();

    sqlx::query(
        r#"
        INSERT INTO user_activation_tokens (user_id, token, expires_at)
        VALUES ($1, $2, $3)
        "#,
    )
    .bind(user_id)
    .bind(&activation_token)
    .bind(expires_at)
    .execute(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Nije lepo sacuvan aktivacioni token: {}", e),
        )
    })?;

    let activation_link = format!(
        "http://localhost:4200/activate?token={}",
        activation_token
    );

    send_activation_email(
        &payload.email,
        &activation_link,
        &state.mail_username,
        &state.mail_password,
        &state.mail_from,
    )
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Greska pri slanju aktivacionog email-a: {}", e),
        )
    })?;

    Ok((
        StatusCode::CREATED,
        Json(RegisterUserResponse {
            message: "Registracija uspesna.".to_string(),
        }),
    ))
}

pub async fn login_user(
    State(state): State<AppState>,
    Json(payload): Json<LoginRequest>,
) -> Result<(StatusCode, Json<LoginResponse>), (StatusCode, String)> {
    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT id, email, password_hash, first_name, last_name, phone_number, is_active
        FROM users
        WHERE email = $1
        "#,
    )
    .bind(&payload.email)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Greska pri trazenju korisnika: {}", e),
        )
    })?;

    let user = match user {
        Some(user) => user,
        None => {
            return Err((
                StatusCode::UNAUTHORIZED,
                "Pogresan email ili lozinka.".to_string(),
            ))
        }
    };

    let password_valid = verify(&payload.password, &user.password_hash)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Greska pri proveri lozinke: {}", e),
            )
        })?;

    if !password_valid {
        return Err((
            StatusCode::UNAUTHORIZED,
            "Pogresan email ili lozinka.".to_string(),
        ));
    }

    if !user.is_active {
        return Err((
            StatusCode::FORBIDDEN,
            "Nalog nije aktiviran.".to_string(),
        ));
    }

    let expiration = Utc::now() + Duration::hours(24);

    let claims = Claims {
        sub: user.id,
        email: user.email.clone(),
        exp: expiration.timestamp() as usize,
    };

    let token = encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(state.jwt_secret.as_bytes()),
    )
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Greska pri generisanju tokena: {}", e),
        )
    })?;

    Ok((
        StatusCode::OK,
        Json(LoginResponse {
            message: "Login uspesan.".to_string(),
            token,
        }),
    ))
}

pub async fn activate_user(
    State(state): State<AppState>,
    axum::extract::Query(params): axum::extract::Query<std::collections::HashMap<String, String>>,
) -> Result<(StatusCode, String), (StatusCode, String)> {
    let token = params
        .get("token")
        .ok_or((StatusCode::BAD_REQUEST, "Token nedostaje".to_string()))?;

    let record = sqlx::query!(
        r#"
        SELECT user_id, expires_at
        FROM user_activation_tokens
        WHERE token = $1
        "#,
        token
    )
    .fetch_optional(&state.db)
    .await
    .map_err(|e| (
        StatusCode::INTERNAL_SERVER_ERROR,
        format!("DB greska: {}", e)
    ))?;

    let record = match record {
        Some(r) => r,
        None => return Err((StatusCode::BAD_REQUEST, "Nevalidan token".to_string())),
    };

    if record.expires_at < Utc::now().naive_utc() {
        return Err((StatusCode::BAD_REQUEST, "Token je istekao".to_string()));
    }

    sqlx::query("UPDATE users SET is_active = true WHERE id = $1")
        .bind(record.user_id)
        .execute(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Greska pri aktivaciji korisnika: {}", e),
            )
        })?;

    sqlx::query("DELETE FROM user_activation_tokens WHERE token = $1")
        .bind(token)
        .execute(&state.db)
        .await
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Greska pri brisanju aktivacionog tokena: {}", e),
            )
        })?;

    Ok((StatusCode::OK, "Nalog aktiviran!".to_string()))
}

fn send_activation_email(
    to_email: &str,
    activation_link: &str,
    mail_username: &str,
    mail_password: &str,
    mail_from: &str,
) -> Result<(), String> {
    let email = Message::builder()
        .from(mail_from.parse().map_err(|e| format!("Nevalidan FROM email: {}", e))?)
        .to(to_email.parse().map_err(|e| format!("Nevalidan TO email: {}", e))?)
        .subject("Aktivacija naloga - Smart Parking")
        .header(ContentType::TEXT_PLAIN)
        .body(format!(
            "Zdravo,\n\n\
            Hvala na registraciji na Smart Parking sistem.\n\
            Da biste aktivirali svoj nalog, kliknite na sledeci link:\n\n\
            {}\n\n\
            Ovaj link vazi 24 sata.\n\n\
            Ako niste vi napravili nalog, zanemarite ovu poruku.\n",
            activation_link
        ))
        .map_err(|e| format!("Greska pri kreiranju email poruke: {}", e))?;

    let creds = Credentials::new(
        mail_username.to_string(),
        mail_password.to_string(),
    );

    let mailer = SmtpTransport::relay("smtp.gmail.com")
        .map_err(|e| format!("Greska pri kreiranju SMTP relay-ja: {}", e))?
        .credentials(creds)
        .build();

    mailer
        .send(&email)
        .map_err(|e| format!("Greska pri slanju email-a: {}", e))?;

    Ok(())
}