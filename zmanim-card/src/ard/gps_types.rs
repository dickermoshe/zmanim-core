//! GPS data types and constants for the Rust port of Adafruit GPS library

use chrono::{DateTime, Utc};
use heapless::String;

/// Maximum length of NMEA lines to parse
pub const MAXLINELENGTH: usize = 120;

/// Maximum length of a sentence ID name, including terminating null
pub const NMEA_MAX_SENTENCE_ID: usize = 20;

/// Maximum length of a source ID name, including terminating null  
pub const NMEA_MAX_SOURCE_ID: usize = 3;

/// Type alias for NMEA floating point values
pub type NmeaFloat = f32;

/// Result codes from NMEA sentence validation
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct NmeaCheck {
    pub flags: u8,
}

impl NmeaCheck {
    pub const BAD: u8 = 0;
    pub const HAS_DOLLAR: u8 = 1;
    pub const HAS_CHECKSUM: u8 = 2;
    pub const HAS_NAME: u8 = 4;
    pub const HAS_SOURCE: u8 = 10;
    pub const HAS_SENTENCE: u8 = 20;
    pub const HAS_SENTENCE_P: u8 = 40;

    pub fn new(flags: u8) -> Self {
        Self { flags }
    }

    pub fn has_flag(&self, flag: u8) -> bool {
        (self.flags & flag) != 0
    }

    pub fn is_valid(&self) -> bool {
        self.has_flag(Self::HAS_DOLLAR) && self.has_flag(Self::HAS_CHECKSUM)
    }
}

/// GPS fix quality indicators
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum FixQuality {
    Gps = 1,
    Dgps = 2,
}

impl From<u8> for FixQuality {
    fn from(value: u8) -> Self {
        match value {
            1 => FixQuality::Gps,
            2 => FixQuality::Dgps,
        }
    }
}

/// 3D fix quality indicators
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Fix3DQuality {
    Fix2D = 2,
    Fix3D = 3,
}

impl From<u8> for Fix3DQuality {
    fn from(value: u8) -> Self {
        match value {
            2 => Fix3DQuality::Fix2D,
            3 => Fix3DQuality::Fix3D,
        }
    }
}

/// GPS coordinate direction indicators
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl From<char> for Direction {
    fn from(c: char) -> Self {
        match c.to_ascii_uppercase() {
            'N' => Direction::North,
            'S' => Direction::South,
            'E' => Direction::East,
            'W' => Direction::West,
        }
    }
}

impl Direction {
    pub fn to_char(&self) -> char {
        match self {
            Direction::North => 'N',
            Direction::South => 'S',
            Direction::East => 'E',
            Direction::West => 'W',
        }
    }
}

pub struct Location {
    pub latitude_degrees: NmeaFloat,  // Decimal degrees
    pub longitude_degrees: NmeaFloat, // Decimal degrees
    pub altitude: f32,
}

pub struct Accuracy {
    pub hdop: NmeaFloat, // Horizontal Dilution of Precision
    pub vdop: NmeaFloat, // Vertical Dilution of Precision
    pub pdop: NmeaFloat, // Position Dilution of Precision
}

/// Main GPS data structure containing all parsed information
#[derive(Debug, Clone)]
pub struct GpsData {
    // Time and date
    pub datetime: Option<DateTime<Utc>>,
    pub location: Option<Location>,
    pub accuracy: Option<Accuracy>,

    // Fix information
    pub fix: bool,
    pub fixquality: Option<FixQuality>,
    pub fixquality_3d: Option<Fix3DQuality>,
    pub satellites: Option<u8>,
    pub antenna: Option<u8>,
}

impl Default for GpsData {
    fn default() -> Self {
        Self {
            datetime: None,
            location: None,
            accuracy: None,
            fix: false,
            fixquality: None,
            fixquality_3d: None,
            satellites: None,
            antenna: None,
        }
    }
}

/// Sentence parsing state
#[derive(Debug, Clone)]
pub struct SentenceInfo {
    pub check: NmeaCheck,
    pub source: String<NMEA_MAX_SOURCE_ID>,
    pub sentence: String<NMEA_MAX_SENTENCE_ID>,
}

impl Default for SentenceInfo {
    fn default() -> Self {
        Self {
            check: NmeaCheck::new(NmeaCheck::BAD),
            source: String::new(),
            sentence: String::new(),
        }
    }
}

/// Timing information for GPS sentences
#[derive(Debug, Clone, Copy)]
pub struct GpsTiming {
    pub last_update: u32, // millis() when last full sentence successfully parsed
    pub last_fix: u32,    // millis() when last fix received
    pub last_time: u32,   // millis() when last time received
    pub last_date: u32,   // millis() when last date received
    pub recv_time: u32,   // millis() when last full sentence received
    pub sent_time: u32,   // millis() when first character of last sentence received
}

impl Default for GpsTiming {
    fn default() -> Self {
        // Set times far in the past (middle of millis() range)
        const DEFAULT_TIME: u32 = 2_000_000_000;
        Self {
            last_update: DEFAULT_TIME,
            last_fix: DEFAULT_TIME,
            last_time: DEFAULT_TIME,
            last_date: DEFAULT_TIME,
            recv_time: DEFAULT_TIME,
            sent_time: DEFAULT_TIME,
        }
    }
}
