use std::fs::exists;

use chrono::{NaiveDateTime, Duration};
use sqlx::PgPool;
use uuid::Uuid;

use crate::models::{reservation::{Reservation, ReservationWithSpotInfo}, parking_spot::ParkingSpot};

pub async fn get_parking_spot_by_id(
    pool: &PgPool,
    parking_spot_id: Uuid,
) -> Result<Option<ParkingSpot>, sqlx::Error> {
    let spot = sqlx::query_as::<_, ParkingSpot>(
        r#"
        SELECT id, label, floor, zone, spot_type, is_active, created_at
        FROM parking_spots
        WHERE id = $1
        "#
    )
    .bind(parking_spot_id)
    .fetch_optional(pool)
    .await?;

    Ok(spot)
}

pub async fn has_conflicting_reservation(
    pool: &PgPool,
    parking_spot_id: Uuid,
    start_time: NaiveDateTime,
    end_time: NaiveDateTime,
) -> Result<bool, sqlx::Error> {
    let count: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM reservations
        WHERE parking_spot_id = $1
          AND status = 'Confirmed'
          AND $2 < end_time
          AND $3 > start_time
        "#
    )
    .bind(parking_spot_id)
    .bind(start_time)
    .bind(end_time)
    .fetch_one(pool)
    .await?;

    Ok(count > 0)
}

pub async fn create_reservation(
    pool: &PgPool,
    user_id: i32,
    vehicle_id: i32,
    parking_spot_id: Uuid,
    start_time: NaiveDateTime,
    end_time: NaiveDateTime,
) -> Result<Reservation, sqlx::Error> {
    let reservation = sqlx::query_as::<_, Reservation>(
        r#"
        INSERT INTO reservations (
            id,
            user_id,
            vehicle_id,
            parking_spot_id,
            start_time,
            end_time,
            status,
            created_at,
            updated_at
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7,
            CURRENT_TIMESTAMP AT TIME ZONE 'Europe/Belgrade',
            CURRENT_TIMESTAMP AT TIME ZONE 'Europe/Belgrade')
        RETURNING id, user_id, vehicle_id, parking_spot_id, start_time, end_time, status, created_at, updated_at
        "#
    )
    .bind(Uuid::new_v4())
    .bind(user_id)
    .bind(vehicle_id)
    .bind(parking_spot_id)
    .bind(start_time)
    .bind(end_time)
    .bind("Confirmed")
    .fetch_one(pool)
    .await?;

    Ok(reservation)
}

pub async fn get_my_reservations(
    pool: &PgPool,
    user_id: i32,
) -> Result<Vec<ReservationWithSpotInfo>, sqlx::Error> {
    let reservations = sqlx::query_as::<_, ReservationWithSpotInfo>(
        r#"
        SELECT
            r.id,
            r.parking_spot_id,
            ps.label AS parking_spot_label,
            ps.zone,
            r.start_time,
            r.end_time,
            r.status
        FROM reservations r
        JOIN parking_spots ps ON r.parking_spot_id = ps.id
        WHERE r.user_id = $1
        AND status = 'Confirmed'
        ORDER BY r.start_time DESC
        "#
    )
    .bind(user_id)
    .fetch_all(pool)
    .await?;

    Ok(reservations)
}

pub async fn get_reservation_by_id(
    pool: &PgPool,
    reservation_id: Uuid,
) -> Result<Option<Reservation>, sqlx::Error> {
    let reservation = sqlx::query_as::<_, Reservation>(
        r#"
        SELECT id, user_id, vehicle_id, parking_spot_id, start_time, end_time, status, created_at, updated_at
        FROM reservations
        WHERE id = $1
        "#
    )
    .bind(reservation_id)
    .fetch_optional(pool)
    .await?;

    Ok(reservation)
}

pub async fn cancel_reservation(
    pool: &PgPool,
    reservation_id: Uuid,
) -> Result<Reservation, sqlx::Error> {
    let reservation = sqlx::query_as::<_, Reservation>(
        r#"
        UPDATE reservations
        SET status = 'Cancelled',
            updated_at = CURRENT_TIMESTAMP AT TIME ZONE 'Europe/Belgrade'
        WHERE id = $1
        RETURNING id, user_id, vehicle_id, parking_spot_id, start_time, end_time, status, created_at, updated_at
        "#
    )
    .bind(reservation_id)
    .fetch_one(pool)
    .await?;

    Ok(reservation)
}

pub async fn expire_old(
    pool: &PgPool,
    now: NaiveDateTime,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        UPDATE reservations
        SET status = 'Expired',
            updated_at = CURRENT_TIMESTAMP AT TIME ZONE 'Europe/Belgrade'
        WHERE status = 'Confirmed'
          AND start_time + INTERVAL '20 minutes' < $1
        "#,
    )
    .bind(now)
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

pub async fn mark_used(
    pool: &PgPool,
    reservation_id: Uuid,
) -> Result<u64, sqlx::Error> {
    let result = sqlx::query(
        r#"
        UPDATE reservations
        SET status = 'Used',
            updated_at = CURRENT_TIMESTAMP AT TIME ZONE 'Europe/Belgrade'
        WHERE id = $1
          AND status = 'Confirmed'
        "#,
    )
    .bind(reservation_id)
    .execute(pool)
    .await?;

    Ok(result.rows_affected())
}

pub async fn find_valid_for_entry(
    pool: &PgPool,
    user_id: i32,
    vehicle_id: i32,
    time: NaiveDateTime,
) -> Result<Option<Reservation>, sqlx::Error> {
    let allowed_from = time - Duration::minutes(20);
    let allowed_to = time + Duration::minutes(20);

    let reservation = sqlx::query_as::<_, Reservation>(
        r#"
        SELECT
            id,
            user_id,
            vehicle_id,
            parking_spot_id,
            start_time,
            end_time,
            status,
            created_at,
            updated_at
        FROM reservations
        WHERE user_id = $1
          AND vehicle_id = $2
          AND status = 'Confirmed'
          AND start_time BETWEEN $3 AND $4
        ORDER BY start_time ASC
        LIMIT 1
        "#,
    )
    .bind(user_id)
    .bind(vehicle_id)
    .bind(allowed_from)
    .bind(allowed_to)
    .fetch_optional(pool)
    .await?;

    Ok(reservation)
}

pub async fn has_overlapping_reservation_for_vehicle(
    pool: &PgPool,
    vehicle_id: i32,
    start_time: NaiveDateTime,
    end_time: NaiveDateTime,
) -> Result<bool, sqlx::Error> {
    let exists = sqlx::query_scalar(
        r#"
        SELECT EXISTS (
            SELECT 1
            FROM reservations
            WHERE vehicle_id = $1
              AND status IN ('Pending', 'Confirmed')
              AND $2 < end_time
              AND $3 > start_time
        )
        "#,
    )
    .bind(vehicle_id)
    .bind(start_time)
    .bind(end_time)
    .fetch_one(pool)
    .await?;

    Ok(exists)
}