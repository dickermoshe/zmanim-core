use defmt::println;
use feather_m4::hal::clock::GenericClockController;
use feather_m4::hal::nb;
use feather_m4::hal::prelude::*;
use feather_m4::hal::sercom::uart::{self, RxDuplex, TxDuplex, Uart};

use feather_m4::pac::Mclk;
use feather_m4::{self as bsp, UartPads, UartRx, UartSercom, UartTx};
use heapless::Vec;
use nmea::Nmea;
use nmea::SentenceType;
pub const MAXLINELENGTH: usize = 120;
struct GPS {
    /// The UART peripheral for receiving data
    rx: Uart<uart::Config<UartPads>, RxDuplex>,
    /// The UART peripheral for sending data
    tx: Uart<uart::Config<UartPads>, TxDuplex>,
    /// The buffer for receiving data
    rx_buffer: [u8; 64],
    /// The buffer for the current line as it is being built
    line_buffer: Vec<u8, MAXLINELENGTH>,
    /// The index of the current character in the line buffer
    line_idx: usize,
    /// The tick count when the first character of the current line was received
    first_char_time: Option<u32>,
    /// How many ticks have passed since the last update
    tick_count: u32,
    /// The NMEA parser
    nmea: Nmea,
    /// The last time a complete line was received
    last_update: Option<u32>,
}

impl GPS {
    pub fn new(
        clocks: &mut GenericClockController,
        uart_rx: UartRx,
        uart_tx: UartTx,
        uart_sercom: UartSercom,
        mclk: &mut Mclk,
    ) -> GPS {
        // Setup UART peripheral and attach DMA channels
        let uart = bsp::uart(clocks, 9600.Hz(), uart_sercom, mclk, uart_rx, uart_tx);
        let (rx, tx) = uart.split();
        GPS {
            rx,
            tx,
            rx_buffer: [0x00; 64],
            line_buffer: Vec::new(),
            line_idx: 0,
            first_char_time: None,
            tick_count: 0,
            nmea: Nmea::default(),
            last_update: None,
        }
    }

    pub fn tick(&mut self) {
        // Update the tick count on each tick
        self.tick_count += 1;

        // Flush the RX buffer. This is important to ensure that the RX buffer is
        // empty before we start receiving data.
        self.rx.flush_rx_buffer();

        // Read the data into the newly emptied buffer
        for c in self.rx_buffer.iter_mut() {
            *c = nb::block!(feather_m4::hal::prelude::_embedded_hal_serial_Read::read(
                &mut self.rx
            ))
            .unwrap();
        }

        // Process the data in the buffer
        let buffer = self.rx_buffer;
        for c in buffer.iter() {
            self.process_char(*c);
        }
    }

    fn _write_byte(&mut self, c: u8) {
        let _ = self.tx.write(c);
    }

    fn process_char(&mut self, c: u8) {
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
    fn process_complete_line(&mut self, current_time: u32) {
        // Convert buffer to string, handling potential UTF-8 issues
        if let Ok(line_str) = core::str::from_utf8(&self.line_buffer) {
            // Parse the NMEA sentence using the nmea crate
            self.nmea.parse(line_str).ok();

            // Record this tick as the last update time
            self.last_update = Some(current_time);

            // Reset for next line
            self.line_buffer.clear();
            self.line_idx = 0;
            self.first_char_time = None;
        }
    }
    fn send_command(&mut self, command: &str) {
        for c in command.as_bytes().iter() {
            self._write_byte(*c);
        }
        // Send newline
        self._write_byte(b'\r');
        self._write_byte(b'\n');
    }
}
