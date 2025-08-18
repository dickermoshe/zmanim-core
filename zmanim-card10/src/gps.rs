use chrono::Datelike;
use embassy_executor::Spawner;
use embassy_sync::{
    blocking_mutex::raw::{CriticalSectionRawMutex, NoopRawMutex},
    mutex::Mutex,
    pubsub::{PubSubChannel, WaitResult},
    signal::Signal,
};
use esp_hal::{
    gpio::{Level, Output, OutputConfig, OutputPin},
    rtc_cntl::Rtc,
};

use esp_hal::{
    gpio::interconnect::{PeripheralInput, PeripheralOutput},
    uart::{Config, Instance, RxConfig, Uart, UartRx},
    Async,
};
use esp_println::println;
use heapless::{String as HString, Vec as HVec};

use nmea::Nmea;
use static_cell::StaticCell;

use crate::gps_data::GpsData;
const READ_BUF_SIZE: usize = 64;
const LINE_BUF_CAPACITY: usize = 128;
pub type GpsDataChannelType = PubSubChannel<NoopRawMutex, GpsData, 64, 2, 64>;
type RtcType = Mutex<CriticalSectionRawMutex, Option<Rtc<'static>>>;

pub enum GpsState {
    On,
    Off,
}
pub static GPS_STATE: Signal<CriticalSectionRawMutex, GpsState> = Signal::new();

/// Start the GPS tasks.
///
/// This function spawns the following tasks:
/// - uart_line_reader: Reads bytes from the UART an emits lines to the incoming_line_channel
/// - process_gps_task: Processes GPS data from the incoming_line_channel and publishes to the gps_data_channel
/// - write_gps_task: Writes lines to the UART from the outgoing_line_channel
/// - gps_data_logger: Logs GPS data to the screen
pub fn init_gps(
    tx_pin: impl PeripheralOutput<'static>,
    rx_pin: impl PeripheralInput<'static>,
    uart: impl Instance + 'static,
    spawner: &Spawner,
    gps_enable_pin: impl OutputPin + 'static,
) -> &'static GpsDataChannelType {
    let config = Config::default()
        .with_rx(RxConfig::default().with_fifo_full_threshold(READ_BUF_SIZE as u16))
        .with_baudrate(9600);
    let uart0 = Uart::new(uart, config)
        .unwrap()
        .with_tx(tx_pin)
        .with_rx(rx_pin)
        .into_async();
    let (rx, _) = uart0.split();

    static GPS_ENABLE_PIN: StaticCell<Output<'static>> = StaticCell::new();
    let gps_enable_pin = GPS_ENABLE_PIN.init(Output::new(
        gps_enable_pin,
        Level::Low,
        OutputConfig::default(),
    ));

    // The process_gps task publishes GPS data to the GPS_DATA_CHANNEL
    // Other tasks can subscribe to the GPS_DATA_CHANNEL to get the latest GPS data
    // Again, we use a pubsub channel so that multiple tasks can subscribe to the same channel
    static GPS_DATA_CHANNEL_CELL: StaticCell<GpsDataChannelType> = StaticCell::new();
    let gps_data_channel = GPS_DATA_CHANNEL_CELL.init(PubSubChannel::new());

    spawner.spawn(gps_task(rx, gps_data_channel)).ok();
    spawner.spawn(handle_gps_state(gps_enable_pin)).ok();
    // spawner.spawn(gps_data_logger(gps_data_channel)).ok();
    gps_data_channel
}

#[embassy_executor::task]
async fn handle_gps_state(gps_enable_pin: &'static mut Output<'static>) {
    loop {
        let state = GPS_STATE.wait().await;
        match state {
            GpsState::On => {
                println!("GPS enabled");
                gps_enable_pin.set_low();
            }
            GpsState::Off => {
                println!("GPS disabled");
                gps_enable_pin.set_high();
            }
        }
    }
}

/// Read bytes from the UART and store them in a buffer
#[embassy_executor::task]
async fn gps_task(mut rx: UartRx<'static, Async>, gps_data_channel: &'static GpsDataChannelType) {
    let publisher = gps_data_channel.publisher().unwrap();
    let mut main_buffer: HVec<u8, LINE_BUF_CAPACITY> = HVec::new();
    let mut buf = [0u8; READ_BUF_SIZE];

    loop {
        let result = embedded_io_async::Read::read(&mut rx, &mut buf).await;
        match result {
            Ok(len) => {
                if len > 0 {
                    for &b in &buf[..len] {
                        // Ignore carriage return, we'll handle it with the newline
                        if b == b'\r' {
                            continue;
                        }
                        if b == b'\n' {
                            // Parse the line as a string
                            let copied_buffer = main_buffer.clone();
                            main_buffer.clear();

                            let parsed_str_result = core::str::from_utf8(&copied_buffer);
                            if parsed_str_result.is_err() {
                                println!("Error parsing string: {:?}", parsed_str_result);
                                continue;
                            }
                            let parsed_str = parsed_str_result.unwrap();

                            // Parse the line as an NMEA sentence
                            let mut line: HString<LINE_BUF_CAPACITY> = HString::new();
                            let _ = line.push_str(parsed_str);
                            let mut nmea = Nmea::default();
                            let parse_result = nmea.parse(line.as_str());

                            // If the line is not a valid NMEA sentence, continue
                            if parse_result.is_err() {
                                println!("Error parsing NMEA sentence: {:?}", parse_result);
                                continue;
                            }

                            // Get the timestamp from the NMEA sentence
                            let timestamp = {
                                match (nmea.fix_time, nmea.fix_date) {
                                    (Some(fix_time), Some(fix_date)) => {
                                        // If the year is 2080 or higher, the GPS is probably
                                        // not valid, so we'll return None
                                        if fix_date.year() >= 2080 {
                                            None
                                        } else {
                                            Some(
                                                chrono::NaiveDateTime::new(fix_date, fix_time)
                                                    .and_utc()
                                                    .timestamp_millis(),
                                            )
                                        }
                                    }
                                    _ => None,
                                }
                            };

                            // Create the GPS data
                            let data = GpsData {
                                latitude: nmea.latitude,
                                longitude: nmea.longitude,
                                altitude: nmea.altitude,
                                speed_over_ground: nmea.speed_over_ground,
                                timestamp,
                                num_of_fix_satellites: nmea.num_of_fix_satellites,
                                hdop: nmea.hdop,
                                vdop: nmea.vdop,
                                pdop: nmea.pdop,
                                geoid_separation: nmea.geoid_separation,
                            };
                            println!("Publishing GPS data: {:?}", data);

                            // Publish the GPS data
                            publisher.publish_immediate(data);
                        } else {
                            // Otherwise, add the byte to the line buffer
                            let _ = main_buffer.push(b);
                        }
                    }
                }
            }
            Err(e) => {
                println!("UART read error: {:?}", e);
            }
        }
    }
}

/// GPS data logger task that listens to GPS data and prints it to the screen
#[embassy_executor::task]
async fn gps_data_logger(gps_data_channel: &'static GpsDataChannelType) {
    let mut subscriber = gps_data_channel.subscriber().unwrap();
    loop {
        let gps_data = subscriber.next_message().await;
        match gps_data {
            WaitResult::Message(data) => {
                println!("GPS Data: {:?}", data);
            }
            WaitResult::Lagged(_) => continue,
        }
    }
}
