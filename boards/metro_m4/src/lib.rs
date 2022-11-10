#![no_std]
#![deny(missing_docs)]

//! Board support crate for Adafruit's Metro M4 Express,
//! an ATSAMD51-based board in an 'Arduino compatible'
//! shape and pinout

pub use atsamd_hal as hal;
pub use hal::ehal;
pub use hal::pac;

#[cfg(feature = "rt")]
pub use cortex_m_rt::entry;

use hal::{
    clock::GenericClockController,
    qspi::{OneShot, Qspi},
    sercom::{
        i2c, spi,
        uart::{self, BaudMode, Oversampling},
        IoSet1, IoSet6,
    },
    time::Hertz,
    gpio
};

use pac::MCLK;

#[cfg(feature = "usb")]
use hal::usb::{usb_device::bus::UsbBusAllocator, UsbBus};

//hal::bsp_peripherals!(
//    SERCOM2 { SpiSercom }
//    SERCOM3 { UartSercom }
//    SERCOM5 { I2cSercom }
//);
#[doc = "Alias for the [`SERCOM2`](atsamd_hal::pac::SERCOM2) peripheral"]
pub type SpiSercom = atsamd_hal::pac::SERCOM2;
#[doc = "Alias for the [`SERCOM3`](atsamd_hal::pac::SERCOM3) peripheral"]
pub type UartSercom = atsamd_hal::pac::SERCOM3;
#[doc = "Alias for the [`SERCOM5`](atsamd_hal::pac::SERCOM5) peripheral"]
pub type I2cSercom = atsamd_hal::pac::SERCOM5;
#[doc =
" Refer to fields of the [`Peripherals`](atsamd_hal::pac::Peripherals)"]
#[doc = " struct by alternate names"]
#[doc = ""]
#[doc = " This macro can be used to access fields of the `Peripherals`"]
#[doc = " struct by alternate names. The available aliases are:"]
#[doc = ""]
#[doc =
"    - [`SERCOM2`](atsamd_hal::pac::SERCOM2) \
                    can be refered to with the type alias [`SpiSercom`] and \
                    accessed as the field name `spi_sercom`\n    - [`SERCOM3`](atsamd_hal::pac::SERCOM3) \
                    can be refered to with the type alias [`UartSercom`] and \
                    accessed as the field name `uart_sercom`\n    - [`SERCOM5`](atsamd_hal::pac::SERCOM5) \
                    can be refered to with the type alias [`I2cSercom`] and \
                    accessed as the field name `i2c_sercom`\n"]
#[doc = ""]
#[doc = " For example. suppose `display_spi` were an alternate name for"]
#[doc = " the `SERCOM4` peripheral. You could use the `periph_alias!`"]
#[doc = " macro to access it like this:"]
#[doc = ""]
#[doc = " ```"]
#[doc = " let mut peripherals = pac::Peripherals::take().unwrap();"]
#[doc = " // Replace this"]
#[doc = " let display_spi = peripherals.SERCOM4;"]
#[doc = " // With this"]
#[doc = " let display_spi = periph_alias!(peripherals.display_spi);"]
#[doc = " ```"]
#[macro_export]
macro_rules! periph_alias {
    ($peripherals : ident.spi_sercom) =>
    {
        {
            macro_rules! peripheral_alias_spi_sercom
            { () => { $peripherals.SERCOM2 } ; } peripheral_alias_spi_sercom!
            ()
        }
    } ; ($peripherals : ident.uart_sercom) =>
    {
        {
            macro_rules! peripheral_alias_uart_sercom
            { () => { $peripherals.SERCOM3 } ; } peripheral_alias_uart_sercom!
            ()
        }
    } ; ($peripherals : ident.i2c_sercom) =>
    {
        {
            macro_rules! peripheral_alias_i2c_sercom
            { () => { $peripherals.SERCOM5 } ; } peripheral_alias_i2c_sercom!
            ()
        }
    } ;
}








//hal::bsp_pins!(
//    PA02 {
//        /// Analog pin 0.  Can act as a true analog output
//        /// as it has a DAC (which is not currently supported
//        /// by this hal) as well as input.
//        name: a0
//    }
//    PA05 {
//        /// Analog Pin 1
//        name: a1
//    }
//    PA06 {
//        /// Analog Pin 2
//        name: a2
//    }
//    PA04 {
//        /// Analog Pin 3
//        name: a3
//    }
//    PB08 {
//        /// Analog Pin 4
//        name: a4
//    }
//        /// Analog Pin 5
//    PB09 {
//        name: a5
//    }
//
//    PA23{
//        /// Pin 0, rx.
//        name: d0
//        aliases: {
//            AlternateC: UartRx
//        }
//    }
//    PA22{
//        /// Pin 1, tx.
//        name: d1
//        aliases: {
//            AlternateC: UartTx
//        }
//
//    }
//    PB17 {
//        /// Pin 2
//        name: d2
//    }
//    PB16 {
//        /// Pin 3
//        name: d3
//    }
//    PB13 {
//        /// Pin 4
//        name: d4
//    }
//    PB14 {
//        /// Pin 5
//        name: d5
//    }
//    PB15 {
//        /// Pin 6
//        name: d6
//    }
//    PB12 {
//        /// Pin 7
//        name: d7
//    }
//    PA21 {
//        /// Pin 8
//        name: d8
//    }
//    PA20 {
//        /// Pin 9
//        name: d9
//    }
//    PA18 {
//        /// Pin 10
//        name: d10
//    }
//    PA19 {
//        /// Pin 11
//        name: d11
//    }
//    PA17 {
//        /// Pin 12
//        name: d12
//    }
//    PA16 {
//        /// Digital pin number 13, which is also attached to
//        /// the red LED.  PWM capable.
//        name: d13
//        aliases: {
//            PushPullOutput: RedLed
//        }
//    }
//    PB02 {
//        /// The I2C data line
//        name: sda
//        aliases: {
//            AlternateD: Sda
//        }
//    }
//    PB03 {
//        /// The I2C clock line
//        name: scl
//        aliases: {
//            AlternateD: Scl
//        }
//    }
//
//    PB22 {
//        /// The data line attached to the neopixel.
//        /// Is also attached to SWCLK.
//        name: neopixel
//    }
//
//    PA13 {
//        /// The SPI SCLK attached the to 2x3 header
//        name: sclk
//        aliases: {
//            AlternateC: Sclk
//        }
//    }
//    PA12 {
//        /// The SPI MOSI attached the to 2x3 header
//        name: mosi
//        aliases: {
//            AlternateC: Mosi
//        }
//    }
//    PA14 {
//        /// The SPI MISO attached the to 2x3 header
//        name: miso
//        aliases: {
//            AlternateC: Miso
//        }
//    }
//
//    PB10 {
//        /// The SCK pin attached to the on-board SPI flash
//        name: flash_sclk
//        aliases: {
//            AlternateH: FlashSclk
//        }
//    }
//    PB11 {
//        /// The CS pin attached to the on-board SPI flash
//        name: flash_cs
//        aliases: {
//            AlternateH: FlashCs
//        }
//    }
//    PA08 {
//        /// The D0 pin attached to the on-board SPI flash
//        name: flash_d0
//        aliases: {
//            AlternateH: FlashD0
//        }
//    }
//    PA09 {
//        /// The D1 pin attached to the on-board SPI flash
//        name: flash_d1
//        aliases: {
//            AlternateH: FlashD1
//        }
//    }
//    PA10 {
//        /// The D1 pin attached to the on-board SPI flash
//        name: flash_d2
//        aliases: {
//            AlternateH: FlashD2
//        }
//    }
//    PA11 {
//        /// The D1 pin attached to the on-board SPI flash
//        name: flash_d3
//        aliases: {
//            AlternateH: FlashD3
//        }
//    }
//
//    PA24 {
//        /// The USB D- pad
//        name: usb_dm
//        aliases: {
//            AlternateG: UsbDm
//        }
//
//    }
//    PA25 {
//        /// The USB D+ pad
//        name: usb_dp
//        aliases: {
//            AlternateG: UsbDp
//        }
//
//    }
//);


/// BSP replacement for the HAL
/// [`Pins`](atsamd_hal::gpio::Pins) type
///
/// This type is intended to provide more meaningful names for the
/// given pins.
pub struct Pins {
    port: Option<crate::pac::PORT>,
    #[



    doc = r" Analog pin 0.  Can act as a true analog output"]
    #[doc = r" as it has a DAC (which is not currently supported"]
    #[doc = r" by this hal) as well as input."]
    pub a0: crate::gpio::Pin<crate::gpio::PA02, crate::gpio::Reset>,
    #[doc = r" Analog Pin 1"]
    pub a1: crate::gpio::Pin<crate::gpio::PA05, crate::gpio::Reset>,
    #[doc = r" Analog Pin 2"]
    pub a2: crate::gpio::Pin<crate::gpio::PA06, crate::gpio::Reset>,
    #[doc = r" Analog Pin 3"]
    pub a3: crate::gpio::Pin<crate::gpio::PA04, crate::gpio::Reset>,
    #[doc = r" Analog Pin 4"]
    pub a4: crate::gpio::Pin<crate::gpio::PB08, crate::gpio::Reset>,
    #[doc = r" Analog Pin 5"]
    pub a5: crate::gpio::Pin<crate::gpio::PB09, crate::gpio::Reset>,
    #[

    doc = r" Pin 0, rx."]
    #[doc =
    "\nThis field can also be accessed using the [`pin_alias!`] \
                                macro with the following alternate names:\n    "]
    #[doc = "uart_rx, "]
    pub d0: crate::gpio::Pin<crate::gpio::PA23, crate::gpio::Reset>,
    #[doc = r" Pin 1, tx."]
    #[doc =
    "\nThis field can also be accessed using the [`pin_alias!`] \
                                macro with the following alternate names:\n    "]
    #[doc = "uart_tx, "]
    pub d1: crate::gpio::Pin<crate::gpio::PA22, crate::gpio::Reset>,
    #[

    doc = r" Pin 2"]
    pub d2: crate::gpio::Pin<crate::gpio::PB17, crate::gpio::Reset>,
    #[doc = r" Pin 3"]
    pub d3: crate::gpio::Pin<crate::gpio::PB16, crate::gpio::Reset>,
    #[doc = r" Pin 4"]
    pub d4: crate::gpio::Pin<crate::gpio::PB13, crate::gpio::Reset>,
    #[doc = r" Pin 5"]
    pub d5: crate::gpio::Pin<crate::gpio::PB14, crate::gpio::Reset>,
    #[doc = r" Pin 6"]
    pub d6: crate::gpio::Pin<crate::gpio::PB15, crate::gpio::Reset>,
    #[doc = r" Pin 7"]
    pub d7: crate::gpio::Pin<crate::gpio::PB12, crate::gpio::Reset>,
    #[doc = r" Pin 8"]
    pub d8: crate::gpio::Pin<crate::gpio::PA21, crate::gpio::Reset>,
    #[doc = r" Pin 9"]
    pub d9: crate::gpio::Pin<crate::gpio::PA20, crate::gpio::Reset>,
    #[doc = r" Pin 10"]
    pub d10: crate::gpio::Pin<crate::gpio::PA18, crate::gpio::Reset>,
    #[doc = r" Pin 11"]
    pub d11: crate::gpio::Pin<crate::gpio::PA19, crate::gpio::Reset>,
    #[doc = r" Pin 12"]
    pub d12: crate::gpio::Pin<crate::gpio::PA17, crate::gpio::Reset>,
    #[doc = r" Digital pin number 13, which is also attached to"]
    #[doc = r" the red LED.  PWM capable."]
    #[doc =
    "\nThis field can also be accessed using the [`pin_alias!`] \
                                macro with the following alternate names:\n    "]
    #[doc = "red_led, "]
    pub d13: crate::gpio::Pin<crate::gpio::PA16, crate::gpio::Reset>,
    #[doc = r" The I2C data line"]
    #[doc =
    "\nThis field can also be accessed using the [`pin_alias!`] \
                                macro with the following alternate names:\n    "]
    #[doc = "sda, "]
    pub sda: crate::gpio::Pin<crate::gpio::PB02, crate::gpio::Reset>,
    #[doc = r" The I2C clock line"]
    #[doc =
    "\nThis field can also be accessed using the [`pin_alias!`] \
                                macro with the following alternate names:\n    "]
    #[doc = "scl, "]
    pub scl: crate::gpio::Pin<crate::gpio::PB03, crate::gpio::Reset>,
    #[

    doc = r" The data line attached to the neopixel."]
    #[doc = r" Is also attached to SWCLK."]
    pub neopixel: crate::gpio::Pin<crate::gpio::PB22, crate::gpio::Reset>,
    #[

    doc = r" The SPI SCLK attached the to 2x3 header"]
    #[doc =
    "\nThis field can also be accessed using the [`pin_alias!`] \
                                macro with the following alternate names:\n    "]
    #[doc = "sclk, "]
    pub sclk: crate::gpio::Pin<crate::gpio::PA13, crate::gpio::Reset>,
    #[doc = r" The SPI MOSI attached the to 2x3 header"]
    #[doc =
    "\nThis field can also be accessed using the [`pin_alias!`] \
                                macro with the following alternate names:\n    "]
    #[doc = "mosi, "]
    pub mosi: crate::gpio::Pin<crate::gpio::PA12, crate::gpio::Reset>,
    #[doc = r" The SPI MISO attached the to 2x3 header"]
    #[doc =
    "\nThis field can also be accessed using the [`pin_alias!`] \
                                macro with the following alternate names:\n    "]
    #[doc = "miso, "]
    pub miso: crate::gpio::Pin<crate::gpio::PA14, crate::gpio::Reset>,
    #[

    doc = r" The SCK pin attached to the on-board SPI flash"]
    #[doc =
    "\nThis field can also be accessed using the [`pin_alias!`] \
                                macro with the following alternate names:\n    "]
    #[doc = "flash_sclk, "]
    pub flash_sclk: crate::gpio::Pin<crate::gpio::PB10, crate::gpio::Reset>,
    #[doc = r" The CS pin attached to the on-board SPI flash"]
    #[doc =
    "\nThis field can also be accessed using the [`pin_alias!`] \
                                macro with the following alternate names:\n    "]
    #[doc = "flash_cs, "]
    pub flash_cs: crate::gpio::Pin<crate::gpio::PB11, crate::gpio::Reset>,
    #[doc = r" The D0 pin attached to the on-board SPI flash"]
    #[doc =
    "\nThis field can also be accessed using the [`pin_alias!`] \
                                macro with the following alternate names:\n    "]
    #[doc = "flash_d0, "]
    pub flash_d0: crate::gpio::Pin<crate::gpio::PA08, crate::gpio::Reset>,
    #[doc = r" The D1 pin attached to the on-board SPI flash"]
    #[doc =
    "\nThis field can also be accessed using the [`pin_alias!`] \
                                macro with the following alternate names:\n    "]
    #[doc = "flash_d1, "]
    pub flash_d1: crate::gpio::Pin<crate::gpio::PA09, crate::gpio::Reset>,
    #[doc = r" The D1 pin attached to the on-board SPI flash"]
    #[doc =
    "\nThis field can also be accessed using the [`pin_alias!`] \
                                macro with the following alternate names:\n    "]
    #[doc = "flash_d2, "]
    pub flash_d2: crate::gpio::Pin<crate::gpio::PA10, crate::gpio::Reset>,
    #[doc = r" The D1 pin attached to the on-board SPI flash"]
    #[doc =
    "\nThis field can also be accessed using the [`pin_alias!`] \
                                macro with the following alternate names:\n    "]
    #[doc = "flash_d3, "]
    pub flash_d3: crate::gpio::Pin<crate::gpio::PA11, crate::gpio::Reset>,
    #[

    doc = r" The USB D- pad"]
    #[doc =
    "\nThis field can also be accessed using the [`pin_alias!`] \
                                macro with the following alternate names:\n    "]
    #[doc = "usb_dm, "]
    pub usb_dm: crate::gpio::Pin<crate::gpio::PA24, crate::gpio::Reset>,
    #[

    doc = r" The USB D+ pad"]
    #[doc =
    "\nThis field can also be accessed using the [`pin_alias!`] \
                                macro with the following alternate names:\n    "]
    #[doc = "usb_dp, "]
    pub usb_dp: crate::gpio::Pin<crate::gpio::PA25, crate::gpio::Reset>,
}
impl Pins {
    /// Take ownership of the PAC [`PORT`] and split it into
    /// discrete [`Pin`]s.
    ///
    /// This struct serves as a replacement for the HAL [`Pins`]
    /// struct. It is intended to provide more meaningful names for
    /// each [`Pin`] in a BSP. Any [`Pin`] not defined by the BSP is
    /// dropped.
    ///
    /// [`PORT`](atsamd_hal::pac::PORT)
    /// [`Pin`](atsamd_hal::gpio::Pin)
    /// [`Pins`](atsamd_hal::gpio::Pins)
    #[inline]
    pub fn new(port: crate::pac::PORT) -> Self {
        let mut pins = crate::gpio::Pins::new(port);
        Self {
            port: Some(unsafe { pins.port() }),
            a0: pins.pa02,
            a1: pins.pa05,
            a2: pins.pa06,
            a3: pins.pa04,
            a4: pins.pb08,

            #[doc = r" Analog Pin 5"]
            a5: pins.pb09,
            d0: pins.pa23,
            d1: pins.pa22,
            d2: pins.pb17,
            d3: pins.pb16,
            d4: pins.pb13,
            d5: pins.pb14,
            d6: pins.pb15,
            d7: pins.pb12,
            d8: pins.pa21,
            d9: pins.pa20,
            d10: pins.pa18,
            d11: pins.pa19,
            d12: pins.pa17,
            d13: pins.pa16,
            sda: pins.pb02,
            scl: pins.pb03,
            neopixel: pins.pb22,
            sclk: pins.pa13,
            mosi: pins.pa12,
            miso: pins.pa14,
            flash_sclk: pins.pb10,
            flash_cs: pins.pb11,
            flash_d0: pins.pa08,
            flash_d1: pins.pa09,
            flash_d2: pins.pa10,
            flash_d3: pins.pa11,
            usb_dm: pins.pa24,
            usb_dp: pins.pa25,
        }
    }
    /// Take the PAC [`PORT`]
    ///
    /// The [`PORT`] can only be taken once. Subsequent calls to
    /// this function will panic.
    ///
    /// # Safety
    ///
    /// Direct access to the [`PORT`] could allow you to invalidate
    /// the compiler's type-level tracking, so it is unsafe.
    ///
    /// [`PORT`](atsamd_hal::pac::PORT)
    #[inline]
    pub unsafe fn port(&mut self) -> crate::pac::PORT {
        self.port.take().unwrap()
    }
}
#[doc = " Alias for a configured [`Pin`](atsamd_hal::gpio::Pin)"]
pub type UartRx =
    crate::gpio::Pin<crate::gpio::PA23, crate::gpio::AlternateC>;
#[doc = "[`PinId`](atsamd_hal::gpio::PinId) for the [`"]
#[doc = "UartRx`] alias"]
pub type UartRxId = crate::gpio::PA23;
#[doc = "[`PinMode`](atsamd_hal::gpio::PinMode) for the [`"]
#[doc = "UartRx`] alias"]
pub type UartRxMode = crate::gpio::AlternateC;
#[doc = "[DynPinId](atsamd_hal::gpio::DynPinId) "]
#[doc = "for the `UartRx` alias."]
pub const UART_RX_ID: crate::gpio::DynPinId =
    <crate::gpio::PA23 as crate::gpio::PinId>::DYN;
#[doc = "[DynPinMode](atsamd_hal::gpio::DynPinMode) "]
#[doc = "for the `UartRx` alias."]
pub const UART_RX_MODE: crate::gpio::DynPinMode =
    <crate::gpio::AlternateC as crate::gpio::PinMode>::DYN;
#[doc = " Alias for a configured [`Pin`](atsamd_hal::gpio::Pin)"]
pub type UartTx =
    crate::gpio::Pin<crate::gpio::PA22, crate::gpio::AlternateC>;
#[doc = "[`PinId`](atsamd_hal::gpio::PinId) for the [`"]
#[doc = "UartTx`] alias"]
pub type UartTxId = crate::gpio::PA22;
#[doc = "[`PinMode`](atsamd_hal::gpio::PinMode) for the [`"]
#[doc = "UartTx`] alias"]
pub type UartTxMode = crate::gpio::AlternateC;
#[doc = "[DynPinId](atsamd_hal::gpio::DynPinId) "]
#[doc = "for the `UartTx` alias."]
pub const UART_TX_ID: crate::gpio::DynPinId =
    <crate::gpio::PA22 as crate::gpio::PinId>::DYN;
#[doc = "[DynPinMode](atsamd_hal::gpio::DynPinMode) "]
#[doc = "for the `UartTx` alias."]
pub const UART_TX_MODE: crate::gpio::DynPinMode =
    <crate::gpio::AlternateC as crate::gpio::PinMode>::DYN;
#[doc = " Alias for a configured [`Pin`](atsamd_hal::gpio::Pin)"]
pub type RedLed =
    crate::gpio::Pin<crate::gpio::PA16, crate::gpio::PushPullOutput>;
#[doc = "[`PinId`](atsamd_hal::gpio::PinId) for the [`"]
#[doc = "RedLed`] alias"]
pub type RedLedId = crate::gpio::PA16;
#[doc = "[`PinMode`](atsamd_hal::gpio::PinMode) for the [`"]
#[doc = "RedLed`] alias"]
pub type RedLedMode = crate::gpio::PushPullOutput;
#[doc = "[DynPinId](atsamd_hal::gpio::DynPinId) "]
#[doc = "for the `RedLed` alias."]
pub const RED_LED_ID: crate::gpio::DynPinId =
    <crate::gpio::PA16 as crate::gpio::PinId>::DYN;
#[doc = "[DynPinMode](atsamd_hal::gpio::DynPinMode) "]
#[doc = "for the `RedLed` alias."]
pub const RED_LED_MODE: crate::gpio::DynPinMode =
    <crate::gpio::PushPullOutput as crate::gpio::PinMode>::DYN;
#[doc = " Alias for a configured [`Pin`](atsamd_hal::gpio::Pin)"]
pub type Sda = crate::gpio::Pin<crate::gpio::PB02, crate::gpio::AlternateD>;
#[doc = "[`PinId`](atsamd_hal::gpio::PinId) for the [`"]
#[doc = "Sda`] alias"]
pub type SdaId = crate::gpio::PB02;
#[doc = "[`PinMode`](atsamd_hal::gpio::PinMode) for the [`"]
#[doc = "Sda`] alias"]
pub type SdaMode = crate::gpio::AlternateD;
#[doc = "[DynPinId](atsamd_hal::gpio::DynPinId) "]
#[doc = "for the `Sda` alias."]
pub const SDA_ID: crate::gpio::DynPinId =
    <crate::gpio::PB02 as crate::gpio::PinId>::DYN;
#[doc = "[DynPinMode](atsamd_hal::gpio::DynPinMode) "]
#[doc = "for the `Sda` alias."]
pub const SDA_MODE: crate::gpio::DynPinMode =
    <crate::gpio::AlternateD as crate::gpio::PinMode>::DYN;
#[doc = " Alias for a configured [`Pin`](atsamd_hal::gpio::Pin)"]
pub type Scl = crate::gpio::Pin<crate::gpio::PB03, crate::gpio::AlternateD>;
#[doc = "[`PinId`](atsamd_hal::gpio::PinId) for the [`"]
#[doc = "Scl`] alias"]
pub type SclId = crate::gpio::PB03;
#[doc = "[`PinMode`](atsamd_hal::gpio::PinMode) for the [`"]
#[doc = "Scl`] alias"]
pub type SclMode = crate::gpio::AlternateD;
#[doc = "[DynPinId](atsamd_hal::gpio::DynPinId) "]
#[doc = "for the `Scl` alias."]
pub const SCL_ID: crate::gpio::DynPinId =
    <crate::gpio::PB03 as crate::gpio::PinId>::DYN;
#[doc = "[DynPinMode](atsamd_hal::gpio::DynPinMode) "]
#[doc = "for the `Scl` alias."]
pub const SCL_MODE: crate::gpio::DynPinMode =
    <crate::gpio::AlternateD as crate::gpio::PinMode>::DYN;
#[doc = " Alias for a configured [`Pin`](atsamd_hal::gpio::Pin)"]
pub type Sclk = crate::gpio::Pin<crate::gpio::PA13, crate::gpio::AlternateC>;
#[doc = "[`PinId`](atsamd_hal::gpio::PinId) for the [`"]
#[doc = "Sclk`] alias"]
pub type SclkId = crate::gpio::PA13;
#[doc = "[`PinMode`](atsamd_hal::gpio::PinMode) for the [`"]
#[doc = "Sclk`] alias"]
pub type SclkMode = crate::gpio::AlternateC;
#[doc = "[DynPinId](atsamd_hal::gpio::DynPinId) "]
#[doc = "for the `Sclk` alias."]
pub const SCLK_ID: crate::gpio::DynPinId =
    <crate::gpio::PA13 as crate::gpio::PinId>::DYN;
#[doc = "[DynPinMode](atsamd_hal::gpio::DynPinMode) "]
#[doc = "for the `Sclk` alias."]
pub const SCLK_MODE: crate::gpio::DynPinMode =
    <crate::gpio::AlternateC as crate::gpio::PinMode>::DYN;
#[doc = " Alias for a configured [`Pin`](atsamd_hal::gpio::Pin)"]
pub type Mosi = crate::gpio::Pin<crate::gpio::PA12, crate::gpio::AlternateC>;
#[doc = "[`PinId`](atsamd_hal::gpio::PinId) for the [`"]
#[doc = "Mosi`] alias"]
pub type MosiId = crate::gpio::PA12;
#[doc = "[`PinMode`](atsamd_hal::gpio::PinMode) for the [`"]
#[doc = "Mosi`] alias"]
pub type MosiMode = crate::gpio::AlternateC;
#[doc = "[DynPinId](atsamd_hal::gpio::DynPinId) "]
#[doc = "for the `Mosi` alias."]
pub const MOSI_ID: crate::gpio::DynPinId =
    <crate::gpio::PA12 as crate::gpio::PinId>::DYN;
#[doc = "[DynPinMode](atsamd_hal::gpio::DynPinMode) "]
#[doc = "for the `Mosi` alias."]
pub const MOSI_MODE: crate::gpio::DynPinMode =
    <crate::gpio::AlternateC as crate::gpio::PinMode>::DYN;
#[doc = " Alias for a configured [`Pin`](atsamd_hal::gpio::Pin)"]
pub type Miso = crate::gpio::Pin<crate::gpio::PA14, crate::gpio::AlternateC>;
#[doc = "[`PinId`](atsamd_hal::gpio::PinId) for the [`"]
#[doc = "Miso`] alias"]
pub type MisoId = crate::gpio::PA14;
#[doc = "[`PinMode`](atsamd_hal::gpio::PinMode) for the [`"]
#[doc = "Miso`] alias"]
pub type MisoMode = crate::gpio::AlternateC;
#[doc = "[DynPinId](atsamd_hal::gpio::DynPinId) "]
#[doc = "for the `Miso` alias."]
pub const MISO_ID: crate::gpio::DynPinId =
    <crate::gpio::PA14 as crate::gpio::PinId>::DYN;
#[doc = "[DynPinMode](atsamd_hal::gpio::DynPinMode) "]
#[doc = "for the `Miso` alias."]
pub const MISO_MODE: crate::gpio::DynPinMode =
    <crate::gpio::AlternateC as crate::gpio::PinMode>::DYN;
#[doc = " Alias for a configured [`Pin`](atsamd_hal::gpio::Pin)"]
pub type FlashSclk =
    crate::gpio::Pin<crate::gpio::PB10, crate::gpio::AlternateH>;
#[doc = "[`PinId`](atsamd_hal::gpio::PinId) for the [`"]
#[doc = "FlashSclk`] alias"]
pub type FlashSclkId = crate::gpio::PB10;
#[doc = "[`PinMode`](atsamd_hal::gpio::PinMode) for the [`"]
#[doc = "FlashSclk`] alias"]
pub type FlashSclkMode = crate::gpio::AlternateH;
#[doc = "[DynPinId](atsamd_hal::gpio::DynPinId) "]
#[doc = "for the `FlashSclk` alias."]
pub const FLASH_SCLK_ID: crate::gpio::DynPinId =
    <crate::gpio::PB10 as crate::gpio::PinId>::DYN;
#[doc = "[DynPinMode](atsamd_hal::gpio::DynPinMode) "]
#[doc = "for the `FlashSclk` alias."]
pub const FLASH_SCLK_MODE: crate::gpio::DynPinMode =
    <crate::gpio::AlternateH as crate::gpio::PinMode>::DYN;
#[doc = " Alias for a configured [`Pin`](atsamd_hal::gpio::Pin)"]
pub type FlashCs =
    crate::gpio::Pin<crate::gpio::PB11, crate::gpio::AlternateH>;
#[doc = "[`PinId`](atsamd_hal::gpio::PinId) for the [`"]
#[doc = "FlashCs`] alias"]
pub type FlashCsId = crate::gpio::PB11;
#[doc = "[`PinMode`](atsamd_hal::gpio::PinMode) for the [`"]
#[doc = "FlashCs`] alias"]
pub type FlashCsMode = crate::gpio::AlternateH;
#[doc = "[DynPinId](atsamd_hal::gpio::DynPinId) "]
#[doc = "for the `FlashCs` alias."]
pub const FLASH_CS_ID: crate::gpio::DynPinId =
    <crate::gpio::PB11 as crate::gpio::PinId>::DYN;
#[doc = "[DynPinMode](atsamd_hal::gpio::DynPinMode) "]
#[doc = "for the `FlashCs` alias."]
pub const FLASH_CS_MODE: crate::gpio::DynPinMode =
    <crate::gpio::AlternateH as crate::gpio::PinMode>::DYN;
#[doc = " Alias for a configured [`Pin`](atsamd_hal::gpio::Pin)"]
pub type FlashD0 =
    crate::gpio::Pin<crate::gpio::PA08, crate::gpio::AlternateH>;
#[doc = "[`PinId`](atsamd_hal::gpio::PinId) for the [`"]
#[doc = "FlashD0`] alias"]
pub type FlashD0Id = crate::gpio::PA08;
#[doc = "[`PinMode`](atsamd_hal::gpio::PinMode) for the [`"]
#[doc = "FlashD0`] alias"]
pub type FlashD0Mode = crate::gpio::AlternateH;
#[doc = "[DynPinId](atsamd_hal::gpio::DynPinId) "]
#[doc = "for the `FlashD0` alias."]
pub const FLASH_D0_ID: crate::gpio::DynPinId =
    <crate::gpio::PA08 as crate::gpio::PinId>::DYN;
#[doc = "[DynPinMode](atsamd_hal::gpio::DynPinMode) "]
#[doc = "for the `FlashD0` alias."]
pub const FLASH_D0_MODE: crate::gpio::DynPinMode =
    <crate::gpio::AlternateH as crate::gpio::PinMode>::DYN;
#[doc = " Alias for a configured [`Pin`](atsamd_hal::gpio::Pin)"]
pub type FlashD1 =
    crate::gpio::Pin<crate::gpio::PA09, crate::gpio::AlternateH>;
#[doc = "[`PinId`](atsamd_hal::gpio::PinId) for the [`"]
#[doc = "FlashD1`] alias"]
pub type FlashD1Id = crate::gpio::PA09;
#[doc = "[`PinMode`](atsamd_hal::gpio::PinMode) for the [`"]
#[doc = "FlashD1`] alias"]
pub type FlashD1Mode = crate::gpio::AlternateH;
#[doc = "[DynPinId](atsamd_hal::gpio::DynPinId) "]
#[doc = "for the `FlashD1` alias."]
pub const FLASH_D1_ID: crate::gpio::DynPinId =
    <crate::gpio::PA09 as crate::gpio::PinId>::DYN;
#[doc = "[DynPinMode](atsamd_hal::gpio::DynPinMode) "]
#[doc = "for the `FlashD1` alias."]
pub const FLASH_D1_MODE: crate::gpio::DynPinMode =
    <crate::gpio::AlternateH as crate::gpio::PinMode>::DYN;
#[doc = " Alias for a configured [`Pin`](atsamd_hal::gpio::Pin)"]
pub type FlashD2 =
    crate::gpio::Pin<crate::gpio::PA10, crate::gpio::AlternateH>;
#[doc = "[`PinId`](atsamd_hal::gpio::PinId) for the [`"]
#[doc = "FlashD2`] alias"]
pub type FlashD2Id = crate::gpio::PA10;
#[doc = "[`PinMode`](atsamd_hal::gpio::PinMode) for the [`"]
#[doc = "FlashD2`] alias"]
pub type FlashD2Mode = crate::gpio::AlternateH;
#[doc = "[DynPinId](atsamd_hal::gpio::DynPinId) "]
#[doc = "for the `FlashD2` alias."]
pub const FLASH_D2_ID: crate::gpio::DynPinId =
    <crate::gpio::PA10 as crate::gpio::PinId>::DYN;
#[doc = "[DynPinMode](atsamd_hal::gpio::DynPinMode) "]
#[doc = "for the `FlashD2` alias."]
pub const FLASH_D2_MODE: crate::gpio::DynPinMode =
    <crate::gpio::AlternateH as crate::gpio::PinMode>::DYN;
#[doc = " Alias for a configured [`Pin`](atsamd_hal::gpio::Pin)"]
pub type FlashD3 =
    crate::gpio::Pin<crate::gpio::PA11, crate::gpio::AlternateH>;
#[doc = "[`PinId`](atsamd_hal::gpio::PinId) for the [`"]
#[doc = "FlashD3`] alias"]
pub type FlashD3Id = crate::gpio::PA11;
#[doc = "[`PinMode`](atsamd_hal::gpio::PinMode) for the [`"]
#[doc = "FlashD3`] alias"]
pub type FlashD3Mode = crate::gpio::AlternateH;
#[doc = "[DynPinId](atsamd_hal::gpio::DynPinId) "]
#[doc = "for the `FlashD3` alias."]
pub const FLASH_D3_ID: crate::gpio::DynPinId =
    <crate::gpio::PA11 as crate::gpio::PinId>::DYN;
#[doc = "[DynPinMode](atsamd_hal::gpio::DynPinMode) "]
#[doc = "for the `FlashD3` alias."]
pub const FLASH_D3_MODE: crate::gpio::DynPinMode =
    <crate::gpio::AlternateH as crate::gpio::PinMode>::DYN;
#[doc = " Alias for a configured [`Pin`](atsamd_hal::gpio::Pin)"]
pub type UsbDm = crate::gpio::Pin<crate::gpio::PA24, crate::gpio::AlternateG>;
#[doc = "[`PinId`](atsamd_hal::gpio::PinId) for the [`"]
#[doc = "UsbDm`] alias"]
pub type UsbDmId = crate::gpio::PA24;
#[doc = "[`PinMode`](atsamd_hal::gpio::PinMode) for the [`"]
#[doc = "UsbDm`] alias"]
pub type UsbDmMode = crate::gpio::AlternateG;
#[doc = "[DynPinId](atsamd_hal::gpio::DynPinId) "]
#[doc = "for the `UsbDm` alias."]
pub const USB_DM_ID: crate::gpio::DynPinId =
    <crate::gpio::PA24 as crate::gpio::PinId>::DYN;
#[doc = "[DynPinMode](atsamd_hal::gpio::DynPinMode) "]
#[doc = "for the `UsbDm` alias."]
pub const USB_DM_MODE: crate::gpio::DynPinMode =
    <crate::gpio::AlternateG as crate::gpio::PinMode>::DYN;
#[doc = " Alias for a configured [`Pin`](atsamd_hal::gpio::Pin)"]
pub type UsbDp = crate::gpio::Pin<crate::gpio::PA25, crate::gpio::AlternateG>;
#[doc = "[`PinId`](atsamd_hal::gpio::PinId) for the [`"]
#[doc = "UsbDp`] alias"]
pub type UsbDpId = crate::gpio::PA25;
#[doc = "[`PinMode`](atsamd_hal::gpio::PinMode) for the [`"]
#[doc = "UsbDp`] alias"]
pub type UsbDpMode = crate::gpio::AlternateG;
#[doc = "[DynPinId](atsamd_hal::gpio::DynPinId) "]
#[doc = "for the `UsbDp` alias."]
pub const USB_DP_ID: crate::gpio::DynPinId =
    <crate::gpio::PA25 as crate::gpio::PinId>::DYN;
#[doc = "[DynPinMode](atsamd_hal::gpio::DynPinMode) "]
#[doc = "for the `UsbDp` alias."]
pub const USB_DP_MODE: crate::gpio::DynPinMode =
    <crate::gpio::AlternateG as crate::gpio::PinMode>::DYN;
#[doc = " Refer to fields of the [`Pins`] struct by alternate names"]
#[doc = ""]
#[doc = " This macro can be used to access fields of the [`Pins`] struct"]
#[doc = " by alternate names. See the `Pins` documentation for a list of"]
#[doc = " the availabe pin aliases."]
#[doc = ""]
#[doc = " For example. suppose `spi_mosi` were an alternate name for the"]
#[doc = " `serial_out` pin of the `Pins` struct. You could use the"]
#[doc = " `pin_alias!` macro to access it like this:"]
#[doc = ""]
#[doc = " ```"]
#[doc = " let mut peripherals = pac::Peripherals::take().unwrap();"]
#[doc = " let pins = bsp::Pins::new(peripherals.PORT);"]
#[doc = " // Replace this"]
#[doc = " let mosi = pins.serial_out;"]
#[doc = " // With this"]
#[doc = " let mosi = pin_alias!(pins.spi_mosi);"]
#[doc = " ```"]
#[macro_export]
macro_rules! pin_alias {
    ($pins : ident.a0) => { $pins.a0 } ; ($pins : ident.a1) => { $pins.a1 } ;
    ($pins : ident.a2) => { $pins.a2 } ; ($pins : ident.a3) => { $pins.a3 } ;
    ($pins : ident.a4) => { $pins.a4 } ; ($pins : ident.a5) => { $pins.a5 } ;
    ($pins : ident.d0) => { $pins.d0 } ; ($pins : ident.d1) => { $pins.d1 } ;
    ($pins : ident.d2) => { $pins.d2 } ; ($pins : ident.d3) => { $pins.d3 } ;
    ($pins : ident.d4) => { $pins.d4 } ; ($pins : ident.d5) => { $pins.d5 } ;
    ($pins : ident.d6) => { $pins.d6 } ; ($pins : ident.d7) => { $pins.d7 } ;
    ($pins : ident.d8) => { $pins.d8 } ; ($pins : ident.d9) => { $pins.d9 } ;
    ($pins : ident.d10) => { $pins.d10 } ; ($pins : ident.d11) =>
    { $pins.d11 } ; ($pins : ident.d12) => { $pins.d12 } ; ($pins : ident.d13)
    => { $pins.d13 } ; ($pins : ident.sda) => { $pins.sda } ;
    ($pins : ident.scl) => { $pins.scl } ; ($pins : ident.neopixel) =>
    { $pins.neopixel } ; ($pins : ident.sclk) => { $pins.sclk } ;
    ($pins : ident.mosi) => { $pins.mosi } ; ($pins : ident.miso) =>
    { $pins.miso } ; ($pins : ident.flash_sclk) => { $pins.flash_sclk } ;
    ($pins : ident.flash_cs) => { $pins.flash_cs } ; ($pins : ident.flash_d0)
    => { $pins.flash_d0 } ; ($pins : ident.flash_d1) => { $pins.flash_d1 } ;
    ($pins : ident.flash_d2) => { $pins.flash_d2 } ; ($pins : ident.flash_d3)
    => { $pins.flash_d3 } ; ($pins : ident.usb_dm) => { $pins.usb_dm } ;
    ($pins : ident.usb_dp) => { $pins.usb_dp } ; ($pins : ident.uart_rx) =>
    {
        {
            macro_rules! pin_alias_uart_rx { () => { $pins.d0 } ; }
            pin_alias_uart_rx! ()
        }
    } ; ($pins : ident.uart_tx) =>
    {
        {
            macro_rules! pin_alias_uart_tx { () => { $pins.d1 } ; }
            pin_alias_uart_tx! ()
        }
    } ; ($pins : ident.red_led) =>
    {
        {
            macro_rules! pin_alias_red_led { () => { $pins.d13 } ; }
            pin_alias_red_led! ()
        }
    } ; ($pins : ident.sda) =>
    {
        {
            macro_rules! pin_alias_sda { () => { $pins.sda } ; }
            pin_alias_sda! ()
        }
    } ; ($pins : ident.scl) =>
    {
        {
            macro_rules! pin_alias_scl { () => { $pins.scl } ; }
            pin_alias_scl! ()
        }
    } ; ($pins : ident.sclk) =>
    {
        {
            macro_rules! pin_alias_sclk { () => { $pins.sclk } ; }
            pin_alias_sclk! ()
        }
    } ; ($pins : ident.mosi) =>
    {
        {
            macro_rules! pin_alias_mosi { () => { $pins.mosi } ; }
            pin_alias_mosi! ()
        }
    } ; ($pins : ident.miso) =>
    {
        {
            macro_rules! pin_alias_miso { () => { $pins.miso } ; }
            pin_alias_miso! ()
        }
    } ; ($pins : ident.flash_sclk) =>
    {
        {
            macro_rules! pin_alias_flash_sclk { () => { $pins.flash_sclk } ; }
            pin_alias_flash_sclk! ()
        }
    } ; ($pins : ident.flash_cs) =>
    {
        {
            macro_rules! pin_alias_flash_cs { () => { $pins.flash_cs } ; }
            pin_alias_flash_cs! ()
        }
    } ; ($pins : ident.flash_d0) =>
    {
        {
            macro_rules! pin_alias_flash_d0 { () => { $pins.flash_d0 } ; }
            pin_alias_flash_d0! ()
        }
    } ; ($pins : ident.flash_d1) =>
    {
        {
            macro_rules! pin_alias_flash_d1 { () => { $pins.flash_d1 } ; }
            pin_alias_flash_d1! ()
        }
    } ; ($pins : ident.flash_d2) =>
    {
        {
            macro_rules! pin_alias_flash_d2 { () => { $pins.flash_d2 } ; }
            pin_alias_flash_d2! ()
        }
    } ; ($pins : ident.flash_d3) =>
    {
        {
            macro_rules! pin_alias_flash_d3 { () => { $pins.flash_d3 } ; }
            pin_alias_flash_d3! ()
        }
    } ; ($pins : ident.usb_dm) =>
    {
        {
            macro_rules! pin_alias_usb_dm { () => { $pins.usb_dm } ; }
            pin_alias_usb_dm! ()
        }
    } ; ($pins : ident.usb_dp) =>
    {
        {
            macro_rules! pin_alias_usb_dp { () => { $pins.usb_dp } ; }
            pin_alias_usb_dp! ()
        }
    } ;
}



/// SPI pads for the labelled SPI peripheral
///
/// You can use these pads with other, user-defined [`spi::Config`]urations.
pub type SpiPads = spi::Pads<SpiSercom, IoSet1, Miso, Mosi, Sclk>;

/// SPI master for the labelled SPI peripheral
///
/// This type implements [`FullDuplex<u8>`](ehal::spi::FullDuplex).
pub type Spi = spi::Spi<spi::Config<SpiPads>, spi::Duplex>;

/// Convenience for setting up the 2x3 header block for SPI.
/// This powers up SERCOM2 and configures it for use as an
/// SPI Master in SPI Mode 0.
pub fn spi_master(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom: SpiSercom,
    mclk: &mut pac::MCLK,
    sclk: impl Into<Sclk>,
    mosi: impl Into<Mosi>,
    miso: impl Into<Miso>,
) -> Spi {
    let gclk0 = clocks.gclk0();
    let clock = clocks.sercom2_core(&gclk0).unwrap();
    let freq = clock.freq();
    let (miso, mosi, sclk) = (miso.into(), mosi.into(), sclk.into());
    let pads =
        spi::Pads::default()
            .data_in(miso)
            .data_out(mosi)
            .sclk(sclk);

    spi::Config::new(   mclk,
                        sercom,
                        pads,
                        freq)
                            .baud(baud)
                            .spi_mode(spi::MODE_0)
                            .enable()
}

/// Convenience for setting up the onboard QSPI flash.
/// Enables the clocks for the QSPI peripheral in single data rate mode
/// assuming 120MHz system clock, for 4MHz QSPI mode 0 operation.
#[allow(clippy::too_many_arguments)]
pub fn qspi_master(
    mclk: &mut MCLK,
    qspi: pac::QSPI,
    sclk: impl Into<FlashSclk>,
    cs: impl Into<FlashCs>,
    data0: impl Into<FlashD0>,
    data1: impl Into<FlashD1>,
    data2: impl Into<FlashD2>,
    data3: impl Into<FlashD3>,
) -> Qspi<OneShot> {
    Qspi::new(
        mclk,
        qspi,
        sclk.into(),
        cs.into(),
        data0.into(),
        data1.into(),
        data2.into(),
        data3.into(),
    )
}

/// I2C pads for the labelled I2C peripheral
///
/// You can use these pads with other, user-defined [`i2c::Config`]urations.
pub type I2cPads = i2c::Pads<I2cSercom, IoSet6, Sda, Scl>;

/// I2C master for the labelled I2C peripheral
///
/// This type implements [`Read`](ehal::blocking::i2c::Read),
/// [`Write`](ehal::blocking::i2c::Write) and
/// [`WriteRead`](ehal::blocking::i2c::WriteRead).
pub type I2c = i2c::I2c<i2c::Config<I2cPads>>;

/// Convenience for setting up the labelled SDA, SCL pins to
/// operate as an I2C master running at the specified frequency.
pub fn i2c_master(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom: I2cSercom,
    mclk: &mut pac::MCLK,
    sda: impl Into<Sda>,
    scl: impl Into<Scl>,
) -> I2c {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.sercom5_core(&gclk0).unwrap();
    let freq = clock.freq();
    let baud = baud.into();
    let pads = i2c::Pads::new(sda.into(), scl.into());
    i2c::Config::new(mclk, sercom, pads, freq)
        .baud(baud)
        .enable()
}

/// UART Pads for the labelled UART peripheral
pub type UartPads = uart::Pads<UartSercom, IoSet1, UartRx, UartTx>;

/// UART device for the labelled RX & TX pins
pub type Uart = uart::Uart<uart::Config<UartPads>, uart::Duplex>;

/// Convenience for setting up the labelled RX, TX pins to
/// operate as a UART device running at the specified baud.
pub fn uart(
    clocks: &mut GenericClockController,
    baud: impl Into<Hertz>,
    sercom: UartSercom,
    mclk: &mut pac::MCLK,
    uart_rx: impl Into<UartRx>,
    uart_tx: impl Into<UartTx>,
) -> Uart {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.sercom3_core(&gclk0).unwrap();
    let baud = baud.into();
    let pads = uart::Pads::default().rx(uart_rx.into()).tx(uart_tx.into());
    uart::Config::new(mclk, sercom, pads, clock.freq())
        .baud(baud, BaudMode::Fractional(Oversampling::Bits16))
        .enable()
}

#[cfg(feature = "usb")]
/// Convenience function for setting up USB
pub fn usb_allocator(
    usb: pac::USB,
    clocks: &mut GenericClockController,
    mclk: &mut pac::MCLK,
    dm: impl Into<UsbDm>,
    dp: impl Into<UsbDp>,
) -> UsbBusAllocator<UsbBus> {
    let gclk0 = clocks.gclk0();
    let clock = &clocks.usb(&gclk0).unwrap();
    let (dm, dp) = (dm.into(), dp.into());
    UsbBusAllocator::new(UsbBus::new(clock, mclk, dm, dp, usb))
}
