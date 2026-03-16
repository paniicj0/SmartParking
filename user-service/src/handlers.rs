use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Duration, Utc};

use axum::{
    extract::State,
    http::{header, HeaderMap, StatusCode},
    Json,
};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use sqlx::PgPool;

use crate::models::{
    Claims, LoginRequest, LoginResponse, RegisterUserRequest, RegisterUserResponse,
    User, UserProfileResponse,
};

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub jwt_secret: String,
}

pub async fn health() -> &'static str {
    "API radi"
}

fn extract_claims_from_token(
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

    sqlx::query(
        r#"
        INSERT INTO users (email, password_hash, first_name, last_name, phone_number, is_active)
        VALUES ($1, $2, $3, $4, $5, $6)
        "#
    )
    .bind(&payload.email)
    .bind(&hashed_password)
    .bind(&payload.first_name)
    .bind(&payload.last_name)
    .bind(&payload.phone_number)
    .bind(true)
    .execute(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Greska pri cuvanju korisnika: {}", e),
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
        "#
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

pub async fn get_me(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<UserProfileResponse>, (StatusCode, String)> {
    let claims = extract_claims_from_token(&headers, &state.jwt_secret)?;

    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT id, email, password_hash, first_name, last_name, phone_number, is_active
        FROM users
        WHERE id = $1
        "#
    )
    .bind(claims.sub)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Greska pri citanju korisnika: {}", e),
        )
    })?;

    let user = match user {
        Some(user) => user,
        None => {
            return Err((
                StatusCode::NOT_FOUND,
                "Korisnik nije pronadjen.".to_string(),
            ))
        }
    };

    Ok(Json(UserProfileResponse {
        id: user.id,
        email: user.email,
        first_name: user.first_name,
        last_name: user.last_name,
        phone_number: user.phone_number,
        is_active: user.is_active,
    }))
}