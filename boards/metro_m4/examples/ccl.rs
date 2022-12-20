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
//use bsp::Pins;
use hal::clock::GenericClockController;
//use hal::prelude::*;
use hal::dac::Dac;
use pac::{  interrupt,
            CorePeripherals,
            Peripherals,
        };


use ehal::blocking::delay::DelayMs;
use hal::delay::Delay;

use rtt_target::{rprint, rprintln, rtt_init_print};
//use core::sync::atomic::{AtomicUsize, Ordering};
//use core::cell::RefCell;
//use cortex_m::peripheral::NVIC;
//use cortex_m::{interrupt::Mutex};
//use crate::pac::gclk::genctrl::SRC_A::DFLL;
//use pac::gclk::pchctrl::GEN_A::GCLK0;   //  Cyclops has a centralized
//                                        //  point where all clock are setup...
//                                        //  Make sure the clock is correct...


//static G_LED: Mutex<RefCell<Option<   hal::gpio::Pin<hal::gpio::PA16, hal::gpio::PushPullOutput>   >>> = Mutex::new(RefCell::new(None));
//use hal::gpio::PushPull;
//static G_LED: Mutex<RefCell<Option<   hal::gpio::Pin<hal::gpio::PA16, hal::gpio::Output<PushPull>>   >>> = Mutex::new(RefCell::new(None));
use hal::gpio::B;
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

    //  ToDo: Make sure DAC Vout pins are properly configured.
    //  todo!();
    //      PA02 :: Dac Vout0: LED_T_CTRL_BF: Bright temperature control
    //      PA05 :: Dac Vout1: LED_T_CTRL_DF: Darkfield temperature control
    //      LED_T_CTRL_BF: DAC_VOUT0: PA02
    //      LED_T_CTRL_DF: DAC_VOUT1: PA11
    let pins = bsp::Pins::new(peripherals.PORT);


    let _lut_0_in_1 = pins.a1.into_alternate::<N>();
//  let _lut_0_in_0 = pins.d13.into_alternate::<N>();

    let _lut_0_out = pins.d11.into_alternate::<N>();

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

    /*
        LUTCTRL
            TRUTH =
                IN   2 1 0 | Out
                    ------+----
                    x x 0 | 0
                    x x 1 | 1
                    x x 0 | 0
                    x x 1 | 1
                    x x 0 | 0
                    x x 1 | 1
                    x x 0 | 0
                    x x 1 | 1

                    x = masked!     */


    //self.ccl.lutctrl[0].modify(|_, w| unsafe{w.truth().bits(0b1111_1111)});
    peripherals.CCL.lutctrl[0].modify(|_, w| unsafe{w.truth().bits(0b1100_1100)});
    //self.ccl.lutctrl[0].modify(|_, w| unsafe{w.truth().bits(0b1100_1100)});


    /*  LUTEO = 0   LUT event output is disabled    */
    peripherals.CCL.lutctrl[0].modify(|_, w| w.luteo().clear_bit());
    /*  LUTEi = 0   LUT incoming event is disabled  */
    peripherals.CCL.lutctrl[0].modify(|_, w| w.lutei().clear_bit());
    /*  INSELx
            INSEL0 = MASK
            INSEL1 = IO
            INSEL2 = MASK   */
            peripherals.CCL.lutctrl[0].modify(|_, w| w.insel0().mask());
    peripherals.CCL.lutctrl[0].modify(|_, w| w.insel1().io());
    peripherals.CCL.lutctrl[0].modify(|_, w| w.insel2().mask());





    //peripherals.CCL.lutctrl[0].modify(|_, w| w.filtsel().filter()  );




    /*  ENABLE = 1  */
    peripherals.CCL.lutctrl[0].modify(|_, w| w.enable().set_bit());



    //  CCL CTRL Enable (CTRL.ENABLE)
    peripherals.CCL.ctrl.modify(|_, w| w.enable().set_bit());
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





#[interrupt]
fn DAC_OTHER() {
    //  Give some feedback in the terminal
    rprint!(" DAC_OTHER interrupt ");

    //  To do:  clear interrupt flag etc...
    todo!();

//    /*  Execute closure in critical section...   */
//    cortex_m::interrupt::free(|cs| {
//        disable interrupt flag...
//    })
}



#[interrupt]
fn DAC_EMPTY_0() {
    //  Give some feedback in the terminal
    rprint!(" DAC_EMPTY_0 interrupt ");

    //  To do:  clear interrupt flag etc...
    todo!();

//    /*  Execute closure in critical section...   */
//    cortex_m::interrupt::free(|cs| {
//        disable interrupt flag...
//    })
}



#[interrupt]
fn DAC_EMPTY_1() {
    //  Give some feedback in the terminal
    rprint!(" DAC_EMPTY_1 interrupt ");

    //  To do:  clear interrupt flag etc...
    todo!();

//    /*  Execute closure in critical section...   */
//    cortex_m::interrupt::free(|cs| {
//        disable interrupt flag...
//    })
}



#[interrupt]
fn DAC_RESRDY_0() {
    //  Give some feedback in the terminal
    rprint!(" DAC_RESRDY_0 interrupt ");

    //  To do:  clear interrupt flag etc...
    todo!();

//    /*  Execute closure in critical section...   */
//    cortex_m::interrupt::free(|cs| {
//        disable interrupt flag...
//    })
}



#[interrupt]
fn DAC_RESRDY_1() {
    //  Give some feedback in the terminal
    rprint!(" DAC_RESRDY_1 interrupt ");

    //  To do:  clear interrupt flag etc...
    todo!();

//    /*  Execute closure in critical section...   */
//    cortex_m::interrupt::free(|cs| {
//        disable interrupt flag...
//    })
}


