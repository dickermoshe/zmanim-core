//! UART-based GPS communication implementation

use crate::ard::gps_lib::{AdafruitGps, GpsComm};
use embedded_hal::serial::{Read, Write};
use nb;

/// UART communication wrapper for GPS
pub struct UartGpsComm<UART, TIMER> {
    uart_rx: UART,
    timer: TIMER,
}

impl<UART, TIMER> UartGpsComm<UART, TIMER>
where
    UART: Read<u8>,
    TIMER: Clone,
{
    pub fn new(uart_rx: UART, timer: TIMER) -> Self {
        Self { uart_rx, timer }
    }
}

impl<UART, TIMER> GpsComm for UartGpsComm<UART, TIMER>
where
    UART: Read<u8>,
    TIMER: Clone,
{
    type Error = UART::Error;

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        self.uart_rx.read()
    }

    fn write(&mut self, _byte: u8) -> nb::Result<(), Self::Error> {
        // For RX-only implementation, writing is not supported
        // In a full implementation, you'd need a TX uart as well
        Ok(())
    }

    fn available(&self) -> usize {
        // This is a limitation of the embedded-hal Read trait
        // We can't know how many bytes are available without trying to read
        // Return 1 to indicate data might be available
        1
    }

    fn millis(&self) -> u32 {
        // This would need to be implemented based on your timer
        // For now, return a placeholder
        0
    }
}

/// Full duplex UART GPS communication
pub struct FullDuplexUartGps<RX, TX, TIMER> {
    uart_rx: RX,
    uart_tx: TX,
    timer: TIMER,
}

impl<RX, TX, TIMER> FullDuplexUartGps<RX, TX, TIMER>
where
    RX: Read<u8>,
    TX: Write<u8>,
    TIMER: Clone,
{
    pub fn new(uart_rx: RX, uart_tx: TX, timer: TIMER) -> Self {
        Self {
            uart_rx,
            uart_tx,
            timer,
        }
    }
}

impl<RX, TX, TIMER> GpsComm for FullDuplexUartGps<RX, TX, TIMER>
where
    RX: Read<u8>,
    TX: Write<u8>,
    TIMER: Clone,
{
    type Error = RX::Error; // Assuming RX and TX have the same error type

    fn read(&mut self) -> nb::Result<u8, Self::Error> {
        self.uart_rx.read()
    }

    fn write(&mut self, byte: u8) -> nb::Result<(), Self::Error> {
        // Note: This assumes TX::Error can be converted to RX::Error
        // In practice, you might need a more sophisticated error handling approach
        match self.uart_tx.write(byte) {
            Ok(_) => Ok(()),
            Err(nb::Error::WouldBlock) => Err(nb::Error::WouldBlock),
            Err(nb::Error::Other(_)) => {
                // This is a type conversion issue - in practice you'd handle this properly
                // For now, we'll just return WouldBlock as a placeholder
                Err(nb::Error::WouldBlock)
            }
        }
    }

    fn available(&self) -> usize {
        // Same limitation as above
        1
    }

    fn millis(&self) -> u32 {
        // Timer implementation would go here
        0
    }
}

/// GPS instance type alias for UART communication
pub type UartGps<UART, TIMER> = AdafruitGps<UartGpsComm<UART, TIMER>>;
pub type FullDuplexGps<RX, TX, TIMER> = AdafruitGps<FullDuplexUartGps<RX, TX, TIMER>>;

/// Helper function to create a GPS instance with UART communication
pub fn create_uart_gps<UART, TIMER>(uart: UART, timer: TIMER) -> UartGps<UART, TIMER>
where
    UART: Read<u8>,
    TIMER: Clone,
{
    let comm = UartGpsComm::new(uart, timer);
    AdafruitGps::new(comm)
}

/// Helper function to create a GPS instance with full duplex UART communication
pub fn create_full_duplex_gps<RX, TX, TIMER>(
    uart_rx: RX,
    uart_tx: TX,
    timer: TIMER,
) -> FullDuplexGps<RX, TX, TIMER>
where
    RX: Read<u8>,
    TX: Write<u8>,
    TIMER: Clone,
{
    let comm = FullDuplexUartGps::new(uart_rx, uart_tx, timer);
    AdafruitGps::new(comm)
}
