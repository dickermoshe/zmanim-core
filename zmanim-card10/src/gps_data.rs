use core::fmt::Write as _;
use serde::{Deserialize, Serialize};
/// Structure to hold all GPS data, with fields that can be updated independently
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GpsData {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub altitude: Option<f32>,
    pub speed_over_ground: Option<f32>,
    pub timestamp: Option<i64>,
    pub num_of_fix_satellites: Option<u32>,
    pub hdop: Option<f32>,
    pub vdop: Option<f32>,
    pub pdop: Option<f32>,
    pub geoid_separation: Option<f32>,
}

impl Default for GpsData {
    fn default() -> Self {
        Self {
            latitude: None,
            longitude: None,
            altitude: None,
            speed_over_ground: None,
            timestamp: None,
            num_of_fix_satellites: None,
            hdop: None,
            vdop: None,
            pdop: None,
            geoid_separation: None,
        }
    }
}

impl GpsData {
    /// Check if we have a valid position fix
    pub fn has_position_fix(&self) -> bool {
        self.latitude.is_some() && self.longitude.is_some()
    }
}
