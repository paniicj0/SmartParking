use sqlx::PgPool;

use crate::dto::parking_spot_dto::{DailyOccupancyResponse, ParkingOccupancyResponse, ParkingSpotResponse,CreateParkingSpotRequest,
    UpdateParkingSpotRequest,ParkingSpotStatusResponse};
use crate::repositories::parking_spot_repository;
use chrono::NaiveDateTime;

use uuid::Uuid;

use crate::{
    models::parking_spot::ParkingSpot,
};


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

pub async fn create_spot(
    pool: &PgPool,
    req: CreateParkingSpotRequest,
) -> Result<ParkingSpotResponse, sqlx::Error> {
    let spot = parking_spot_repository::create_spot(pool, req).await?;
    Ok(map_to_response(spot))
}

pub async fn update_spot(
    pool: &PgPool,
    id: Uuid,
    req: UpdateParkingSpotRequest,
) -> Result<Option<ParkingSpotResponse>, sqlx::Error> {
    let updated = parking_spot_repository::update_spot(pool, id, req).await?;

    Ok(updated.map(map_to_response))
}

pub async fn delete_spot(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
    parking_spot_repository::delete_spot(pool, id).await
}

fn map_to_response(spot: ParkingSpot) -> ParkingSpotResponse {
    ParkingSpotResponse {
        id: spot.id,
        label: spot.label,
        floor: spot.floor,
        zone: spot.zone,
        spot_type: spot.spot_type,
        is_active: spot.is_active,
    }
}


pub async fn get_occupancy_summary(
    pool: &sqlx::PgPool,
) -> Result<ParkingOccupancyResponse, sqlx::Error> {
    let (total_spots, active_spots, inactive_spots, reserved_spots, free_spots) =
        parking_spot_repository::get_occupancy_summary(pool).await?;

    let occupancy_percentage = if active_spots > 0 {
        (reserved_spots as f64 / active_spots as f64) * 100.0
    } else {
        0.0
    };

    Ok(ParkingOccupancyResponse {
        total_spots,
        active_spots,
        inactive_spots,
        reserved_spots,
        free_spots,
        occupancy_percentage,
    })
}

pub async fn get_daily_occupancy(
    pool: &sqlx::PgPool,
) -> Result<Vec<DailyOccupancyResponse>, sqlx::Error> {
    let rows = parking_spot_repository::get_daily_occupancy(pool).await?;

    Ok(rows
        .into_iter()
        .map(|(date, occupied_count)| DailyOccupancyResponse {
            date,
            occupied_count,
        })
        .collect())
}