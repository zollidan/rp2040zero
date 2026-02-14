#![no_std]
#![no_main]

use defmt_rtt as _;
use panic_probe as _;
use rp2040_hal as hal;
use rp2040_hal::entry;
use rp2040_hal::clocks::init_clocks_and_plls;
use rp2040_hal::watchdog::Watchdog;
use rp2040_hal::Clock;
use rp2040_hal::pio::PIOExt;
use rp2040_hal::Timer;
use smart_leds::{SmartLedsWrite, RGB8};
use ws2812_pio::Ws2812;

#[unsafe(link_section = ".boot2")]
#[used]
pub static BOOT2: [u8; 256] = rp2040_boot2::BOOT_LOADER_W25Q080;

const XTAL_FREQ_HZ: u32 = 12_000_000;

#[entry]
fn main() -> ! {
    let mut pac = hal::pac::Peripherals::take().unwrap();
    let mut watchdog = Watchdog::new(pac.WATCHDOG);

    let clocks = init_clocks_and_plls(
        XTAL_FREQ_HZ,
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    )
    .ok()
    .unwrap();

    let sio = hal::Sio::new(pac.SIO);
    let pins = hal::gpio::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );

    let timer = Timer::new(pac.TIMER, &mut pac.RESETS, &clocks);

    let (mut pio, sm0, _, _, _) = pac.PIO0.split(&mut pac.RESETS);
    let mut ws = Ws2812::new(
        pins.gpio16.into_function(),
        &mut pio,
        sm0,
        clocks.peripheral_clock.freq(),
        timer.count_down(),
    );

    let core = hal::pac::CorePeripherals::take().unwrap();
    let mut delay = cortex_m::delay::Delay::new(core.SYST, clocks.system_clock.freq().to_Hz());


    let green: [RGB8; 1] = [RGB8 { r: 0, g: 255, b: 0 }];
    let red: [RGB8; 1] = [RGB8 { r: 255, g: 0, b: 0 }];
    let blue: [RGB8; 1] = [RGB8 { r: 0, g: 0, b: 255 }];
    
    loop {
        ws.write(green.iter().copied()).unwrap();
        delay.delay_ms(500);

        ws.write(red.iter().copied()).unwrap();
        delay.delay_ms(500);
        
        ws.write(blue.iter().copied()).unwrap();
        delay.delay_ms(500);
    }
}