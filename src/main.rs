// --- system info ---
// waveshare rp2040zero
// Dual-core ARM Cortex M0+ processor, flexible clock running up to 133 MHz
// Built-in 264KB SRAM and 2MB Flash
// LED on GP16
// --- system info ---
#![no_std]
#![no_main]

use embedded_hal::digital::OutputPin;
use panic_probe as _;
use rp2040_hal as hal;
use rp2040_hal::entry;

#[link_section = ".boot2"]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

#[entry]
fn main() -> ! {
    let mut pac = hal::pac::Peripherals::take().unwrap();
    let sio = hal::Sio::new(pac.SIO);

    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let mut led = pins.gpio16.into_push_pull_output();

    loop {
        led.set_high().unwrap();
        cortex_m::asm::delay(5_000_000);
        led.set_low().unwrap();
        cortex_m::asm::delay(5_000_000);
    }
}
