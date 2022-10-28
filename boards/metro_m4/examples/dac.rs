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
            DAC_OTHER,      //  ToDo: fix "error[E0603]: function `DAC_OTHER` is private"
        //    DAC_EMPTY_0,
        //    DAC_EMPTY_1,
        //    DAC_RESRDY_0,
        //    DAC_RESRDY_1
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

static G_TIMER: Mutex<RefCell<Option<hal::timer::TimerCounter<TC4>>>> =
    Mutex::new(RefCell::new(None));



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

    //  instantiate Timer 4
    let gclk0 = gen_clcks.gclk0();
    let tc4 = gen_clcks.tc4_tc5(&gclk0).unwrap();
    let mut timer_4 =
        //TimerCounter4::tc4_(
        TimerCounter::<TC4>::tc4_(
            &tc4,                   //  &clock::Tc4Tc5Clock
            peripherals.TC4,        //  TC4
            &mut peripherals.MCLK   //  MCLK
        );

    //  Configure the interrupt controller
    unsafe {
        core.NVIC.set_priority(interrupt::TC4, 1);
        NVIC::unmask(interrupt::TC4);
    }

    //  Start Timer...
    timer_4.start(10.hz());

    //  Enable interrupt...
    timer_4.enable_interrupt();

    //  Store timer in the GLOBAL TIMER so it can be
    //  accessed from the interrupt ISR
    cortex_m::interrupt::free(|cs| {
        G_TIMER.borrow(cs).replace(Some(timer_4));
    });



    //  Inidialise the dac..
    let dac0 =
        Dac::init(   &mut peripherals.MCLK,
                    peripherals.DAC);
    //  Enable dac
    dac0.enable_dac_controller();
    //  Chnage the dac DATA register to start a new conversion...
    dac0.start_conversion(0);

    //  Enable Interrupts





    // PA02 :: Dac Vout0: LED_T_CTRL_BF: Bright temperature control
    // PA05 :: Dac Vout1: LED_T_CTRL_DF: Darkfield temperature control
    let pins = bsp::Pins::new(peripherals.PORT);
    let dac_0_out_pin = pins.a0.into_alternate::<B>();
    let dac_1_out_pin = pins.a0.into_alternate::<B>();




    //  Select and set the Pin we need...
    let pins = bsp::Pins::new(peripherals.PORT);
    let red_led = pins.d13.into_push_pull_output();

    //  Store red_led in the GLOBAL LED so it can be
    //  accessed from the interrupt ISR
    cortex_m::interrupt::free(|cs| {
        G_LED.borrow(cs).replace(Some(red_led));
    });













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

    /*  Execute closure in critical section...   */
    cortex_m::interrupt::free(|cs| {


    })
}



//#[interrupt]
//fn DAC_EMPTY_0() {
//    //  Give some feedback in the terminal
//    rprint!(" DAC_EMPTY_0 interrupt ");
//
//    /*  Execute closure in critical section...   */
//    cortex_m::interrupt::free(|cs| {
//
//
//    })
//}
//
//
//
//#[interrupt]
//fn DAC_EMPTY_1() {
//    //  Give some feedback in the terminal
//    rprint!(" DAC_EMPTY_1 interrupt ");
//
//    /*  Execute closure in critical section...   */
//    cortex_m::interrupt::free(|cs| {
//
//
//    })
//}
//
//
//
//#[interrupt]
//fn DAC_RESRDY_0() {
//    //  Give some feedback in the terminal
//    rprint!(" DAC_RESRDY_0 interrupt ");
//
//    /*  Execute closure in critical section...   */
//    cortex_m::interrupt::free(|cs| {
//
//
//    })
//}
//
//
//
//#[interrupt]
//fn DAC_RESRDY_1() {
//    //  Give some feedback in the terminal
//    rprint!(" DAC_RESRDY_1 interrupt ");
//
//    /*  Execute closure in critical section...   */
//    cortex_m::interrupt::free(|cs| {
//
//
//    })
//}


