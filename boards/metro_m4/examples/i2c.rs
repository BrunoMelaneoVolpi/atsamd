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
use hal::prelude::*;        //  <pin>.set_high().unwrap();
//use hal::sercom::i2c;
//use hal::time::MegaHertz;
use hal::time::KiloHertz;
use hal::time::Hertz;

use hal::ehal::blocking::delay::DelayMs;
use hal::delay::Delay;

use rtt_target::{/*rprint,*/ rprintln, rtt_init_print};







use mlx90632::Mlx90632;
use mlx90632::mlx90632_types::Mlx90632Address;
use mlx90632::mlx90632_types::Mlx90632Error;
use mlx90632::mlx90632_types::AddrPin;








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
         _shut_down,
         mlx90632_addr_pin,) = (pins.pb02,
                        pins.scl,
                        pins.d13,
                        pins.d12);

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



////    /*  Setup Shutdown pin...   */
//    let mut shut_down = _shut_down.into_push_pull_output();
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




    ////    /*  Setup mlx90632_addr pin...   */
    let mut mlx90632_addr_pin = mlx90632_addr_pin.into_push_pull_output();
    mlx90632_addr_pin.set_low().unwrap();
    delay.delay_ms(10u16);
    mlx90632_addr_pin.set_high().unwrap();


    //let i2c_address = Mlx90632Address::mlx90632_default_addr(AddrPin::GND);
    let i2c_address = Mlx90632Address::mlx90632_default_addr(AddrPin::VDD);
    //let i2c_address =
    //    Mlx90632Address::alternate_addr::<Mlx90632Error<()>>(   0x3B,
    //                                                            AddrPin::VDD).unwrap();


    //rprintln!("===== Testing alternate_addr       1 ====");
    //    testing_alternate_address(0x01_u8, AddrPin::VDD);
    //    testing_alternate_address(0x01_u8, AddrPin::GND);
    //    testing_alternate_address(0x00_u8, AddrPin::VDD);
    //    testing_alternate_address(0x00_u8, AddrPin::GND);
    //
    //    testing_alternate_address(0xFF_u8       , AddrPin::VDD);
    //    testing_alternate_address(0xFF_u8       , AddrPin::GND);
    //    testing_alternate_address(0xFF_u8 - 1_u8, AddrPin::VDD);
    //    testing_alternate_address(0xFF_u8 - 1_u8, AddrPin::GND);
    //    let i2c_address = Mlx90632Address::mlx90632_default_addr(AddrPin::VDD);
    //rprintln!("===== Testing alternate_addr      2 ====");


    rprintln!("===== Hertz::from(MLX90632_BAUD).0 :: {:?} ", Hertz::from(MLX90632_BAUD).0);

    rprintln!("===== mlx90632 init ====");


    let freed_sys = delay.free();

    let mut mlx_delay = Delay::new(freed_sys, &mut clocks);
    mlx_delay.delay_ms(100u16);



    /*  **************************************************************** */

    //let mut mlx90632 =
    //    Mlx90632::new(  i2c,
    //                    i2c_address,
    //                    Hertz::from(MLX90632_BAUD).0).unwrap();

    //match mlx90632 {
    //    Ok(_) => {
    //        rprintln!("=====        init Ok  ==== ");
    //    }
    //    Err(e) => {
    //        rprintln!("=====        init Err {:?} ==== ", e);
    //    }
    //}

    //let mut mlx90632_1 = mlx90632.unwrap();


    //print_u16(" id0                             ", mlx90632.ee_params.id0             );
    //print_u16(" id1                             ", mlx90632.ee_params.id1             );
    //print_u16(" id2                             ", mlx90632.ee_params.id2             );
    //print_u16(" id_crc                          ", mlx90632.ee_params.id_crc          );
    //rprintln!("  ee_product_code_fov                 : {:?} ", mlx90632.ee_params.ee_product_code_fov            );
    //rprintln!("  ee_product_code_package             : {:?} ", mlx90632.ee_params.ee_product_code_package        );
    //rprintln!("  ee_product_code_accuracy_range      : {:?} ", mlx90632.ee_params.ee_product_code_accuracy_range );
    //print_u16(" ee_version                      ", mlx90632.ee_params.ee_version      );
    //rprintln!("  ee_version_extended_rng_support     : {:?} ", mlx90632.ee_params.ee_version_extended_rng_support );


    //print_i32(" EE_P_R              ", mlx90632.ee_params.ee_p_r          );
    //print_i32(" EE_P_G              ", mlx90632.ee_params.ee_p_g          );
    //print_i32(" EE_P_T              ", mlx90632.ee_params.ee_p_t          );
    //print_i32(" EE_P_O              ", mlx90632.ee_params.ee_p_o          );
    ////print_u32(" EE_AA               ", mlx90632.ee_params.ee_aa           );
    ////print_u32(" EE_AB               ", mlx90632.ee_params.ee_ab           );
    ////print_u32(" EE_BA               ", mlx90632.ee_params.ee_ba           );
    ////print_u32(" EE_BB               ", mlx90632.ee_params.ee_bb           );
    ////print_u32(" EE_CA               ", mlx90632.ee_params.ee_ca           );
    ////print_u32(" EE_CB               ", mlx90632.ee_params.ee_cb           );
    ////print_u32(" EE_DA               ", mlx90632.ee_params.ee_da           );
    ////print_u32(" EE_DB               ", mlx90632.ee_params.ee_db           );
    //print_i32(" EE_EA               ", mlx90632.ee_params.ee_ea           );
    //print_i32(" EE_EB               ", mlx90632.ee_params.ee_eb           );
    //print_i32(" EE_FA               ", mlx90632.ee_params.ee_fa           );
    //print_i32(" EE_FB               ", mlx90632.ee_params.ee_fb           );
    //print_i32(" EE_GA               ", mlx90632.ee_params.ee_ga           );
    //delay.delay_ms(100u16);
    //print_i16(" EE_GB               ", mlx90632.ee_params.ee_gb   );
    //print_i16(" EE_KA               ", mlx90632.ee_params.ee_ka   );
    ////print_i16(" EE_KB               ", mlx90632.ee_params.ee_kb   );
    //print_i16(" EE_HA               ", mlx90632.ee_params.ee_ha   );
    //print_i16(" EE_HB               ", mlx90632.ee_params.ee_hb   );
    //delay.delay_ms(100u16);


    //rprintln!("===== mlx90632 mlx90632_get_chip_info ====");
    //let info = mlx90632.mlx90632_get_chip_info().unwrap();
    //rprintln!("===== info ====");
    //print_u16(" id0                 ", info.id0             );
    //print_u16(" id1                 ", info.id1             );
    //print_u16(" id2                 ", info.id2             );
    //print_u16(" id_crc              ", info.id_crc          );
    //print_u16(" ee_product_code     ", info.ee_product_code );
    //print_u16(" ee_version          ", info.ee_version      );
    //delay.delay_ms(500u16);
//
    //print_u32(" EE_P_R              ", info.ee_p_r          );
    //print_u32(" EE_P_G              ", info.ee_p_g          );
    //print_u32(" EE_P_T              ", info.ee_p_t          );
    //print_u32(" EE_P_O              ", info.ee_p_o          );
    //print_u32(" EE_AA               ", info.ee_aa           );
    //print_u32(" EE_AB               ", info.ee_ab           );
    //print_u32(" EE_BA               ", info.ee_ba           );
    //print_u32(" EE_BB               ", info.ee_bb           );
    //delay.delay_ms(500u16);
//
    //print_u32(" EE_CA               ", info.ee_ca           );
    //print_u32(" EE_CB               ", info.ee_cb           );
    //print_u32(" EE_DA               ", info.ee_da           );
    //print_u32(" EE_DB               ", info.ee_db           );
    //print_u32(" EE_EA               ", info.ee_ea           );
    //print_u32(" EE_EB               ", info.ee_eb           );
    //print_u32(" EE_FA               ", info.ee_fa           );
    //print_u32(" EE_FB               ", info.ee_fb           );
    //print_u32(" EE_GA               ", info.ee_ga           );
    //delay.delay_ms(100u16);
//
    //print_u16(" EE_GB               ", info.ee_gb   );
    //print_u16(" EE_KA               ", info.ee_ka   );
    //print_u16(" EE_KB               ", info.ee_kb   );
    //print_u16(" EE_HA               ", info.ee_ha   );
    //print_u16(" EE_HB               ", info.ee_hb   );
    //delay.delay_ms(100u16);
//
    //print_u16(" customer_data0      ", info.customer_data0   );
    //print_u16(" customer_data1      ", info.customer_data1   );
    //print_u16(" customer_data2      ", info.customer_data2   );
    //print_u16(" customer_data3      ", info.customer_data3   );
    //print_u16(" customer_data4      ", info.customer_data4   );
    //print_u16(" customer_data5      ", info.customer_data5   );
    //print_u16(" customer_data6      ", info.customer_data6   );
    //print_u16(" customer_data7      ", info.customer_data7   );
    //delay.delay_ms(100u16);
//
    //print_u16(" EE_CONTROL          ", info.ee_control            );
    //print_u16(" EE_I2C_ADDRESS      ", info.ee_i2c_address        );
    //print_u16(" EE_MEAS_1           ", info.ee_meas_1             );
    //print_u16(" EE_MEAS_2           ", info.ee_meas_2             );
    //print_u16(" REG_I2C_ADDRESS     ", info.reg_i2c_address       );
    //print_u16(" REG_CONTROL         ", info.reg_control           );
    //print_u16(" REG_STATUS          ", info.reg_status            );
    //delay.delay_ms(100u16);
//
    //print_u16(" RAM_01              ", info.ram_01                );
    //print_u16(" RAM_02              ", info.ram_02                );
    //print_u16(" RAM_03              ", info.ram_03                );
    //print_u16(" RAM_04              ", info.ram_04                );
    //print_u16(" RAM_05              ", info.ram_05                );
    //print_u16(" RAM_06              ", info.ram_06                );
    //print_u16(" RAM_07              ", info.ram_07                );
    //print_u16(" RAM_08              ", info.ram_08                );
    //print_u16(" RAM_09              ", info.ram_09                );
    //print_u16(" RAM_10              ", info.ram_10                );
    //delay.delay_ms(100u16);
//
    //print_u16(" RAM_11              ", info.ram_11                );
    //print_u16(" RAM_12              ", info.ram_12                );
    //print_u16(" RAM_13              ", info.ram_13                );
    //print_u16(" RAM_14              ", info.ram_14                );
    //print_u16(" RAM_15              ", info.ram_15                );
    //print_u16(" RAM_16              ", info.ram_16                );
    //print_u16(" RAM_17              ", info.ram_17                );
    //print_u16(" RAM_18              ", info.ram_18                );
    //print_u16(" RAM_19              ", info.ram_19                );
    //print_u16(" RAM_20              ", info.ram_20                );
    //delay.delay_ms(100u16);
//
    //print_u16(" RAM_21              ", info.ram_21                );
    //print_u16(" RAM_22              ", info.ram_22                );
    //print_u16(" RAM_23              ", info.ram_23                );
    //print_u16(" RAM_24              ", info.ram_24                );
    //print_u16(" RAM_25              ", info.ram_25                );
    //print_u16(" RAM_26              ", info.ram_26                );
    //print_u16(" RAM_27              ", info.ram_27                );
    //print_u16(" RAM_28              ", info.ram_28                );
    //print_u16(" RAM_29              ", info.ram_29                );
    //print_u16(" RAM_30              ", info.ram_30                );
    //delay.delay_ms(100u16);
//
    //print_u16(" RAM_31              ", info.ram_31                );
    //print_u16(" RAM_32              ", info.ram_32                );
    //print_u16(" RAM_33              ", info.ram_33                );
    //print_u16(" RAM_34              ", info.ram_34                );
    //print_u16(" RAM_35              ", info.ram_35                );
    //print_u16(" RAM_36              ", info.ram_36                );
    //print_u16(" RAM_37              ", info.ram_37                );
    //print_u16(" RAM_38              ", info.ram_38                );
    //print_u16(" RAM_39              ", info.ram_39                );
    //delay.delay_ms(100u16);
//
    //print_u16(" RAM_40              ", info.ram_40                );
    //print_u16(" RAM_41              ", info.ram_41                );
    //print_u16(" RAM_42              ", info.ram_42                );
    //print_u16(" RAM_43              ", info.ram_43                );
    //print_u16(" RAM_44              ", info.ram_44                );
    //print_u16(" RAM_45              ", info.ram_45                );
    //print_u16(" RAM_46              ", info.ram_46                );
    //print_u16(" RAM_47              ", info.ram_47                );
    //print_u16(" RAM_48              ", info.ram_48                );
    //print_u16(" RAM_49              ", info.ram_49                );
    //print_u16(" RAM_50              ", info.ram_50                );
    //delay.delay_ms(100u16);
//
    //print_u16(" RAM_51              ", info.ram_51                );
    //print_u16(" RAM_52              ", info.ram_52                );
    //print_u16(" RAM_53              ", info.ram_53                );
    //print_u16(" RAM_54              ", info.ram_54                );
    //print_u16(" RAM_55              ", info.ram_55                );
    //print_u16(" RAM_56              ", info.ram_56                );
    //print_u16(" RAM_57              ", info.ram_57                );
    //print_u16(" RAM_58              ", info.ram_58                );
    //print_u16(" RAM_59              ", info.ram_59                );
    //print_u16(" RAM_60              ", info.ram_60                );

    /*  **************************************************************** */
    /*  **************************************************************** */
    /*  **************************************************************** */
    rprintln!("==== shared_bus =========== ");

    let i2c_address_1 = Mlx90632Address::mlx90632_default_addr(AddrPin::VDD);

    let i2c_bus = shared_bus::BusManagerSimple::new(i2c);
    //let interface = I2CDisplayInterface::new(i2c_bus.acquire_i2c());    //  <<<<<<<<<<<<<<  Sharing of the I2C bus!!!!

    //  Init sensor 1
    let i2c_proxy_1 = i2c_bus.acquire_i2c();
    let mut mlx90632_1 =
        Mlx90632::new(  i2c_proxy_1,
                        i2c_address_1,
                        Hertz::from(MLX90632_BAUD).0);
    match mlx90632_1 {
        Ok(_) => {
            rprintln!("=====        init Ok  ==== ");
        }
        Err(ref e) => {
            rprintln!("=====        init Err {:?} ==== ", e);
        }
    }
    let mut mlx90632_1 = mlx90632_1.unwrap();


    //  Simulate sensor 2
    mlx90632_addr_pin.set_low().unwrap();

    //  Init sensor 2
    let i2c_address_2 = Mlx90632Address::mlx90632_default_addr(AddrPin::GND);

    let i2c_proxy_2 = i2c_bus.acquire_i2c();
    let mut mlx90632_2 =
            Mlx90632::new(  i2c_proxy_2,
                            i2c_address_2,
                            Hertz::from(MLX90632_BAUD).0);


    match mlx90632_2 {
        Ok(_) => {
            rprintln!("=====        init Ok  ==== ");
        }
        Err(ref e) => {
            rprintln!("=====        init Err {:?} ==== ", e);
        }
    }
    let mut mlx90632_2 = mlx90632_2.unwrap();


    // Back to sensor 1
    mlx90632_addr_pin.set_high().unwrap();

    rprintln!("==== get_temperature =========== ");




    match mlx90632_1.get_temparature_extended_mode(&mut mlx_delay) {
        Ok(x) => {
            rprintln!("  ===== Temp is {:?} ", x);
        },
        Err(x) => {
            rprintln!("  ===== Err getting temp ERR ({:?}) ", x);
        }
    }

    let mut counter : u32 = 0;

    loop {
        if (counter%1000000) == 0 {
            let temp_1 = mlx90632_1.get_temparature(&mut mlx_delay);
            let temp_2 = mlx90632_2.get_temparature(&mut mlx_delay);
            match (temp_1, temp_2) {
                (Ok(t1), Ok(t2)) => {
                    rprintln!("  {} ===== Temp1 {:?} :::  Temp2 {:?}", counter, t1, t2);
                },
                //Err(x) => {
                (_e1, _e2) => {
                    rprintln!("  {} =====  t1 ({:?}) ::::::::: t2 ({:?}) ", counter, _e1, _e2);
                }
            }
        }
        counter += 1;

        if counter == 15000000{
            /*  Error simulation:   Change the sensor address by changing its addr pin...  */
            mlx90632_addr_pin.set_low().unwrap();
        }
    }

}


#[allow(dead_code)]
fn print_u16(label: &str, val: u16) {
    rprintln!(" {}    : {:#02x} ", label, val);
}

#[allow(dead_code)]
fn print_u32(label: &str, val: u32) {
    rprintln!(" {}    : {:#04x} ", label, val);
}

#[allow(dead_code)]
fn print_i16(label: &str, val: i16) {
    rprintln!(" {}    : {:#02x} ", label, val);
}

#[allow(dead_code)]
fn print_i32(label: &str, val: i32) {
    rprintln!(" {}    : {:#04x} ", label, val);
}

#[allow(dead_code)]
fn testing_alternate_address(  addr:u8, pin: AddrPin) -> () {
    let i2c_address =
        Mlx90632Address::alternate_addr::<Mlx90632Error<()>>(addr, pin);
    match i2c_address {
        Ok(x) => {
            rprintln!("===== i2c_address OK({:?}) ", x);
        },
        Err(x) => {
            rprintln!("===== i2c_address ERR({:?}) ", x);
        }
    }
}

