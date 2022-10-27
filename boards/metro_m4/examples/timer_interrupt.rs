#![no_std]
#![no_main]

use hal::sercom::uart::Clock;
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
use pac::{interrupt, CorePeripherals, Peripherals, TC4};

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


//static LED: Mutex<RefCell<Option<   bsp::Pins   >>> = Mutex::new(RefCell::new(None));
//static LED: Mutex<RefCell<Option<   bsp::Pins::Pin<PA16, Output<PushPull>>   >>> = Mutex::new(RefCell::new(None));
//static LED: Mutex<RefCell<Option<   bsp::RedLed   >>> = Mutex::new(RefCell::new(None));
//static LED: Mutex<RefCell<Option<   Pins   >>> = Mutex::new(RefCell::new(None));
//static G_LED: Mutex<RefCell<Option<   hal::gpio::Pin<xxx, xxx>/*<PD13,PushPullOutput>*/   >>> = Mutex::new(RefCell::new(None));

//static G_LED: Mutex<RefCell<Option<   hal::gpio::Pin<hal::gpio::PA16, hal::gpio::PushPullOutput>   >>> = Mutex::new(RefCell::new(None));
use hal::gpio::PushPull;
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
    timer_4.start(4.hz());

    //  Enable interrupt...
    timer_4.enable_interrupt();

    //  Store timer in the GLOBAL TIMER so it can be
    //  accessed from the interrupt ISR
    cortex_m::interrupt::free(|cs| {
        G_TIMER.borrow(cs).replace(Some(timer_4));
    });












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
fn TC4() {
    //  Give some feedback in the terminal
    rprint!(".");

    /*  Execute closure in critical section...   */
    cortex_m::interrupt::free(|cs| {

//      RefCell<Option<   Pins   >>> = new(RefCell::new(None);
        let mut red_led_ref =
            G_LED.borrow(cs).borrow_mut();

        let red_led = red_led_ref.as_mut().unwrap();
        red_led.set_high().unwrap();

//        now toggle it...





        //  Get the timer reference so we can clear the interrupt flag...
        let mut rc = G_TIMER.borrow(cs).borrow_mut();
        let timer = rc.as_mut().unwrap();

        //  Reset the interrupt flag so that the ISR does not trigger again.
        //timer.clear_all_irq();

        if timer.is_ovf_int_flag_set() {
            timer.clear_ovf_irq();
        }

        if timer.is_err_int_flag_set() {
            timer.clear_err_irq();
            panic!("Timmer err interrupt should not be triggered!!");
        }

        if timer.is_mc0_int_flag_set() {
            timer.clear_mc0_irq();
            panic!("Timmer err interrupt should not be triggered!!");
        }

        if timer.is_mc1_int_flag_set() {
            timer.clear_mc1_irq();
            panic!("Timmer err interrupt should not be triggered!!");
        }
    })
}
