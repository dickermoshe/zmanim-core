#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
use chrono::{DateTime, Timelike};
use core::{fmt::Write, time};
use embassy_executor::Spawner;
use embassy_sync::pubsub::WaitResult;
use embassy_time::{Duration, Timer};
use embedded_graphics::{
    Drawable,
    geometry::Point,
    mono_font::MonoTextStyle,
    text::{Text, TextStyle},
};
use profont::{PROFONT_9_POINT, PROFONT_10_POINT, PROFONT_18_POINT};

use esp_hal::Async;
use esp_hal::analog::adc::{Adc, AdcConfig, AdcPin, Attenuation};
use esp_hal::clock::CpuClock;
use esp_hal::peripherals::{ADC1, GPIO1};
use esp_hal::rtc_cntl::Rtc;
use esp_hal::timer::systimer::SystemTimer;
use esp_println::println;
use static_cell::StaticCell;
use weact_studio_epd::TriColor;
use zmanim_card10::storage::{ConfigStorage, ZmanimConfig};
use zmanim_card10::{display::Display, gps_data::GpsData};
use zmanim_card10::{
    display::init_display,
    gps::{GPS_STATE, GpsDataChannelType, GpsState, init_gps},
};
use zmanim_core::{NOAACalculator, geolocation::GeoLocation};
use zmanim_core::{
    astronomical_calendar::AstronomicalCalendarTrait,
    zmanim_calendar::{ZmanimCalendar, ZmanimCalendarTrait},
};

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

    // let timestamp = 1755548752000;
    // let latitude = 40.085521663953486;
    // let longitude = -74.20940519301038;
    let latitude: f64;
    let longitude: f64;
    let timestamp: i64;

    if let Some(config) = config_option.as_ref()
        && config.has_location()
    {
        timestamp = wait_for_gps(gps_data_channel, WaitFor::TimeFix)
            .await
            .timestamp
            .unwrap();
        latitude = config.latitude.unwrap();
        longitude = config.longitude.unwrap();
    } else {
        let gps_data = wait_for_gps(gps_data_channel, WaitFor::LocationAndTimeFix).await;
        timestamp = gps_data.timestamp.unwrap();
        latitude = gps_data.latitude.unwrap();
        longitude = gps_data.longitude.unwrap();
        config_storage.write_config(&ZmanimConfig::new().with_location(latitude, longitude));
    };

    // Disable the GPS once we have the user's location and time
    GPS_STATE.signal(GpsState::Off);

    let mut display = init_display(
        peripherals.GPIO18,
        peripherals.GPIO19,
        peripherals.GPIO22,
        peripherals.GPIO23,
        peripherals.GPIO21,
        peripherals.GPIO2,
        peripherals.SPI2,
    )
    .await;

    println!("Timestamp: {}", timestamp);

    let tz_offset = -4.0 * 60.0 * 60.0 * 1000.0;
    println!("Drawing zmanim");
    draw_zmanim(&mut display, timestamp, latitude, longitude, tz_offset).await;

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

fn format_time(timestamp: Option<f64>, name: &str, tz_offset: f64) -> heapless::String<64> {
    let mut line: heapless::String<64> = heapless::String::new();
    line.push_str(name).unwrap();
    line.push_str(": ").unwrap();

    let datetime = timestamp
        .map(|timestamp| DateTime::from_timestamp_millis((timestamp + tz_offset) as i64))
        .flatten();

    if let Some(datetime) = datetime {
        let (is_pm, hours) = datetime.hour12();
        let minute = datetime.minute();
        let _ = write!(
            line,
            "{:01}:{:02} {}",
            hours,
            minute,
            if is_pm { "PM" } else { "AM" }
        );
    } else {
        line.push_str("N/A").unwrap();
    }

    line
}

async fn draw_zmanim(
    display: &mut Display,
    timestamp: i64,
    latitude: f64,
    longitude: f64,
    tz_offset: f64,
) {
    let location = GeoLocation::new(latitude, longitude, 0.0).unwrap();
    let calendar = ZmanimCalendar::new(
        timestamp,
        &location,
        NOAACalculator::new(),
        true,
        true,
        18.0,
    );
    let alos = format_time(calendar.get_alos_hashachar(), "Alos", tz_offset);

    let alos72 = format_time(calendar.get_alos72(), "Alos72", tz_offset);

    let sunrise = format_time(
        calendar.get_astronomical_calendar().get_sunrise(),
        "Sunrise",
        tz_offset,
    );
    let sof_zman_shma_gra = format_time(calendar.get_sof_zman_shma_gra(), "Shma GRA", tz_offset);
    let sof_zman_shma_mga = format_time(calendar.get_sof_zman_shma_mga(), "Shma MGA", tz_offset);
    let sof_zman_tfila_gra = format_time(calendar.get_sof_zman_tfila_gra(), "Tfila GRA", tz_offset);
    let sof_zman_tfila_mga = format_time(calendar.get_sof_zman_tfila_mga(), "Tfila MGA", tz_offset);
    let mincha_gedola = format_time(calendar.get_chatzos(), "Chatzos", tz_offset);
    let mincha_gedola_default = format_time(
        calendar.get_mincha_gedola_default(),
        "Min Gedola ",
        tz_offset,
    );
    let mincha_ketana = format_time(
        calendar.get_mincha_ketana_default(),
        "Min Ketana",
        tz_offset,
    );
    let plag_hamincha = format_time(calendar.get_plag_hamincha_default(), "Plag", tz_offset);
    let sunset = format_time(
        calendar.get_astronomical_calendar().get_sunset(),
        "Sunset",
        tz_offset,
    );
    let tzais = format_time(calendar.get_tzais(), "Tzais", tz_offset);
    let tzais72 = format_time(calendar.get_tzais72(), "Tzais72", tz_offset);
    println!(
        "{}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, {}, ",
        alos,
        alos72,
        sunrise,
        sof_zman_shma_gra,
        sof_zman_shma_mga,
        sof_zman_tfila_gra,
        sof_zman_tfila_mga,
        mincha_gedola,
        mincha_gedola_default,
        mincha_ketana,
        plag_hamincha,
        sunset,
        tzais,
        tzais72
    );
    let style = MonoTextStyle::new(&PROFONT_9_POINT, TriColor::Black);
    display
        .draw_text(alos.as_str(), Point::new(0, 10), style)
        .await;
    display
        .draw_text(alos72.as_str(), Point::new(0, 20), style)
        .await;
    display
        .draw_text(sunrise.as_str(), Point::new(0, 30), style)
        .await;
    display
        .draw_text(sof_zman_shma_gra.as_str(), Point::new(0, 40), style)
        .await;
    display
        .draw_text(sof_zman_shma_mga.as_str(), Point::new(0, 50), style)
        .await;
    display
        .draw_text(sof_zman_tfila_gra.as_str(), Point::new(0, 60), style)
        .await;
    display
        .draw_text(sof_zman_tfila_mga.as_str(), Point::new(0, 70), style)
        .await;
    display
        .draw_text(mincha_gedola.as_str(), Point::new(0, 80), style)
        .await;
    display
        .draw_text(mincha_gedola_default.as_str(), Point::new(0, 90), style)
        .await;
    display
        .draw_text(mincha_ketana.as_str(), Point::new(0, 100), style)
        .await;
    display
        .draw_text(plag_hamincha.as_str(), Point::new(0, 110), style)
        .await;
    display
        .draw_text(sunset.as_str(), Point::new(0, 120), style)
        .await;
    display
        .draw_text(tzais.as_str(), Point::new(0, 130), style)
        .await;
    display
        .draw_text(tzais72.as_str(), Point::new(0, 140), style)
        .await;
    display.go().await;
}
