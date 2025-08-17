#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
use defmt::info;
use embassy_executor::Spawner;
use embassy_sync::blocking_mutex::raw::{CriticalSectionRawMutex, NoopRawMutex};
use embassy_sync::mutex::Mutex;
use embassy_sync::pubsub::WaitResult;
use embassy_time::{Duration, Timer};
use embedded_io_async::Read;
use embedded_storage::nor_flash::ReadNorFlash;
use esp_hal::analog::adc::{self, Adc, AdcConfig, AdcPin, Attenuation};
use esp_hal::clock::CpuClock;
use esp_hal::gpio::{Input, InputConfig, Output, OutputConfig};
use esp_hal::peripherals::{ADC1, GPIO1};
use esp_hal::rtc_cntl::{Rtc, RtcClock};
use esp_hal::timer::systimer::SystemTimer;


use esp_hal::uart::{Config, Uart};

use esp_hal::Async;
use esp_storage::FlashStorage;
use static_cell::StaticCell;
use zmanim_card10::gps::{init_gps, GpsDataChannelType, GpsState, GPS_STATE, RTC};
use zmanim_card10::gps_data::GpsData;

use zmanim_card10::string_from_buffer;
use {esp_backtrace as _, esp_println as _};

// This creates a default app-descriptor required by the esp-idf bootloader.
// For more information see: <https://docs.espressif.com/projects/esp-idf/en/stable/esp32/api-reference/system/app_image_format.html#application-description>
esp_bootloader_esp_idf::esp_app_desc!();

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    let timer0 = SystemTimer::new(peripherals.SYSTIMER);
    esp_hal_embassy::init(timer0.alarm0);
    info!("Embassy initialized!");

    let rtc = Rtc::new(peripherals.LPWR);
    {
        *(RTC.lock().await) = Some(rtc);
    }
    info!("RTC initialized!");

    let gps_data_channel = init_gps(
        peripherals.GPIO16,
        peripherals.GPIO17,
        peripherals.UART0,
        &spawner,
        peripherals.GPIO0,
    );
    info!("GPS initialized!");

    // Spawn battery monitoring task
    let mut adc1_config = AdcConfig::<ADC1>::default();

    let pin = adc1_config.enable_pin(peripherals.GPIO1, Attenuation::_11dB);

    static ADC1_CELL: StaticCell<Adc<'static, ADC1, Async>> = StaticCell::new();
    let adc1 = ADC1_CELL.init(Adc::new(peripherals.ADC1, adc1_config).into_async());

    spawner.spawn(battery_monitor(adc1, pin)).ok();
    info!("Battery monitoring initialized!");

    // let gps_data = wait_for_fix(gps_data_channel).await;

    // Configure GPIO2 as LED output for blinking
    let mut led = Output::new(
        peripherals.GPIO2,
        esp_hal::gpio::Level::Low,
        OutputConfig::default(),
    );

    info!("Starting main loop with LED blinking");
    let flash = FlashStorage::new();
    flash.

    let gps_data = wait_for_fix(gps_data_channel).await;

    // Main loop with LED blinking
    loop {
        // Blink LED on
        led.set_high();
        info!("LED ON");
        Timer::after(Duration::from_secs(1)).await;

        // Blink LED off
        led.set_low();
        info!("LED OFF");
        Timer::after(Duration::from_secs(1)).await;
    }
}

#[embassy_executor::task]
async fn battery_monitor(
    adc1: &'static mut Adc<'static, ADC1<'static>, Async>,
    mut pin: AdcPin<GPIO1<'static>, ADC1<'static>>,
) {
    // Configure GPIO1 as analog input for battery voltage reading
    // For now, we'll just log that we're monitoring the battery
    info!("Battery voltage monitoring active on GPIO1");

    loop {
        let voltage = adc1.read_oneshot(&mut pin).await;

        info!("Battery voltage: {}", (voltage as f64 * 2.0) / 1000.0);
        Timer::after(Duration::from_secs(1)).await;
    }
}

async fn wait_for_fix(gps_data_channel: &'static GpsDataChannelType) -> GpsData {
    let mut subscriber = gps_data_channel.subscriber().unwrap();
    loop {
        let data = subscriber.next_message().await;
        match data {
            WaitResult::Message(data) => {
                if data.fix_time.is_some() && data.fix_date.is_some() {
                    info!("GPS fix: {}", data.pretty_print().as_str());
                    info!("Turning GPS off");
                    GPS_STATE.signal(GpsState::Off);

                    return data;
                } else {
                    info!("No GPS fix");
                }
            }
            _ => {}
        }
    }
}
