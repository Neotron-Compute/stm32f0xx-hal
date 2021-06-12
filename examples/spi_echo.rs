#![no_main]
#![no_std]

use panic_halt as _;

use stm32f0xx_hal as hal;

use crate::hal::{
    pac,
    prelude::*,
    spi::{Mode, Phase, Polarity, SpiP},
};

use cortex_m_rt::entry;

/// An SPI echo example
///
/// Any bytes you send to this SPI Peripheral, you will get back one byte later.
#[entry]
fn main() -> ! {
    const MODE: Mode = Mode {
        polarity: Polarity::IdleHigh,
        phase: Phase::CaptureOnSecondTransition,
    };

    if let Some(p) = pac::Peripherals::take() {
        let mut flash = p.FLASH;
        let mut rcc = p.RCC.configure().freeze(&mut flash);

        let gpioa = p.GPIOA.split(&mut rcc);

        let (sck, miso, mosi, nss) = cortex_m::interrupt::free(move |cs| {
            (
                // SPI pins
                gpioa.pa5.into_alternate_af0(cs),
                gpioa.pa6.into_alternate_af0(cs),
                gpioa.pa7.into_alternate_af0(cs),
                // Chip Select input (active low)
                gpioa.pa9.into_pull_up_input(cs),
            )
        });

        // Configure SPI in Peripheral Mode
        let mut spi = SpiP::spi1(p.SPI1, (sck, miso, mosi), MODE, &mut rcc);

        // TODO: This could use some timeouts
        let mut data = 0xFF;
        loop {
            while nss.is_high().unwrap() {
                // Wait for Chip Select to go low
                cortex_m::asm::nop();
            }

            while nss.is_low().unwrap() {
                // Load a byte into the output
                // spi.send_u8(data).unwrap();
                // Wait until it has been exchanged with another one
                // Get the new byte
                // data = spi.read_u8().unwrap();
            }
        }
    }

    loop {
        continue;
    }
}
