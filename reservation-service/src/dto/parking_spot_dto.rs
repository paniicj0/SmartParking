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