use defmt::info;
use embassy_executor::Spawner;
use embassy_sync::{
    blocking_mutex::raw::NoopRawMutex,
    channel::Channel,
    pubsub::{PubSubChannel, WaitResult},
};
use esp_hal::{
    gpio::interconnect::{PeripheralInput, PeripheralOutput},
    uart::{Config, Instance, RxConfig, Uart, UartRx, UartTx},
    Async,
};
use heapless::{String as HString, Vec as HVec};

use nmea::Nmea;
use static_cell::StaticCell;

use crate::gps_data::GpsData;
const READ_BUF_SIZE: usize = 64;
const LINE_BUF_CAPACITY: usize = 128;
type INCOMING_LINE_CHANNEL_TYPE =
    PubSubChannel<NoopRawMutex, HString<LINE_BUF_CAPACITY>, 64, 2, 64>;
type OUTGOING_LINE_CHANNEL_TYPE = Channel<NoopRawMutex, HString<LINE_BUF_CAPACITY>, 64>;
type GPS_STATE_CHANNEL_TYPE = Channel<NoopRawMutex, GpsState, 64>;
type GPS_DATA_CHANNEL_TYPE = PubSubChannel<NoopRawMutex, GpsData, 64, 1, 64>;
pub fn start_gps(
    tx_pin: impl PeripheralOutput<'static>,
    rx_pin: impl PeripheralInput<'static>,
    uart: impl Instance + 'static,
    spawner: &Spawner,
) {
    let config = Config::default()
        .with_rx(RxConfig::default().with_fifo_full_threshold(READ_BUF_SIZE as u16))
        .with_baudrate(9600);
    let uart0 = Uart::new(uart, config)
        .unwrap()
        .with_tx(tx_pin)
        .with_rx(rx_pin)
        .into_async();
    let (rx, tx) = uart0.split();

    // The listen_gps task publishes lines to the NEW_LINE_CHANNEL
    // The process_gps task subscribes to the NEW_LINE_CHANNEL and publishes GPS data to the GPS_DATA_CHANNEL
    // Any call to wait_for_sentence will also register and quickly unregister the subscriber to the NEW_LINE_CHANNEL
    // Becuase both taksks need to receive the same lines, we use a pubsub channel
    static INCOMING_LINE_CHANNEL_CELL: StaticCell<INCOMING_LINE_CHANNEL_TYPE> = StaticCell::new();
    let incoming_line_channel = INCOMING_LINE_CHANNEL_CELL.init(PubSubChannel::new());
    spawner
        .spawn(uart_line_reader(rx, incoming_line_channel))
        .ok();

    // The process_gps task publishes GPS data to the GPS_DATA_CHANNEL
    // Other tasks can subscribe to the GPS_DATA_CHANNEL to get the latest GPS data
    // Again, we use a pubsub channel so that multiple tasks can subscribe to the same channel
    static GPS_DATA_CHANNEL_CELL: StaticCell<GPS_DATA_CHANNEL_TYPE> = StaticCell::new();
    let gps_data_channel = GPS_DATA_CHANNEL_CELL.init(PubSubChannel::new());

    // The write_gps task subscribes to the OUTGOING_LINE_CHANNEL and writes the lines to the UART
    // Becuase there is only a single subscriber, we use a simple channel
    static OUTGOING_LINE_CHANNEL_CELL: StaticCell<OUTGOING_LINE_CHANNEL_TYPE> = StaticCell::new();
    let outgoing_line_channel = OUTGOING_LINE_CHANNEL_CELL.init(Channel::new());

    static GPS_STATE_CHANNEL_CELL: StaticCell<Channel<NoopRawMutex, GpsState, 64>> =
        StaticCell::new();
    let gps_state_channel = GPS_STATE_CHANNEL_CELL.init(Channel::new());

    spawner
        .spawn(write_gps_task(tx, outgoing_line_channel))
        .ok();
    spawner
        .spawn(process_gps_task(incoming_line_channel, gps_data_channel))
        .ok();
    spawner
        .spawn(gps_state_task(gps_state_channel, outgoing_line_channel))
        .ok();

    // Spawn GPS data logger task
    spawner.spawn(gps_data_logger(gps_data_channel)).ok();

    // Spawn GPS state task
    spawner.spawn(set_periodic_mode(outgoing_line_channel)).ok();
}

#[embassy_executor::task]
async fn set_periodic_mode(outgoing_line_channel: &'static OUTGOING_LINE_CHANNEL_TYPE) {
    // Awake for 5 minutes, then sleep for 1 hour
    let packet = "$PMTK225,2,300000,3600000,300000,3600000*29\r\n";
    send_sentence(packet, outgoing_line_channel).await;
}

/// Read bytes from the UART and store them in a buffer
#[embassy_executor::task]
async fn uart_line_reader(
    mut rx: UartRx<'static, Async>,
    line_channel: &'static INCOMING_LINE_CHANNEL_TYPE,
) {
    let publisher = line_channel.publisher().unwrap();
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
                                info!("Publishing line: {}", line.as_str());
                                publisher.publish_immediate(line);
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

/// The write_gps task subscribes to the OUTGOING_LINE_CHANNEL and writes the lines to the UART
#[embassy_executor::task]
async fn write_gps_task(
    mut tx: UartTx<'static, Async>,
    outgoing_line_channel: &'static OUTGOING_LINE_CHANNEL_TYPE,
) {
    loop {
        let line = outgoing_line_channel.receive().await;
        defmt::info!("Writing line: {:?}", line.as_str());
        tx.write_async(line.as_bytes()).await.unwrap();
    }
}

/// The process_gps task subscribes to the INCOMING_LINE_CHANNEL and publishes GPS data to the GPS_DATA_CHANNEL
#[embassy_executor::task]
pub async fn process_gps_task(
    incoming_line_channel: &'static INCOMING_LINE_CHANNEL_TYPE,
    gps_data_channel: &'static GPS_DATA_CHANNEL_TYPE,
) {
    let mut latest_gps_data = GpsData::default();
    let mut subscriber = incoming_line_channel.subscriber().unwrap();
    let publisher = gps_data_channel.publisher().unwrap();
    loop {
        let line = match subscriber.next_message().await {
            WaitResult::Message(line) => line,
            WaitResult::Lagged(_) => continue,
        };
        let mut nmea_parser = Nmea::default();
        let result = nmea_parser.parse(line.as_str());
        match result {
            Ok(_) => {
                latest_gps_data.update_from_nmea(&nmea_parser);
                publisher.publish(latest_gps_data.clone()).await;
            }
            Err(e) => {
                defmt::info!("Parse Error on line: {}", line.as_str());
            }
        }
    }
}

#[embassy_executor::task]
pub async fn gps_state_task(
    gps_state_channel: &'static GPS_STATE_CHANNEL_TYPE,
    outgoing_line_channel: &'static OUTGOING_LINE_CHANNEL_TYPE,
) {
    loop {
        let state = gps_state_channel.receive().await;
        match state {
            GpsState::Enabled => {
                defmt::info!("Enabling GPS");
                // Send a newline to the GPS to wake it up
                send_sentence("\r\n", outgoing_line_channel).await;

                defmt::info!("GPS enabled");
            }
            GpsState::Disabled => {
                defmt::info!("Disabling GPS");
                // Send a command to the GPS to disable the GPS
                send_sentence(PMTK_STANDBY, outgoing_line_channel).await;
                send_sentence("\r\n", outgoing_line_channel).await;
                defmt::info!("GPS disabled");
            }
        }
    }
}

pub async fn send_sentence(
    sentence: &str,
    outgoing_line_channel: &'static OUTGOING_LINE_CHANNEL_TYPE,
) {
    let mut s: HString<LINE_BUF_CAPACITY> = HString::new();
    let _ = s.push_str(sentence);
    outgoing_line_channel.send(s).await
}

/// GPS data logger task that listens to GPS data and prints it to the screen
#[embassy_executor::task]
async fn gps_data_logger(gps_data_channel: &'static GPS_DATA_CHANNEL_TYPE) {
    let mut subscriber = gps_data_channel.subscriber().unwrap();
    loop {
        let gps_data = subscriber.next_message().await;
        match gps_data {
            WaitResult::Message(data) => {
                defmt::info!("GPS Data: {}", data.pretty_print().as_str());
            }
            WaitResult::Lagged(_) => continue,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum GpsState {
    Enabled,
    Disabled,
}

// Standby Command and boot successful message
const PMTK_STANDBY: &str = "$PMTK161,0*28";
// const PMTK_STANDBY_SUCCESS: &str = "$PMTK001,161,3*36";
const PMTK_AWAKE: &str = "$PMTK010,002*2D";
