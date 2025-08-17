use defmt::info;
use embassy_executor::Spawner;
use embassy_sync::{
    blocking_mutex::raw::{CriticalSectionRawMutex, NoopRawMutex},
    mutex::Mutex,
    pubsub::PubSubChannel,
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
use heapless::{String as HString, Vec as HVec};

use nmea::Nmea;
use static_cell::StaticCell;

use crate::gps_data::GpsData;
const READ_BUF_SIZE: usize = 64;
const LINE_BUF_CAPACITY: usize = 128;
pub type GpsDataChannelType = PubSubChannel<NoopRawMutex, GpsData, 64, 2, 64>;
type RtcType = Mutex<CriticalSectionRawMutex, Option<Rtc<'static>>>;

pub static RTC: RtcType = Mutex::new(None);
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
    gps_data_channel
}

#[embassy_executor::task]
async fn handle_gps_state(gps_enable_pin: &'static mut Output<'static>) {
    loop {
        let state = GPS_STATE.wait().await;
        match state {
            GpsState::On => {
                info!("GPS enabled");
                gps_enable_pin.set_low();
            }
            GpsState::Off => {
                info!("GPS disabled");
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
                    info!("Read {} bytes from UART", len);
                    for &b in &buf[..len] {
                        // Ignore carriage return, we'll handle it with the newline
                        if b == b'\r' {
                            continue;
                        }
                        if b == b'\n' {
                            if !main_buffer.is_empty() {
                                let line_str = core::str::from_utf8(&main_buffer).unwrap_or("");
                                let mut line: HString<LINE_BUF_CAPACITY> = HString::new();
                                let _ = line.push_str(line_str);
                                let mut nmea = Nmea::default();
                                let _ = nmea.parse(line.as_str());
                                let data = GpsData {
                                    latitude: nmea.latitude,
                                    longitude: nmea.longitude,
                                    altitude: nmea.altitude,
                                    speed_over_ground: nmea.speed_over_ground,
                                    fix_time: nmea.fix_time,
                                    fix_date: nmea.fix_date,
                                    fix_type: nmea.fix_type,
                                    num_of_fix_satellites: nmea.num_of_fix_satellites,
                                    hdop: nmea.hdop,
                                    vdop: nmea.vdop,
                                    pdop: nmea.pdop,
                                    geoid_separation: nmea.geoid_separation,
                                };
                                info!("Publishing line: {}", line.as_str());

                                info!("Publishing GPS data: {}", data.pretty_print().as_str());
                                publisher.publish(data).await;
                                main_buffer.clear();
                            }
                        } else {
                            // Otherwise, add the byte to the line buffer
                            let _ = main_buffer.push(b);
                        }
                    }
                }
            }
            Err(e) => {
                info!("UART read error: {:?}", e);
            }
        }
    }
}
