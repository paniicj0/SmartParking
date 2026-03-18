use sqlx::PgPool;

use crate::models::parking_spot::ParkingSpot;
use chrono::NaiveDateTime;

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