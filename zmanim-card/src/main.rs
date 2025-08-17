#![no_std]
#![no_main]
mod gps;
use adafruit_kb2040 as _;
use adafruit_kb2040::hal::uart::UartPeripheral;
use chrono::TimeZone;
use chrono::Utc;
use embassy_sync::blocking_mutex::raw::CriticalSectionRawMutex;
use embassy_sync::blocking_mutex::raw::ThreadModeRawMutex;
use embassy_sync::mutex;
use embassy_sync::pubsub::WaitResult;
use embassy_sync::signal::Signal;
use zmanim_core::zmanim_calendar::ZmanimCalendarTrait;

use crate::gps::GPS_DATA_CHANNEL;
use crate::gps::GPS_STATE_CHANNEL;
use crate::gps::GpsState;
use crate::gps::gps_state_task;
use crate::gps::listen_gps_task;
use crate::gps::process_gps_task;
use crate::gps::write_gps_task;

use chrono::{Datelike, Timelike, Weekday};
use core::fmt::Write;
use core::str::FromStr;
use embassy_executor::Spawner;
use embassy_time::Timer;
use gpio::{Level, Output};
use heapless::String as HString;
use static_cell::StaticCell;
use zmanim_core::utils::GeoLocation;
use zmanim_core::utils::NOAACalculator;
use zmanim_core::zmanim_calendar::ZmanimCalendar;
use {defmt_rtt as _, panic_probe as _};

bind_interrupts!(struct Irqs {
    USBCTRL_IRQ => InterruptHandler<USB>;
    UART0_IRQ => BufferedInterruptHandler<UART0>;
});

#[embassy_executor::task]
async fn logger_task(driver: Driver<'static, USB>) {
    embassy_usb_logger::run!(1024, log::LevelFilter::Info, driver);
}

async fn start_gps(
    spawner: Spawner,
    tx_pin: Peri<'static, PIN_0>,
    rx_pin: Peri<'static, PIN_1>,
    uart: Peri<'static, UART0>,
) {
    // Create buffers for reading and writing to the UART
    static TX_BUF: StaticCell<[u8; 64]> = StaticCell::new();
    let tx_buf = &mut TX_BUF.init([0; 64])[..];
    static RX_BUF: StaticCell<[u8; 512]> = StaticCell::new();
    let rx_buf = &mut RX_BUF.init([0; 512])[..];
    // Create the UART peripheral
    let mut config = Config::default();
    config.baudrate = 9_600;
    let uart = BufferedUart::new(uart, tx_pin, rx_pin, Irqs, tx_buf, rx_buf, config);
    let (tx, rx) = uart.split();

    // Create the GPS object
    let tz = chrono_tz::Tz::US__Eastern;
    let lat = 40.0828;
    let lng = -74.2094;
    let elevation = 0.0;

    spawner.spawn(listen_gps_task(rx)).unwrap();
    spawner.spawn(process_gps_task()).unwrap();
    spawner.spawn(write_gps_task(tx)).unwrap();
    spawner.spawn(gps_state_task()).unwrap();

    spawner.spawn(log_zmanim(tz, lat, lng, elevation)).unwrap();
}

#[embassy_executor::main]
async fn main(spawner: Spawner) -> ! {
    log::info!("Program start");

    let mut led = Output::new(p.PIN_2, Level::Low);
    let driver = Driver::new(p.USB, Irqs);

    spawner.spawn(logger_task(driver)).unwrap();
    Timer::after_millis(1000).await;

    let (tx_pin, rx_pin, uart) = (p.PIN_0, p.PIN_1, p.UART0);
    start_gps(spawner, tx_pin, rx_pin, uart).await;
    loop {
        led.set_high();
        log::info!("led on!");
        Timer::after_millis(100).await;

        led.set_low();
        log::info!("led off!");
        Timer::after_millis(2000).await;
    }
}

fn localize_timestamp<Tz: TimeZone>(timestamp: f64, tz: Tz) -> Option<chrono::DateTime<Tz>> {
    let dt_utc = Utc.timestamp_millis_opt(timestamp as i64).single();
    let local = dt_utc.map(|dt| dt.with_timezone(&tz));
    local
}

fn log_localized_timestamp(label: HString<32>, tz: chrono_tz::Tz, timestamp: Option<f64>) {
    let local = timestamp.map(|t| localize_timestamp(t, tz)).flatten();
    if let Some(local) = local {
        // Weekday full name
        let weekday_str = match local.weekday() {
            Weekday::Mon => "Monday",
            Weekday::Tue => "Tuesday",
            Weekday::Wed => "Wednesday",
            Weekday::Thu => "Thursday",
            Weekday::Fri => "Friday",
            Weekday::Sat => "Saturday",
            Weekday::Sun => "Sunday",
        };

        // 12-hour time with AM/PM
        let hour24 = local.hour();
        let minute = local.minute();
        let (ampm, hour12) = if hour24 == 0 {
            ("AM", 12)
        } else if hour24 < 12 {
            ("AM", hour24)
        } else if hour24 == 12 {
            ("PM", 12)
        } else {
            ("PM", hour24 - 12)
        };

        let mut s: HString<32> = HString::new();
        let _ = write!(s, "{}, {}:{:02}{}", weekday_str, hour12, minute, ampm);
        log::info!("{}: {}", label, s);
    } else {
        log::info!("{}: None", label);
    }
}

#[embassy_executor::task]
async fn log_zmanim(tz: chrono_tz::Tz, lat: f64, lng: f64, elevation: f64) {
    log::info!(
        "Zmanim logging started (tz={}, lat={}, lng={}, elevation={})",
        tz,
        lat,
        lng,
        elevation
    );

    let mut cycles: u32 = 0;
    let mut subscriber = GPS_DATA_CHANNEL.subscriber().unwrap();
    loop {
        let gps_data = match subscriber.next_message().await {
            WaitResult::Message(gps_data) => gps_data,
            WaitResult::Lagged(_) => continue,
        };
        log::info!("GPS data: {:?}", gps_data);
        if let (Some(fix_date), Some(fix_time)) = (gps_data.fix_date, gps_data.fix_time) {
            let dt = chrono::NaiveDateTime::new(fix_date, fix_time);
            let timestamp = dt.and_utc().timestamp_millis();
            log_localized_timestamp(
                HString::from_str("Current time").unwrap(),
                tz,
                Some(timestamp as f64),
            );
            let calc = NOAACalculator::new();
            let geo_location = GeoLocation::new(lat, lng, elevation);
            if let Ok(geo_location) = geo_location {
                let zmanim =
                    ZmanimCalendar::new(timestamp, &geo_location, &calc, false, false, 18.0);

                log_localized_timestamp(
                    HString::from_str("Alos72").unwrap(),
                    tz,
                    zmanim.get_alos72(),
                );
                log_localized_timestamp(
                    HString::from_str("AlosHashachar").unwrap(),
                    tz,
                    zmanim.get_alos_hashachar(),
                );

                log_localized_timestamp(
                    HString::from_str("Chatzos").unwrap(),
                    tz,
                    zmanim.get_chatzos(),
                );
                log_localized_timestamp(
                    HString::from_str("ChatzosAsHalfDay").unwrap(),
                    tz,
                    zmanim.get_chatzos_as_half_day(),
                );
                log_localized_timestamp(
                    HString::from_str("Tzais").unwrap(),
                    tz,
                    zmanim.get_tzais(),
                );
                log_localized_timestamp(
                    HString::from_str("Tzais72").unwrap(),
                    tz,
                    zmanim.get_tzais72(),
                );
                log_localized_timestamp(
                    HString::from_str("CandleLighting").unwrap(),
                    tz,
                    zmanim.get_candle_lighting(),
                );
                log::info!("Raw timestamp: {}", timestamp);
                log::info!(
                    "UTC timestamp: {:?}",
                    Utc.timestamp_millis_opt(timestamp as i64).single()
                );
                log::info!(
                    "Local timestamp: {:?}",
                    localize_timestamp(timestamp as f64, tz)
                );
                Timer::after_millis(1000).await;
                GPS_STATE_CHANNEL.send(GpsState::Disabled).await;
                Timer::after_millis(30_000).await;
                GPS_STATE_CHANNEL.send(GpsState::Enabled).await;
            } else {
                log::error!(
                    "Invalid geolocation: lat: {}, lng: {}, elevation: {}",
                    lat,
                    lng,
                    elevation
                );
            }
        } else {
            // No fix yet; report occasionally to avoid noisy logs
            if cycles % 10 == 0 {
                log::info!("Waiting for GPS fix...");
            }
        }

        cycles = cycles.wrapping_add(1);
        Timer::after_millis(1000).await;
    }
}
