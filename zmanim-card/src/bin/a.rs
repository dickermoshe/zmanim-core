#![no_std]
#![no_main]

use bsp::hal;
use defmt::info;
use embassy_executor::Spawner;
use embassy_time::{Duration, Timer};
use feather_m4::hal::{
    clock::GenericClockController,
    gpio::{Input, PullUp},
    sercom::uart::{self, Uart},
};

use feather_m4::{self as bsp, UartPads, periph_alias, pin_alias};
use hal::fugit::MillisDurationU32;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::{ehal::digital::StatefulOutputPin, pac::Tc4, timer::TimerCounter};
use zmanim_card as _;

hal::bind_interrupts!(struct Irqs {
    TC4 => hal::timer::InterruptHandler<Tc4>;
});

// Declare async tasks
#[embassy_executor::task]
async fn blink() {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.gclk,
        &mut peripherals.mclk,
        &mut peripherals.osc32kctrl,
        &mut peripherals.oscctrl,
        &mut peripherals.nvmctrl,
    );
    let pins = bsp::Pins::new(peripherals.port);
    let mut red_led = pins.d13.into_push_pull_output();
    let timer_clock = clocks.gclk0();
    let tc45 = &clocks.tc4_tc5(&timer_clock).unwrap();
    let timer = TimerCounter::tc4_(tc45, peripherals.tc4, &mut peripherals.mclk);
    let mut timer = timer.into_future(Irqs);

    loop {
        timer
            .delay(MillisDurationU32::from_ticks(500).convert())
            .await;
        // Timekeeping is globally available, no need to mess with hardware timers.
        red_led.set_high();
        timer
            .delay(MillisDurationU32::from_ticks(150).convert())
            .await;
        red_led.set_low();
        timer
            .delay(MillisDurationU32::from_ticks(150).convert())
            .await;
    }
}

// Main is itself an async task as well.
#[embassy_executor::main]
async fn main(spawner: Spawner) {
    let mut peripherals = Peripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.gclk,
        &mut peripherals.mclk,
        &mut peripherals.osc32kctrl,
        &mut peripherals.oscctrl,
        &mut peripherals.nvmctrl,
    );
    let pins = bsp::Pins::new(peripherals.port);

    // Set up button on D12 with pull-up resistor
    let mut button = pins.d12.into_pull_up_input();
    let mut red_led = pins.d13.into_push_pull_output();
    red_led.into_future(Irqs);

    info!("Waiting for button press on D12 to continue...");

    // Wait for button press (button goes LOW when pressed due to pull-up)
    loop {
        if button.is_low().unwrap() {
            info!("Button pressed! Continuing execution...");
            break;
        }
        // Small delay to prevent busy waiting
        Timer::after(Duration::from_millis(10)).await;
    }

    // Spawned tasks run in the background, concurrently.
    spawner.spawn(blink()).unwrap();
}
