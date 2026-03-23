use axum::{
    extract::{State, Path},
    http::{HeaderMap, StatusCode},
    Json,
};

use crate::{
    dto::user_dto::{ChangePasswordRequest, UpdateProfileRequest, UserProfileResponse, UserEmailResponse, MessageResponse},
    models::User,
    state::AppState,
};

use crate::handlers::auth_handler::extract_claims_from_token;

pub async fn get_me(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<UserProfileResponse>, (StatusCode, String)> {
    let claims = extract_claims_from_token(&headers, &state.jwt_secret)?;

    let user = sqlx::query_as::<_, UserProfileResponse>(
        r#"
        SELECT id, email, first_name, last_name, phone_number, is_active
        FROM users
        WHERE id = $1
        "#,
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

    println!("GET_ME user id iz tokena: {}", claims.sub);

    match user {
        Some(user) => Ok(Json(user)),
        None => Err((
            StatusCode::NOT_FOUND,
            "Korisnik nije pronadjen.".to_string(),
        )),
    }
}

pub async fn update_me(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<UpdateProfileRequest>,
) -> Result<Json<UserProfileResponse>, (StatusCode, String)> {
    let claims = extract_claims_from_token(&headers, &state.jwt_secret)?;

    if payload.first_name.trim().is_empty() || payload.last_name.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Ime i prezime su obavezni.".to_string(),
        ));
    }

    let updated_user = sqlx::query_as::<_, UserProfileResponse>(
        r#"
        UPDATE users
        SET first_name = $1,
            last_name = $2,
            phone_number = $3
        WHERE id = $4
        RETURNING id, email, first_name, last_name, phone_number, is_active
        "#,
    )
    .bind(payload.first_name.trim())
    .bind(payload.last_name.trim())
    .bind(payload.phone_number.as_ref().map(|p| p.trim()).filter(|p| !p.is_empty()))
    .bind(claims.sub)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Greška pri ažuriranju korisnika: {}", e),
        )
    })?;

    match updated_user {
        Some(user) => Ok(Json(user)),
        None => Err((
            StatusCode::NOT_FOUND,
            "Korisnik nije pronađen.".to_string(),
        )),
    }
}

pub async fn change_password(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<ChangePasswordRequest>,
) -> Result<StatusCode, (StatusCode, String)> {
    let claims = extract_claims_from_token(&headers, &state.jwt_secret)?;

    if payload.old_password.trim().is_empty() || payload.new_password.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Stara i nova lozinka su obavezne.".to_string(),
        ));
    }

    if payload.new_password.len() < 8 {
        return Err((
            StatusCode::BAD_REQUEST,
            "Nova lozinka mora imati najmanje 8 karaktera.".to_string(),
        ));
    }

    let user = sqlx::query_as::<_, User>(
        r#"
        SELECT id, email, password_hash, first_name, last_name, phone_number, is_active
        FROM users
        WHERE id = $1
        "#,
    )
    .bind(claims.sub)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Greška pri čitanju korisnika: {}", e),
        )
    })?;

    let user = match user {
        Some(user) => user,
        None => {
            return Err((
                StatusCode::NOT_FOUND,
                "Korisnik nije pronađen.".to_string(),
            ))
        }
    };

    let is_old_password_valid = bcrypt::verify(&payload.old_password, &user.password_hash)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Greška pri proveri lozinke: {}", e),
            )
        })?;

    if !is_old_password_valid {
        return Err((
            StatusCode::UNAUTHORIZED,
            "Stara lozinka nije ispravna.".to_string(),
        ));
    }

    let new_password_hash = bcrypt::hash(&payload.new_password, bcrypt::DEFAULT_COST)
        .map_err(|e| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Greška pri hash-ovanju nove lozinke: {}", e),
            )
        })?;

    sqlx::query(
        r#"
        UPDATE users
        SET password_hash = $1
        WHERE id = $2
        "#,
    )
    .bind(new_password_hash)
    .bind(claims.sub)
    .execute(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Greška pri ažuriranju lozinke: {}", e),
        )
    })?;

    Ok(StatusCode::OK)
}


pub async fn get_user_email_by_id(
    State(state): State<AppState>,
    Path(user_id): Path<i32>,
) -> Result<(StatusCode, Json<UserEmailResponse>), (StatusCode, Json<MessageResponse>)> {
    let user = sqlx::query_as::<_, UserEmailResponse>(
        r#"
        SELECT id as user_id, email
        FROM users
        WHERE id = $1
        "#
    )
    .bind(user_id)
    .fetch_optional(&state.db)
    .await
    .map_err(|_| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(MessageResponse {
                message: "Greška pri učitavanju korisnika.".to_string(),
            }),
        )
    })?;

    let user = user.ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(MessageResponse {
                message: "Korisnik nije pronađen.".to_string(),
            }),
        )
    })?;

    Ok((StatusCode::OK, Json(user)))
}