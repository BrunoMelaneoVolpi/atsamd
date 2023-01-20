//! This example showcases the i2c module.

#![no_std]
#![no_main]

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use metro_m4 as bsp;

use bsp::hal;
use bsp::pac;
use bsp::{entry, periph_alias};

use cortex_m::asm;
use pac::{CorePeripherals, Peripherals};


use hal::clock::GenericClockController;
//use hal::dmac::{DmaController, PriorityLevel};
use hal::ehal::blocking::i2c::WriteRead;
use hal::prelude::*;
use hal::sercom::i2c;

use hal::ehal::blocking::delay::DelayMs;
use hal::delay::Delay;

use rtt_target::{/*rprint,*/ rprintln, rtt_init_print};

const LENGTH: usize = 1;
const ADDRESS: u8 = 0x77;

#[entry]
fn main() -> ! {


    /*  Initialise remote print...   */
    rtt_init_print!();
    rprintln!("===== i2c ===========");


    let mut peripherals = Peripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

//    let mclk = peripherals.MCLK;
    //let dmac = peripherals.DMAC;
    let pins = bsp::Pins::new(peripherals.PORT);

    // Take SDA and SCL
    let (sda, scl) = (pins.pb02, pins.scl);


//sda.into_pull_down_disabled();

    let i2c_sercom = periph_alias!(peripherals.i2c_sercom);
    let mclk = &mut peripherals.MCLK;
    let mut i2c =
        bsp::i2c_master(& mut clocks,
                        100.khz(),
                        i2c_sercom,
                        mclk,
                        sda,
                        scl);



    let core = CorePeripherals::take().unwrap();
    let mut delay = Delay::new(core.SYST, &mut clocks);


    let mut buffer = [0x00; 1];
//    panic!("asdfasdfasdfasdfasdfasdfasdfasdf");
    // Test writing then reading from an I2C chip
    const ADDRESS: u8 = 0x60;

    let x = i2c.write_read(ADDRESS, &[0x12], &mut buffer);
    //delay.delay_us(1u16);
    //rprintln!("===== i2c tx Rx ===========");
    match x {
        Result::Ok(t) => {
            rprintln!("Ok({:?})", t)
        }
        Result::Err(e) => {
            rprintln!("Err({:?})", e)
        }
    }
    //x.unwrap();


    let mut x = [0u8;1];
    loop {
        delay.delay_ms(10u16);

        x[0] += 1;

        let _result = i2c.write_read(ADDRESS, &x, &mut buffer);
        rprintln!("===== {:?}", x[0]);
    }

    rprintln!("===== i2c looping ===========");

    loop {
        // Go to sleep
        asm::wfi();
    }
}
