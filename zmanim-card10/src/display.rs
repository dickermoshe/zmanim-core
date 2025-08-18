use core::fmt::Write;
use display_interface_spi::SPIInterface;
use embedded_graphics::mono_font::MonoTextStyle;
use embedded_graphics::prelude::Point;
use embedded_graphics::text::renderer::TextRenderer;
use embedded_graphics::text::Text;
use embedded_graphics::text::TextStyle;
use embedded_graphics::Drawable;
use embedded_hal_bus::spi::ExclusiveDevice;
use esp_hal::gpio::Input;
use esp_hal::gpio::Level;
use esp_hal::gpio::{InputConfig, Pull};
use esp_hal::gpio::{Output, OutputConfig};
use esp_hal::peripherals::SPI2;
use esp_hal::peripherals::*;
use esp_hal::spi::master::{Config, Spi};
use esp_hal::spi::Mode;
use esp_hal::time::Rate;
use esp_hal::Async;
use esp_println::println;
use profont::PROFONT_18_POINT;
use weact_studio_epd::{
    graphics::Display290TriColor, DisplayDriver, TriColor, WeActStudio290TriColorDriver,
};

pub struct Display {
    driver: DriverType,
    display: DisplayType,
}
impl Display {
    pub async fn draw_text<S>(&mut self, text: &str, position: Point, character_style: S)
    where
        S: TextRenderer<Color = TriColor>,
    {
        println!("SCREEN: Registering text: {}", text);
        let _ = Text::with_text_style(text, position, character_style, TextStyle::default())
            .draw(&mut self.display);
        println!("SCREEN: Text registered");
    }
    pub async fn clear(&mut self) {
        println!("SCREEN: Clearing");
        self.driver.clear_red_buffer().await.unwrap();
        self.driver.clear_bw_buffer().await.unwrap();
        println!("SCREEN: Cleared");
    }
    pub async fn go(&mut self) {
        println!("SCREEN: Going");
        self.driver.full_update(&self.display).await.unwrap();
        println!("SCREEN: Updated");
    }
}

type DriverType = DisplayDriver<
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
type DisplayType = Display290TriColor;

pub async fn init_display(
    sda_mosi: GPIO18<'static>,
    scl_sck: GPIO19<'static>,
    cs: GPIO22<'static>,
    dc: GPIO23<'static>,
    busy: GPIO21<'static>,
    rst: GPIO2<'static>,
    spi2: SPI2<'static>,
) -> Display {
    let spi_bus = Spi::new(
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
    display.set_rotation(weact_studio_epd::graphics::DisplayRotation::Rotate0);
    driver.init().await.unwrap();

    return Display { driver, display };
}
