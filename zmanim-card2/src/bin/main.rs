#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]

use defmt::info;
use embassy_executor::Spawner;
use esp_hal::clock::CpuClock;
use esp_hal::timer::systimer::SystemTimer;
use panic_rtt_target as _;
use zmanim_card2::gps::start_gps;

esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    // Initialize logging
    rtt_target::rtt_init_defmt!();

    // Initialize peripherals
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);

    // Initialize timer
    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);

    // Initialize UART
    start_gps(
        peripherals.GPIO16,
        peripherals.GPIO17,
        peripherals.UART0,
        &spawner,
    );

    info!("Embassy initialized!");
}
