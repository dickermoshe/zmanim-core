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
use embedded_hal_bus::spi::ExclusiveDevice;
use embedded_storage::nor_flash::{NorFlash, ReadNorFlash};
use embedded_storage::nor_flash::{NorFlashError, NorFlashErrorKind};
use esp_hal::peripherals::SPI2;
use profont::{PROFONT_18_POINT, PROFONT_24_POINT};

use display_interface_spi::SPIInterface;
use esp_bootloader_esp_idf::partitions;
use esp_hal::peripherals::*;
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
    Color, DisplayDriver, TriColor, WeActStudio290BlackWhiteDriver, WeActStudio290TriColorDriver,
};
pub type Driver = DisplayDriver<
    SPIInterface<
        ExclusiveDevice<Spi<'static, Async>, Output<'static>, embassy_time::Delay>,
        Output<'static>,
    >,
    Input<'static>,
    Output<'static>,
    embassy_time::Delay,
    128,
    128,
    296,
    TriColor,
>;
pub type Display = Display290TriColor;

use zmanim_core::zmanim_calendar::{ZmanimCalendar, ZmanimCalendarTrait};
use zmanim_core::{geolocation::GeoLocation, NOAACalculator};
pub async fn init_display(
    sda_mosi: GPIO1<'static>,
    scl_sck: GPIO1<'static>,
    cs: GPIO1<'static>,
    dc: GPIO1<'static>,
    busy: GPIO1<'static>,
    rst: GPIO1<'static>,
    spi2: SPI2<'static>,
) -> (Driver, Display) {
    let mut spi_bus = Spi::new(
        spi2,
        Config::default()
            .with_frequency(Rate::from_khz(100))
            .with_mode(Mode::_0),
    )
    .unwrap()
    .with_sck(scl_sck)
    .with_mosi(sda_mosi)
    .into_async();
    let cs = Output::new(cs, Level::High, OutputConfig::default());
    let busy = Input::new(busy, InputConfig::default().with_pull(Pull::Up));
    let dc = Output::new(dc, Level::Low, OutputConfig::default());
    let rst = Output::new(rst, Level::High, OutputConfig::default());

    println!("Intializing SPI Device...");
    let spi_device = ExclusiveDevice::new(spi_bus, cs, embassy_time::Delay)
        .expect("SPI device initialize error");
    let spi_interface = SPIInterface::new(spi_device, dc);
    let mut driver =
        WeActStudio290TriColorDriver::new(spi_interface, busy, rst, embassy_time::Delay);
    let mut display = Display290TriColor::new();
    display.set_rotation(weact_studio_epd::graphics::DisplayRotation::Rotate90);
    driver.init().await.unwrap();
    return (driver, display);
}
