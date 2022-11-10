
/*
BrunoMelaneo@VCH-LT-00177 MINGW64 /c/Users/brunomelaneo/Documents/VitalBio/SOFTWARE/atsamd_my_fork/atsamd/boards/metro_m4 (implementing_the_dac)
$ cargo build --release --features=atsamd-hal/dma  --example spi
*/

#![no_std]
#![no_main]

//use atsamd_hal::dmac::Ch1;
use metro_m4 as bsp;

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

use hal::dmac::{DmaController, PriorityLevel};


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

    let mut delay = Delay::new(core.SYST, &mut clocks);
    let pins = bsp::Pins::new(peripherals.PORT);

    let miso = pins.miso;
    let mosi = pins.mosi;
    let sclk = pins.sclk;
    let spi_sercom = periph_alias!(peripherals.spi_sercom);
    let mclk = &mut peripherals.MCLK;
    /*  DMA setup...    */
    let dmac = peripherals.DMAC;
    let mut dmac = DmaController::init( dmac,
                                        &mut peripherals.PM);

    let dma_channels = dmac.split();
    let dma_ch1 = dma_channels.1.init(PriorityLevel::LVL0);

    //  Initialise SPI instance...
    let spi =
        bsp::spi_master(&mut clocks,
                        3.mhz(),
                        spi_sercom,
                        mclk,
                        sclk,
                        mosi,
                        miso);


    //static mut BUFFER: [u8; 50] = [0xff; 50];
    static mut BUFFER: [u8; 13] = *b"Hello, world!";
    unsafe{
        let _dma_transfer = spi.send_with_dma(&mut BUFFER, dma_ch1, |_|{});
        // dma_transfer -> Transfer<Channel<Ch::Id, Busy>, BufferPair<B, Self>, W>
        //let (dma_ch1, _, _) = dma_transfer.wait();

//        any::type_name(dma_transfer);
        //let dma_transfer = spi.send_with_dma(&mut BUFFER, dma_ch1, |_|{});
    }




    loop {
//        //for byte in b"Hello, world!" {
//        //for byte in b"123" {
//        for byte in msg {
//            nb::block!(spi.send(*byte)).unwrap();
//        }
        delay.delay_ms(1u16);

        cortex_m::asm::wfi();

    }
}

