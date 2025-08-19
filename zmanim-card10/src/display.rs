use embedded_graphics::Drawable;
use embedded_graphics::prelude::Point;
use embedded_graphics::prelude::*;
use embedded_graphics::text::Text;
use embedded_graphics::text::TextStyle;
use embedded_graphics::text::renderer::TextRenderer;
use embedded_hal_bus::spi::ExclusiveDevice;
use epd_waveshare::epd4in2::Display4in2;
use epd_waveshare::epd4in2::Epd4in2;
use epd_waveshare::prelude::*;
use esp_hal::Async;
use esp_hal::delay::Delay;
use esp_hal::gpio::Input;
use esp_hal::gpio::Level;
use esp_hal::gpio::{InputConfig, Pull};
use esp_hal::gpio::{Output, OutputConfig};
use esp_hal::peripherals::SPI2;
use esp_hal::peripherals::*;
use esp_hal::spi::Mode;
use esp_hal::spi::master::{Config, Spi};
use esp_hal::time::Rate;
use esp_println::println;
use profont::PROFONT_18_POINT;
pub struct Display<'a> {
    epd: Epd4in2<
        ExclusiveDevice<Spi<'a, Async>, Output<'a>, esp_hal::delay::Delay>,
        Input<'a>,
        Output<'a>,
        Output<'a>,
        esp_hal::delay::Delay,
    >,
    display: Display4in2,
    spi_device: ExclusiveDevice<Spi<'a, Async>, Output<'a>, esp_hal::delay::Delay>,
    delay: esp_hal::delay::Delay,
}
impl<'a> Display<'a> {
    pub async fn draw_text<S>(
        &mut self,
        text: &str,
        position: Point,
        character_style: S,
        text_style: TextStyle,
    ) where
        S: TextRenderer<Color = epd_waveshare::color::Color>,
    {
        println!("SCREEN: Registering text: {}", text);
        let _ = Text::with_text_style(text, position, character_style, text_style)
            .draw(&mut self.display);
        println!("SCREEN: Text registered");
    }

    pub async fn clear(&mut self) {
        println!("SCREEN: Clearing");
        self.display
            .clear(epd_waveshare::color::Color::White)
            .unwrap();
        println!("SCREEN: Cleared");
    }

    pub async fn go(&mut self) {
        println!("SCREEN: Going");
        self.epd
            .update_frame(&mut self.spi_device, self.display.buffer(), &mut self.delay)
            .unwrap();
        self.epd
            .display_frame(&mut self.spi_device, &mut self.delay)
            .unwrap();
        println!("SCREEN: Updated");
    }

    // Simple methods for easy text updates
    pub async fn add_text(&mut self, text: &str, x: i32, y: i32) {
        use embedded_graphics::mono_font::MonoTextStyle;
        use profont::PROFONT_9_POINT;

        let style = MonoTextStyle::new(&PROFONT_9_POINT, epd_waveshare::color::Color::Black);
        self.draw_text(text, Point::new(x, y), style, TextStyle::default())
            .await;
    }

    pub async fn add_text_large(&mut self, text: &str, x: i32, y: i32) {
        use embedded_graphics::mono_font::MonoTextStyle;

        let style = MonoTextStyle::new(&PROFONT_18_POINT, epd_waveshare::color::Color::Black);
        self.draw_text(text, Point::new(x, y), style, TextStyle::default())
            .await;
    }

    pub async fn clear_and_update(&mut self) {
        self.clear().await;
        self.go().await;
    }

    pub async fn update_display(&mut self) {
        self.go().await;
    }
}

pub async fn init_display(
    sda_mosi: GPIO18<'static>,
    scl_sck: GPIO19<'static>,
    cs: GPIO22<'static>,
    dc: GPIO23<'static>,
    busy: GPIO21<'static>,
    rst: GPIO2<'static>,
    spi2: SPI2<'static>,
) -> Display<'static> {
    let mut delay = Delay::new();

    let spi_bus = Spi::new(
        spi2,
        Config::default()
            .with_frequency(Rate::from_mhz(10))
            .with_mode(Mode::_0),
    )
    .unwrap()
    .with_sck(scl_sck)
    .with_mosi(sda_mosi)
    .into_async();
    let cs = Output::new(cs, Level::High, OutputConfig::default());
    let busy = Input::new(busy, InputConfig::default());
    let dc = Output::new(dc, Level::High, OutputConfig::default());
    let mut rst = Output::new(rst, Level::High, OutputConfig::default());

    println!("Initializing SPI Device...");
    let mut spi_device =
        ExclusiveDevice::new(spi_bus, cs, delay).expect("SPI device initialize error");

    println!("BUSY pin state: {}", busy.is_low());

    println!("Initializing EPD...");
    let epd = Epd4in2::new(&mut spi_device, busy, dc, rst, &mut delay, None).unwrap();
    let display = Display4in2::default();

    Display {
        epd,
        display,
        spi_device,
        delay: delay,
    }
}
