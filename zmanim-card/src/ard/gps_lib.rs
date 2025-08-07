//! Rust GPS library using the nmea crate for parsing
//! Port of Adafruit GPS library adapted for Rust embedded systems

use crate::ard::gps_types::*;
use heapless::{String, Vec};
use nb;

/// GPS communication interface trait
pub trait GpsComm {
    type Error;

    /// Read a single byte from the GPS module
    fn read(&mut self) -> nb::Result<u8, Self::Error>;

    /// Write a single byte to the GPS module
    fn write(&mut self, byte: u8) -> nb::Result<(), Self::Error>;

    /// Check if data is available to read
    fn available(&self) -> usize;

    /// Get current time in milliseconds (for timing calculations)
    fn millis(&self) -> u32;
}

/// Main GPS driver structure
pub struct AdafruitGps<COMM> {
    comm: COMM,

    // Data buffers
    line_buffer: Vec<u8, MAXLINELENGTH>,
    current_line: String<MAXLINELENGTH>,
    last_line: String<MAXLINELENGTH>,

    // State
    data: GpsData,
    timing: GpsTiming,
    sentence_info: SentenceInfo,

    // Flags
    received_flag: bool,
    paused: bool,
    in_standby_mode: bool,
    no_comms: bool,

    // Line buffering state
    line_idx: usize,
    first_char_time: u32,
}

impl<COMM> AdafruitGps<COMM>
where
    COMM: GpsComm,
{
    /// Create a new GPS instance with the given communication interface
    pub fn new(comm: COMM) -> Self {
        Self {
            comm,
            line_buffer: Vec::new(),
            current_line: String::new(),
            last_line: String::new(),
            data: GpsData::default(),
            timing: GpsTiming::default(),
            sentence_info: SentenceInfo::default(),
            received_flag: false,
            paused: false,
            in_standby_mode: false,
            no_comms: false,
            line_idx: 0,
            first_char_time: 0,
        }
    }

    /// Initialize the GPS module with the given baud rate
    pub fn begin(&mut self, _baud: u32) -> Result<(), COMM::Error> {
        // Implementation would depend on the specific communication interface
        // For now, just reset internal state
        self.common_init();
        Ok(())
    }

    /// Common initialization for all constructor types
    fn common_init(&mut self) {
        self.received_flag = false;
        self.paused = false;
        self.in_standby_mode = false;
        self.line_idx = 0;
        self.data = GpsData::default();
        self.timing = GpsTiming::default();
        self.line_buffer.clear();
        self.current_line.clear();
        self.last_line.clear();
    }

    /// Check how many bytes are available to read
    pub fn available(&self) -> usize {
        if self.paused {
            0
        } else {
            self.comm.available()
        }
    }

    /// Write a byte to the GPS module
    pub fn write(&mut self, byte: u8) -> nb::Result<(), COMM::Error> {
        self.comm.write(byte)
    }

    /// Read and process one character from the GPS device
    pub fn read(&mut self) -> Result<Option<u8>, COMM::Error> {
        if self.paused || self.no_comms {
            return Ok(None);
        }

        let current_time = self.comm.millis();

        match self.comm.read() {
            Ok(c) => {
                // Track first character time for sentence timing
                if self.first_char_time == 0 {
                    self.first_char_time = current_time;
                }

                // Add character to line buffer
                if self.line_idx < MAXLINELENGTH - 1 {
                    if let Ok(_) = self.line_buffer.push(c) {
                        self.line_idx += 1;
                    }
                }

                // Check for end of line
                if c == b'\n' {
                    self.process_complete_line(current_time);
                }

                Ok(Some(c))
            }
            Err(nb::Error::WouldBlock) => Ok(None),
            Err(nb::Error::Other(e)) => Err(e),
        }
    }

    /// Process a complete NMEA line
    fn process_complete_line(&mut self, current_time: u32) {
        // Convert buffer to string, handling potential UTF-8 issues
        if let Ok(line_str) = core::str::from_utf8(&self.line_buffer) {
            // Store the previous line
            self.last_line.clear();
            let _ = self.last_line.push_str(&self.current_line);

            // Update current line
            self.current_line.clear();
            let _ = self.current_line.push_str(line_str.trim());

            // Parse the NMEA sentence using the nmea crate
            let line_copy = self.current_line.clone();
            self.parse_nmea_sentence(&line_copy);
        }

        // Reset for next line
        self.line_buffer.clear();
        self.line_idx = 0;
        self.received_flag = true;
        self.timing.recv_time = current_time;
        self.timing.sent_time = self.first_char_time;
        self.first_char_time = 0;
    }

    /// Parse NMEA sentence using the nmea crate
    fn parse_nmea_sentence(&mut self, sentence: &str) {
        // This would use the nmea crate to parse the sentence
        // For now, we'll implement basic parsing structure

        if sentence.is_empty() || !sentence.starts_with('$') {
            return;
        }

        // Update timing for successful parse
        self.timing.last_update = self.comm.millis();

        // Extract sentence type for routing
        if let Some(comma_pos) = sentence.find(',') {
            let header = &sentence[1..comma_pos]; // Skip the '$'

            if header.len() >= 5 {
                let source = &header[0..2];
                let sentence_type = &header[2..5];

                // Update sentence info
                self.sentence_info.source.clear();
                let _ = self.sentence_info.source.push_str(source);
                self.sentence_info.sentence.clear();
                let _ = self.sentence_info.sentence.push_str(sentence_type);

                // Route to appropriate parser based on sentence type
                match sentence_type {
                    "GGA" => self.parse_gga_sentence(sentence),
                    "RMC" => self.parse_rmc_sentence(sentence),
                    "GLL" => self.parse_gll_sentence(sentence),
                    "GSA" => self.parse_gsa_sentence(sentence),
                    "GSV" => self.parse_gsv_sentence(sentence),
                    _ => {} // Unknown or unsupported sentence type
                }
            }
        }
    }

    /// Parse GGA (Global Positioning System Fix Data) sentence
    fn parse_gga_sentence(&mut self, _sentence: &str) {
        // Implementation would use nmea crate's GGA parser
        // Update relevant fields in self.data
        self.timing.last_fix = self.comm.millis();
    }

    /// Parse RMC (Recommended Minimum) sentence
    fn parse_rmc_sentence(&mut self, _sentence: &str) {
        // Implementation would use nmea crate's RMC parser
        // Update relevant fields in self.data
        self.timing.last_time = self.comm.millis();
        self.timing.last_date = self.comm.millis();
    }

    /// Parse GLL (Geographic Position - Latitude/Longitude) sentence
    fn parse_gll_sentence(&mut self, _sentence: &str) {
        // Implementation would use nmea crate's GLL parser
    }

    /// Parse GSA (GPS DOP and active satellites) sentence
    fn parse_gsa_sentence(&mut self, _sentence: &str) {
        // Implementation would use nmea crate's GSA parser
    }

    /// Parse GSV (Satellites in view) sentence
    fn parse_gsv_sentence(&mut self, _sentence: &str) {
        // Implementation would use nmea crate's GSV parser
    }

    /// Send a command to the GPS device
    pub fn send_command(&mut self, command: &str) -> Result<(), COMM::Error> {
        for byte in command.bytes() {
            match self.write(byte) {
                Ok(_) => {}
                Err(nb::Error::WouldBlock) => {
                    // Handle would block - might need to retry
                }
                Err(nb::Error::Other(e)) => return Err(e),
            }
        }
        // Send newline
        match self.write(b'\r') {
            Ok(_) => {}
            Err(nb::Error::Other(e)) => return Err(e),
            _ => {}
        }
        match self.write(b'\n') {
            Ok(_) => {}
            Err(nb::Error::Other(e)) => return Err(e),
            _ => {}
        }
        Ok(())
    }

    /// Check if a new NMEA sentence has been received
    pub fn new_nmea_received(&self) -> bool {
        self.received_flag
    }

    /// Get the last received NMEA sentence and clear the received flag
    pub fn last_nmea(&mut self) -> &str {
        self.received_flag = false;
        &self.last_line
    }

    /// Pause or unpause receiving new data
    pub fn pause(&mut self, paused: bool) {
        self.paused = paused;
    }

    /// Get current GPS data
    pub fn data(&self) -> &GpsData {
        &self.data
    }

    /// Get mutable reference to GPS data
    pub fn data_mut(&mut self) -> &mut GpsData {
        &mut self.data
    }

    /// Get timing information
    pub fn timing(&self) -> &GpsTiming {
        &self.timing
    }

    /// Get sentence information
    pub fn sentence_info(&self) -> &SentenceInfo {
        &self.sentence_info
    }

    /// Calculate seconds since last fix
    pub fn seconds_since_fix(&self) -> f32 {
        (self.comm.millis().saturating_sub(self.timing.last_fix)) as f32 / 1000.0
    }

    /// Calculate seconds since last time update
    pub fn seconds_since_time(&self) -> f32 {
        (self.comm.millis().saturating_sub(self.timing.last_time)) as f32 / 1000.0
    }

    /// Calculate seconds since last date update
    pub fn seconds_since_date(&self) -> f32 {
        (self.comm.millis().saturating_sub(self.timing.last_date)) as f32 / 1000.0
    }

    /// Put GPS module into standby mode
    pub fn standby(&mut self) -> Result<bool, COMM::Error> {
        if self.in_standby_mode {
            Ok(false) // Already in standby
        } else {
            self.in_standby_mode = true;
            self.send_command("$PMTK161,0*28")?; // PMTK_STANDBY command
            Ok(true)
        }
    }

    /// Wake up GPS module from standby
    pub fn wakeup(&mut self) -> Result<bool, COMM::Error> {
        if self.in_standby_mode {
            self.in_standby_mode = false;
            self.send_command("")?; // Send empty command to wake up
            Ok(true)
        } else {
            Ok(false) // Not in standby
        }
    }

    /// Wait for a specific sentence type
    pub fn wait_for_sentence(
        &mut self,
        sentence_type: &str,
        max_attempts: u8,
    ) -> Result<bool, COMM::Error> {
        let mut attempts = 0;

        while attempts < max_attempts {
            // Try to read data
            match self.read()? {
                Some(_) => {
                    if self.new_nmea_received() {
                        let nmea = self.last_nmea();
                        attempts += 1;

                        if nmea.contains(sentence_type) {
                            return Ok(true);
                        }
                    }
                }
                None => {
                    // No data available, continue
                }
            }
        }

        Ok(false)
    }
}

/// Helper functions for coordinate conversion
impl<COMM> AdafruitGps<COMM> {
    /// Convert NMEA coordinate format (DDMM.MMMM) to decimal degrees
    pub fn nmea_to_degrees(nmea_coord: f32) -> f32 {
        let degrees = libm::floorf(nmea_coord / 100.0);
        let minutes = nmea_coord - (degrees * 100.0);
        degrees + (minutes / 60.0)
    }

    /// Convert decimal degrees to fixed point representation
    pub fn degrees_to_fixed(degrees: f32) -> i32 {
        (degrees * 10_000_000.0) as i32
    }

    /// Convert fixed point representation to decimal degrees
    pub fn fixed_to_degrees(fixed: i32) -> f32 {
        fixed as f32 / 10_000_000.0
    }
}
