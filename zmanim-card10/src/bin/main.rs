#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
use chrono::DateTime;
use core::fmt::Write;
use defmt::{info, Debug2Format};
use embassy_executor::Spawner;
use embassy_sync::pubsub::WaitResult;
use embassy_time::{Duration, Timer};
use embedded_graphics::{
    geometry::Point,
    mono_font::MonoTextStyle,
    text::{Text, TextStyle},
    Drawable,
};
use profont::{PROFONT_18_POINT, PROFONT_24_POINT};

use embedded_hal_bus::spi::ExclusiveDevice;
use embedded_storage::nor_flash::{NorFlash, ReadNorFlash};
use embedded_storage::nor_flash::{NorFlashError, NorFlashErrorKind};

use display_interface_spi::SPIInterface;
use esp_bootloader_esp_idf::partitions;
use esp_hal::peripherals::{ADC1, GPIO1};
use esp_hal::rtc_cntl::Rtc;
use esp_hal::spi::master::{Config, Spi};
use esp_hal::spi::Mode;
use esp_hal::time::Rate;
use esp_hal::timer::systimer::SystemTimer;
use esp_hal::{
    analog::adc::{Adc, AdcConfig, AdcPin, Attenuation},
    gpio::Level,
};
use esp_hal::{clock::CpuClock, gpio::Input};
use esp_hal::{
    delay::Delay,
    gpio::{Output, OutputConfig},
};
use esp_hal::{
    dma::{DmaRxBuf, DmaTxBuf},
    gpio::{InputConfig, Pull},
};
use esp_hal::{dma_buffers, Async};
use esp_println::println;
use esp_storage::FlashStorage;
use heapless::Vec;
use static_cell::StaticCell;
use weact_studio_epd::{
    graphics::{Display290BlackWhite, Display290TriColor},
    Color, TriColor, WeActStudio290BlackWhiteDriver, WeActStudio290TriColorDriver,
};
use zmanim_card10::gps_data::GpsData;
use zmanim_card10::storage::{ConfigStorage, ZmanimConfig};
use zmanim_card10::{
    display::init_display,
    gps::{init_gps, GpsDataChannelType, GpsState, GPS_STATE, RTC},
};
use zmanim_core::zmanim_calendar::{ZmanimCalendar, ZmanimCalendarTrait};
use zmanim_core::{geolocation::GeoLocation, NOAACalculator};

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
    println!("Embassy initialized!");

    let rtc = Rtc::new(peripherals.LPWR);
    {
        *(RTC.lock().await) = Some(rtc);
    }
    println!("RTC initialized!");

    let gps_data_channel = init_gps(
        peripherals.GPIO16,
        peripherals.GPIO17,
        peripherals.UART0,
        &spawner,
        peripherals.GPIO0,
    );
    println!("GPS initialized!");

    // Spawn battery monitoring task
    let mut adc1_config = AdcConfig::<ADC1>::default();

    let pin = adc1_config.enable_pin(peripherals.GPIO1, Attenuation::_11dB);

    static ADC1_CELL: StaticCell<Adc<'static, ADC1, Async>> = StaticCell::new();
    let adc1 = ADC1_CELL.init(Adc::new(peripherals.ADC1, adc1_config).into_async());

    spawner.spawn(battery_monitor(adc1, pin)).ok();
    println!("Battery monitoring initialized!");

    // Read the config from storage
    let config_storage = ConfigStorage::new();
    let config_option = config_storage.read_config();
    println!("Config read from storage: {:?}", config_option);
    let wait_for = if config_option.is_some() && config_option.as_ref().unwrap().has_location() {
        WaitFor::TimeFix
    } else {
        WaitFor::LocationAndTimeFix
    };
    println!("Waiting for GPS data: {:?}", wait_for);
    let gps_data = wait_for_gps(gps_data_channel, wait_for).await;
    println!("Updated GPS data: {:?}", gps_data);
    config_storage.write_config(
        &config_option
            .unwrap_or(ZmanimConfig::new())
            .with_location(gps_data.clone()),
    );

    // Disable the GPS once we have the data
    GPS_STATE.signal(GpsState::Off);

    let sda_mosi = peripherals.GPIO18;
    let scl_sck = peripherals.GPIO19;
    let cs = peripherals.GPIO22;
    let dc = peripherals.GPIO23;
    let busy = peripherals.GPIO21;
    let rst = peripherals.GPIO2;

    let style = MonoTextStyle::new(&PROFONT_18_POINT, TriColor::Black);
    let timestamp = {
        let rtc_guard = RTC.lock().await;
        let rtc = rtc_guard.as_ref().unwrap();
        rtc.current_time_us() as i64 / 1000
    };
    println!("Timestamp: {}", timestamp);
    let location = GeoLocation::new(
        gps_data.latitude.unwrap().clone(),
        gps_data.longitude.unwrap().clone(),
        0.0,
    )
    .unwrap();
    let noaa_calculator = NOAACalculator::new();
    let calendar = ZmanimCalendar::new(timestamp, &location, &noaa_calculator, true, true, 18.0);

    let alos = calendar.get_alos_hashachar().unwrap();
    println!("Alos: {}", alos);
    let datetime = DateTime::from_timestamp_millis(alos as i64 - (1000 * 60 * 60 * 4)).unwrap();
    let mut line: heapless::String<64> = heapless::String::new();
    line.push_str("Alos:").unwrap();
    let _ = write!(line, "{:?}", datetime);
    println!("Line: {}", line);

    let (mut driver, mut display) =
        init_display(sda_mosi, scl_sck, cs, dc, busy, rst, peripherals.SPI2).await;

    let _ = Text::with_text_style(
        line.as_str(),
        Point::new(8, 68),
        style,
        TextStyle::default(),
    )
    .draw(&mut display);

    driver.full_update(&display).await.unwrap();

    // // Configure GPIO2 as LED output for blinking
    // let mut led = Output::new(
    //     peripherals.GPIO2,
    //     esp_hal::gpio::Level::Low,
    //     OutputConfig::default(),
    // );
    println!("Starting main loop with LED blinking");
    loop {
        // Blink LED on
        // led.set_high();
        println!("LED ON");
        Timer::after(Duration::from_secs(1)).await;

        // Blink LED off
        // led.set_low();
        println!("LED OFF");
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
    println!("Battery voltage monitoring active on GPIO1");

    loop {
        let voltage = adc1.read_oneshot(&mut pin).await;

        println!("Battery voltage: {}", (voltage as f64 * 2.0) / 1000.0);
        Timer::after(Duration::from_secs(1)).await;
    }
}

#[derive(Debug)]
enum WaitFor {
    TimeFix,
    LocationAndTimeFix,
}

async fn wait_for_gps(gps_data_channel: &'static GpsDataChannelType, wait_for: WaitFor) -> GpsData {
    let mut subscriber = gps_data_channel.subscriber().unwrap();
    loop {
        let data = subscriber.next_message().await;
        match data {
            WaitResult::Message(data) => match wait_for {
                WaitFor::TimeFix => {
                    if data.timestamp.is_some() {
                        return data;
                    }
                }
                WaitFor::LocationAndTimeFix => {
                    if data.latitude.is_some()
                        && data.longitude.is_some()
                        && data.timestamp.is_some()
                    {
                        return data;
                    }
                }
            },
            _ => {}
        }
    }
}
