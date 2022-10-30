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
use bsp::Pins;
use hal::clock::GenericClockController;
use hal::prelude::*;
use hal::dac::Dac;
use pac::{  interrupt,
            CorePeripherals,
            Peripherals,
            TC4,
        };


use ehal::blocking::delay::DelayMs;
use hal::delay::Delay;

use rtt_target::{rprint, rprintln, rtt_init_print};
//use core::sync::atomic::{AtomicUsize, Ordering};
use core::{
    cell::RefCell,
};

use atsamd_hal::time::U32Ext;

//use hal::timer::TimerCounter4;
use hal::timer::TimerCounter;
//use hal::timer::Count16;
use cortex_m::peripheral::NVIC;

use cortex_m::{interrupt::Mutex};
use crate::pac::gclk::genctrl::SRC_A::DFLL;
use pac::gclk::pchctrl::GEN_A::GCLK0;   //  Cyclops has a centralized
                                        //  point where all clock are setup...
                                        //  Make sure the clock is correct...


//static G_LED: Mutex<RefCell<Option<   hal::gpio::Pin<hal::gpio::PA16, hal::gpio::PushPullOutput>   >>> = Mutex::new(RefCell::new(None));
use hal::gpio::PushPull;
use hal::gpio::B;
static G_LED: Mutex<RefCell<Option<   hal::gpio::Pin<hal::gpio::PA16, hal::gpio::Output<PushPull>>   >>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();
    let mut gen_clcks =
        GenericClockController::with_external_32kosc(
            peripherals.GCLK,
            &mut peripherals.MCLK,
            &mut peripherals.OSC32KCTRL,
            &mut peripherals.OSCCTRL,
            &mut peripherals.NVMCTRL,
    );

    //  ToDo:   Make sure the generic clock "GCLK_DAC" has been properly...
    //let dac_clock =
    //    gen_clcks.configure_gclk_divider_and_source(GCLK0,
    //         1, DFLL, false);
    //todo!();


    //  instantiate Timer 4
    let gclk0 = gen_clcks.gclk0();
    let tc4 = gen_clcks.tc4_tc5(&gclk0).unwrap();
    let mut timer_4 =
        TimerCounter::<TC4>::tc4_(
            &tc4,                   //  &clock::Tc4Tc5Clock
            peripherals.TC4,        //  TC4
            &mut peripherals.MCLK   //  MCLK
        );

    //  Configure the interrupt controller
    unsafe {
        core.NVIC.set_priority(interrupt::DAC_OTHER     , 1);
        core.NVIC.set_priority(interrupt::DAC_EMPTY_0   , 1);
        core.NVIC.set_priority(interrupt::DAC_EMPTY_1   , 1);
        core.NVIC.set_priority(interrupt::DAC_RESRDY_0  , 1);
        core.NVIC.set_priority(interrupt::DAC_RESRDY_1  , 1);

        NVIC::unmask(interrupt::DAC_OTHER   );
        NVIC::unmask(interrupt::DAC_EMPTY_0 );
        NVIC::unmask(interrupt::DAC_EMPTY_1 );
        NVIC::unmask(interrupt::DAC_RESRDY_0);
        NVIC::unmask(interrupt::DAC_RESRDY_1);
    }





    //  Inidialise the dac..
    let (mut dac, dac0, _dac1) =
        Dac::init(   &mut peripherals.MCLK,
                    peripherals.DAC);

    //  Enable dac
    dac = dac.enable_dac_controller();
    //  Change the dac0 DATA register to start a new conversion...
    dac0.start_conversion(&mut dac);



    //  Enable Interrupts
    //  Todo...




    // PA02 :: Dac Vout0: LED_T_CTRL_BF: Bright temperature control
    // PA05 :: Dac Vout1: LED_T_CTRL_DF: Darkfield temperature control
    let pins = bsp::Pins::new(peripherals.PORT);
    let dac_0_out_pin =
        pins.a0.into_alternate::<B>();
    let dac_1_out_pin =
        pins.a5.into_alternate::<B>();




//    //  Select and set the Pin we need...
//    let pins = bsp::Pins::new(peripherals.PORT);
//    let red_led = pins.d13.into_push_pull_output();
//
//    //  Store red_led in the GLOBAL LED so it can be
//    //  accessed from the interrupt ISR
//    cortex_m::interrupt::free(|cs| {
//        G_LED.borrow(cs).replace(Some(red_led));
//    });




    /*  Initialise remote print...   */
    rtt_init_print!();
    rprintln!("================");

    //  Delay to be used in the loop...
    let mut delay = Delay::new(core.SYST, &mut gen_clcks);
    loop {
        rprintln!("Hello, world!");
        delay.delay_ms(1000u16);
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


