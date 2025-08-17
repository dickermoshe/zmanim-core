use chrono::Datelike;
use core::fmt::Write as _;
use defmt::info;
use nmea::{sentences::FixType, Nmea};
/// Structure to hold all GPS data, with fields that can be updated independently
#[derive(Debug, Clone)]
pub struct GpsData {
    pub latitude: Option<f64>,
    pub longitude: Option<f64>,
    pub altitude: Option<f32>,
    pub speed_over_ground: Option<f32>,
    pub fix_time: Option<chrono::NaiveTime>,
    pub fix_date: Option<chrono::NaiveDate>,
    pub fix_type: Option<FixType>,
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
            fix_time: None,
            fix_date: None,
            fix_type: None,
            num_of_fix_satellites: None,
            hdop: None,
            vdop: None,
            pdop: None,
            geoid_separation: None,
        }
    }
}

impl GpsData {
    /// Update GPS data fields only if the new data is better (not None and potentially more recent)
    pub fn update_from_nmea(&mut self, nmea: &Nmea) -> bool {
        let mut changed = false;
        // Update position data if we have new valid data
        if let Some(lat) = nmea.latitude {
            self.latitude = Some(lat);
            changed = true;
        }
        if let Some(lon) = nmea.longitude {
            self.longitude = Some(lon);
            changed = true;
        }

        // Update altitude if we have new data
        if let Some(alt) = nmea.altitude {
            self.altitude = Some(alt);
            changed = true;
        }

        // Update speed if we have new data
        if let Some(speed) = nmea.speed_over_ground {
            self.speed_over_ground = Some(speed);
            changed = true;
        }

        // Update time and date if we have new data
        if let Some(time) = nmea.fix_time {
            self.fix_time = Some(time);
            changed = true;
        }
        if let Some(date) = nmea.fix_date {
            // Assume that dates after 2050 are invalid
            // This means that this device wont work after 2050
            if date.year() > 2050 {
                self.fix_date = None;
            } else {
                self.fix_date = Some(date);
                changed = true;
            }
        }

        // Update fix type if we have new data
        if let Some(fix_type) = &nmea.fix_type {
            self.fix_type = Some(fix_type.clone());
            changed = true;
        }

        // Update satellite count if we have new data (prefer higher counts)
        if let Some(sat_count) = nmea.num_of_fix_satellites {
            self.num_of_fix_satellites = Some(sat_count);
            changed = true;
        }

        // Update DOP values if we have new data (prefer lower values for better precision)
        if let Some(hdop) = nmea.hdop {
            self.hdop = Some(hdop);
            changed = true;
        }

        if let Some(vdop) = nmea.vdop {
            self.vdop = Some(vdop);
            changed = true;
        }

        if let Some(pdop) = nmea.pdop {
            self.pdop = Some(pdop);
            changed = true;
        }

        // Update geoid separation if we have new data
        if let Some(geoid) = nmea.geoid_separation {
            self.geoid_separation = Some(geoid);
            changed = true;
        }

        changed
    }

    /// Check if we have a valid position fix
    pub fn has_position_fix(&self) -> bool {
        self.latitude.is_some()
            && self.longitude.is_some()
            && self.fix_type.as_ref().map_or(false, |fix| fix.is_valid())
    }

    /// Get the age of the data in ticks

    /// Pretty print the GPS data for logging
    ///
    /// Example output:
    /// "📍 Position: 40.712800°N, -74.006000°E | Altitude: 10.5m | Speed: 25.3km/h | Fix: Gps | Sats: 8 | HDOP: 1.2 | VDOP: 1.8 | PDOP: 2.1 | Time: 14:30:25 | Date: 2024-01-15 | Age: 150ms"
    pub fn pretty_print(&self) -> heapless::String<256> {
        let mut output = heapless::String::new();

        // Position information
        if let (Some(lat), Some(lon)) = (self.latitude, self.longitude) {
            let _ = write!(output, "📍 Position: {:.6}°N, {:.6}°E", lat, lon);
        } else {
            let _ = write!(output, "📍 Position: No fix");
        }

        // Altitude
        if let Some(alt) = self.altitude {
            let _ = write!(output, " | Altitude: {:.1}m", alt);
        }

        // Speed
        if let Some(speed) = self.speed_over_ground {
            let _ = write!(output, " | Speed: {:.1}km/h", speed * 1.852); // Convert knots to km/h
        }

        // Fix information
        if let Some(fix_type) = &self.fix_type {
            let _ = write!(output, " | Fix: {:?}", fix_type);
        }

        // Satellite count
        if let Some(sat_count) = self.num_of_fix_satellites {
            let _ = write!(output, " | Sats: {}", sat_count);
        }

        // DOP values
        if let Some(hdop) = self.hdop {
            let _ = write!(output, " | HDOP: {:.1}", hdop);
        }
        if let Some(vdop) = self.vdop {
            let _ = write!(output, " | VDOP: {:.1}", vdop);
        }
        if let Some(pdop) = self.pdop {
            let _ = write!(output, " | PDOP: {:.1}", pdop);
        }

        // Time and date
        if let Some(time) = self.fix_time {
            let _ = write!(output, " | Time: {}", time);
        }
        if let Some(date) = self.fix_date {
            let _ = write!(output, " | Date: {}", date);
        }

        output
    }
}
