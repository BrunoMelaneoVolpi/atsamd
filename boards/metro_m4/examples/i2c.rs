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

//use cortex_m::asm;
use pac::{CorePeripherals, Peripherals};


use hal::clock::GenericClockController;
//use hal::dmac::{DmaController, PriorityLevel};
//use hal::ehal::blocking::i2c::WriteRead;
//use hal::prelude::*;
//use hal::sercom::i2c;
//use hal::time::MegaHertz;
use hal::time::KiloHertz;
use hal::time::Hertz;

use hal::ehal::blocking::delay::DelayMs;
use hal::delay::Delay;

use rtt_target::{/*rprint,*/ rprintln, rtt_init_print};







use mlx90632::Mlx90632;
use mlx90632::mlx90632_types::Mlx90632Address;








struct ReadWrite(u8);
struct MemoryMap;
#[allow(dead_code)]
impl MemoryMap{
    pub const PADC_MSB : ReadWrite = ReadWrite(0x00);   //  10-bit Pressure ADC output value MSB 8
    pub const PADC_LSB : ReadWrite = ReadWrite(0x01);   //  10-bit Pressure ADC output value LSB 2
    pub const TADC_MSB : ReadWrite = ReadWrite(0x02);   //  10-bit Temperature ADC output value MSB 8
    pub const TACD_LSB : ReadWrite = ReadWrite(0x03);   //  10-bit Temperature ADC output value LSB 2
    pub const A0_MSB   : ReadWrite = ReadWrite(0x04);   //  a0 coefficient MSB 8
    pub const A0_LSB   : ReadWrite = ReadWrite(0x05);   //  a0 coefficient LSB 8
    pub const B1_MSB   : ReadWrite = ReadWrite(0x06);   //  b1 coefficient MSB 8
    pub const B1_LSB   : ReadWrite = ReadWrite(0x07);   //  b1 coefficient LSB 8
    pub const B2_MSB   : ReadWrite = ReadWrite(0x08);   //  b2 coefficient MSB 8
    pub const B2_LSB   : ReadWrite = ReadWrite(0x09);   //  b2 coefficient LSB 8
    pub const C12_MSB  : ReadWrite = ReadWrite(0x0A);   //  c12 coefficient MSB 8
    pub const C12_LSB  : ReadWrite = ReadWrite(0x0B);   //  c12 coefficient LSB
    pub const CONVERT  : ReadWrite = ReadWrite(0x12);   //  Start Pressure and Temperature Conversion
}


//const MLX90632_BAUD: u32 = 100;
//const MLX90632_BAUD: MegaHertz = MegaHertz(1);
const MLX90632_BAUD: KiloHertz = KiloHertz(100);


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


    let pins = bsp::Pins::new(peripherals.PORT);

    //  Take SDA and SCL
    let (sda,
         scl,
         _shut_down
         ) = (pins.pb02, pins.scl, pins.d13);

    /*  Setup I2C interface...  */
    let i2c_sercom = periph_alias!(peripherals.i2c_sercom);
    let mclk = &mut peripherals.MCLK;
    let i2c =
        bsp::i2c_master(& mut clocks,
                        MLX90632_BAUD, //.khz(),
                        i2c_sercom,
                        mclk,
                        sda,
                        scl);

    /*  Setup delay feature...      */
    let core = CorePeripherals::take().unwrap();
    let mut delay = Delay::new(core.SYST, &mut clocks);



//    /*  Setup Shutdown pin...   */
//    let mut shut_down = shut_down.into_push_pull_output();
//
//    /*  Reset device...         */
//    /*      Shut down...        */
//    shut_down.set_low().unwrap();
//    /*      Wait...             */
//    delay.delay_us(100u16);
//    /*      Revive the thing... */
//    shut_down.set_high().unwrap();
//    /*      Let it wake up...   */
//    delay.delay_us(100u16);
//
//
//
//
//
//    // Test writing then reading from an I2C chip
//    //let mut buffer = [0x00; 1];
//    //i2c.write_read(ADDRESS, &[0x00], &mut buffer).unwrap();
//
//
//    //  panic!("asdfasdfasdfasdfasdfasdfasdfasdf");
//    //  Test writing then reading from an I2C chip
//    const ADDRESS: u8 = 0x60;
//
//    /*  Documentation is SSSHIIITTTT
//            Read Coeficients...
//                <-I2c Write->   <-I2c read->
//                [0xC0], [0x04], [0xC1], [0x3E], [0xCE], [0xB3], [0xF9], [0xC5], [0x17], [0x33], [0xC8]
//                addW    coef    addR    <--------------- Read Value ------------------------------->        */
//    const NUMBER_OF_COEFFICIENTS : usize = 8;
//
//    let mut buffer = [0x00; NUMBER_OF_COEFFICIENTS];
//    for coefficients in buffer{
//        rprintln!("     data {:#02x}", coefficients);
//    }
//
//    rprintln!("==== Reading coefficients ============");
//    let _x = i2c.write(ADDRESS, &[MemoryMap::A0_MSB.0]);
//
//    let _x = i2c.read(ADDRESS, &mut buffer);
//
//    for coefficients in buffer{
//        rprintln!("     data {:#02x}", coefficients);
//    }
//
//    rprintln!("================");
//    //  Start conversion...
//    let x = i2c.write(ADDRESS, &[MemoryMap::CONVERT.0, 00_u8]);      //  ok
//    match x {
//        Result::Ok(t) => {
//            rprintln!("Ok({:?})", t)
//        }
//        Result::Err(e) => {
//            loop {
//                rprintln!("Err({:?})", e);
//            } //  Forever...
//        }
//    }
//
//    delay.delay_us(1000u16);
//
//    /*  Get Pressure...     */
//    rprintln!("==== Reading pressure ============");
//    const PRESSURE_TEMP_BYTES : usize = 2;
//    let mut pressure_temperature = [0x00; PRESSURE_TEMP_BYTES];
//    for press in pressure_temperature{
//        rprintln!("     press {:#02x}", press);
//    }
//
//    rprintln!(" ------------------------------------ ");
//    let _x = i2c.write(ADDRESS, &[MemoryMap::PADC_MSB.0]);
//
//    let _x = i2c.read(ADDRESS, &mut pressure_temperature);
//
//    for press in pressure_temperature{
//        rprintln!("     press {:#02x}", press);
//    }
//
//
//    /*  Get Temperature...  */
//    rprintln!("==== Reading temperature ============");
//    pressure_temperature = [0x00; PRESSURE_TEMP_BYTES];
//    for temperature in pressure_temperature{
//        rprintln!("     temperature {:#02x}", temperature);
//    }
//
//    rprintln!(" ------------------------------------ ");
//    let _x = i2c.write(ADDRESS, &[MemoryMap::TADC_MSB.0]);
//
//    let _x = i2c.read(ADDRESS, &mut pressure_temperature);
//
//    for temperature in pressure_temperature{
//        rprintln!("     temperature {:#02x}", temperature);
//    }




    let i2c_address : Mlx90632Address = Mlx90632Address(0x60);
    rprintln!("===== Hertz::from(MLX90632_BAUD).0 :: {:?} ", Hertz::from(MLX90632_BAUD).0);

    let mut mlx90632 =
        Mlx90632::new(  i2c,
                        i2c_address,
                        Hertz::from(MLX90632_BAUD).0,
                        ).unwrap();

    mlx90632.debug_write_read();



    loop {
        delay.delay_ms(1000u16);
        rprintln!("===== DEMO OVER ====");
    }


}
