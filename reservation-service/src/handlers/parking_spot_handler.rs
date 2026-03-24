use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    Json,
};
use chrono::NaiveDateTime;
use uuid::Uuid;

use crate::{
    app_state::AppState,
    middleware::admin::ensure_admin,
    middleware::auth::AuthUser,
    dto::parking_spot_dto::{
        CreateParkingSpotRequest,
        ParkingSpotResponse,
        ParkingSpotStatusResponse,
        SpotStatusQueryParams,
        UpdateParkingSpotRequest, DailyOccupancyResponse, ParkingOccupancyResponse,
    },
    services::parking_spot_service,
};

pub async fn get_all_spots(
    State(state): State<AppState>,
) -> Result<Json<Vec<ParkingSpotResponse>>, StatusCode> {
    let spots = parking_spot_service::get_all_spots(&state.db)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(spots))
}

pub async fn get_spot_statuses(
    State(state): State<AppState>,
    Query(params): Query<SpotStatusQueryParams>,
) -> Result<Json<Vec<ParkingSpotStatusResponse>>, StatusCode> {
    let start_time = NaiveDateTime::parse_from_str(&params.start_time, "%Y-%m-%dT%H:%M:%S")
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    let end_time = NaiveDateTime::parse_from_str(&params.end_time, "%Y-%m-%dT%H:%M:%S")
        .map_err(|_| StatusCode::BAD_REQUEST)?;

    if start_time >= end_time {
        return Err(StatusCode::BAD_REQUEST);
    }

    let spots = parking_spot_service::get_spot_statuses(&state.db, start_time, end_time)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    Ok(Json(spots))
}

pub async fn create_parking_spot(
    State(state): State<AppState>,
    user: AuthUser,
    Json(req): Json<CreateParkingSpotRequest>,
) -> Result<(StatusCode, Json<ParkingSpotResponse>), (StatusCode, String)> {
    ensure_admin(&user)?;

    let spot = parking_spot_service::create_spot(&state.db, req)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    Ok((StatusCode::CREATED, Json(spot)))
}

pub async fn update_parking_spot(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
    Json(req): Json<UpdateParkingSpotRequest>,
) -> Result<Json<ParkingSpotResponse>, (StatusCode, String)> {
    ensure_admin(&user)?;

    let updated = parking_spot_service::update_spot(&state.db, id, req)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    match updated {
        Some(spot) => Ok(Json(spot)),
        None => Err((StatusCode::NOT_FOUND, "Parking mesto nije pronađeno.".to_string())),
    }
}

pub async fn delete_parking_spot(
    State(state): State<AppState>,
    user: AuthUser,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, (StatusCode, String)> {
    ensure_admin(&user)?;

    let deleted = parking_spot_service::delete_spot(&state.db, id)
        .await
        .map_err(|e| (StatusCode::INTERNAL_SERVER_ERROR, e.to_string()))?;

    if deleted {
        Ok(StatusCode::NO_CONTENT)
    } else {
        Err((StatusCode::NOT_FOUND, "Parking mesto nije pronađeno.".to_string()))
    }
}

pub async fn get_occupancy_summary(
    State(state): State<AppState>,
    user: AuthUser,
) -> Result<Json<ParkingOccupancyResponse>, (StatusCode, String)> {
    ensure_admin(&user)?;

    let result = parking_spot_service::get_occupancy_summary(&state.db)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Greška pri dohvatanju zauzetosti.".to_string(),
            )
        })?;

    Ok(Json(result))
}

pub async fn get_daily_occupancy(
    State(state): State<AppState>,
    user: AuthUser,
) -> Result<Json<Vec<DailyOccupancyResponse>>, (StatusCode, String)> {
    ensure_admin(&user)?;

    let result = parking_spot_service::get_daily_occupancy(&state.db)
        .await
        .map_err(|_| {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Greška pri dohvatanju analitike.".to_string(),
            )
        })?;

    Ok(Json(result))
}