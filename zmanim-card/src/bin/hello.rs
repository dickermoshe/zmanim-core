#![no_std]
#![no_main]

use bsp::hal;
use feather_m4::hal::sercom::uart::{self, Uart};
use feather_m4::{self as bsp, UartPads, periph_alias, pin_alias};

use bsp::entry;
use hal::clock::GenericClockController;
use hal::pac::{CorePeripherals, Peripherals, interrupt};
use hal::prelude::*;
use hal::usb::UsbBus;

use usb_device::bus::UsbBusAllocator;
use usb_device::prelude::*;
use usbd_serial::{SerialPort, USB_CLASS_CDC};
use zmanim_card as _; // global logger + panicking-behavior + memory layout

use core::ptr::{addr_of, addr_of_mut};
use cortex_m::asm::delay as cycle_delay;
use cortex_m::peripheral::NVIC;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.gclk,
        &mut peripherals.mclk,
        &mut peripherals.osc32kctrl,
        &mut peripherals.oscctrl,
        &mut peripherals.nvmctrl,
    );
    let pins = bsp::Pins::new(peripherals.port);
    let mut red_led: bsp::RedLed = pins.d13.into();

    let bus_allocator = unsafe {
        USB_ALLOCATOR = Some(bsp::usb_allocator(
            pins.usb_dm,
            pins.usb_dp,
            peripherals.usb,
            &mut clocks,
            &mut peripherals.mclk,
        ));
        (*addr_of!(USB_ALLOCATOR)).as_ref().unwrap()
    };

    unsafe {
        USB_SERIAL = Some(SerialPort::new(bus_allocator));
        USB_BUS = Some(
            UsbDeviceBuilder::new(bus_allocator, UsbVidPid(0x16c0, 0x27dd))
                .device_class(USB_CLASS_CDC)
                .build(),
        );
    }

    unsafe {
        core.NVIC.set_priority(interrupt::USB_OTHER, 1);
        core.NVIC.set_priority(interrupt::USB_TRCPT0, 1);
        core.NVIC.set_priority(interrupt::USB_TRCPT1, 1);
        NVIC::unmask(interrupt::USB_OTHER);
        NVIC::unmask(interrupt::USB_TRCPT0);
        NVIC::unmask(interrupt::USB_TRCPT1);
    }
    // Take RX and TX pins
    let uart_rx = pin_alias!(pins.uart_rx);
    let uart_tx = pin_alias!(pins.uart_tx);
    let uart_sercom = periph_alias!(peripherals.uart_sercom);

    // Set up a fast UART for debug/Serial output at 115200 baud
    let serial_uart: Uart<uart::Config<UartPads>, uart::Duplex> = bsp::uart(
        &mut clocks,
        9600.Hz(),
        uart_sercom,
        &mut peripherals.mclk,
        uart_rx,
        uart_tx,
    );

    // Split the UART into RX and TX halves
    let (mut uart_rx, _uart_tx) = serial_uart.split();

    // Send startup message to USB
    send_usb_message(b"UART-to-USB bridge started. Listening for GPS data at 9600 baud...\r\n");

    let mut led_counter = 0u32;
    let mut gps_buffer = [0u8; 256]; // Buffer for GPS data
    let mut buffer_pos = 0;

    loop {
        // Poll USB to keep the connection alive
        poll_usb();

        // Try to read GPS data from UART
        match uart_rx.read() {
            Ok(byte) => {
                // Send individual bytes immediately for real-time streaming
                send_usb_message(&[byte]);
            }
            Err(nb::Error::WouldBlock) => {
                // No data available, continue polling
                cycle_delay(1000); // Small delay to prevent busy waiting
            }
            Err(nb::Error::Other(_)) => {
                // UART error occurred
                // send_usb_message(b"[UART ERROR]\r\n");
                cycle_delay(1000); // Longer delay on error
            }
        }
    }
}

static mut USB_ALLOCATOR: Option<UsbBusAllocator<UsbBus>> = None;
static mut USB_BUS: Option<UsbDevice<UsbBus>> = None;
static mut USB_SERIAL: Option<SerialPort<UsbBus>> = None;

fn send_usb_message(msg: &[u8]) {
    unsafe {
        if let Some(serial) = (*addr_of_mut!(USB_SERIAL)).as_mut() {
            let _ = serial.write(msg);
        }
    }
}

fn poll_usb() {
    unsafe {
        if let Some(usb_dev) = (*addr_of_mut!(USB_BUS)).as_mut() {
            if let Some(serial) = (*addr_of_mut!(USB_SERIAL)).as_mut() {
                usb_dev.poll(&mut [serial]);
                let mut buf = [0u8; 64];

                if let Ok(count) = serial.read(&mut buf) {
                    for (i, c) in buf.iter().enumerate() {
                        if i >= count {
                            break;
                        }
                        serial.write(&[*c]).unwrap();
                    }
                };
            };
        };
    };
}

#[interrupt]
fn USB_OTHER() {
    poll_usb();
}

#[interrupt]
fn USB_TRCPT0() {
    poll_usb();
}

#[interrupt]
fn USB_TRCPT1() {
    poll_usb();
}
