#![no_std]
#![no_main]
#![deny(
    clippy::mem_forget,
    reason = "mem::forget is generally not safe to do with esp_hal types, especially those \
    holding buffers for the duration of a data transfer."
)]
use chrono::{DateTime, Datelike, NaiveDateTime, Timelike};
use core::{fmt::Write, time};
use ds323x::DateTimeAccess;
use ds323x::{Ds323x, ic::DS3231};
use embassy_executor::Spawner;
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex, pubsub::WaitResult, signal::Signal,
};
use embassy_time::{Duration, Timer, WithTimeout};
use embedded_graphics::{
    Drawable,
    geometry::Point,
    mono_font::MonoTextStyle,
    text::{Alignment, Text, TextStyle},
};
use embedded_storage::nor_flash::{NorFlash, ReadNorFlash};
use esp_hal::gpio::{Level, Output, OutputConfig};
use esp_storage::FlashStorage;
use futures::{FutureExt, join};
use futures_lite::future::race_with_seed;
use profont::{
    PROFONT_9_POINT, PROFONT_10_POINT, PROFONT_12_POINT, PROFONT_14_POINT, PROFONT_18_POINT,
};
use serde::{Deserialize, Serialize};

use esp_hal::clock::CpuClock;
use esp_hal::i2c::lp_i2c::LpI2c;
use esp_hal::peripherals::{ADC1, GPIO1};
use esp_hal::rng::Rng;
use esp_hal::rtc_cntl::Rtc;
use esp_hal::timer::systimer::SystemTimer;
use esp_hal::{Async, i2c::master::I2c};
use esp_hal::{
    analog::adc::{Adc, AdcConfig, AdcPin, Attenuation},
    i2c::master::Config,
};
use esp_println::println;
use static_cell::StaticCell;
use weact_studio_epd::TriColor;
extern crate alloc;
use zmanim_card::storage::{Storage, ZmanimConfig};
use zmanim_card::{display::Display, gps_data::GpsData};
use zmanim_card::{
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

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct Data {
    pub user_id: u32,
}

#[esp_hal_embassy::main]
async fn main(spawner: Spawner) {
    esp_alloc::heap_allocator!(size: 64 * 1024);

    let config = esp_hal::Config::default().with_cpu_clock(CpuClock::max());
    let peripherals = esp_hal::init(config);
    let timer0 = SystemTimer::new(peripherals.SYSTIMER);

    esp_hal_embassy::init(timer0.alarm0);
    println!("Embassy initialized!");

    let gps_data_channel = init_gps(
        peripherals.GPIO17,
        peripherals.UART0,
        &spawner,
        peripherals.GPIO0,
    );
    println!("GPS initialized!");

    // Initialize the DS3231 RTC
    let clock_power = peripherals.GPIO21;
    let mut clock_power = Output::new(clock_power, Level::High, OutputConfig::default());
    let clock_sda = peripherals.GPIO1;
    let clock_scl = peripherals.GPIO2;
    let i2c = I2c::new(peripherals.I2C0, Config::default())
        .unwrap()
        .with_sda(clock_sda)
        .with_scl(clock_scl);
    let mut ds3231 = Ds323x::new_ds3231(i2c);
    let datetime = ds3231.datetime();
    if datetime.as_ref().is_err() {
        println!("Error getting RTC datetime: {:?}", datetime.as_ref().err());
    }
    let datetime = datetime.unwrap();

    println!("RTC datetime: {:?}", datetime);

    // Read the config from storage
    let mut config_storage = Storage::new();
    let config = config_storage.read().unwrap_or(ZmanimConfig::new());
    println!("Config: {:?}", config);

    // Fetch the location from storage and the timestamp from the RTC
    // If the RTC is not set, we will wait for the GPS to get the time
    // If the Location is not set, we will wait for the GPS to get the location
    let timestamp = {
        if datetime.year() == 2000 {
            None
        } else {
            Some(datetime.and_utc().timestamp_millis())
        }
    };
    let is_first_time = timestamp.is_none();
    let latitude = config.latitude;
    let longitude = config.longitude;
    static ZMANIM_ARGS_SIGNAL: Signal<CriticalSectionRawMutex, ZmanimArgs> = Signal::new();
    spawner
        .spawn(get_time_and_location(
            &gps_data_channel,
            &ZMANIM_ARGS_SIGNAL,
            latitude,
            longitude,
            timestamp,
        ))
        .unwrap();
    let zmanim_args = ZMANIM_ARGS_SIGNAL.wait().await;
    println!("Zmanim args: {:?}", zmanim_args);

    // We will only update the time if this is the first time we are setting the time
    // for the 1st time.
    if is_first_time {
        ds3231
            .set_datetime(
                &DateTime::from_timestamp_millis(zmanim_args.timestamp)
                    .unwrap()
                    .naive_utc(),
            )
            .unwrap();
    }

    // Save the location to storage
    config_storage.write(config.with_location(zmanim_args.latitude, zmanim_args.longitude));

    //Disable the GPS
    GPS_STATE.signal(GpsState::Off);
    // Disable the clock power
    clock_power.set_low();

    loop {
        println!("Yay!");
        Timer::after(Duration::from_secs(1)).await;
    }

    // let mut display = init_display(
    //     peripherals.GPIO18,
    //     peripherals.GPIO19,
    //     peripherals.GPIO22,
    //     peripherals.GPIO23,
    //     peripherals.GPIO21,
    //     peripherals.GPIO2,
    //     peripherals.SPI2,
    // )
    // .await;
    // let style = MonoTextStyle::new(&PROFONT_9_POINT, TriColor::Black);
    // display
    //     .draw_text(
    //         "Waiting fosdsdsdr GPS fix",
    //         Point::new(128 / 2, 296 / 2 - 50),
    //         style,
    //         TextStyle::with_alignment(Alignment::Center),
    //     )
    //     .await;
    // display
    //     .draw_text(
    //         "Place me on a fsdsddddddddddddddlat\nsurface with direct\nline of sight\nof the sky",
    //         Point::new(128 / 2, 296 / 2 + 20),
    //         style,
    //         TextStyle::with_alignment(Alignment::Center),
    //     )
    //     .await;
    // display.go().await;

    // // Read the config from storage
    // let mut config_storage = Storage::new();
    // let config = config_storage.read().unwrap_or(ZmanimConfig::new());
    // println!("Config read from storage: {:?}", config);
    // static ZMANIM_ARGS_SIGNAL: Signal<CriticalSectionRawMutex, ZmanimArgs> = Signal::new();

    // spawner
    //     .spawn(get_time_and_location(
    //         &gps_data_channel,
    //         &ZMANIM_ARGS_SIGNAL,
    //         config.latitude,
    //         config.longitude,
    //     ))
    //     .unwrap();
    // // Sometimes we have the data instantly, we will first wait for 5 seconds for the data to be available.
    // // After that we will show a message to the user to place the device on a flat surface and direct line of sight to the sky.
    // // We don't want to show that message if we have the data instantly.
    // let wait_for_args = ZMANIM_ARGS_SIGNAL.wait().map(|i| Some(i));
    // let wait_for_timeout = Timer::after(Duration::from_secs(5))
    //     .into_future()
    //     .map(|_| None);
    // let result = race_with_seed(wait_for_args, wait_for_timeout, 0).await;
    // let zmanim_args = if result.is_some() {
    //     result.unwrap()
    // } else {
    //     let style = MonoTextStyle::new(&PROFONT_9_POINT, TriColor::Black);
    //     display
    //         .draw_text(
    //             "Waiting for GPS fix",
    //             Point::new(128 / 2, 296 / 2 - 50),
    //             style,
    //             TextStyle::with_alignment(Alignment::Center),
    //         )
    //         .await;
    //     display
    //         .draw_text(
    //             "Place me on a flat\nsurface with direct\nline of sight\nof the sky",
    //             Point::new(128 / 2, 296 / 2 + 20),
    //             style,
    //             TextStyle::with_alignment(Alignment::Center),
    //         )
    //         .await;
    //     let (_, zmanim_args) = join!(display.go(), ZMANIM_ARGS_SIGNAL.wait());
    //     zmanim_args
    // };
    // println!("Zmanim args: {:?}", zmanim_args);

    // // Disable the GPS once we have the user's location and time
    // GPS_STATE.signal(GpsState::Off);

    // // Save the location to storage
    // config_storage.write(config.with_location(zmanim_args.latitude, zmanim_args.longitude));

    // // TODO: Get this from a dial/button/etc.
    // let tz_offset = -4.0 * 60.0 * 60.0 * 1000.0;
    // println!("Drawing zmanim");
    // draw_zmanim(
    //     &mut display,
    //     zmanim_args.timestamp,
    //     zmanim_args.latitude,
    //     zmanim_args.longitude,
    //     tz_offset,
    // )
    // .await;

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

#[derive(Debug)]
struct ZmanimArgs {
    latitude: f64,
    longitude: f64,
    timestamp: i64,
}

#[embassy_executor::task]
async fn get_time_and_location(
    gps_channel: &'static GpsDataChannelType,
    zmanim_args_signal: &'static Signal<CriticalSectionRawMutex, ZmanimArgs>,
    current_latitude: Option<f64>,
    current_longitude: Option<f64>,
    current_timestamp: Option<i64>,
) {
    let mut subscriber = gps_channel.subscriber().unwrap();
    let mut current_timestamp = current_timestamp;
    let mut current_latitude = current_latitude;
    let mut current_longitude = current_longitude;
    loop {
        if current_timestamp.is_some() && current_latitude.is_some() && current_longitude.is_some()
        {
            zmanim_args_signal.signal(ZmanimArgs {
                latitude: current_latitude.unwrap(),
                longitude: current_longitude.unwrap(),
                timestamp: current_timestamp.unwrap(),
            });
            break;
        }
        let data = subscriber.next_message().await;
        match data {
            WaitResult::Message(data) => {
                current_timestamp = data.timestamp;
                current_latitude = data.latitude;
                current_longitude = data.longitude;
            }
            WaitResult::Lagged(_) => {}
        }
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

fn format_time(timestamp: Option<f64>, name: &str, tz_offset: f64) -> alloc::string::String {
    let mut line: alloc::string::String = alloc::string::String::new();
    line.push_str(name);
    line.push_str(": ");

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
        line.push_str("N/A");
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
    let calendar = ZmanimCalendar::new(timestamp, &location, true, true, 18.0);
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
    display.clear().await;
    display
        .draw_text(
            alos.as_str(),
            Point::new(0, 10),
            style,
            TextStyle::default(),
        )
        .await;
    display
        .draw_text(
            alos72.as_str(),
            Point::new(0, 20),
            style,
            TextStyle::default(),
        )
        .await;
    display
        .draw_text(
            sunrise.as_str(),
            Point::new(0, 30),
            style,
            TextStyle::default(),
        )
        .await;
    display
        .draw_text(
            sof_zman_shma_gra.as_str(),
            Point::new(0, 40),
            style,
            TextStyle::default(),
        )
        .await;
    display
        .draw_text(
            sof_zman_shma_mga.as_str(),
            Point::new(0, 50),
            style,
            TextStyle::default(),
        )
        .await;
    display
        .draw_text(
            sof_zman_tfila_gra.as_str(),
            Point::new(0, 60),
            style,
            TextStyle::default(),
        )
        .await;
    display
        .draw_text(
            sof_zman_tfila_mga.as_str(),
            Point::new(0, 70),
            style,
            TextStyle::default(),
        )
        .await;
    display
        .draw_text(
            mincha_gedola.as_str(),
            Point::new(0, 80),
            style,
            TextStyle::default(),
        )
        .await;
    display
        .draw_text(
            mincha_gedola_default.as_str(),
            Point::new(0, 90),
            style,
            TextStyle::default(),
        )
        .await;
    display
        .draw_text(
            mincha_ketana.as_str(),
            Point::new(0, 100),
            style,
            TextStyle::default(),
        )
        .await;
    display
        .draw_text(
            plag_hamincha.as_str(),
            Point::new(0, 110),
            style,
            TextStyle::default(),
        )
        .await;
    display
        .draw_text(
            sunset.as_str(),
            Point::new(0, 120),
            style,
            TextStyle::default(),
        )
        .await;
    display
        .draw_text(
            tzais.as_str(),
            Point::new(0, 130),
            style,
            TextStyle::default(),
        )
        .await;
    display
        .draw_text(
            tzais72.as_str(),
            Point::new(0, 140),
            style,
            TextStyle::default(),
        )
        .await;
    display.go().await;
}
