#![no_std]
#![no_main]

use metro_m4 as bsp;

use bsp::ehal;
use bsp::hal;
use bsp::pac;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::prelude::*;
use pac::{CorePeripherals, Peripherals};

use ehal::blocking::delay::DelayMs;
use hal::delay::Delay;

use rtt_target::{rprintln, rtt_init_print};
use core::sync::atomic::{AtomicUsize, Ordering};


#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );
    let pins = bsp::Pins::new(peripherals.PORT);
    let mut red_led = pins.d13.into_push_pull_output();


    let mut delay = Delay::new(core.SYST, &mut clocks);

//    let delay_at_low_lvl: u16 = 50u16;
    let delay_at_low_lvl: u16 = 100u16;

    rtt_init_print!();
    rprintln!("================");

    loop {
        static COUNTER: AtomicUsize = AtomicUsize::new(0);
        rprintln!("Hello, world! {}", COUNTER.fetch_add(1, Ordering::Relaxed) );

        delay.delay_ms(1000u16);
        red_led.set_high().unwrap();
        delay.delay_ms(delay_at_low_lvl);
        red_led.set_low().unwrap();
        delay.delay_ms(delay_at_low_lvl);
        red_led.set_high().unwrap();
        delay.delay_ms(delay_at_low_lvl);
        red_led.set_low().unwrap();
    }
}
