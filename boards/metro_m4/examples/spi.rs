
/*
BrunoMelaneo@VCH-LT-00177 MINGW64 /c/Users/brunomelaneo/Documents/VitalBio/SOFTWARE/atsamd_my_fork/atsamd/boards/metro_m4 (implementing_the_dac)
$ cargo build --release --features=atsamd-hal/dma  --example spi
*/

#![no_std]
#![no_main]


use metro_m4 as bsp;

//use bsp::{hal, Cs};
use bsp::hal;

#[cfg(not(feature = "use_semihosting"))]
use panic_halt as _;
#[cfg(feature = "use_semihosting")]
use panic_semihosting as _;

use bsp::{entry, periph_alias};
use hal::clock::GenericClockController;
use hal::delay::Delay;
use hal::pac::{CorePeripherals, Peripherals};
use hal::prelude::*;
use hal::dmac::{DmaController,
                PriorityLevel,
            };


mod dac_mcp48fvb12_1;
//mod dac_mcp48fvb12_2;

use dac_mcp48fvb12_1::CmdDacOutput;
use dac_mcp48fvb12_1::Cmd0x08Vref;
use dac_mcp48fvb12_1::Cmd0x09PowerDown;
use dac_mcp48fvb12_1::Cmd0x0AGain;
use crate::dac_mcp48fvb12_1::DacId;
use crate::dac_mcp48fvb12_1::CmdType;
use crate::dac_mcp48fvb12_1::VrefSource;
use crate::dac_mcp48fvb12_1::Power;
use crate::dac_mcp48fvb12_1::Gain;
//use crate::dac_mcp48fvb12_1::SetOutput;
use crate::dac_mcp48fvb12_1::SetCommandType;
use crate::dac_mcp48fvb12_1::BuildCommand;
use crate::dac_mcp48fvb12_1::CommandStream;
//use crate::dac_mcp48fvb12_1::DAC_COMMAND_SIZE;
//use core::default::Default;






use hal::pac::interrupt;
use cortex_m::peripheral::NVIC;

use rtt_target::{rprint, rprintln, rtt_init_print};

use hal::dmac::InterruptFlags;


//static TIMER:    Mutex<RefCell<Option<hal::timer::TimerCounter<TC4>>>> = Mutex::new(RefCell::new(None));
//static TRANSFER: Mutex<RefCell<Option<Transfer<dyn AnyChannel, dyn AnyBufferPair >>>> = Mutex::new(RefCell::new(None));

//  Needed to define the state type of "TRANSFER":
use cortex_m::interrupt::Mutex;
use core::cell::RefCell;
//use hal::gpio::C;
//use hal::gpio::Alternate;
//use hal::gpio::PA12;
//use hal::gpio::PA13;
//use hal::gpio::PA14;
//use hal::gpio::Pin;
//use hal::sercom::IoSet1;
//use hal::sercom::spi::Spi;
//use hal::sercom::spi::Config;
//use hal::sercom::spi::Pads;
//use hal::sercom::spi::Duplex;
//use hal::dmac::{Transfer,
//                BufferPair,
//                Channel,
//                Ch1,
//                Busy,
//                //CallbackStatus,
//  };
//use hal::pac::SERCOM2;

//  static TRANSFER:  Mutex
//                    <
//                        RefCell
//                        <
//                            Option
//                            <
//                                Transfer
//                                <
//                                    Channel
//                                    <
//                                        Ch1,
//                                        Busy
//                                    >,
//                                    BufferPair
//                                    <
//                                        &mut [u8; 13],
//                                        Spi
//                                        <
//                                            Config
//                                            <
//                                                Pads
//                                                <SERCOM2,
//                                                    IoSet1,
//                                                    Pin
//                                                    <
//                                                        PA14,
//                                                        Alternate<C>
//                                                    >,
//                                                    Pin
//                                                    <
//                                                        PA12,
//                                                        Alternate<C>
//                                                    >,
//                                                    Pin
//                                                    <
//                                                        PA13,
//                                                        Alternate<C>
//                                                    >
//                                                >
//                                            >,
//                                            Duplex
//                                        >
//                                    >
//                                    ,
//                                    ()
//                                    //impl FnOnce
//                                    //FnOnce<CallbackStatus>
//                                    //FnOnce(CallbackStatus)
//                                    //fn (CallbackStatus)  could not make callback to compile
//                                    //                      so removed it and use "|_| {}" instead.
//                                >
//                            >
//                        >
//                    > = Mutex::new(RefCell::new(None));


    static DMAC:  Mutex
    <
        RefCell
        <
            Option
            <
                DmaController
            >
        >
    > = Mutex::new(RefCell::new(None));



#[entry]
fn main() -> ! {

    /*  Initialise remote print...   */
    rtt_init_print!();
    rprintln!("\n================");

    let mut peripherals = Peripherals::take().unwrap();
    let mut core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.MCLK,
        &mut peripherals.OSC32KCTRL,
        &mut peripherals.OSCCTRL,
        &mut peripherals.NVMCTRL,
    );

    let mut delay = Delay::new(core.SYST, &mut clocks);
    let pins = bsp::Pins::new(peripherals.PORT);

    let miso    = pins.miso;
    let mosi    = pins.mosi;
    let sclk    = pins.sclk;
    let cs      = pins.cs;


    let spi_sercom = periph_alias!(peripherals.spi_sercom);
    let mclk = &mut peripherals.MCLK;

    /*  DMA setup...    */
    /*  Enable all interrupts for now...    */

    //  Configure DAC interrupts at global interrupt controller
    unsafe {
        core.NVIC.set_priority(interrupt::DMAC_0     , 1);
        core.NVIC.set_priority(interrupt::DMAC_1     , 1);
        core.NVIC.set_priority(interrupt::DMAC_2     , 1);
        core.NVIC.set_priority(interrupt::DMAC_3     , 1);
        core.NVIC.set_priority(interrupt::DMAC_OTHER , 1);

        NVIC::unmask(interrupt::DMAC_0     );
        NVIC::unmask(interrupt::DMAC_1     );
        NVIC::unmask(interrupt::DMAC_2     );
        NVIC::unmask(interrupt::DMAC_3     );
        NVIC::unmask(interrupt::DMAC_OTHER );
    }

    let dmac = peripherals.DMAC;
    let mut dmac = DmaController::init( dmac,
                                        &mut peripherals.PM);

    let dma_channels = dmac.split();

    let dma_ch1 = dma_channels.1.init(PriorityLevel::LVL0);
//    let dma_ch2 = dma_channels.2.init(PriorityLevel::LVL0);

    //  Initialise SPI instance...
    let spi =
        bsp::spi_master(&mut clocks,
                        3.mhz(),
                        spi_sercom,
                        mclk,
                        sclk,
                        mosi,
                        miso,
                        cs//  pass the chip select pin so that the sercom drives the CS pin autonomously
                    );


//      loop {
//          for byte in b"Hello, world!" {
//              nb::block!(spi.send(*byte)).unwrap();
//          }
//          delay.delay_ms(1000u16);
//      }


    let mut cmd1 = CmdDacOutput::new(DacId::Dac0);
    rprintln!("   cmd1 :: {:?}", cmd1);
    cmd1.set_output(0xFFu16);
    rprintln!("   cmd1 :: {:?}", cmd1);

    rprintln!("--------------------------------------");
    let mut cmd2 = CmdDacOutput::new(DacId::Dac1);
    rprintln!("   cmd2 :: {:?}", cmd2);
    cmd2.set_output(0xFFu16);
    rprintln!("   cmd2 :: {:?}", cmd2);

    cmd2.set_cmd_type(CmdType::READ);
    rprintln!("   cmd2 :: {:?}", cmd2);

    rprintln!("--------------------------------------");
    let mut cmd_0x08_vref = Cmd0x08Vref::new();
    cmd_0x08_vref.set_vref( VrefSource::VrefUnbuffered,
                            VrefSource::VddUnbuffered);
    cmd_0x08_vref.build_command();
    rprintln!("cmd_0x08_Vref :: {:?}", cmd_0x08_vref);

    rprintln!("--------------------------------------");
    let mut cmd_0x09_power_down = Cmd0x09PowerDown::new();
    cmd_0x09_power_down.set_power_down( Power::NormalOperation,
                                        Power::NormalOperation);
    cmd_0x09_power_down.build_command();
    rprintln!("cmd_0x09_power_down :: {:?}", cmd_0x09_power_down);

    rprintln!("--------------------------------------");
    let mut cmd_0x0a_gain = Cmd0x0AGain::new();
    cmd_0x0a_gain.set_gain( Gain::Gainx2,
                            Gain::Gainx2);
    cmd_0x0a_gain.build_command();
    rprintln!("cmd_0x0a_gain :: {:?}", cmd_0x0a_gain);



    //  static mut STREAM______xxxxxxxx: CommandStream = CommandStream::new();
    //  static mut STREAM______xxxxxxxx: CommandStream = Default::default();
    //  STREAM______xxxxxxxx =

    cmd2.build_command();

    static mut STREAM : [u8 ; 3] = [0; 3];

    unsafe
    {
        rprintln!("   STREAM :: {:?}", STREAM);

        /*  Setup Tx transfer...    */
        let _dma_transfer =
            spi.send_with_dma(&mut STREAM,
                dma_ch1,
                |_| {}   //  Could not make "callback" to work...
        );

//  works!        static mut BUFFER_TX: [u8; 13] = *b"Hello, world!";
//  works!        /*  Setup Tx transfer...    */
//  works!        let _dma_transfer =
//  works!            spi.send_with_dma(&mut BUFFER_TX,
//  works!                dma_ch1,
//  works!                |_| {}   //  Could not make "callback" to work...
//  works!        );

//        /*  Setup Rx transfer...    */
//        let _dma_transfer =
//            spi.receive_with_dma(&mut BUFFER_RX,
//                dma_ch2,
//                |_| {}   //  Could not make "callback" to work...
//        );


        cortex_m::interrupt::free(|cs| {
            DMAC.borrow(cs).replace(Some(dmac));
        });
    }




    loop {
//        //for byte in b"Hello, world!" {
//        //for byte in b"123" {
//        for byte in msg {
//            nb::block!(spi.send(*byte)).unwrap();
//        }
        delay.delay_ms(1u16);

//        cortex_m::asm::wfi();

    }
}

//fn callback (sts: CallbackStatus)
//{
//    rprint!("\n   Callback... ");
//    match sts
//    {
//        CallbackStatus::TransferComplete   => rprint!("\n    sts :: CallbackStatus::TransferComplete   "),
//        CallbackStatus::TransferError      => rprint!("\n    sts :: CallbackStatus::TransferError      "),
//        CallbackStatus::TransferSuspended  => rprint!("\n    sts :: CallbackStatus::TransferSuspended  "),
//    }
//}


#[interrupt]
fn DMAC_1() {
    rprint!(" DMAC_1 interrupt ");


    cortex_m::interrupt::free(|cs| {
        //  Get the xfer reference...
        let mut dmac_ref_mut = DMAC.borrow(cs).borrow_mut();
        let dmac_option = dmac_ref_mut.as_mut();
        let dmac = dmac_option.unwrap();

        let mut channels = dmac.split();

//        let dma_ch2 = channels.2;

        let mut flags = InterruptFlags::new();


        //  To do:
        //      Figure out why the interrupt triggered?
        //      Clear the interrupt flag etc...
        flags.set_tcmpl(true);  assert_eq!(flags.tcmpl(),   true);
        flags.set_terr(true);   assert_eq!(flags.terr(),    true);
        flags.set_susp(true);   assert_eq!(flags.susp(),    true);

        let interrup = channels.1.check_and_clear_interrupts(flags);

        rprint!("\n    Why did the interrupt fire   {} {} {}",
            interrup.tcmpl(),
            interrup.terr(),
            interrup.susp(),
        );

        if true == interrup.tcmpl() {
            rprint!("\n    tcmpl: TransferComplete   ");
        }
        else {
            if true == interrup.terr() {
                rprint!("\n    terr: TransferError      ");
            }
            else{
                if true == interrup.susp() {
                    rprint!("\n    susp: TransferSuspended  ");
                }
                else {
                    rprint!("\n .........  Unexpected...  ");
                }
            }
        }

        /*  Was it a Tx or Rx transfer that triggered the interrupt?    */
        rprint!("\n  Interrupt Trigger Source: {} ",
            channels.1.interrupt_trigger_source()
        );
    })
}


#[interrupt]
fn DMAC_0() {
    rprint!("\n DMAC_0 interrupt ");

    //  To do:  clear interrupt flag etc...
    todo!();
}
#[interrupt]
fn DMAC_2() {
    rprint!("\n DMAC_2 interrupt ");

    //  To do:  clear interrupt flag etc...
    todo!();
}
#[interrupt]
fn DMAC_3() {
    rprint!("\n DMAC_3 interrupt ");

    //  To do:  clear interrupt flag etc...
    todo!();
}
#[interrupt]
fn DMAC_OTHER() {
    rprint!("\n DMAC_OTHER interrupt ");

    //  To do:  clear interrupt flag etc...
    todo!();
}