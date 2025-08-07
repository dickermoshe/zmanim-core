use feather_m4::hal::clock::GenericClockController;
use feather_m4::hal::prelude::*;
use feather_m4::hal::sercom::uart::{self, RxDuplex, TxDuplex, Uart};
use feather_m4::pac::Mclk;
use feather_m4::{self as bsp, UartPads, UartRx, UartSercom, UartTx};
use heapless::String;

use crate::ard::gps_example::GpsApplication;

/// Set up GPS UART communication
pub fn setup_gps(
    clocks: &mut GenericClockController,
    rx: UartRx,
    tx: UartTx,
    serial: UartSercom,
    mclk: &mut Mclk,
) {
    // Set up UART for GPS communication at 9600 baud
    let uart = bsp::uart(clocks, 9600.Hz(), serial, mclk, rx, tx);
    let (uart_rx, uart_tx): (
        Uart<uart::Config<UartPads>, RxDuplex>,
        Uart<uart::Config<UartPads>, TxDuplex>,
    ) = uart.split();
}

struct GPS {
    uart_rx: Uart<uart::Config<UartPads>, RxDuplex>,
    uart_tx: Uart<uart::Config<UartPads>, TxDuplex>,
    recvdflag: bool,
    paused: bool,
    data: GpsData,
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
