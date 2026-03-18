use sqlx::PgPool;

use crate::dto::parking_spot_dto::ParkingSpotResponse;
use crate::repositories::parking_spot_repository;
use chrono::NaiveDateTime;

use crate::dto::parking_spot_dto::ParkingSpotStatusResponse;

pub async fn get_all_spots(pool: &PgPool) -> Result<Vec<ParkingSpotResponse>, sqlx::Error> {
    let spots = parking_spot_repository::get_all_spots(pool).await?;

    let response = spots
        .into_iter()
        .map(|spot| ParkingSpotResponse {
            id: spot.id,
            label: spot.label,
            floor: spot.floor,
            zone: spot.zone,
            spot_type: spot.spot_type,
            is_active: spot.is_active,
        })
        .collect();

    Ok(response)
}

pub async fn get_spot_statuses(
    pool: &PgPool,
    start_time: NaiveDateTime,
    end_time: NaiveDateTime,
) -> Result<Vec<ParkingSpotStatusResponse>, sqlx::Error> {
    let spots_with_status =
        parking_spot_repository::get_spot_statuses(pool, start_time, end_time).await?;

    let response = spots_with_status
        .into_iter()
        .map(|(spot, available)| ParkingSpotStatusResponse {
            id: spot.id,
            label: spot.label,
            floor: spot.floor,
            zone: spot.zone,
            spot_type: spot.spot_type,
            is_active: spot.is_active,
            available,
        })
        .collect();

    Ok(response)
}