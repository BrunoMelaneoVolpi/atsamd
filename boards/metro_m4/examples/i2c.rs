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




    let i2c_address = Mlx90632Address::mlx90632_default_addr(AddrPin::GND);

    rprintln!("===== Hertz::from(MLX90632_BAUD).0 :: {:?} ", Hertz::from(MLX90632_BAUD).0);

    let mut mlx90632 =
        Mlx90632::new(  i2c,
                        i2c_address,
                        Hertz::from(MLX90632_BAUD).0,
                        ).unwrap();

    //mlx90632.debug_write_read();
    let info = mlx90632.mlx90632_get_chip_info().unwrap();
        rprintln!("===== info ====");

        print_u16(" id0                 ", info.id0);
        print_u16(" id1                 ", info.id1            );
        print_u16(" id2                 ", info.id2            );
        print_u16(" id_crc              ", info.id_crc         );
        print_u16(" ee_product_code     ", info.ee_product_code);
        print_u16(" ee_version          ", info.ee_version     );
        print_u32(" EE_P_R              ", info.EE_P_R  );
        print_u32(" EE_P_G              ", info.EE_P_G  );
        print_u32(" EE_P_T              ", info.EE_P_T  );
        print_u32(" EE_P_O              ", info.EE_P_O  );
        print_u32(" EE_AA               ", info.EE_AA   );
        print_u32(" EE_AB               ", info.EE_AB   );
        print_u32(" EE_BA               ", info.EE_BA   );
        print_u32(" EE_BB               ", info.EE_BB   );
        print_u32(" EE_CA               ", info.EE_CA   );
        print_u32(" EE_CB               ", info.EE_CB   );
        print_u32(" EE_DA               ", info.EE_DA   );
        print_u32(" EE_DB               ", info.EE_DB   );
        print_u32(" EE_EA               ", info.EE_EA   );
        print_u32(" EE_EB               ", info.EE_EB   );
        print_u32(" EE_FA               ", info.EE_FA   );
        print_u32(" EE_FB               ", info.EE_FB   );
        print_u32(" EE_GA               ", info.EE_GA   );

        rprintln!(" ------------- ");
        //print_u16(" EE_GA  signed         : {} ", (info.EE_GA) as i32);
        rprintln!(" ------------- ");

        delay.delay_ms(100u16);

        print_u16(" EE_GB               ", info.EE_GB   );
        print_u16(" EE_KA               ", info.EE_KA   );
        print_u16(" EE_KB               ", info.EE_KB   );
        print_u16(" EE_HA               ", info.EE_HA   );
        print_u16(" EE_HB               ", info.EE_HB   );

        delay.delay_ms(100u16);



        print_u16(" customer_data0      ", info.customer_data0   );
        print_u16(" customer_data1      ", info.customer_data1   );
        print_u16(" customer_data2      ", info.customer_data2   );
        print_u16(" customer_data3      ", info.customer_data3   );
        print_u16(" customer_data4      ", info.customer_data4   );
        print_u16(" customer_data5      ", info.customer_data5   );
        print_u16(" customer_data6      ", info.customer_data6   );
        print_u16(" customer_data7      ", info.customer_data7   );

        delay.delay_ms(100u16);



        print_u16(" EE_CONTROL          ", info.EE_CONTROL            );
        print_u16(" EE_I2C_ADDRESS      ", info.EE_I2C_ADDRESS        );
        print_u16(" EE_MEAS_1           ", info.EE_MEAS_1             );
        print_u16(" EE_MEAS_2           ", info.EE_MEAS_2             );
        print_u16(" REG_I2C_ADDRESS     ", info.REG_I2C_ADDRESS       );
        print_u16(" REG_CONTROL         ", info.REG_CONTROL           );
        print_u16(" REG_STATUS          ", info.REG_STATUS            );
        delay.delay_ms(100u16);


        print_u16(" RAM_01              ", info.RAM_01                );
        print_u16(" RAM_02              ", info.RAM_02                );
        print_u16(" RAM_03              ", info.RAM_03                );
        print_u16(" RAM_04              ", info.RAM_04                );
        print_u16(" RAM_05              ", info.RAM_05                );
        print_u16(" RAM_06              ", info.RAM_06                );
        print_u16(" RAM_07              ", info.RAM_07                );
        print_u16(" RAM_08              ", info.RAM_08                );
        print_u16(" RAM_09              ", info.RAM_09                );
        print_u16(" RAM_10              ", info.RAM_10                );
        delay.delay_ms(100u16);


        print_u16(" RAM_11              ", info.RAM_11                );
        print_u16(" RAM_12              ", info.RAM_12                );
        print_u16(" RAM_13              ", info.RAM_13                );
        print_u16(" RAM_14              ", info.RAM_14                );
        print_u16(" RAM_15              ", info.RAM_15                );
        print_u16(" RAM_16              ", info.RAM_16                );
        print_u16(" RAM_17              ", info.RAM_17                );
        print_u16(" RAM_18              ", info.RAM_18                );
        print_u16(" RAM_19              ", info.RAM_19                );
        print_u16(" RAM_20              ", info.RAM_20                );
        delay.delay_ms(100u16);


        print_u16(" RAM_21              ", info.RAM_21                );
        print_u16(" RAM_22              ", info.RAM_22                );
        print_u16(" RAM_23              ", info.RAM_23                );
        print_u16(" RAM_24              ", info.RAM_24                );
        print_u16(" RAM_25              ", info.RAM_25                );
        print_u16(" RAM_26              ", info.RAM_26                );
        print_u16(" RAM_27              ", info.RAM_27                );
        print_u16(" RAM_28              ", info.RAM_28                );
        print_u16(" RAM_29              ", info.RAM_29                );
        print_u16(" RAM_30              ", info.RAM_30                );
        delay.delay_ms(100u16);


        print_u16(" RAM_31              ", info.RAM_31                );
        print_u16(" RAM_32              ", info.RAM_32                );
        print_u16(" RAM_33              ", info.RAM_33                );
        print_u16(" RAM_34              ", info.RAM_34                );
        print_u16(" RAM_35              ", info.RAM_35                );
        print_u16(" RAM_36              ", info.RAM_36                );
        print_u16(" RAM_37              ", info.RAM_37                );
        print_u16(" RAM_38              ", info.RAM_38                );
        print_u16(" RAM_39              ", info.RAM_39                );
        delay.delay_ms(100u16);


        print_u16(" RAM_40              ", info.RAM_40                );
        print_u16(" RAM_41              ", info.RAM_41                );
        print_u16(" RAM_42              ", info.RAM_42                );
        print_u16(" RAM_43              ", info.RAM_43                );
        print_u16(" RAM_44              ", info.RAM_44                );
        print_u16(" RAM_45              ", info.RAM_45                );
        print_u16(" RAM_46              ", info.RAM_46                );
        print_u16(" RAM_47              ", info.RAM_47                );
        print_u16(" RAM_48              ", info.RAM_48                );
        print_u16(" RAM_49              ", info.RAM_49                );
        print_u16(" RAM_50              ", info.RAM_50                );
        delay.delay_ms(100u16);


        print_u16(" RAM_51              ", info.RAM_51                );
        print_u16(" RAM_52              ", info.RAM_52                );
        print_u16(" RAM_53              ", info.RAM_53                );
        print_u16(" RAM_54              ", info.RAM_54                );
        print_u16(" RAM_55              ", info.RAM_55                );
        print_u16(" RAM_56              ", info.RAM_56                );
        print_u16(" RAM_57              ", info.RAM_57                );
        print_u16(" RAM_58              ", info.RAM_58                );
        print_u16(" RAM_59              ", info.RAM_59                );
        print_u16(" RAM_60              ", info.RAM_60                );


    rprintln!("==== mlx90632_start_measurement ===========");
    mlx90632.mlx90632_start_measurement();



    let mut counter : u32 = 0;
    loop {
        delay.delay_ms(1000u16);
        rprintln!("===== DEMO OVER ==== {}", counter);
        counter += 1;
    }
}


fn print_u16(label: &str, val: u16) {
    rprintln!(" {}    : {:#02x} ", label, val);
}

fn print_u32(label: &str, val: u32) {
    rprintln!(" {}    : {:#04x} ", label, val);
}