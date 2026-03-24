use sqlx::PgPool;
use uuid::Uuid;

use crate::models::parking_spot::ParkingSpot;
use chrono::{NaiveDateTime, Utc};
use crate::dto::parking_spot_dto::{CreateParkingSpotRequest, UpdateParkingSpotRequest};

pub async fn get_all_spots(pool: &PgPool) -> Result<Vec<ParkingSpot>, sqlx::Error> {
    let spots = sqlx::query_as::<_, ParkingSpot>(
        r#"
        SELECT id, label, floor, zone, spot_type, is_active, created_at
        FROM parking_spots
        ORDER BY label
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(spots)
}


pub async fn get_spot_statuses(
    pool: &PgPool,
    start_time: NaiveDateTime,
    end_time: NaiveDateTime,
) -> Result<Vec<(ParkingSpot, bool)>, sqlx::Error> {
    let rows = sqlx::query_as::<_, ParkingSpot>(
        r#"
        SELECT id, label, floor, zone, spot_type, is_active, created_at
        FROM parking_spots
        ORDER BY label
        "#
    )
    .fetch_all(pool)
    .await?;

    let mut result = Vec::new();

    for spot in rows {
        let conflict_count: i64 = sqlx::query_scalar(
            r#"
            SELECT COUNT(*)
            FROM reservations
            WHERE parking_spot_id = $1
              AND status = 'Confirmed'
              AND $2 < end_time
              AND $3 > start_time
            "#
        )
        .bind(spot.id)
        .bind(start_time)
        .bind(end_time)
        .fetch_one(pool)
        .await?;

        let available = spot.is_active && conflict_count == 0;
        result.push((spot, available));
    }

    Ok(result)
}


pub async fn get_spot_by_id(pool: &PgPool, id: Uuid) -> Result<Option<ParkingSpot>, sqlx::Error> {
    let spot = sqlx::query_as::<_, ParkingSpot>(
        r#"
        SELECT id, label, floor, zone, spot_type, is_active, created_at
        FROM parking_spots
        WHERE id = $1
        "#
    )
    .bind(id)
    .fetch_optional(pool)
    .await?;

    Ok(spot)
}

pub async fn create_spot(
    pool: &PgPool,
    req: CreateParkingSpotRequest,
) -> Result<ParkingSpot, sqlx::Error> {
    let spot = sqlx::query_as::<_, ParkingSpot>(
        r#"
        INSERT INTO parking_spots (id, label, floor, zone, spot_type, is_active, created_at)
        VALUES ($1, $2, $3, $4, $5, $6, NOW())
        RETURNING id, label, floor, zone, spot_type, is_active, created_at
        "#
    )
    .bind(Uuid::new_v4())
    .bind(req.label)
    .bind(req.floor)
    .bind(req.zone)
    .bind(req.spot_type)
    .bind(req.is_active)
    .fetch_one(pool)
    .await?;

    Ok(spot)
}

pub async fn update_spot(
    pool: &PgPool,
    id: Uuid,
    req: UpdateParkingSpotRequest,
) -> Result<Option<ParkingSpot>, sqlx::Error> {
    let spot = sqlx::query_as::<_, ParkingSpot>(
        r#"
        UPDATE parking_spots
        SET label = $2,
            floor = $3,
            zone = $4,
            spot_type = $5,
            is_active = $6
        WHERE id = $1
        RETURNING id, label, floor, zone, spot_type, is_active, created_at
        "#
    )
    .bind(id)
    .bind(req.label)
    .bind(req.floor)
    .bind(req.zone)
    .bind(req.spot_type)
    .bind(req.is_active)
    .fetch_optional(pool)
    .await?;

    Ok(spot)
}

pub async fn delete_spot(pool: &PgPool, id: Uuid) -> Result<bool, sqlx::Error> {
    let deleted = sqlx::query(
        r#"
        DELETE FROM parking_spots
        WHERE id = $1
        "#
    )
    .bind(id)
    .execute(pool)
    .await?;

    Ok(deleted.rows_affected() > 0)
}

pub async fn get_occupancy_summary(
    pool: &PgPool,
) -> Result<(i64, i64, i64, i64, i64), sqlx::Error> {
    let total_spots: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*) FROM parking_spots
        "#
    )
    .fetch_one(pool)
    .await?;

    let active_spots: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*) FROM parking_spots
        WHERE is_active = true
        "#
    )
    .fetch_one(pool)
    .await?;

    let inactive_spots = total_spots - active_spots;

    let occupied_spots: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(DISTINCT parking_spot_id)
        FROM parking_sessions
        WHERE status = 'Active'
        "#
    )
    .fetch_one(pool)
    .await?;

    let free_spots = active_spots - occupied_spots;

    Ok((total_spots, active_spots, inactive_spots, occupied_spots, free_spots))
}

pub async fn get_daily_occupancy(
    pool: &PgPool,
) -> Result<Vec<(String, i64)>, sqlx::Error> {
    let rows = sqlx::query_as::<_, (String, i64)>(
        r#"
        SELECT DATE(start_time)::text as date, COUNT(*) as occupied_count
        FROM reservations
        WHERE status = 'Confirmed'
            AND start_time >= NOW() - INTERVAL '7 days'
        GROUP BY DATE(start_time)
        ORDER BY DATE(start_time)
        "#
    )
    .fetch_all(pool)
    .await?;

    Ok(rows)
}