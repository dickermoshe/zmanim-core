use feather_m4::hal::clock::GenericClockController;
use feather_m4::hal::dmac::{Ch0, Ch1, Channel, DmaController, PriorityLevel, Ready};
use feather_m4::hal::embedded_io::{Read, Write};
use feather_m4::hal::prelude::*;
use feather_m4::hal::sercom::uart::{self, RxDuplex, TxDuplex, Uart};
use feather_m4::hal::typelevel::NoneT;
use feather_m4::pac::{Dmac, Mclk, Pm};
use feather_m4::{self as bsp, UartPads, UartRx, UartSercom, UartTx};
struct BufferedUart {
    rx: Uart<uart::Config<UartPads>, RxDuplex, Channel<Ch0, Ready>>,
    tx: Uart<uart::Config<UartPads>, TxDuplex, NoneT, Channel<Ch1, Ready>>,
    rx_buffer: [u8; 64],
    tx_buffer: [u8; 64],
}

impl BufferedUart {
    pub fn new(
        clocks: &mut GenericClockController,
        uart_rx: UartRx,
        uart_tx: UartTx,
        uart_sercom: UartSercom,
        mclk: &mut Mclk,
        dmac: Dmac,
        pm: &mut Pm,
    ) -> BufferedUart {
        // Setup DMA channels for later use
        let mut dmac = DmaController::init(dmac, pm);
        let channels = dmac.split();

        let chan0 = channels.0.init(PriorityLevel::Lvl0);
        let chan1 = channels.1.init(PriorityLevel::Lvl0);

        // Setup UART peripheral and attach DMA channels
        let uart = bsp::uart(clocks, 9600.Hz(), uart_sercom, mclk, uart_rx, uart_tx)
            .with_rx_channel(chan0)
            .with_tx_channel(chan1);
        let (rx, tx) = uart.split();
        BufferedUart {
            rx,
            tx,
            rx_buffer: [0x00; 64],
            tx_buffer: [0x00; 64],
        }
    }

    pub fn tick(&mut self) {}
}
