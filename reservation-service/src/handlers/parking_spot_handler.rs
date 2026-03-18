use axum::{
    extract::{Query, State},
    http::StatusCode,
    Json,
};
use chrono::NaiveDateTime;

use crate::{
    app_state::AppState,
    dto::parking_spot_dto::{
        ParkingSpotResponse,
        ParkingSpotStatusResponse,
        SpotStatusQueryParams,
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