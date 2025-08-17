use embassy_rp::uart::BufferedUartRx;
use embassy_rp::uart::BufferedUartTx;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::channel::Channel;
use embassy_sync::pubsub::PubSubBehavior;
use embassy_sync::pubsub::PubSubChannel;
use embassy_sync::pubsub::WaitResult;
use embassy_time::with_timeout;
use embassy_time::Duration;
use embassy_time::Ticker;
use heapless::Vec;

use nmea::sentences::FixType;
use nmea::Nmea;
pub const MAXLINELENGTH: usize = 150;
use core::fmt::Write as _;
use core::u8;
use embedded_io_async::Read;
use embedded_io_async::Write;

type String = heapless::String<MAXLINELENGTH>;

// The listen_gps task publishes lines to the NEW_LINE_CHANNEL
// The process_gps task subscribes to the NEW_LINE_CHANNEL and publishes GPS data to the GPS_DATA_CHANNEL
// Any call to wait_for_sentence will also register and quickly unregister the subscriber to the NEW_LINE_CHANNEL
// Becuase both taksks need to receive the same lines, we use a pubsub channel
static INCOMING_LINE_CHANNEL: PubSubChannel<ThreadModeRawMutex, String, 64, 2, 64> =
    PubSubChannel::new();

// The process_gps task publishes GPS data to the GPS_DATA_CHANNEL
// Other tasks can subscribe to the GPS_DATA_CHANNEL to get the latest GPS data
// Again, we use a pubsub channel so that multiple tasks can subscribe to the same channel
pub static GPS_DATA_CHANNEL: PubSubChannel<ThreadModeRawMutex, GpsData, 64, 1, 64> =
    PubSubChannel::new();

// The write_gps task subscribes to the OUTGOING_LINE_CHANNEL and writes the lines to the UART
// Becuase there is only a single subscriber, we use a simple channel
static OUTGOING_LINE_CHANNEL: Channel<ThreadModeRawMutex, String, 64> = Channel::new();

#[derive(Debug, Clone, PartialEq)]
pub enum GpsState {
    Enabled,
    Disabled,
}

pub static GPS_STATE_CHANNEL: Channel<ThreadModeRawMutex, GpsState, 1> = Channel::new();

/// The listen_gps task reads from the UART and publishes lines to the NEW_LINE_CHANNEL
#[embassy_executor::task]
pub async fn listen_gps_task(mut rx: BufferedUartRx) {
    let mut ticker = Ticker::every(Duration::from_millis(50));
    let mut line_buffer: String = String::new();

    loop {
        // Read bytes first without holding the mutex.
        let mut buf = [0u8; 512];
        let n = rx.read(&mut buf).await.unwrap_or(0);
        if n > 0 {
            for &b in &buf[..n] {
                // Ignore carriage return, we'll handle it with the newline
                if b == b'\r' {
                    continue;
                }
                // If we have a newline, send the line buffer to the channel
                // and clear the line buffer
                if b == b'\n' {
                    INCOMING_LINE_CHANNEL.publish_immediate(line_buffer.clone());
                    line_buffer.clear();
                    continue;
                }
                // Otherwise, add the byte to the line buffer
                line_buffer.push(b as char).unwrap();
            }
        }

        // Yield periodically to keep system responsive even if GPS is very chatty.
        ticker.next().await;
    }
}

/// The write_gps task subscribes to the OUTGOING_LINE_CHANNEL and writes the lines to the UART
#[embassy_executor::task]
pub async fn write_gps_task(mut tx: BufferedUartTx) {
    loop {
        let line = OUTGOING_LINE_CHANNEL.receive().await;
        log::info!("Writing line: {:?}", line);
        tx.write(line.as_bytes()).await.unwrap();
    }
}

#[embassy_executor::task]
pub async fn gps_state_task() {
    loop {
        let state = GPS_STATE_CHANNEL.receive().await;
        match state {
            GpsState::Enabled => {
                log::info!("Enabling GPS");
                // Send a newline to the GPS to wake it up
                send_sentence(heapless::String::try_from("\r\n").unwrap()).await;

                log::info!("GPS enabled");
            }
            GpsState::Disabled => {
                log::info!("Disabling GPS");
                // Send a command to the GPS to disable the GPS
                send_sentence(heapless::String::try_from(PMTK_STANDBY).unwrap()).await;
                log::info!("GPS disabled");
            }
        }
    }
}

/// The process_gps task subscribes to the INCOMING_LINE_CHANNEL and publishes GPS data to the GPS_DATA_CHANNEL
#[embassy_executor::task]
pub async fn process_gps_task() {
    let mut latest_gps_data = GpsData::default();
    let mut subscriber = INCOMING_LINE_CHANNEL.subscriber().unwrap();
    let publisher = GPS_DATA_CHANNEL.publisher().unwrap();
    loop {
        let line = match subscriber.next_message().await {
            WaitResult::Message(line) => line,
            WaitResult::Lagged(_) => continue,
        };
        let mut nmea_parser = Nmea::default();
        let result = nmea_parser.parse(&line);
        match result {
            Ok(_) => {
                latest_gps_data.update_from_nmea(&nmea_parser);
                publisher.publish(latest_gps_data.clone()).await;
            }
            Err(e) => {
                log::info!("Parse Error: {:?} | Line: {:?}", e, line);
            }
        }
    }
}

pub async fn send_sentence(sentence: String) {
    OUTGOING_LINE_CHANNEL.send(sentence).await
}

pub async fn wait_for_sentence(sentence: String) {
    let mut subscriber = INCOMING_LINE_CHANNEL.subscriber().unwrap();
    loop {
        let line = match subscriber.next_message().await {
            WaitResult::Message(line) => line,
            WaitResult::Lagged(_) => continue,
        };
        if line == sentence {
            log::info!("Sentence received: {:?}", line);
            return;
        } else {
            log::info!("Sentence not received: {:?} != {:?}", line, sentence);
        }
    }
}

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
// Standby Command and boot successful message
const PMTK_STANDBY: &str = "$PMTK161,0*28";
// const PMTK_STANDBY_SUCCESS: &str = "$PMTK001,161,3*36";
const PMTK_AWAKE: &str = "$PMTK010,002*2D";
