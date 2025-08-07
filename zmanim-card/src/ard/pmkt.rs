//! PMTK GPS Command Constants
//!
//! This module contains constants for PMTK commands used with GPS modules
//! that use the MTK33x9 chipset (like Adafruit Ultimate GPS).
//!
//! Based on the Adafruit GPS library PMTK definitions.

/// Different commands to set the update rate from once a second (1 Hz) to 10 times
/// a second (10Hz). Note that these only control the rate at which the position is
/// echoed, to actually speed up the position fix you must also send one of the
/// position fix rate commands below too.

/// Once every 10 seconds, 100 millihertz
pub const PMTK_SET_NMEA_UPDATE_100_MILLIHERTZ: &str = "$PMTK220,10000*2F";

/// Once every 5 seconds, 200 millihertz
pub const PMTK_SET_NMEA_UPDATE_200_MILLIHERTZ: &str = "$PMTK220,5000*1B";

/// 1 Hz update rate
pub const PMTK_SET_NMEA_UPDATE_1HZ: &str = "$PMTK220,1000*1F";

/// 2 Hz update rate
pub const PMTK_SET_NMEA_UPDATE_2HZ: &str = "$PMTK220,500*2B";

/// 5 Hz update rate
pub const PMTK_SET_NMEA_UPDATE_5HZ: &str = "$PMTK220,200*2C";

/// 10 Hz update rate
pub const PMTK_SET_NMEA_UPDATE_10HZ: &str = "$PMTK220,100*2F";

// Position fix update rate commands

/// Once every 10 seconds, 100 millihertz
pub const PMTK_API_SET_FIX_CTL_100_MILLIHERTZ: &str = "$PMTK300,10000,0,0,0,0*2C";

/// Once every 5 seconds, 200 millihertz
pub const PMTK_API_SET_FIX_CTL_200_MILLIHERTZ: &str = "$PMTK300,5000,0,0,0,0*18";

/// 1 Hz position fix rate
pub const PMTK_API_SET_FIX_CTL_1HZ: &str = "$PMTK300,1000,0,0,0,0*1C";

/// 5 Hz position fix rate (maximum supported)
pub const PMTK_API_SET_FIX_CTL_5HZ: &str = "$PMTK300,200,0,0,0,0*2F";

// Baud rate settings

/// Set baud rate to 115200 bps
pub const PMTK_SET_BAUD_115200: &str = "$PMTK251,115200*1F";

/// Set baud rate to 57600 bps
pub const PMTK_SET_BAUD_57600: &str = "$PMTK251,57600*2C";

/// Set baud rate to 9600 bps
pub const PMTK_SET_BAUD_9600: &str = "$PMTK251,9600*17";

// NMEA output configuration commands

/// Turn on only the GPGLL sentence
pub const PMTK_SET_NMEA_OUTPUT_GLLONLY: &str = "$PMTK314,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0*29";

/// Turn on only the GPRMC sentence
pub const PMTK_SET_NMEA_OUTPUT_RMCONLY: &str = "$PMTK314,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0*29";

/// Turn on only the GPVTG sentence
pub const PMTK_SET_NMEA_OUTPUT_VTGONLY: &str = "$PMTK314,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0*29";

/// Turn on just the GPGGA sentence
pub const PMTK_SET_NMEA_OUTPUT_GGAONLY: &str = "$PMTK314,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0*29";

/// Turn on just the GPGSA sentence
pub const PMTK_SET_NMEA_OUTPUT_GSAONLY: &str = "$PMTK314,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0*29";

/// Turn on just the GPGSV sentence
pub const PMTK_SET_NMEA_OUTPUT_GSVONLY: &str = "$PMTK314,0,0,0,0,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0*29";

/// Turn on GPRMC and GPGGA sentences
pub const PMTK_SET_NMEA_OUTPUT_RMCGGA: &str = "$PMTK314,0,1,0,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0*28";

/// Turn on GPRMC, GPGGA and GPGSA sentences
pub const PMTK_SET_NMEA_OUTPUT_RMCGGAGSA: &str =
    "$PMTK314,0,1,0,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0,0*29";

/// Turn on ALL THE DATA
pub const PMTK_SET_NMEA_OUTPUT_ALLDATA: &str = "$PMTK314,1,1,1,1,1,1,0,0,0,0,0,0,0,0,0,0,0,0,0*28";

/// Turn off NMEA output
pub const PMTK_SET_NMEA_OUTPUT_OFF: &str = "$PMTK314,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0,0*28";

// LOCUS (logging) commands

/// Start logging data
pub const PMTK_LOCUS_STARTLOG: &str = "$PMTK185,0*22";

/// Stop logging data
pub const PMTK_LOCUS_STOPLOG: &str = "$PMTK185,1*23";

/// Acknowledge the start or stop command
pub const PMTK_LOCUS_STARTSTOPACK: &str = "$PMTK001,185,3*3C";

/// Query the logging status
pub const PMTK_LOCUS_QUERY_STATUS: &str = "$PMTK183*38";

/// Erase the log flash data
pub const PMTK_LOCUS_ERASE_FLASH: &str = "$PMTK184,1*22";

/// If flash is full, log will overwrite old data with new logs
pub const LOCUS_OVERLAP: u8 = 0;

/// If flash is full, logging will stop
pub const LOCUS_FULLSTOP: u8 = 1;

// SBAS/WAAS commands

/// Enable search for SBAS satellite (only works with 1Hz output rate)
pub const PMTK_ENABLE_SBAS: &str = "$PMTK313,1*2E";

/// Use WAAS for DGPS correction data
pub const PMTK_ENABLE_WAAS: &str = "$PMTK301,2*2E";

// Power management commands

/// Standby command & boot successful message
pub const PMTK_STANDBY: &str = "$PMTK161,0*28";

/// Standby success response (not needed currently)
pub const PMTK_STANDBY_SUCCESS: &str = "$PMTK001,161,3*36";

/// Wake up command
pub const PMTK_AWAKE: &str = "$PMTK010,002*2D";

// Information commands

/// Ask for the release and version
pub const PMTK_Q_RELEASE: &str = "$PMTK605*31";

// Antenna commands

/// Request for updates on antenna status
pub const PGCMD_ANTENNA: &str = "$PGCMD,33,1*6C";

/// Don't show antenna status messages
pub const PGCMD_NOANTENNA: &str = "$PGCMD,33,0*6D";

// Configuration constants

/// How long to wait when we're looking for a response
pub const MAXWAITSENTENCE: u8 = 10;
