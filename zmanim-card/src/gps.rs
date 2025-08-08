use embassy_rp::uart::BufferedUartRx;
use embassy_time::Duration;
use embassy_time::Ticker;
use heapless::Vec;
use nmea::sentences::FixType;
use nmea::Nmea;
pub const MAXLINELENGTH: usize = 150;
use core::fmt::Write;
use embedded_io_async::Read;
use heapless::String as HString;

use crate::GPS;

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
    pub last_update_tick: Option<u32>,
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
            last_update_tick: None,
        }
    }
}

impl GpsData {
    /// Update GPS data fields only if the new data is better (not None and potentially more recent)
    pub fn update_from_nmea(&mut self, nmea: &Nmea, current_tick: u32) -> bool {
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
            self.fix_date = Some(date);
            changed = true;
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

        // Update the last update tick
        self.last_update_tick = Some(current_tick);
        changed
    }

    /// Check if we have a valid position fix
    pub fn has_position_fix(&self) -> bool {
        self.latitude.is_some()
            && self.longitude.is_some()
            && self.fix_type.as_ref().map_or(false, |fix| fix.is_valid())
    }

    /// Get the age of the data in ticks
    pub fn data_age(&self, current_tick: u32) -> Option<u32> {
        self.last_update_tick
            .map(|last| current_tick.saturating_sub(last))
    }

    /// Pretty print the GPS data for logging
    ///
    /// Example output:
    /// "📍 Position: 40.712800°N, -74.006000°E | Altitude: 10.5m | Speed: 25.3km/h | Fix: Gps | Sats: 8 | HDOP: 1.2 | VDOP: 1.8 | PDOP: 2.1 | Time: 14:30:25 | Date: 2024-01-15 | Age: 150ms"
    pub fn pretty_print(&self) -> HString<256> {
        let mut output = HString::new();

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

        // Data age
        if let Some(age) = self.data_age(self.last_update_tick.unwrap_or(0)) {
            let _ = write!(output, " | Age: {}ms", age * 50); // Assuming 50ms tick rate
        }

        output
    }
}

pub struct Gps {
    /// The buffer for the current line as it is being built
    line_buffer: Vec<u8, MAXLINELENGTH>,
    /// The index of the current character in the line buffer
    line_idx: usize,
    /// The tick count when the first character of the current line was received
    first_char_time: Option<u32>,
    /// How many ticks have passed since the last update
    tick_count: u32,
    /// The accumulated GPS data
    gps_data: GpsData,
    /// The last time a complete line was received
    last_update: Option<u32>,
}

#[embassy_executor::task]
pub async fn run_gps(mut rx: BufferedUartRx) {
    let mut ticker = Ticker::every(Duration::from_millis(50));
    loop {
        // Read whatever bytes are currently available (at least 1) from UART without holding the GPS lock.
        let mut buf = [0u8; 64];
        if let Ok(n) = rx.read(&mut buf).await {
            // Briefly lock GPS only to process the received bytes.
            let mut gps_lock = GPS.lock().await;
            if let Some(gps) = &mut *gps_lock {
                for &b in &buf[..n] {
                    gps.process_char(b);
                }
            }
        }
        // Yield periodically to keep system responsive even if GPS is very chatty.
        ticker.next().await;
    }
}

impl Gps {
    pub fn new() -> Gps {
        Gps {
            line_buffer: Vec::new(),
            line_idx: 0,
            first_char_time: None,
            tick_count: 0,
            gps_data: GpsData::default(),
            last_update: None,
        }
    }

    // fn _write_byte(&mut self, c: u8) {
    //     let _ = self.tx.write(c);
    // }

    pub fn process_char(&mut self, c: u8) {
        let arr = [c];
        let c_str = core::str::from_utf8(&arr);
        if let Ok(c_str) = c_str {
            // Ignore carriage return, we'll handle it with the newline
            if c_str == "\r" {
                return;
            }
            // If this is the first character of a line, record the tick count
            if self.first_char_time.is_none() {
                self.first_char_time = Some(self.tick_count);
            }

            // Add the character to the line buffer if we have space
            if self.line_idx < MAXLINELENGTH - 1 {
                if let Ok(_) = self.line_buffer.push(c) {
                    self.line_idx += 1;
                }
            }

            // If we have a complete line, process it
            if c == b'\n' {
                self.process_complete_line(self.tick_count);
            }
        }
    }
    fn process_complete_line(&mut self, current_time: u32) {
        // Convert buffer to string, handling potential UTF-8 issues
        if let Ok(line_str) = core::str::from_utf8(&self.line_buffer) {
            // Strip the "\r\n" from the end of the line
            let line_str = line_str.trim_end();

            // Create a new parser for this sentence
            let mut nmea_parser = Nmea::default();

            // Parse the NMEA sentence using the nmea crate
            let result = nmea_parser.parse(line_str);

            match result {
                Ok(parsed) => {
                    log::info!("Parse Result: {:?}", parsed);
                }
                Err(e) => {
                    log::info!("Parse Error: {:?}", e);
                }
            }
            log::info!("Raw NMEA: {:?}", line_str);

            // Record this tick as the last update time
            self.last_update = Some(current_time);

            // Update GPS data from the parsed NMEA sentence
            let changed = self.gps_data.update_from_nmea(&nmea_parser, current_time);

            // Log the latest complete GPS data
            if changed {
                log::info!("GPS Data: {}", self.gps_data.pretty_print());
            }

            // Reset for next line
            self.line_buffer.clear();
            self.line_idx = 0;
            self.first_char_time = None;
        }
    }
    // fn send_command(&mut self, command: &str) {
    //     for c in command.as_bytes().iter() {
    //         self._write_byte(*c);
    //     }
    //     // Send newline
    //     self._write_byte(b'\r');
    //     self._write_byte(b'\n');
    // }

    /// Get a reference to the current GPS data
    pub fn get_gps_data(&self) -> &GpsData {
        &self.gps_data
    }

    /// Get a mutable reference to the GPS data
    pub fn get_gps_data_mut(&mut self) -> &mut GpsData {
        &mut self.gps_data
    }
}
