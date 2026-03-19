use axum::{
    extract::{State, Query},
    http::StatusCode,
    Json,
};

use crate::{
    dto::{EntryRequest, EntryResponse, MessageResponse},
    middleware::auth::AuthUser,
    service::parking_session_service,
    app_state::AppState,
    dto::{ActiveParkingSessionResponse, ExitRequest, ExitResponse, ParkingHistoryQuery, ParkingHistoryResponse}
};

pub async fn enter_parking(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(request): Json<EntryRequest>,
) -> Result<(StatusCode, Json<EntryResponse>), (StatusCode, Json<MessageResponse>)> {
    match parking_session_service::enter_parking(
        &state.db,
        &state.reservation_service_url,
        auth_user.user_id,
        request.gate_id,
        request.vehicle_id,
    )
    .await
    {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(message) => Err((StatusCode::BAD_REQUEST, Json(MessageResponse { message }))),
    }
}



pub async fn get_active_session(
    State(state): State<AppState>,
    auth_user: AuthUser,
) -> Result<(StatusCode, Json<ActiveParkingSessionResponse>), (StatusCode, Json<MessageResponse>)> {
    match parking_session_service::get_active_session(&state.db, auth_user.user_id).await {
        Ok(Some(session)) => Ok((StatusCode::OK, Json(session))),
        Ok(None) => Err((
            StatusCode::NOT_FOUND,
            Json(MessageResponse {
                message: "Korisnik nema aktivno parkiranje.".to_string(),
            }),
        )),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(MessageResponse {
                message: "Greška prilikom učitavanja aktivne parking sesije.".to_string(),
            }),
        )),
    }
}

pub async fn exit_parking(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Json(request): Json<ExitRequest>,
) -> Result<(StatusCode, Json<ExitResponse>), (StatusCode, Json<MessageResponse>)> {
    match parking_session_service::exit_parking(&state.db, auth_user.user_id, request.gate_id).await {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(message) => Err((StatusCode::BAD_REQUEST, Json(MessageResponse { message }))),
    }
}

pub async fn get_parking_history(
    State(state): State<AppState>,
    auth_user: AuthUser,
    Query(query): Query<ParkingHistoryQuery>,
) -> Result<(StatusCode, Json<ParkingHistoryResponse>), (StatusCode, Json<MessageResponse>)> {
    match parking_session_service::get_parking_history(
        &state.db,
        auth_user.user_id,
        query.status,
    )
    .await
    {
        Ok(response) => Ok((StatusCode::OK, Json(response))),
        Err(_) => Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(MessageResponse {
                message: "Greška prilikom učitavanja istorije parkiranja.".to_string(),
            }),
        )),
    }
}