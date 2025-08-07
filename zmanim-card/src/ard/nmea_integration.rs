//! NMEA parsing integration using the nmea crate

use crate::ard::gps_types::*;

/// NMEA parser wrapper that integrates with our GPS data structures
pub struct NmeaParser {
    parser: nmea::Nmea,
}

impl NmeaParser {
    /// Create a new NMEA parser
    pub fn new() -> Self {
        Self {
            parser: nmea::Nmea::default(),
        }
    }

    /// Parse an NMEA sentence and update GPS data
    pub fn parse_sentence(
        &mut self,
        sentence: &str,
        gps_data: &mut GpsData,
    ) -> Result<(), &'static str> {
        // The AeroRust nmea crate uses the parse method on the Nmea struct
        // which accumulates parsed data internally
        match self.parser.parse(sentence) {
            Ok(_) => {
                // Update GPS data from the parser's internal state
                self.update_gps_data_from_parser(gps_data);
                Ok(())
            }
            Err(_) => Err("Failed to parse NMEA sentence"),
        }
    }

    /// Update GPS data from the parser's internal state
    fn update_gps_data_from_parser(&self, gps_data: &mut GpsData) {
        // Update position data
        if let Some(lat) = self.parser.latitude {
            gps_data.latitude_degrees = lat as NmeaFloat;
            gps_data.latitude = Self::degrees_to_nmea_format(lat as NmeaFloat);
            gps_data.latitude_fixed = Self::degrees_to_fixed(lat as NmeaFloat);
        }

        if let Some(lon) = self.parser.longitude {
            gps_data.longitude_degrees = lon as NmeaFloat;
            gps_data.longitude = Self::degrees_to_nmea_format(lon as NmeaFloat);
            gps_data.longitude_fixed = Self::degrees_to_fixed(lon as NmeaFloat);
        }

        // Update altitude
        if let Some(alt) = self.parser.altitude {
            gps_data.altitude = alt as NmeaFloat;
        }

        // Note: speed and course are not directly available in the nmea crate
        // They would need to be extracted from specific sentence types

        // Update fix information
        gps_data.fix = self.parser.fix_type.is_some();

        // Update satellite count
        if let Some(sats) = self.parser.num_of_fix_satellites {
            gps_data.satellites = sats as u8;
        }

        // Update DOP values
        if let Some(hdop) = self.parser.hdop {
            gps_data.hdop = hdop as NmeaFloat;
        }

        if let Some(vdop) = self.parser.vdop {
            gps_data.vdop = vdop as NmeaFloat;
        }

        if let Some(pdop) = self.parser.pdop {
            gps_data.pdop = pdop as NmeaFloat;
        }

        // Update time information
        if let Some(datetime) = self.parser.fix_timestamp() {
            gps_data.hour = datetime.hour() as u8;
            gps_data.minute = datetime.minute() as u8;
            gps_data.seconds = datetime.second() as u8;
            gps_data.milliseconds = (datetime.nanosecond() / 1_000_000) as u16;
            // Note: NaiveTime doesn't have date information
            // Date information would need to be handled separately
        }

        // Update fix quality based on fix type
        if let Some(_fix_type) = &self.parser.fix_type {
            // The nmea crate doesn't expose GgaFixType in the public API
            // We'll set fix quality based on whether we have a fix
            gps_data.fixquality = if self.parser.fix_type.is_some() {
                FixQuality::Gps
            } else {
                FixQuality::Invalid
            };
        }
    }

    /// Convert decimal degrees to NMEA format (DDMM.MMMM)
    fn degrees_to_nmea_format(degrees: NmeaFloat) -> NmeaFloat {
        let abs_degrees = degrees.abs();
        let deg_part = libm::floorf(abs_degrees);
        let min_part = (abs_degrees - deg_part) * 60.0;
        deg_part * 100.0 + min_part
    }

    /// Convert decimal degrees to fixed point representation
    fn degrees_to_fixed(degrees: NmeaFloat) -> i32 {
        (degrees * 10_000_000.0) as i32
    }
}

impl Default for NmeaParser {
    fn default() -> Self {
        Self::new()
    }
}

/// Extension trait to add NMEA parsing capabilities to the GPS library
pub trait NmeaParsingExt {
    /// Parse an NMEA sentence and update internal GPS data
    fn parse_nmea_with_crate(&mut self, sentence: &str) -> Result<(), &'static str>;
}

// This would be implemented for AdafruitGps, but we can't do that here due to orphan rules
// Instead, users would need to create a wrapper or use composition
