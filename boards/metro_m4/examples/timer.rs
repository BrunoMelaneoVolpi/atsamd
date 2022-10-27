#![no_std]
#![no_main]

use metro_m4 as bsp;

use bsp::hal;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::entry;
use hal::clock::GenericClockController;
use hal::pac::Peripherals;
use hal::prelude::*;
use hal::timer::TimerCounter;

use rtt_target::{rprintln, rtt_init_print};

use nb::block;

#[entry]
fn main() -> ! {
    let mut peripherals =
        Peripherals::take().unwrap();

    let mut clocks =
        GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

    let pins =
        bsp::Pins::new(peripherals.PORT);

    let gclk0 =
        clocks.gclk0();

    let tc2_3 =
        clocks.tc2_tc3(&gclk0).unwrap();

    let mut timer =
        TimerCounter::tc3_( &tc2_3,
                             peripherals.TC3,
                            &mut peripherals.MCLK);

    // start a 5Hz timer
    timer.start(2.hz());
    let mut red_led = pins.d13.into_push_pull_output();



    rtt_init_print!();
    rprintln!("================");



    // toggle the led at the frequency set by the timer
    loop {
        rprintln!("Hello, world!");
        red_led.set_high().unwrap();
        block!(timer.wait()).ok();
        red_led.set_low().unwrap();
        block!(timer.wait()).ok();
    }
}
