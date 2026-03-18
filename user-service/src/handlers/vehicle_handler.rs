use axum::{
    extract::{Path, State},
    http::{HeaderMap, StatusCode},
    Json,
};

use crate::{
    dto::vehicle_dto::{CreateVehicleRequest, UpdateVehicleRequest, VehicleResponse},
    state::AppState,
};

use crate::handlers::auth_handler::extract_claims_from_token;

pub async fn get_my_vehicles(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Vec<VehicleResponse>>, (StatusCode, String)> {
    let claims = extract_claims_from_token(&headers, &state.jwt_secret)?;
    println!("GET_VEHICLES user id iz tokena: {}", claims.sub);

    let vehicles = sqlx::query_as::<_, VehicleResponse>(
        r#"
        SELECT id, license_plate, name
        FROM vehicles
        WHERE user_id = $1
        ORDER BY id DESC
        "#,
    )
    .bind(claims.sub)
    .fetch_all(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Greška pri čitanju vozila: {}", e),
        )
    })?;

    
    println!("Pronadjeno vozila: {}", vehicles.len());

    Ok(Json(vehicles))
}

pub async fn create_vehicle(
    State(state): State<AppState>,
    headers: HeaderMap,
    Json(payload): Json<CreateVehicleRequest>,
) -> Result<Json<VehicleResponse>, (StatusCode, String)> {
    let claims = extract_claims_from_token(&headers, &state.jwt_secret)?;

    if payload.license_plate.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Tablica je obavezna.".to_string(),
        ));
    }

    let license_plate = payload.license_plate.trim().to_uppercase();
    let name = payload
        .name
        .as_ref()
        .map(|n| n.trim().to_string())
        .filter(|n| !n.is_empty());

    let vehicle = sqlx::query_as::<_, VehicleResponse>(
        r#"
        INSERT INTO vehicles (user_id, license_plate, name)
        VALUES ($1, $2, $3)
        RETURNING id, license_plate, name
        "#,
    )
    .bind(claims.sub)
    .bind(license_plate)
    .bind(name)
    .fetch_one(&state.db)
    .await
    .map_err(|e| {
        let msg = e.to_string();

        if msg.contains("unique") || msg.contains("vehicles_user_id_license_plate_key") {
            (
                StatusCode::CONFLICT,
                "Ova tablica je već uneta.".to_string(),
            )
        } else {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Greška pri dodavanju vozila: {}", e),
            )
        }
    })?;

    Ok(Json(vehicle))
}

pub async fn update_vehicle(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<i32>,
    Json(payload): Json<UpdateVehicleRequest>,
) -> Result<Json<VehicleResponse>, (StatusCode, String)> {
    let claims = extract_claims_from_token(&headers, &state.jwt_secret)?;

    if payload.license_plate.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            "Tablica je obavezna.".to_string(),
        ));
    }

    let license_plate = payload.license_plate.trim().to_uppercase();
    let name = payload
        .name
        .as_ref()
        .map(|n| n.trim().to_string())
        .filter(|n| !n.is_empty());

    let vehicle = sqlx::query_as::<_, VehicleResponse>(
        r#"
        UPDATE vehicles
        SET license_plate = $1,
            name = $2
        WHERE id = $3 AND user_id = $4
        RETURNING id, license_plate, name
        "#,
    )
    .bind(license_plate)
    .bind(name)
    .bind(id)
    .bind(claims.sub)
    .fetch_optional(&state.db)
    .await
    .map_err(|e| {
        let msg = e.to_string();

        if msg.contains("unique") || msg.contains("vehicles_user_id_license_plate_key") {
            (
                StatusCode::CONFLICT,
                "Ova tablica je već uneta.".to_string(),
            )
        } else {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                format!("Greška pri izmeni vozila: {}", e),
            )
        }
    })?;

    match vehicle {
        Some(vehicle) => Ok(Json(vehicle)),
        None => Err((
            StatusCode::NOT_FOUND,
            "Vozilo nije pronađeno.".to_string(),
        )),
    }
}

pub async fn delete_vehicle(
    State(state): State<AppState>,
    headers: HeaderMap,
    Path(id): Path<i32>,
) -> Result<StatusCode, (StatusCode, String)> {
    let claims = extract_claims_from_token(&headers, &state.jwt_secret)?;

    let result = sqlx::query(
        r#"
        DELETE FROM vehicles
        WHERE id = $1 AND user_id = $2
        "#,
    )
    .bind(id)
    .bind(claims.sub)
    .execute(&state.db)
    .await
    .map_err(|e| {
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            format!("Greška pri brisanju vozila: {}", e),
        )
    })?;

    if result.rows_affected() == 0 {
        return Err((
            StatusCode::NOT_FOUND,
            "Vozilo nije pronađeno.".to_string(),
        ));
    }

    Ok(StatusCode::NO_CONTENT)
}