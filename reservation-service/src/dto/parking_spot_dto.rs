use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct ParkingSpotResponse {
    pub id: Uuid,
    pub label: String,
    pub floor: Option<i32>,
    pub zone: String,
    pub spot_type: String,
    pub is_active: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ParkingSpotStatusResponse {
    pub id: Uuid,
    pub label: String,
    pub floor: Option<i32>,
    pub zone: String,
    pub spot_type: String,
    pub is_active: bool,
    pub available: bool,
}

#[derive(Debug, Deserialize)]
pub struct SpotStatusQueryParams {
    pub start_time: String,
    pub end_time: String,
}


#[derive(Debug, Deserialize)]
pub struct CreateParkingSpotRequest {
    pub label: String,
    pub floor: Option<i32>,
    pub zone: String,
    pub spot_type: String,
    pub is_active: bool,
}

#[derive(Debug, Deserialize)]
pub struct UpdateParkingSpotRequest {
    pub label: String,
    pub floor: Option<i32>,
    pub zone: String,
    pub spot_type: String,
    pub is_active: bool,
}

#[derive(Debug, Serialize)]
pub struct ParkingOccupancyResponse {
    pub total_spots: i64,
    pub active_spots: i64,
    pub inactive_spots: i64,
    pub reserved_spots: i64,
    pub free_spots: i64,
    pub occupancy_percentage: f64,
}

#[derive(Debug, Serialize)]
pub struct DailyOccupancyResponse {
    pub date: String,
    pub occupied_count: i64,
}