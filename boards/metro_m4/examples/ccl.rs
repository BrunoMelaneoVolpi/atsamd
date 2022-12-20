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
use hal::ccl;//::Ccl;
use pac::{  CorePeripherals, Peripherals};


use ehal::blocking::delay::DelayMs;
use hal::delay::Delay;

use rtt_target::{/*rprint,*/ rprintln, rtt_init_print};

use hal::gpio::N;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut gen_clcks =
        GenericClockController::with_external_32kosc(
            peripherals.GCLK,
            &mut peripherals.MCLK,
            &mut peripherals.OSC32KCTRL,
            &mut peripherals.OSCCTRL,
            &mut peripherals.NVMCTRL,
    );

    let pins = bsp::Pins::new(peripherals.PORT);

//    let _lut_0_in_1 = pins.a1.into_alternate::<N>();
//    let _lut_0_out = pins.d11.into_alternate::<N>();

    let _lut_0_in_1 = pins.pb00.into_alternate::<N>();
    let _lut_0_out = pins.pb02.into_alternate::<N>();


    peripherals.MCLK.apbcmask.modify(|_, w| w.ccl_().set_bit());


    /*  Reset just in case...   */
    peripherals.CCL.ctrl.modify(|_, w| w.swrst().set_bit());

    //  CCL CTRL Enable (CTRL.ENABLE)
    peripherals.CCL.ctrl.modify(|_, w| w.enable().clear_bit());

    //  LUT0 Conf
    /*
        LUTCTRL
            TRUTH =
                IN   2 1 0 | Out
                    ------+----
                    x 0 x | 0
                    x 0 x | 0
                    x 1 x | 1
                    x 1 x | 1
                    x 0 x | 0
                    x 0 x | 0
                    x 1 x | 1
                    x 1 x | 1

                    x = masked!     */

    /*  Truth table using CCL IN[1] only    */
    /*  LUTEO = 0   LUT event output is disabled    */
    /*  LUTEi = 0   LUT incoming event is disabled  */
    /*  INSELx
            INSEL0 = MASK
            INSEL1 = IO
            INSEL2 = MASK   */
    peripherals.CCL.lutctrl[0].modify(|_, w| unsafe{w.truth().bits(0b1100_1100)});

    peripherals.CCL.lutctrl[0].modify(|_, w| w
        .luteo().clear_bit()
        .lutei().clear_bit()
        .insel0().mask()
        .insel1().io()
        .insel2().mask());

    /*  ENABLE LUT  */
    peripherals.CCL.lutctrl[0].modify(|_, w| w.enable().set_bit());

    //  Enable CCL module (CTRL.ENABLE)
    peripherals.CCL.ctrl.modify(|_, w| w.enable().set_bit());



    /*  Initialise remote print...   */
    rtt_init_print!();
    rprintln!("================");


    //  Delay to be used in the loop...
    let mut delay = Delay::new(core.SYST, &mut gen_clcks);
    loop {

        delay.delay_ms(1u16);
    }
}




