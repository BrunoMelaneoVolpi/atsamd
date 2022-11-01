//! Configuring the system clock sources.
//! You will typically need to create an instance of `GenericClockController`
//! before you can set up most of the peripherals on the atsamd51 device.
//! The other types in this module are used to enforce at compile time
//! that the peripherals have been correctly configured.
#![allow(clippy::from_over_into)]

use crate::pac::gclk::genctrl::SRC_A::*;
use crate::pac::gclk::pchctrl::GEN_A::*;
use crate::pac::{self, GCLK, MCLK, NVMCTRL, OSC32KCTRL, OSCCTRL};
use crate::time::{Hertz, MegaHertz};

pub type ClockGenId = pac::gclk::pchctrl::GEN_A;
pub type ClockSource = pac::gclk::genctrl::SRC_A;

#[allow(non_camel_case_types)]
pub enum ClockId {
    DFLL48 = 0,
    FDPLL0,
    FDPLL1,
    SLOW_32K,
    EIC,
    FREQM_MSR,
    FREQM_REF,
    SERCOM0_CORE,
    SERCOM1_CORE,
    TC0_TC1,
    USB,
    EVSYS0,
    EVSYS1,
    EVSYS2,
    EVSYS3,
    EVSYS4,
    EVSYS5,
    EVSYS6,
    EVSYS7,
    EVSYS8,
    EVSYS9,
    EVSYS10,
    EVSYS11,
    SERCOM2_CORE,
    SERCOM3_CORE,
    TCC0_TCC1,
    TC2_TC3,
    CAN0,
    CAN1,
    TCC2_TCC3,
    TC4_TC5,
    PDEC,
    AC,
    CCL,
    SERCOM4_CORE,
    SERCOM5_CORE,
    SERCOM6_CORE,
    SERCOM7_CORE,
    TCC4,
    TC6_TC7,
    ADC0,
    ADC1,
    DAC,
    I2S0,
    I2S1,
    SDHC0,
    SDHC1,
    CM4_TRACE,
}

impl From<ClockId> for u8 {
    fn from(clock: ClockId) -> u8 {
        clock as u8
    }
}

/// Represents a configured clock generator.
/// Can be converted into the effective clock frequency.
/// Its primary purpose is to be passed in to methods
/// such as `GenericClockController::tcc2_tc3` to configure
/// the clock for a peripheral.
//#[derive(Clone, Copy)]
pub struct GClock {
    gclk: ClockGenId,
    freq: Hertz,
}

impl Into<Hertz> for GClock {
    fn into(self) -> Hertz {
        self.freq
    }
}

struct State {
    gclk: GCLK,
}

impl State {
    fn reset_gclk(&mut self) {
        self.gclk.ctrla.write(|w| w.swrst().set_bit());
        while self.gclk.ctrla.read().swrst().bit_is_set() || self.gclk.syncbusy.read().bits() != 0 {
        }
    }

    fn wait_for_sync(&mut self) {
        while self.gclk.syncbusy.read().bits() != 0 {}
    }

    fn set_gclk_divider_and_source(
        &mut self,
        gclk: ClockGenId,
        divider: u16,
        src: ClockSource,
        improve_duty_cycle: bool,
    ) {
        // validate the divisor factor based on gclk ID (see 14.8.3)
        let mut divisor_invalid = false;
        if gclk == GCLK1 {
            if divider as u32 >= 2_u32.pow(16) {
                divisor_invalid = true;
            }
        } else if divider >= 2_u16.pow(8) {
            divisor_invalid = true;
        }
        if divisor_invalid {
            panic!("invalid divisor {} for GCLK {}", divider, gclk as u8);
        }

        self.gclk.genctrl[u8::from(gclk) as usize].write(|w| unsafe {
            w.src().variant(src);
            w.div().bits(divider);
            // divide directly by divider, rather than 2^(n+1)
            w.divsel().clear_bit();
            w.idc().bit(improve_duty_cycle);
            w.genen().set_bit();
            w.oe().set_bit()
        });

        self.wait_for_sync();
    }

    fn enable_clock_generator(&mut self, clock: ClockId, generator: ClockGenId) {
        self.gclk.pchctrl[u8::from(clock) as usize].write(|w| unsafe {
            w.gen().bits(generator.into());
            w.chen().set_bit()
        });
        self.wait_for_sync();
    }

    fn configure_standby(&mut self, gclk: ClockGenId, enable: bool) {
        self.gclk.genctrl[u8::from(gclk) as usize].modify(|_, w| w.runstdby().bit(enable));
        self.wait_for_sync();
    }
}

/// `GenericClockController` encapsulates the GCLK hardware.
/// It provides a type safe way to configure the system clocks.
/// Initializing the `GenericClockController` instance configures
/// the system to run at 120MHz by taking the DFLL48
/// and feeding it into the DPLL0 hardware which multiplies the
/// signal by 2.5x.
pub struct GenericClockController {
    state: State,
    gclks: [Hertz; 12],
    used_clocks: u64,
}

impl GenericClockController {
    /// Reset the clock controller, configure the system to run
    /// at 120Mhz and reset various clock dividers.
    pub fn with_internal_32kosc(
        gclk: GCLK,
        mclk: &mut MCLK,
        osc32kctrl: &mut OSC32KCTRL,
        oscctrl: &mut OSCCTRL,
        nvmctrl: &mut NVMCTRL,
    ) -> Self {
        Self::new(gclk, mclk, osc32kctrl, oscctrl, nvmctrl, false)
    }

    /// Reset the clock controller, configure the system to run
    /// at 120Mhz and reset various clock dividers.
    pub fn with_external_32kosc(
        gclk: GCLK,
        mclk: &mut MCLK,
        osc32kctrl: &mut OSC32KCTRL,
        oscctrl: &mut OSCCTRL,
        nvmctrl: &mut NVMCTRL,
    ) -> Self {
        Self::new(  gclk,
                    mclk,
                    osc32kctrl,
                    oscctrl,
                    nvmctrl,
                    true)
    }

    fn new(
        gclk: GCLK,
        mclk: &mut MCLK,
        osc32kctrl: &mut OSC32KCTRL,
        oscctrl: &mut OSCCTRL,
        nvmctrl: &mut NVMCTRL,
        use_external_crystal: bool,
    ) -> Self {
        let mut state = State { gclk };

        set_flash_to_half_auto_wait_state(nvmctrl);
        enable_gclk_apb(mclk);

        if use_external_crystal {
            enable_external_32kosc(osc32kctrl);
            state.reset_gclk();
            state.set_gclk_divider_and_source(GCLK1, 1, XOSC32K, false);
        } else {
            enable_internal_32kosc(osc32kctrl);
            state.reset_gclk();
            state.set_gclk_divider_and_source(GCLK1, 1, OSCULP32K, false);
        }

        while state.gclk.syncbusy.read().genctrl().is_gclk0() {}

        #[cfg(feature = "usb")]
        configure_usb_correction(oscctrl);

        // GCLK5 set to 2MHz
        unsafe {
            state.gclk.genctrl[5].write(|w| {
                w.src().dfll();
                w.genen().set_bit();
                w.div().bits(24)
            });
        }

        while state.gclk.syncbusy.read().genctrl().is_gclk5() {}

        configure_and_enable_dpll0(oscctrl, &mut state.gclk);
        wait_for_dpllrdy(oscctrl);

        unsafe {
            // GCLK0 set to DPLL0 (120MHz)
            state.gclk.genctrl[0].write(|w| {
                w.src().dpll0();
                w.div().bits(1);
                w.oe().set_bit();
                w.genen().set_bit()
            });
        }

        while state.gclk.syncbusy.read().genctrl().is_gclk0() {}

        mclk.cpudiv.write(|w| w.div().div1());

        Self {
            state,
            gclks: [
                OSC120M_FREQ,
                OSC32K_FREQ,
                Hertz(0),
                Hertz(0),
                Hertz(0),
                MegaHertz(2).into(),
                Hertz(0),
                Hertz(0),
                Hertz(0),
                Hertz(0),
                Hertz(0),
                Hertz(0),
            ],
            used_clocks: 1u64 << u8::from(ClockId::FDPLL0),
        }
    }

    /// Returns a `GClock` for gclk0, the 120MHz oscillator.
    pub fn gclk0(&mut self) -> GClock {
        GClock {
            gclk: GCLK0,
            freq: self.gclks[0],
        }
    }

    /// Returns a `GClock` for gclk1, the 32KHz oscillator.
    pub fn gclk1(&mut self) -> GClock {
        GClock {
            gclk: GCLK1,
            freq: self.gclks[1],
        }
    }

    /// Returns the `GClock` for the specified clock generator.
    /// If that clock generator has not yet been configured,
    /// returns None.
    pub fn get_gclk(&mut self, gclk: ClockGenId) -> Option<GClock> {
        let idx = u8::from(gclk) as usize;
        if self.gclks[idx].0 == 0 {
            None
        } else {
            Some(GClock {
                gclk,
                freq: self.gclks[idx],
            })
        }
    }

    /// Configures a clock generator with the specified divider and
    /// source.
    /// `divider` is a linear divider to be applied to the clock
    /// source.  While the hardware also supports an exponential divider,
    /// this function doesn't expose that functionality at this time.
    /// `improve_duty_cycle` is a boolean that, when set to true, enables
    /// a 50/50 duty cycle for odd divider values.
    /// Returns a `GClock` for the configured clock generator.
    /// Returns `None` if the clock generator has already been configured.
    pub fn configure_gclk_divider_and_source(
        &mut self,
        gclk: ClockGenId,
        divider: u16,
        src: ClockSource,
        improve_duty_cycle: bool,
    ) -> Option<GClock> {
        let idx = u8::from(gclk) as usize;
        if self.gclks[idx].0 != 0 {
            return None;
        }
        self.state
            .set_gclk_divider_and_source(gclk, divider, src, improve_duty_cycle);
        let freq: Hertz = match src {
            XOSC32K | OSCULP32K => OSC32K_FREQ,
            GCLKGEN1 => self.gclks[1],
            DFLL => OSC48M_FREQ,
            DPLL0 => OSC120M_FREQ,
            XOSC0 | XOSC1 | GCLKIN | DPLL1 => unimplemented!(),
        };
        self.gclks[idx] = Hertz(freq.0 / divider as u32);
        Some(GClock { gclk, freq })
    }

    /// Enables or disables the given GClk from operation in standby.
    pub fn configure_standby(&mut self, gclk: ClockGenId, enable: bool) {
        self.state.configure_standby(gclk, enable)
    }
}

//macro_rules! clock_generator {
//    (
//        $(
//            $(#[$attr:meta])*
//            ($id:ident, $Type:ident, $clock:ident),
//        )+
//    ) => {
//
//$(
//
///// A typed token that indicates that the clock for the peripheral(s)
///// with the matching name has been configured.
///// The effective clock frequency is available via the `freq` method,
///// or by converting the object into a `Hertz` instance.
///// The peripheral initialization code will typically require passing
///// in this object to prove at compile time that the clock has been
///// correctly initialized.
//$(#[$attr])*
//#[derive(Debug)]
//pub struct $Type {
//    freq: Hertz,
//}
//
//$(#[$attr])*
//impl $Type {
//    /// Returns the frequency of the configured clock
//    pub fn freq(&self) -> Hertz {
//        self.freq
//    }
//}
//$(#[$attr])*
//impl Into<Hertz> for $Type {
//    fn into(self) -> Hertz {
//        self.freq
//    }
//}
//)+
//
//impl GenericClockController {
//    $(
//    /// Configure the clock for peripheral(s) that match the name
//    /// of this function to use the specific clock generator.
//    /// The `GClock` parameter may be one of default clocks
//    /// return from `gclk0()`, `gclk1()` or a clock configured
//    /// by the host application using the `configure_gclk_divider_and_source`
//    /// method.
//    /// Returns a typed token that proves that the clock has been configured;
//    /// the peripheral initialization code will typically require that this
//    /// clock token be passed in to ensure that the clock has been initialized
//    /// appropriately.
//    /// Returns `None` is the specified generic clock has already been
//    /// configured.
//    $(#[$attr])*
//    pub fn $id(&mut self, generator: &GClock) -> Option<$Type> {
//        let bits: u64 = 1 << u8::from(ClockId::$clock) as u64;
//        if (self.used_clocks & bits) != 0 {
//            return None;
//        }
//        self.used_clocks |= bits;
//
//        self.state.enable_clock_generator(ClockId::$clock, generator.gclk);
//        let freq = self.gclks[u8::from(generator.gclk) as usize];
//        Some($Type{freq})
//    }
//    )+
//}
//    }
//}
//////////// melabr

/// A typed token that indicates that the clock for the peripheral(s)
/// with the matching name has been configured.
/// The effective clock frequency is available via the `freq` method,
/// or by converting the object into a `Hertz` instance.
/// The peripheral initialization code will typically require passing
/// in this object to prove at compile time that the clock has been
/// correctly initialized.
pub struct Tc0Tc1Clock {
    freq: Hertz,
}
#[automatically_derived]
impl ::core::fmt::Debug for Tc0Tc1Clock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        Ok(())  //  HACK! melabr        // ::core::fmt::Formatter::debug_struct_field1_finish(f, "Tc0Tc1Clock", "freq", &&self.freq)
    }
}
impl Tc0Tc1Clock {
    /// Returns the frequency of the configured clock
    pub fn freq(&self) -> Hertz { self.freq }
}
impl Into<Hertz> for Tc0Tc1Clock {
    fn into(self) -> Hertz { self.freq }
}
/// A typed token that indicates that the clock for the peripheral(s)
/// with the matching name has been configured.
/// The effective clock frequency is available via the `freq` method,
/// or by converting the object into a `Hertz` instance.
/// The peripheral initialization code will typically require passing
/// in this object to prove at compile time that the clock has been
/// correctly initialized.
pub struct Tcc0Tcc1Clock {
    freq: Hertz,
}
#[automatically_derived]
impl ::core::fmt::Debug for Tcc0Tcc1Clock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        Ok(())  //  HACK! melabr        // ::core::fmt::Formatter::debug_struct_field1_finish(f, "Tcc0Tcc1Clock",            "freq", &&self.freq)
    }
}
impl Tcc0Tcc1Clock {
    /// Returns the frequency of the configured clock
    pub fn freq(&self) -> Hertz { self.freq }
}
impl Into<Hertz> for Tcc0Tcc1Clock {
    fn into(self) -> Hertz { self.freq }
}
/// A typed token that indicates that the clock for the peripheral(s)
/// with the matching name has been configured.
/// The effective clock frequency is available via the `freq` method,
/// or by converting the object into a `Hertz` instance.
/// The peripheral initialization code will typically require passing
/// in this object to prove at compile time that the clock has been
/// correctly initialized.
pub struct Tc2Tc3Clock {
    freq: Hertz,
}
#[automatically_derived]
impl ::core::fmt::Debug for Tc2Tc3Clock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        Ok(())  //  HACK! melabr        // ::core::fmt::Formatter::debug_struct_field1_finish(f, "Tc2Tc3Clock",            "freq", &&self.freq)
    }
}
impl Tc2Tc3Clock {
    /// Returns the frequency of the configured clock
    pub fn freq(&self) -> Hertz { self.freq }
}
impl Into<Hertz> for Tc2Tc3Clock {
    fn into(self) -> Hertz { self.freq }
}
/// A typed token that indicates that the clock for the peripheral(s)
/// with the matching name has been configured.
/// The effective clock frequency is available via the `freq` method,
/// or by converting the object into a `Hertz` instance.
/// The peripheral initialization code will typically require passing
/// in this object to prove at compile time that the clock has been
/// correctly initialized.
pub struct Tcc2Tcc3Clock {
    freq: Hertz,
}
#[automatically_derived]
impl ::core::fmt::Debug for Tcc2Tcc3Clock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        Ok(())  //  HACK! melabr        // ::core::fmt::Formatter::debug_struct_field1_finish(f, "Tcc2Tcc3Clock",            "freq", &&self.freq)
    }
}
impl Tcc2Tcc3Clock {
    /// Returns the frequency of the configured clock
    pub fn freq(&self) -> Hertz { self.freq }
}
impl Into<Hertz> for Tcc2Tcc3Clock {
    fn into(self) -> Hertz { self.freq }
}
/// A typed token that indicates that the clock for the peripheral(s)
/// with the matching name has been configured.
/// The effective clock frequency is available via the `freq` method,
/// or by converting the object into a `Hertz` instance.
/// The peripheral initialization code will typically require passing
/// in this object to prove at compile time that the clock has been
/// correctly initialized.
pub struct Tc4Tc5Clock {
    freq: Hertz,
}
#[automatically_derived]
impl ::core::fmt::Debug for Tc4Tc5Clock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        Ok(())  //  HACK! melabr        // ::core::fmt::Formatter::debug_struct_field1_finish(f, "Tc4Tc5Clock",            "freq", &&self.freq)
    }
}
impl Tc4Tc5Clock {
    /// Returns the frequency of the configured clock
    pub fn freq(&self) -> Hertz { self.freq }
}
impl Into<Hertz> for Tc4Tc5Clock {
    fn into(self) -> Hertz { self.freq }
}
/// A typed token that indicates that the clock for the peripheral(s)
/// with the matching name has been configured.
/// The effective clock frequency is available via the `freq` method,
/// or by converting the object into a `Hertz` instance.
/// The peripheral initialization code will typically require passing
/// in this object to prove at compile time that the clock has been
/// correctly initialized.
pub struct Tcc4Clock {
    freq: Hertz,
}
#[automatically_derived]
impl ::core::fmt::Debug for Tcc4Clock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        Ok(())  //  HACK! melabr        // ::core::fmt::Formatter::debug_struct_field1_finish(f, "Tcc4Clock",            "freq", &&self.freq)
    }
}
impl Tcc4Clock {
    /// Returns the frequency of the configured clock
    pub fn freq(&self) -> Hertz { self.freq }
}
impl Into<Hertz> for Tcc4Clock {
    fn into(self) -> Hertz { self.freq }
}
/// A typed token that indicates that the clock for the peripheral(s)
/// with the matching name has been configured.
/// The effective clock frequency is available via the `freq` method,
/// or by converting the object into a `Hertz` instance.
/// The peripheral initialization code will typically require passing
/// in this object to prove at compile time that the clock has been
/// correctly initialized.
pub struct Tc6Tc7Clock {
    freq: Hertz,
}
#[automatically_derived]
impl ::core::fmt::Debug for Tc6Tc7Clock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        Ok(())  //  HACK! melabr        // ::core::fmt::Formatter::debug_struct_field1_finish(f, "Tc6Tc7Clock",            "freq", &&self.freq)
    }
}
impl Tc6Tc7Clock {
    /// Returns the frequency of the configured clock
    pub fn freq(&self) -> Hertz { self.freq }
}
impl Into<Hertz> for Tc6Tc7Clock {
    fn into(self) -> Hertz { self.freq }
}
/// A typed token that indicates that the clock for the peripheral(s)
/// with the matching name has been configured.
/// The effective clock frequency is available via the `freq` method,
/// or by converting the object into a `Hertz` instance.
/// The peripheral initialization code will typically require passing
/// in this object to prove at compile time that the clock has been
/// correctly initialized.
pub struct Sercom0CoreClock {
    freq: Hertz,
}
#[automatically_derived]
impl ::core::fmt::Debug for Sercom0CoreClock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        Ok(())  //  HACK! melabr        // ::core::fmt::Formatter::debug_struct_field1_finish(f,            "Sercom0CoreClock", "freq", &&self.freq)
    }
}
impl Sercom0CoreClock {
    /// Returns the frequency of the configured clock
    pub fn freq(&self) -> Hertz { self.freq }
}
impl Into<Hertz> for Sercom0CoreClock {
    fn into(self) -> Hertz { self.freq }
}
/// A typed token that indicates that the clock for the peripheral(s)
/// with the matching name has been configured.
/// The effective clock frequency is available via the `freq` method,
/// or by converting the object into a `Hertz` instance.
/// The peripheral initialization code will typically require passing
/// in this object to prove at compile time that the clock has been
/// correctly initialized.
pub struct Sercom1CoreClock {
    freq: Hertz,
}
#[automatically_derived]
impl ::core::fmt::Debug for Sercom1CoreClock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        Ok(())  //  HACK! melabr        // ::core::fmt::Formatter::debug_struct_field1_finish(f,            "Sercom1CoreClock", "freq", &&self.freq)
    }
}
impl Sercom1CoreClock {
    /// Returns the frequency of the configured clock
    pub fn freq(&self) -> Hertz { self.freq }
}
impl Into<Hertz> for Sercom1CoreClock {
    fn into(self) -> Hertz { self.freq }
}
/// A typed token that indicates that the clock for the peripheral(s)
/// with the matching name has been configured.
/// The effective clock frequency is available via the `freq` method,
/// or by converting the object into a `Hertz` instance.
/// The peripheral initialization code will typically require passing
/// in this object to prove at compile time that the clock has been
/// correctly initialized.
pub struct Sercom2CoreClock {
    freq: Hertz,
}
#[automatically_derived]
impl ::core::fmt::Debug for Sercom2CoreClock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        Ok(())  //  HACK! melabr        // ::core::fmt::Formatter::debug_struct_field1_finish(f,            "Sercom2CoreClock", "freq", &&self.freq)
    }
}
impl Sercom2CoreClock {
    /// Returns the frequency of the configured clock
    pub fn freq(&self) -> Hertz { self.freq }
}
impl Into<Hertz> for Sercom2CoreClock {
    fn into(self) -> Hertz { self.freq }
}
/// A typed token that indicates that the clock for the peripheral(s)
/// with the matching name has been configured.
/// The effective clock frequency is available via the `freq` method,
/// or by converting the object into a `Hertz` instance.
/// The peripheral initialization code will typically require passing
/// in this object to prove at compile time that the clock has been
/// correctly initialized.
pub struct Sercom3CoreClock {
    freq: Hertz,
}
#[automatically_derived]
impl ::core::fmt::Debug for Sercom3CoreClock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        Ok(())  //  HACK! melabr        // ::core::fmt::Formatter::debug_struct_field1_finish(f,            "Sercom3CoreClock", "freq", &&self.freq)
    }
}
impl Sercom3CoreClock {
    /// Returns the frequency of the configured clock
    pub fn freq(&self) -> Hertz { self.freq }
}
impl Into<Hertz> for Sercom3CoreClock {
    fn into(self) -> Hertz { self.freq }
}
/// A typed token that indicates that the clock for the peripheral(s)
/// with the matching name has been configured.
/// The effective clock frequency is available via the `freq` method,
/// or by converting the object into a `Hertz` instance.
/// The peripheral initialization code will typically require passing
/// in this object to prove at compile time that the clock has been
/// correctly initialized.
pub struct Sercom4CoreClock {
    freq: Hertz,
}
#[automatically_derived]
impl ::core::fmt::Debug for Sercom4CoreClock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        Ok(())  //  HACK! melabr        // ::core::fmt::Formatter::debug_struct_field1_finish(f,            "Sercom4CoreClock", "freq", &&self.freq)
    }
}
impl Sercom4CoreClock {
    /// Returns the frequency of the configured clock
    pub fn freq(&self) -> Hertz { self.freq }
}
impl Into<Hertz> for Sercom4CoreClock {
    fn into(self) -> Hertz { self.freq }
}
/// A typed token that indicates that the clock for the peripheral(s)
/// with the matching name has been configured.
/// The effective clock frequency is available via the `freq` method,
/// or by converting the object into a `Hertz` instance.
/// The peripheral initialization code will typically require passing
/// in this object to prove at compile time that the clock has been
/// correctly initialized.
pub struct Sercom5CoreClock {
    freq: Hertz,
}
#[automatically_derived]
impl ::core::fmt::Debug for Sercom5CoreClock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        Ok(())  //  HACK! melabr        // ::core::fmt::Formatter::debug_struct_field1_finish(f,            "Sercom5CoreClock", "freq", &&self.freq)
    }
}
impl Sercom5CoreClock {
    /// Returns the frequency of the configured clock
    pub fn freq(&self) -> Hertz { self.freq }
}
impl Into<Hertz> for Sercom5CoreClock {
    fn into(self) -> Hertz { self.freq }
}
/// A typed token that indicates that the clock for the peripheral(s)
/// with the matching name has been configured.
/// The effective clock frequency is available via the `freq` method,
/// or by converting the object into a `Hertz` instance.
/// The peripheral initialization code will typically require passing
/// in this object to prove at compile time that the clock has been
/// correctly initialized.
pub struct UsbClock {
    freq: Hertz,
}
#[automatically_derived]
impl ::core::fmt::Debug for UsbClock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        Ok(())  //  HACK! melabr        // ::core::fmt::Formatter::debug_struct_field1_finish(f, "UsbClock",            "freq", &&self.freq)
    }
}
impl UsbClock {
    /// Returns the frequency of the configured clock
    pub fn freq(&self) -> Hertz { self.freq }
}
impl Into<Hertz> for UsbClock {
    fn into(self) -> Hertz { self.freq }
}
/// A typed token that indicates that the clock for the peripheral(s)
/// with the matching name has been configured.
/// The effective clock frequency is available via the `freq` method,
/// or by converting the object into a `Hertz` instance.
/// The peripheral initialization code will typically require passing
/// in this object to prove at compile time that the clock has been
/// correctly initialized.
pub struct Adc0Clock {
    freq: Hertz,
}
#[automatically_derived]
impl ::core::fmt::Debug for Adc0Clock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        Ok(())  //  HACK! melabr        // ::core::fmt::Formatter::debug_struct_field1_finish(f, "Adc0Clock",            "freq", &&self.freq)
    }
}
impl Adc0Clock {
    /// Returns the frequency of the configured clock
    pub fn freq(&self) -> Hertz { self.freq }
}
impl Into<Hertz> for Adc0Clock {
    fn into(self) -> Hertz { self.freq }
}
/// A typed token that indicates that the clock for the peripheral(s)
/// with the matching name has been configured.
/// The effective clock frequency is available via the `freq` method,
/// or by converting the object into a `Hertz` instance.
/// The peripheral initialization code will typically require passing
/// in this object to prove at compile time that the clock has been
/// correctly initialized.
pub struct Adc1Clock {
    freq: Hertz,
}
#[automatically_derived]
impl ::core::fmt::Debug for Adc1Clock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        Ok(())  //  HACK! melabr        // ::core::fmt::Formatter::debug_struct_field1_finish(f, "Adc1Clock",            "freq", &&self.freq)
    }
}
impl Adc1Clock {
    /// Returns the frequency of the configured clock
    pub fn freq(&self) -> Hertz { self.freq }
}
impl Into<Hertz> for Adc1Clock {
    fn into(self) -> Hertz { self.freq }
}
/// A typed token that indicates that the clock for the peripheral(s)
/// with the matching name has been configured.
/// The effective clock frequency is available via the `freq` method,
/// or by converting the object into a `Hertz` instance.
/// The peripheral initialization code will typically require passing
/// in this object to prove at compile time that the clock has been
/// correctly initialized.
pub struct EicClock {
    freq: Hertz,
}
#[automatically_derived]
impl ::core::fmt::Debug for EicClock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        Ok(())  //  HACK! melabr        // ::core::fmt::Formatter::debug_struct_field1_finish(f, "EicClock",            "freq", &&self.freq)
    }
}
impl EicClock {
    /// Returns the frequency of the configured clock
    pub fn freq(&self) -> Hertz { self.freq }
}
impl Into<Hertz> for EicClock {
    fn into(self) -> Hertz { self.freq }
}
/// A typed token that indicates that the clock for the peripheral(s)
/// with the matching name has been configured.
/// The effective clock frequency is available via the `freq` method,
/// or by converting the object into a `Hertz` instance.
/// The peripheral initialization code will typically require passing
/// in this object to prove at compile time that the clock has been
/// correctly initialized.
pub struct FreqmMsrClock {
    freq: Hertz,
}
#[automatically_derived]
impl ::core::fmt::Debug for FreqmMsrClock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        Ok(())  //  HACK! melabr        // ::core::fmt::Formatter::debug_struct_field1_finish(f, "FreqmMsrClock",            "freq", &&self.freq)
    }
}
impl FreqmMsrClock {
    /// Returns the frequency of the configured clock
    pub fn freq(&self) -> Hertz { self.freq }
}
impl Into<Hertz> for FreqmMsrClock {
    fn into(self) -> Hertz { self.freq }
}
/// A typed token that indicates that the clock for the peripheral(s)
/// with the matching name has been configured.
/// The effective clock frequency is available via the `freq` method,
/// or by converting the object into a `Hertz` instance.
/// The peripheral initialization code will typically require passing
/// in this object to prove at compile time that the clock has been
/// correctly initialized.
pub struct FreqmRefClock {
    freq: Hertz,
}
#[automatically_derived]
impl ::core::fmt::Debug for FreqmRefClock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        Ok(())  //  HACK! melabr        // ::core::fmt::Formatter::debug_struct_field1_finish(f, "FreqmRefClock",            "freq", &&self.freq)
    }
}
impl FreqmRefClock {
    /// Returns the frequency of the configured clock
    pub fn freq(&self) -> Hertz { self.freq }
}
impl Into<Hertz> for FreqmRefClock {
    fn into(self) -> Hertz { self.freq }
}
/// A typed token that indicates that the clock for the peripheral(s)
/// with the matching name has been configured.
/// The effective clock frequency is available via the `freq` method,
/// or by converting the object into a `Hertz` instance.
/// The peripheral initialization code will typically require passing
/// in this object to prove at compile time that the clock has been
/// correctly initialized.
pub struct Evsys0Clock {
    freq: Hertz,
}
#[automatically_derived]
impl ::core::fmt::Debug for Evsys0Clock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        Ok(())  //  HACK! melabr        // ::core::fmt::Formatter::debug_struct_field1_finish(f, "Evsys0Clock",            "freq", &&self.freq)
    }
}
impl Evsys0Clock {
    /// Returns the frequency of the configured clock
    pub fn freq(&self) -> Hertz { self.freq }
}
impl Into<Hertz> for Evsys0Clock {
    fn into(self) -> Hertz { self.freq }
}
/// A typed token that indicates that the clock for the peripheral(s)
/// with the matching name has been configured.
/// The effective clock frequency is available via the `freq` method,
/// or by converting the object into a `Hertz` instance.
/// The peripheral initialization code will typically require passing
/// in this object to prove at compile time that the clock has been
/// correctly initialized.
pub struct Evsys1Clock {
    freq: Hertz,
}
#[automatically_derived]
impl ::core::fmt::Debug for Evsys1Clock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        Ok(())  //  HACK! melabr        // ::core::fmt::Formatter::debug_struct_field1_finish(f, "Evsys1Clock",            "freq", &&self.freq)
    }
}
impl Evsys1Clock {
    /// Returns the frequency of the configured clock
    pub fn freq(&self) -> Hertz { self.freq }
}
impl Into<Hertz> for Evsys1Clock {
    fn into(self) -> Hertz { self.freq }
}
/// A typed token that indicates that the clock for the peripheral(s)
/// with the matching name has been configured.
/// The effective clock frequency is available via the `freq` method,
/// or by converting the object into a `Hertz` instance.
/// The peripheral initialization code will typically require passing
/// in this object to prove at compile time that the clock has been
/// correctly initialized.
pub struct Evsys2Clock {
    freq: Hertz,
}
#[automatically_derived]
impl ::core::fmt::Debug for Evsys2Clock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        Ok(())  //  HACK! melabr        // ::core::fmt::Formatter::debug_struct_field1_finish(f, "Evsys2Clock",            "freq", &&self.freq)
    }
}
impl Evsys2Clock {
    /// Returns the frequency of the configured clock
    pub fn freq(&self) -> Hertz { self.freq }
}
impl Into<Hertz> for Evsys2Clock {
    fn into(self) -> Hertz { self.freq }
}
/// A typed token that indicates that the clock for the peripheral(s)
/// with the matching name has been configured.
/// The effective clock frequency is available via the `freq` method,
/// or by converting the object into a `Hertz` instance.
/// The peripheral initialization code will typically require passing
/// in this object to prove at compile time that the clock has been
/// correctly initialized.
pub struct Evsys3Clock {
    freq: Hertz,
}
#[automatically_derived]
impl ::core::fmt::Debug for Evsys3Clock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        Ok(())  //  HACK! melabr        // ::core::fmt::Formatter::debug_struct_field1_finish(f, "Evsys3Clock",            "freq", &&self.freq)
    }
}
impl Evsys3Clock {
    /// Returns the frequency of the configured clock
    pub fn freq(&self) -> Hertz { self.freq }
}
impl Into<Hertz> for Evsys3Clock {
    fn into(self) -> Hertz { self.freq }
}
/// A typed token that indicates that the clock for the peripheral(s)
/// with the matching name has been configured.
/// The effective clock frequency is available via the `freq` method,
/// or by converting the object into a `Hertz` instance.
/// The peripheral initialization code will typically require passing
/// in this object to prove at compile time that the clock has been
/// correctly initialized.
pub struct Evsys4Clock {
    freq: Hertz,
}
#[automatically_derived]
impl ::core::fmt::Debug for Evsys4Clock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        Ok(())  //  HACK! melabr        // ::core::fmt::Formatter::debug_struct_field1_finish(f, "Evsys4Clock",            "freq", &&self.freq)
    }
}
impl Evsys4Clock {
    /// Returns the frequency of the configured clock
    pub fn freq(&self) -> Hertz { self.freq }
}
impl Into<Hertz> for Evsys4Clock {
    fn into(self) -> Hertz { self.freq }
}
/// A typed token that indicates that the clock for the peripheral(s)
/// with the matching name has been configured.
/// The effective clock frequency is available via the `freq` method,
/// or by converting the object into a `Hertz` instance.
/// The peripheral initialization code will typically require passing
/// in this object to prove at compile time that the clock has been
/// correctly initialized.
pub struct Evsys5Clock {
    freq: Hertz,
}
#[automatically_derived]
impl ::core::fmt::Debug for Evsys5Clock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        Ok(())  //  HACK! melabr        // ::core::fmt::Formatter::debug_struct_field1_finish(f, "Evsys5Clock",            "freq", &&self.freq)
    }
}
impl Evsys5Clock {
    /// Returns the frequency of the configured clock
    pub fn freq(&self) -> Hertz { self.freq }
}
impl Into<Hertz> for Evsys5Clock {
    fn into(self) -> Hertz { self.freq }
}
/// A typed token that indicates that the clock for the peripheral(s)
/// with the matching name has been configured.
/// The effective clock frequency is available via the `freq` method,
/// or by converting the object into a `Hertz` instance.
/// The peripheral initialization code will typically require passing
/// in this object to prove at compile time that the clock has been
/// correctly initialized.
pub struct Evsys6Clock {
    freq: Hertz,
}
#[automatically_derived]
impl ::core::fmt::Debug for Evsys6Clock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        Ok(())  //  HACK! melabr        // ::core::fmt::Formatter::debug_struct_field1_finish(f, "Evsys6Clock",            "freq", &&self.freq)
    }
}
impl Evsys6Clock {
    /// Returns the frequency of the configured clock
    pub fn freq(&self) -> Hertz { self.freq }
}
impl Into<Hertz> for Evsys6Clock {
    fn into(self) -> Hertz { self.freq }
}
/// A typed token that indicates that the clock for the peripheral(s)
/// with the matching name has been configured.
/// The effective clock frequency is available via the `freq` method,
/// or by converting the object into a `Hertz` instance.
/// The peripheral initialization code will typically require passing
/// in this object to prove at compile time that the clock has been
/// correctly initialized.
pub struct Evsys7Clock {
    freq: Hertz,
}
#[automatically_derived]
impl ::core::fmt::Debug for Evsys7Clock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        Ok(())  //  HACK! melabr        // ::core::fmt::Formatter::debug_struct_field1_finish(f, "Evsys7Clock",            "freq", &&self.freq)
    }
}
impl Evsys7Clock {
    /// Returns the frequency of the configured clock
    pub fn freq(&self) -> Hertz { self.freq }
}
impl Into<Hertz> for Evsys7Clock {
    fn into(self) -> Hertz { self.freq }
}
/// A typed token that indicates that the clock for the peripheral(s)
/// with the matching name has been configured.
/// The effective clock frequency is available via the `freq` method,
/// or by converting the object into a `Hertz` instance.
/// The peripheral initialization code will typically require passing
/// in this object to prove at compile time that the clock has been
/// correctly initialized.
pub struct Evsys8Clock {
    freq: Hertz,
}
#[automatically_derived]
impl ::core::fmt::Debug for Evsys8Clock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        Ok(())  //  HACK! melabr        // ::core::fmt::Formatter::debug_struct_field1_finish(f, "Evsys8Clock",            "freq", &&self.freq)
    }
}
impl Evsys8Clock {
    /// Returns the frequency of the configured clock
    pub fn freq(&self) -> Hertz { self.freq }
}
impl Into<Hertz> for Evsys8Clock {
    fn into(self) -> Hertz { self.freq }
}
/// A typed token that indicates that the clock for the peripheral(s)
/// with the matching name has been configured.
/// The effective clock frequency is available via the `freq` method,
/// or by converting the object into a `Hertz` instance.
/// The peripheral initialization code will typically require passing
/// in this object to prove at compile time that the clock has been
/// correctly initialized.
pub struct Evsys9Clock {
    freq: Hertz,
}
#[automatically_derived]
impl ::core::fmt::Debug for Evsys9Clock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        Ok(())  //  HACK! melabr        // ::core::fmt::Formatter::debug_struct_field1_finish(f, "Evsys9Clock",            "freq", &&self.freq)
    }
}
impl Evsys9Clock {
    /// Returns the frequency of the configured clock
    pub fn freq(&self) -> Hertz { self.freq }
}
impl Into<Hertz> for Evsys9Clock {
    fn into(self) -> Hertz { self.freq }
}
/// A typed token that indicates that the clock for the peripheral(s)
/// with the matching name has been configured.
/// The effective clock frequency is available via the `freq` method,
/// or by converting the object into a `Hertz` instance.
/// The peripheral initialization code will typically require passing
/// in this object to prove at compile time that the clock has been
/// correctly initialized.
pub struct Evsys10Clock {
    freq: Hertz,
}
#[automatically_derived]
impl ::core::fmt::Debug for Evsys10Clock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        Ok(())  //  HACK! melabr        // ::core::fmt::Formatter::debug_struct_field1_finish(f, "Evsys10Clock",            "freq", &&self.freq)
    }
}
impl Evsys10Clock {
    /// Returns the frequency of the configured clock
    pub fn freq(&self) -> Hertz { self.freq }
}
impl Into<Hertz> for Evsys10Clock {
    fn into(self) -> Hertz { self.freq }
}
/// A typed token that indicates that the clock for the peripheral(s)
/// with the matching name has been configured.
/// The effective clock frequency is available via the `freq` method,
/// or by converting the object into a `Hertz` instance.
/// The peripheral initialization code will typically require passing
/// in this object to prove at compile time that the clock has been
/// correctly initialized.
pub struct Evsys11Clock {
    freq: Hertz,
}
#[automatically_derived]
impl ::core::fmt::Debug for Evsys11Clock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        Ok(())  //  HACK! melabr        // ::core::fmt::Formatter::debug_struct_field1_finish(f, "Evsys11Clock",            "freq", &&self.freq)
    }
}
impl Evsys11Clock {
    /// Returns the frequency of the configured clock
    pub fn freq(&self) -> Hertz { self.freq }
}
impl Into<Hertz> for Evsys11Clock {
    fn into(self) -> Hertz { self.freq }
}
/// A typed token that indicates that the clock for the peripheral(s)
/// with the matching name has been configured.
/// The effective clock frequency is available via the `freq` method,
/// or by converting the object into a `Hertz` instance.
/// The peripheral initialization code will typically require passing
/// in this object to prove at compile time that the clock has been
/// correctly initialized.
pub struct Can0Clock {
    freq: Hertz,
}
#[automatically_derived]
impl ::core::fmt::Debug for Can0Clock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        Ok(())  //  HACK! melabr        // ::core::fmt::Formatter::debug_struct_field1_finish(f, "Can0Clock",            "freq", &&self.freq)
    }
}
impl Can0Clock {
    /// Returns the frequency of the configured clock
    pub fn freq(&self) -> Hertz { self.freq }
}
impl Into<Hertz> for Can0Clock {
    fn into(self) -> Hertz { self.freq }
}
/// A typed token that indicates that the clock for the peripheral(s)
/// with the matching name has been configured.
/// The effective clock frequency is available via the `freq` method,
/// or by converting the object into a `Hertz` instance.
/// The peripheral initialization code will typically require passing
/// in this object to prove at compile time that the clock has been
/// correctly initialized.
pub struct Can1Clock {
    freq: Hertz,
}
#[automatically_derived]
impl ::core::fmt::Debug for Can1Clock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        Ok(())  //  HACK! melabr        // ::core::fmt::Formatter::debug_struct_field1_finish(f, "Can1Clock",            "freq", &&self.freq)
    }
}
impl Can1Clock {
    /// Returns the frequency of the configured clock
    pub fn freq(&self) -> Hertz { self.freq }
}
impl Into<Hertz> for Can1Clock {
    fn into(self) -> Hertz { self.freq }
}
/// A typed token that indicates that the clock for the peripheral(s)
/// with the matching name has been configured.
/// The effective clock frequency is available via the `freq` method,
/// or by converting the object into a `Hertz` instance.
/// The peripheral initialization code will typically require passing
/// in this object to prove at compile time that the clock has been
/// correctly initialized.
pub struct PdecClock {
    freq: Hertz,
}
#[automatically_derived]
impl ::core::fmt::Debug for PdecClock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        Ok(())  //  HACK! melabr        // ::core::fmt::Formatter::debug_struct_field1_finish(f, "PdecClock",            "freq", &&self.freq)
    }
}
impl PdecClock {
    /// Returns the frequency of the configured clock
    pub fn freq(&self) -> Hertz { self.freq }
}
impl Into<Hertz> for PdecClock {
    fn into(self) -> Hertz { self.freq }
}
/// A typed token that indicates that the clock for the peripheral(s)
/// with the matching name has been configured.
/// The effective clock frequency is available via the `freq` method,
/// or by converting the object into a `Hertz` instance.
/// The peripheral initialization code will typically require passing
/// in this object to prove at compile time that the clock has been
/// correctly initialized.
pub struct AcClock {
    freq: Hertz,
}
#[automatically_derived]
impl ::core::fmt::Debug for AcClock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        Ok(())  //  HACK! melabr        // ::core::fmt::Formatter::debug_struct_field1_finish(f, "AcClock",            "freq", &&self.freq)
    }
}
impl AcClock {
    /// Returns the frequency of the configured clock
    pub fn freq(&self) -> Hertz { self.freq }
}
impl Into<Hertz> for AcClock {
    fn into(self) -> Hertz { self.freq }
}
/// A typed token that indicates that the clock for the peripheral(s)
/// with the matching name has been configured.
/// The effective clock frequency is available via the `freq` method,
/// or by converting the object into a `Hertz` instance.
/// The peripheral initialization code will typically require passing
/// in this object to prove at compile time that the clock has been
/// correctly initialized.
pub struct CclClock {
    freq: Hertz,
}
#[automatically_derived]
impl ::core::fmt::Debug for CclClock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        Ok(())  //  HACK! melabr        // ::core::fmt::Formatter::debug_struct_field1_finish(f, "CclClock",            "freq", &&self.freq)
    }
}
impl CclClock {
    /// Returns the frequency of the configured clock
    pub fn freq(&self) -> Hertz { self.freq }
}
impl Into<Hertz> for CclClock {
    fn into(self) -> Hertz { self.freq }
}
/// A typed token that indicates that the clock for the peripheral(s)
/// with the matching name has been configured.
/// The effective clock frequency is available via the `freq` method,
/// or by converting the object into a `Hertz` instance.
/// The peripheral initialization code will typically require passing
/// in this object to prove at compile time that the clock has been
/// correctly initialized.
pub struct DacClock {
    freq: Hertz,
}
#[automatically_derived]
impl ::core::fmt::Debug for DacClock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        Ok(())  //  HACK! melabr        // ::core::fmt::Formatter::debug_struct_field1_finish(f, "DacClock",            "freq", &&self.freq)
    }
}
impl DacClock {
    /// Returns the frequency of the configured clock
    pub fn freq(&self) -> Hertz { self.freq }
}
impl Into<Hertz> for DacClock {
    fn into(self) -> Hertz { self.freq }
}
/// A typed token that indicates that the clock for the peripheral(s)
/// with the matching name has been configured.
/// The effective clock frequency is available via the `freq` method,
/// or by converting the object into a `Hertz` instance.
/// The peripheral initialization code will typically require passing
/// in this object to prove at compile time that the clock has been
/// correctly initialized.
pub struct I2S0Clock {
    freq: Hertz,
}
#[automatically_derived]
impl ::core::fmt::Debug for I2S0Clock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        Ok(())  //  HACK! melabr        // ::core::fmt::Formatter::debug_struct_field1_finish(f, "I2S0Clock",            "freq", &&self.freq)
    }
}
impl I2S0Clock {
    /// Returns the frequency of the configured clock
    pub fn freq(&self) -> Hertz { self.freq }
}
impl Into<Hertz> for I2S0Clock {
    fn into(self) -> Hertz { self.freq }
}
/// A typed token that indicates that the clock for the peripheral(s)
/// with the matching name has been configured.
/// The effective clock frequency is available via the `freq` method,
/// or by converting the object into a `Hertz` instance.
/// The peripheral initialization code will typically require passing
/// in this object to prove at compile time that the clock has been
/// correctly initialized.
pub struct I2S1Clock {
    freq: Hertz,
}
#[automatically_derived]
impl ::core::fmt::Debug for I2S1Clock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        Ok(())  //  HACK! melabr        // ::core::fmt::Formatter::debug_struct_field1_finish(f, "I2S1Clock",            "freq", &&self.freq)
    }
}
impl I2S1Clock {
    /// Returns the frequency of the configured clock
    pub fn freq(&self) -> Hertz { self.freq }
}
impl Into<Hertz> for I2S1Clock {
    fn into(self) -> Hertz { self.freq }
}
/// A typed token that indicates that the clock for the peripheral(s)
/// with the matching name has been configured.
/// The effective clock frequency is available via the `freq` method,
/// or by converting the object into a `Hertz` instance.
/// The peripheral initialization code will typically require passing
/// in this object to prove at compile time that the clock has been
/// correctly initialized.
pub struct Sdhc0Clock {
    freq: Hertz,
}
#[automatically_derived]
impl ::core::fmt::Debug for Sdhc0Clock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        Ok(())  //  HACK! melabr        // ::core::fmt::Formatter::debug_struct_field1_finish(f, "Sdhc0Clock",            "freq", &&self.freq)
    }
}
impl Sdhc0Clock {
    /// Returns the frequency of the configured clock
    pub fn freq(&self) -> Hertz { self.freq }
}
impl Into<Hertz> for Sdhc0Clock {
    fn into(self) -> Hertz { self.freq }
}
/// A typed token that indicates that the clock for the peripheral(s)
/// with the matching name has been configured.
/// The effective clock frequency is available via the `freq` method,
/// or by converting the object into a `Hertz` instance.
/// The peripheral initialization code will typically require passing
/// in this object to prove at compile time that the clock has been
/// correctly initialized.
pub struct Sdhc1Clock {
    freq: Hertz,
}
#[automatically_derived]
impl ::core::fmt::Debug for Sdhc1Clock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        Ok(())  //  HACK! melabr        // ::core::fmt::Formatter::debug_struct_field1_finish(f, "Sdhc1Clock",            "freq", &&self.freq)
    }
}
impl Sdhc1Clock {
    /// Returns the frequency of the configured clock
    pub fn freq(&self) -> Hertz { self.freq }
}
impl Into<Hertz> for Sdhc1Clock {
    fn into(self) -> Hertz { self.freq }
}
/// A typed token that indicates that the clock for the peripheral(s)
/// with the matching name has been configured.
/// The effective clock frequency is available via the `freq` method,
/// or by converting the object into a `Hertz` instance.
/// The peripheral initialization code will typically require passing
/// in this object to prove at compile time that the clock has been
/// correctly initialized.
pub struct Cm4TraceClock {
    freq: Hertz,
}
#[automatically_derived]
impl ::core::fmt::Debug for Cm4TraceClock {
    fn fmt(&self, f: &mut ::core::fmt::Formatter) -> ::core::fmt::Result {
        Ok(())  //  HACK! melabr        // ::core::fmt::Formatter::debug_struct_field1_finish(f, "Cm4TraceClock",            "freq", &&self.freq)
    }
}
impl Cm4TraceClock {
    /// Returns the frequency of the configured clock
    pub fn freq(&self) -> Hertz { self.freq }
}
impl Into<Hertz> for Cm4TraceClock {
    fn into(self) -> Hertz { self.freq }
}
impl GenericClockController {
    /// Configure the clock for peripheral(s) that match the name
    /// of this function to use the specific clock generator.
    /// The `GClock` parameter may be one of default clocks
    /// return from `gclk0()`, `gclk1()` or a clock configured
    /// by the host application using the `configure_gclk_divider_and_source`
    /// method.
    /// Returns a typed token that proves that the clock has been configured;
    /// the peripheral initialization code will typically require that this
    /// clock token be passed in to ensure that the clock has been initialized
    /// appropriately.
    /// Returns `None` is the specified generic clock has already been
    /// configured.
    pub fn tc0_tc1(&mut self, generator: &GClock) -> Option<Tc0Tc1Clock> {
        let bits: u64 = 1 << u8::from(ClockId::TC0_TC1) as u64;
        if (self.used_clocks & bits) != 0 { return None; }
        self.used_clocks |= bits;
        self.state.enable_clock_generator(ClockId::TC0_TC1, generator.gclk);
        let freq = self.gclks[u8::from(generator.gclk) as usize];
        Some(Tc0Tc1Clock { freq })
    }
    /// Configure the clock for peripheral(s) that match the name
    /// of this function to use the specific clock generator.
    /// The `GClock` parameter may be one of default clocks
    /// return from `gclk0()`, `gclk1()` or a clock configured
    /// by the host application using the `configure_gclk_divider_and_source`
    /// method.
    /// Returns a typed token that proves that the clock has been configured;
    /// the peripheral initialization code will typically require that this
    /// clock token be passed in to ensure that the clock has been initialized
    /// appropriately.
    /// Returns `None` is the specified generic clock has already been
    /// configured.
    pub fn tcc0_tcc1(&mut self, generator: &GClock) -> Option<Tcc0Tcc1Clock> {
        let bits: u64 = 1 << u8::from(ClockId::TCC0_TCC1) as u64;
        if (self.used_clocks & bits) != 0 { return None; }
        self.used_clocks |= bits;
        self.state.enable_clock_generator(ClockId::TCC0_TCC1, generator.gclk);
        let freq = self.gclks[u8::from(generator.gclk) as usize];
        Some(Tcc0Tcc1Clock { freq })
    }
    /// Configure the clock for peripheral(s) that match the name
    /// of this function to use the specific clock generator.
    /// The `GClock` parameter may be one of default clocks
    /// return from `gclk0()`, `gclk1()` or a clock configured
    /// by the host application using the `configure_gclk_divider_and_source`
    /// method.
    /// Returns a typed token that proves that the clock has been configured;
    /// the peripheral initialization code will typically require that this
    /// clock token be passed in to ensure that the clock has been initialized
    /// appropriately.
    /// Returns `None` is the specified generic clock has already been
    /// configured.
    pub fn tc2_tc3(&mut self, generator: &GClock) -> Option<Tc2Tc3Clock> {
        let bits: u64 = 1 << u8::from(ClockId::TC2_TC3) as u64;
        if (self.used_clocks & bits) != 0 { return None; }
        self.used_clocks |= bits;
        self.state.enable_clock_generator(ClockId::TC2_TC3, generator.gclk);
        let freq = self.gclks[u8::from(generator.gclk) as usize];
        Some(Tc2Tc3Clock { freq })
    }
    /// Configure the clock for peripheral(s) that match the name
    /// of this function to use the specific clock generator.
    /// The `GClock` parameter may be one of default clocks
    /// return from `gclk0()`, `gclk1()` or a clock configured
    /// by the host application using the `configure_gclk_divider_and_source`
    /// method.
    /// Returns a typed token that proves that the clock has been configured;
    /// the peripheral initialization code will typically require that this
    /// clock token be passed in to ensure that the clock has been initialized
    /// appropriately.
    /// Returns `None` is the specified generic clock has already been
    /// configured.
    pub fn tcc2_tcc3(&mut self, generator: &GClock) -> Option<Tcc2Tcc3Clock> {
        let bits: u64 = 1 << u8::from(ClockId::TCC2_TCC3) as u64;
        if (self.used_clocks & bits) != 0 { return None; }
        self.used_clocks |= bits;
        self.state.enable_clock_generator(ClockId::TCC2_TCC3, generator.gclk);
        let freq = self.gclks[u8::from(generator.gclk) as usize];
        Some(Tcc2Tcc3Clock { freq })
    }
    /// Configure the clock for peripheral(s) that match the name
    /// of this function to use the specific clock generator.
    /// The `GClock` parameter may be one of default clocks
    /// return from `gclk0()`, `gclk1()` or a clock configured
    /// by the host application using the `configure_gclk_divider_and_source`
    /// method.
    /// Returns a typed token that proves that the clock has been configured;
    /// the peripheral initialization code will typically require that this
    /// clock token be passed in to ensure that the clock has been initialized
    /// appropriately.
    /// Returns `None` is the specified generic clock has already been
    /// configured.
    pub fn tc4_tc5(&mut self, generator: &GClock) -> Option<Tc4Tc5Clock> {
        let bits: u64 = 1 << u8::from(ClockId::TC4_TC5) as u64;
        if (self.used_clocks & bits) != 0 { return None; }
        self.used_clocks |= bits;
        self.state.enable_clock_generator(ClockId::TC4_TC5, generator.gclk);
        let freq = self.gclks[u8::from(generator.gclk) as usize];
        Some(Tc4Tc5Clock { freq })
    }
    /// Configure the clock for peripheral(s) that match the name
    /// of this function to use the specific clock generator.
    /// The `GClock` parameter may be one of default clocks
    /// return from `gclk0()`, `gclk1()` or a clock configured
    /// by the host application using the `configure_gclk_divider_and_source`
    /// method.
    /// Returns a typed token that proves that the clock has been configured;
    /// the peripheral initialization code will typically require that this
    /// clock token be passed in to ensure that the clock has been initialized
    /// appropriately.
    /// Returns `None` is the specified generic clock has already been
    /// configured.
    pub fn tcc4(&mut self, generator: &GClock) -> Option<Tcc4Clock> {
        let bits: u64 = 1 << u8::from(ClockId::TCC4) as u64;
        if (self.used_clocks & bits) != 0 { return None; }
        self.used_clocks |= bits;
        self.state.enable_clock_generator(ClockId::TCC4, generator.gclk);
        let freq = self.gclks[u8::from(generator.gclk) as usize];
        Some(Tcc4Clock { freq })
    }
    /// Configure the clock for peripheral(s) that match the name
    /// of this function to use the specific clock generator.
    /// The `GClock` parameter may be one of default clocks
    /// return from `gclk0()`, `gclk1()` or a clock configured
    /// by the host application using the `configure_gclk_divider_and_source`
    /// method.
    /// Returns a typed token that proves that the clock has been configured;
    /// the peripheral initialization code will typically require that this
    /// clock token be passed in to ensure that the clock has been initialized
    /// appropriately.
    /// Returns `None` is the specified generic clock has already been
    /// configured.
    pub fn tc6_tc7(&mut self, generator: &GClock) -> Option<Tc6Tc7Clock> {
        let bits: u64 = 1 << u8::from(ClockId::TC6_TC7) as u64;
        if (self.used_clocks & bits) != 0 { return None; }
        self.used_clocks |= bits;
        self.state.enable_clock_generator(ClockId::TC6_TC7, generator.gclk);
        let freq = self.gclks[u8::from(generator.gclk) as usize];
        Some(Tc6Tc7Clock { freq })
    }
    /// Configure the clock for peripheral(s) that match the name
    /// of this function to use the specific clock generator.
    /// The `GClock` parameter may be one of default clocks
    /// return from `gclk0()`, `gclk1()` or a clock configured
    /// by the host application using the `configure_gclk_divider_and_source`
    /// method.
    /// Returns a typed token that proves that the clock has been configured;
    /// the peripheral initialization code will typically require that this
    /// clock token be passed in to ensure that the clock has been initialized
    /// appropriately.
    /// Returns `None` is the specified generic clock has already been
    /// configured.
    pub fn sercom0_core(&mut self, generator: &GClock)
        -> Option<Sercom0CoreClock> {
        let bits: u64 = 1 << u8::from(ClockId::SERCOM0_CORE) as u64;
        if (self.used_clocks & bits) != 0 { return None; }
        self.used_clocks |= bits;
        self.state.enable_clock_generator(ClockId::SERCOM0_CORE,
            generator.gclk);
        let freq = self.gclks[u8::from(generator.gclk) as usize];
        Some(Sercom0CoreClock { freq })
    }
    /// Configure the clock for peripheral(s) that match the name
    /// of this function to use the specific clock generator.
    /// The `GClock` parameter may be one of default clocks
    /// return from `gclk0()`, `gclk1()` or a clock configured
    /// by the host application using the `configure_gclk_divider_and_source`
    /// method.
    /// Returns a typed token that proves that the clock has been configured;
    /// the peripheral initialization code will typically require that this
    /// clock token be passed in to ensure that the clock has been initialized
    /// appropriately.
    /// Returns `None` is the specified generic clock has already been
    /// configured.
    pub fn sercom1_core(&mut self, generator: &GClock)
        -> Option<Sercom1CoreClock> {
        let bits: u64 = 1 << u8::from(ClockId::SERCOM1_CORE) as u64;
        if (self.used_clocks & bits) != 0 { return None; }
        self.used_clocks |= bits;
        self.state.enable_clock_generator(ClockId::SERCOM1_CORE,
            generator.gclk);
        let freq = self.gclks[u8::from(generator.gclk) as usize];
        Some(Sercom1CoreClock { freq })
    }
    /// Configure the clock for peripheral(s) that match the name
    /// of this function to use the specific clock generator.
    /// The `GClock` parameter may be one of default clocks
    /// return from `gclk0()`, `gclk1()` or a clock configured
    /// by the host application using the `configure_gclk_divider_and_source`
    /// method.
    /// Returns a typed token that proves that the clock has been configured;
    /// the peripheral initialization code will typically require that this
    /// clock token be passed in to ensure that the clock has been initialized
    /// appropriately.
    /// Returns `None` is the specified generic clock has already been
    /// configured.
    pub fn sercom2_core(&mut self, generator: &GClock)
        -> Option<Sercom2CoreClock> {
        let bits: u64 = 1 << u8::from(ClockId::SERCOM2_CORE) as u64;
        if (self.used_clocks & bits) != 0 { return None; }
        self.used_clocks |= bits;
        self.state.enable_clock_generator(ClockId::SERCOM2_CORE,
            generator.gclk);
        let freq = self.gclks[u8::from(generator.gclk) as usize];
        Some(Sercom2CoreClock { freq })
    }
    /// Configure the clock for peripheral(s) that match the name
    /// of this function to use the specific clock generator.
    /// The `GClock` parameter may be one of default clocks
    /// return from `gclk0()`, `gclk1()` or a clock configured
    /// by the host application using the `configure_gclk_divider_and_source`
    /// method.
    /// Returns a typed token that proves that the clock has been configured;
    /// the peripheral initialization code will typically require that this
    /// clock token be passed in to ensure that the clock has been initialized
    /// appropriately.
    /// Returns `None` is the specified generic clock has already been
    /// configured.
    pub fn sercom3_core(&mut self, generator: &GClock)
        -> Option<Sercom3CoreClock> {
        let bits: u64 = 1 << u8::from(ClockId::SERCOM3_CORE) as u64;
        if (self.used_clocks & bits) != 0 { return None; }
        self.used_clocks |= bits;
        self.state.enable_clock_generator(ClockId::SERCOM3_CORE,
            generator.gclk);
        let freq = self.gclks[u8::from(generator.gclk) as usize];
        Some(Sercom3CoreClock { freq })
    }
    /// Configure the clock for peripheral(s) that match the name
    /// of this function to use the specific clock generator.
    /// The `GClock` parameter may be one of default clocks
    /// return from `gclk0()`, `gclk1()` or a clock configured
    /// by the host application using the `configure_gclk_divider_and_source`
    /// method.
    /// Returns a typed token that proves that the clock has been configured;
    /// the peripheral initialization code will typically require that this
    /// clock token be passed in to ensure that the clock has been initialized
    /// appropriately.
    /// Returns `None` is the specified generic clock has already been
    /// configured.
    pub fn sercom4_core(&mut self, generator: &GClock)
        -> Option<Sercom4CoreClock> {
        let bits: u64 = 1 << u8::from(ClockId::SERCOM4_CORE) as u64;
        if (self.used_clocks & bits) != 0 { return None; }
        self.used_clocks |= bits;
        self.state.enable_clock_generator(ClockId::SERCOM4_CORE,
            generator.gclk);
        let freq = self.gclks[u8::from(generator.gclk) as usize];
        Some(Sercom4CoreClock { freq })
    }
    /// Configure the clock for peripheral(s) that match the name
    /// of this function to use the specific clock generator.
    /// The `GClock` parameter may be one of default clocks
    /// return from `gclk0()`, `gclk1()` or a clock configured
    /// by the host application using the `configure_gclk_divider_and_source`
    /// method.
    /// Returns a typed token that proves that the clock has been configured;
    /// the peripheral initialization code will typically require that this
    /// clock token be passed in to ensure that the clock has been initialized
    /// appropriately.
    /// Returns `None` is the specified generic clock has already been
    /// configured.
    pub fn sercom5_core(&mut self, generator: &GClock)
        -> Option<Sercom5CoreClock> {
        let bits: u64 = 1 << u8::from(ClockId::SERCOM5_CORE) as u64;
        if (self.used_clocks & bits) != 0 { return None; }
        self.used_clocks |= bits;
        self.state.enable_clock_generator(ClockId::SERCOM5_CORE,
            generator.gclk);
        let freq = self.gclks[u8::from(generator.gclk) as usize];
        Some(Sercom5CoreClock { freq })
    }
    /// Configure the clock for peripheral(s) that match the name
    /// of this function to use the specific clock generator.
    /// The `GClock` parameter may be one of default clocks
    /// return from `gclk0()`, `gclk1()` or a clock configured
    /// by the host application using the `configure_gclk_divider_and_source`
    /// method.
    /// Returns a typed token that proves that the clock has been configured;
    /// the peripheral initialization code will typically require that this
    /// clock token be passed in to ensure that the clock has been initialized
    /// appropriately.
    /// Returns `None` is the specified generic clock has already been
    /// configured.
    pub fn usb(&mut self, generator: &GClock) -> Option<UsbClock> {
        let bits: u64 = 1 << u8::from(ClockId::USB) as u64;
        if (self.used_clocks & bits) != 0 { return None; }
        self.used_clocks |= bits;
        self.state.enable_clock_generator(ClockId::USB, generator.gclk);
        let freq = self.gclks[u8::from(generator.gclk) as usize];
        Some(UsbClock { freq })
    }
    /// Configure the clock for peripheral(s) that match the name
    /// of this function to use the specific clock generator.
    /// The `GClock` parameter may be one of default clocks
    /// return from `gclk0()`, `gclk1()` or a clock configured
    /// by the host application using the `configure_gclk_divider_and_source`
    /// method.
    /// Returns a typed token that proves that the clock has been configured;
    /// the peripheral initialization code will typically require that this
    /// clock token be passed in to ensure that the clock has been initialized
    /// appropriately.
    /// Returns `None` is the specified generic clock has already been
    /// configured.
    pub fn adc0(&mut self, generator: &GClock) -> Option<Adc0Clock> {
        let bits: u64 = 1 << u8::from(ClockId::ADC0) as u64;
        if (self.used_clocks & bits) != 0 { return None; }
        self.used_clocks |= bits;
        self.state.enable_clock_generator(ClockId::ADC0, generator.gclk);
        let freq = self.gclks[u8::from(generator.gclk) as usize];
        Some(Adc0Clock { freq })
    }
    /// Configure the clock for peripheral(s) that match the name
    /// of this function to use the specific clock generator.
    /// The `GClock` parameter may be one of default clocks
    /// return from `gclk0()`, `gclk1()` or a clock configured
    /// by the host application using the `configure_gclk_divider_and_source`
    /// method.
    /// Returns a typed token that proves that the clock has been configured;
    /// the peripheral initialization code will typically require that this
    /// clock token be passed in to ensure that the clock has been initialized
    /// appropriately.
    /// Returns `None` is the specified generic clock has already been
    /// configured.
    pub fn adc1(&mut self, generator: &GClock) -> Option<Adc1Clock> {
        let bits: u64 = 1 << u8::from(ClockId::ADC1) as u64;
        if (self.used_clocks & bits) != 0 { return None; }
        self.used_clocks |= bits;
        self.state.enable_clock_generator(ClockId::ADC1, generator.gclk);
        let freq = self.gclks[u8::from(generator.gclk) as usize];
        Some(Adc1Clock { freq })
    }
    /// Configure the clock for peripheral(s) that match the name
    /// of this function to use the specific clock generator.
    /// The `GClock` parameter may be one of default clocks
    /// return from `gclk0()`, `gclk1()` or a clock configured
    /// by the host application using the `configure_gclk_divider_and_source`
    /// method.
    /// Returns a typed token that proves that the clock has been configured;
    /// the peripheral initialization code will typically require that this
    /// clock token be passed in to ensure that the clock has been initialized
    /// appropriately.
    /// Returns `None` is the specified generic clock has already been
    /// configured.
    pub fn eic(&mut self, generator: &GClock) -> Option<EicClock> {
        let bits: u64 = 1 << u8::from(ClockId::EIC) as u64;
        if (self.used_clocks & bits) != 0 { return None; }
        self.used_clocks |= bits;
        self.state.enable_clock_generator(ClockId::EIC, generator.gclk);
        let freq = self.gclks[u8::from(generator.gclk) as usize];
        Some(EicClock { freq })
    }
    /// Configure the clock for peripheral(s) that match the name
    /// of this function to use the specific clock generator.
    /// The `GClock` parameter may be one of default clocks
    /// return from `gclk0()`, `gclk1()` or a clock configured
    /// by the host application using the `configure_gclk_divider_and_source`
    /// method.
    /// Returns a typed token that proves that the clock has been configured;
    /// the peripheral initialization code will typically require that this
    /// clock token be passed in to ensure that the clock has been initialized
    /// appropriately.
    /// Returns `None` is the specified generic clock has already been
    /// configured.
    pub fn freq_m_msr(&mut self, generator: &GClock)
        -> Option<FreqmMsrClock> {
        let bits: u64 = 1 << u8::from(ClockId::FREQM_MSR) as u64;
        if (self.used_clocks & bits) != 0 { return None; }
        self.used_clocks |= bits;
        self.state.enable_clock_generator(ClockId::FREQM_MSR, generator.gclk);
        let freq = self.gclks[u8::from(generator.gclk) as usize];
        Some(FreqmMsrClock { freq })
    }
    /// Configure the clock for peripheral(s) that match the name
    /// of this function to use the specific clock generator.
    /// The `GClock` parameter may be one of default clocks
    /// return from `gclk0()`, `gclk1()` or a clock configured
    /// by the host application using the `configure_gclk_divider_and_source`
    /// method.
    /// Returns a typed token that proves that the clock has been configured;
    /// the peripheral initialization code will typically require that this
    /// clock token be passed in to ensure that the clock has been initialized
    /// appropriately.
    /// Returns `None` is the specified generic clock has already been
    /// configured.
    pub fn freq_m_ref(&mut self, generator: &GClock)
        -> Option<FreqmRefClock> {
        let bits: u64 = 1 << u8::from(ClockId::FREQM_REF) as u64;
        if (self.used_clocks & bits) != 0 { return None; }
        self.used_clocks |= bits;
        self.state.enable_clock_generator(ClockId::FREQM_REF, generator.gclk);
        let freq = self.gclks[u8::from(generator.gclk) as usize];
        Some(FreqmRefClock { freq })
    }
    /// Configure the clock for peripheral(s) that match the name
    /// of this function to use the specific clock generator.
    /// The `GClock` parameter may be one of default clocks
    /// return from `gclk0()`, `gclk1()` or a clock configured
    /// by the host application using the `configure_gclk_divider_and_source`
    /// method.
    /// Returns a typed token that proves that the clock has been configured;
    /// the peripheral initialization code will typically require that this
    /// clock token be passed in to ensure that the clock has been initialized
    /// appropriately.
    /// Returns `None` is the specified generic clock has already been
    /// configured.
    pub fn evsys0(&mut self, generator: &GClock) -> Option<Evsys0Clock> {
        let bits: u64 = 1 << u8::from(ClockId::EVSYS0) as u64;
        if (self.used_clocks & bits) != 0 { return None; }
        self.used_clocks |= bits;
        self.state.enable_clock_generator(ClockId::EVSYS0, generator.gclk);
        let freq = self.gclks[u8::from(generator.gclk) as usize];
        Some(Evsys0Clock { freq })
    }
    /// Configure the clock for peripheral(s) that match the name
    /// of this function to use the specific clock generator.
    /// The `GClock` parameter may be one of default clocks
    /// return from `gclk0()`, `gclk1()` or a clock configured
    /// by the host application using the `configure_gclk_divider_and_source`
    /// method.
    /// Returns a typed token that proves that the clock has been configured;
    /// the peripheral initialization code will typically require that this
    /// clock token be passed in to ensure that the clock has been initialized
    /// appropriately.
    /// Returns `None` is the specified generic clock has already been
    /// configured.
    pub fn evsys1(&mut self, generator: &GClock) -> Option<Evsys1Clock> {
        let bits: u64 = 1 << u8::from(ClockId::EVSYS1) as u64;
        if (self.used_clocks & bits) != 0 { return None; }
        self.used_clocks |= bits;
        self.state.enable_clock_generator(ClockId::EVSYS1, generator.gclk);
        let freq = self.gclks[u8::from(generator.gclk) as usize];
        Some(Evsys1Clock { freq })
    }
    /// Configure the clock for peripheral(s) that match the name
    /// of this function to use the specific clock generator.
    /// The `GClock` parameter may be one of default clocks
    /// return from `gclk0()`, `gclk1()` or a clock configured
    /// by the host application using the `configure_gclk_divider_and_source`
    /// method.
    /// Returns a typed token that proves that the clock has been configured;
    /// the peripheral initialization code will typically require that this
    /// clock token be passed in to ensure that the clock has been initialized
    /// appropriately.
    /// Returns `None` is the specified generic clock has already been
    /// configured.
    pub fn evsys2(&mut self, generator: &GClock) -> Option<Evsys2Clock> {
        let bits: u64 = 1 << u8::from(ClockId::EVSYS2) as u64;
        if (self.used_clocks & bits) != 0 { return None; }
        self.used_clocks |= bits;
        self.state.enable_clock_generator(ClockId::EVSYS2, generator.gclk);
        let freq = self.gclks[u8::from(generator.gclk) as usize];
        Some(Evsys2Clock { freq })
    }
    /// Configure the clock for peripheral(s) that match the name
    /// of this function to use the specific clock generator.
    /// The `GClock` parameter may be one of default clocks
    /// return from `gclk0()`, `gclk1()` or a clock configured
    /// by the host application using the `configure_gclk_divider_and_source`
    /// method.
    /// Returns a typed token that proves that the clock has been configured;
    /// the peripheral initialization code will typically require that this
    /// clock token be passed in to ensure that the clock has been initialized
    /// appropriately.
    /// Returns `None` is the specified generic clock has already been
    /// configured.
    pub fn evsys3(&mut self, generator: &GClock) -> Option<Evsys3Clock> {
        let bits: u64 = 1 << u8::from(ClockId::EVSYS3) as u64;
        if (self.used_clocks & bits) != 0 { return None; }
        self.used_clocks |= bits;
        self.state.enable_clock_generator(ClockId::EVSYS3, generator.gclk);
        let freq = self.gclks[u8::from(generator.gclk) as usize];
        Some(Evsys3Clock { freq })
    }
    /// Configure the clock for peripheral(s) that match the name
    /// of this function to use the specific clock generator.
    /// The `GClock` parameter may be one of default clocks
    /// return from `gclk0()`, `gclk1()` or a clock configured
    /// by the host application using the `configure_gclk_divider_and_source`
    /// method.
    /// Returns a typed token that proves that the clock has been configured;
    /// the peripheral initialization code will typically require that this
    /// clock token be passed in to ensure that the clock has been initialized
    /// appropriately.
    /// Returns `None` is the specified generic clock has already been
    /// configured.
    pub fn evsys4(&mut self, generator: &GClock) -> Option<Evsys4Clock> {
        let bits: u64 = 1 << u8::from(ClockId::EVSYS4) as u64;
        if (self.used_clocks & bits) != 0 { return None; }
        self.used_clocks |= bits;
        self.state.enable_clock_generator(ClockId::EVSYS4, generator.gclk);
        let freq = self.gclks[u8::from(generator.gclk) as usize];
        Some(Evsys4Clock { freq })
    }
    /// Configure the clock for peripheral(s) that match the name
    /// of this function to use the specific clock generator.
    /// The `GClock` parameter may be one of default clocks
    /// return from `gclk0()`, `gclk1()` or a clock configured
    /// by the host application using the `configure_gclk_divider_and_source`
    /// method.
    /// Returns a typed token that proves that the clock has been configured;
    /// the peripheral initialization code will typically require that this
    /// clock token be passed in to ensure that the clock has been initialized
    /// appropriately.
    /// Returns `None` is the specified generic clock has already been
    /// configured.
    pub fn evsys5(&mut self, generator: &GClock) -> Option<Evsys5Clock> {
        let bits: u64 = 1 << u8::from(ClockId::EVSYS5) as u64;
        if (self.used_clocks & bits) != 0 { return None; }
        self.used_clocks |= bits;
        self.state.enable_clock_generator(ClockId::EVSYS5, generator.gclk);
        let freq = self.gclks[u8::from(generator.gclk) as usize];
        Some(Evsys5Clock { freq })
    }
    /// Configure the clock for peripheral(s) that match the name
    /// of this function to use the specific clock generator.
    /// The `GClock` parameter may be one of default clocks
    /// return from `gclk0()`, `gclk1()` or a clock configured
    /// by the host application using the `configure_gclk_divider_and_source`
    /// method.
    /// Returns a typed token that proves that the clock has been configured;
    /// the peripheral initialization code will typically require that this
    /// clock token be passed in to ensure that the clock has been initialized
    /// appropriately.
    /// Returns `None` is the specified generic clock has already been
    /// configured.
    pub fn evsys6(&mut self, generator: &GClock) -> Option<Evsys6Clock> {
        let bits: u64 = 1 << u8::from(ClockId::EVSYS6) as u64;
        if (self.used_clocks & bits) != 0 { return None; }
        self.used_clocks |= bits;
        self.state.enable_clock_generator(ClockId::EVSYS6, generator.gclk);
        let freq = self.gclks[u8::from(generator.gclk) as usize];
        Some(Evsys6Clock { freq })
    }
    /// Configure the clock for peripheral(s) that match the name
    /// of this function to use the specific clock generator.
    /// The `GClock` parameter may be one of default clocks
    /// return from `gclk0()`, `gclk1()` or a clock configured
    /// by the host application using the `configure_gclk_divider_and_source`
    /// method.
    /// Returns a typed token that proves that the clock has been configured;
    /// the peripheral initialization code will typically require that this
    /// clock token be passed in to ensure that the clock has been initialized
    /// appropriately.
    /// Returns `None` is the specified generic clock has already been
    /// configured.
    pub fn evsys7(&mut self, generator: &GClock) -> Option<Evsys7Clock> {
        let bits: u64 = 1 << u8::from(ClockId::EVSYS7) as u64;
        if (self.used_clocks & bits) != 0 { return None; }
        self.used_clocks |= bits;
        self.state.enable_clock_generator(ClockId::EVSYS7, generator.gclk);
        let freq = self.gclks[u8::from(generator.gclk) as usize];
        Some(Evsys7Clock { freq })
    }
    /// Configure the clock for peripheral(s) that match the name
    /// of this function to use the specific clock generator.
    /// The `GClock` parameter may be one of default clocks
    /// return from `gclk0()`, `gclk1()` or a clock configured
    /// by the host application using the `configure_gclk_divider_and_source`
    /// method.
    /// Returns a typed token that proves that the clock has been configured;
    /// the peripheral initialization code will typically require that this
    /// clock token be passed in to ensure that the clock has been initialized
    /// appropriately.
    /// Returns `None` is the specified generic clock has already been
    /// configured.
    pub fn evsys8(&mut self, generator: &GClock) -> Option<Evsys8Clock> {
        let bits: u64 = 1 << u8::from(ClockId::EVSYS8) as u64;
        if (self.used_clocks & bits) != 0 { return None; }
        self.used_clocks |= bits;
        self.state.enable_clock_generator(ClockId::EVSYS8, generator.gclk);
        let freq = self.gclks[u8::from(generator.gclk) as usize];
        Some(Evsys8Clock { freq })
    }
    /// Configure the clock for peripheral(s) that match the name
    /// of this function to use the specific clock generator.
    /// The `GClock` parameter may be one of default clocks
    /// return from `gclk0()`, `gclk1()` or a clock configured
    /// by the host application using the `configure_gclk_divider_and_source`
    /// method.
    /// Returns a typed token that proves that the clock has been configured;
    /// the peripheral initialization code will typically require that this
    /// clock token be passed in to ensure that the clock has been initialized
    /// appropriately.
    /// Returns `None` is the specified generic clock has already been
    /// configured.
    pub fn evsys9(&mut self, generator: &GClock) -> Option<Evsys9Clock> {
        let bits: u64 = 1 << u8::from(ClockId::EVSYS9) as u64;
        if (self.used_clocks & bits) != 0 { return None; }
        self.used_clocks |= bits;
        self.state.enable_clock_generator(ClockId::EVSYS9, generator.gclk);
        let freq = self.gclks[u8::from(generator.gclk) as usize];
        Some(Evsys9Clock { freq })
    }
    /// Configure the clock for peripheral(s) that match the name
    /// of this function to use the specific clock generator.
    /// The `GClock` parameter may be one of default clocks
    /// return from `gclk0()`, `gclk1()` or a clock configured
    /// by the host application using the `configure_gclk_divider_and_source`
    /// method.
    /// Returns a typed token that proves that the clock has been configured;
    /// the peripheral initialization code will typically require that this
    /// clock token be passed in to ensure that the clock has been initialized
    /// appropriately.
    /// Returns `None` is the specified generic clock has already been
    /// configured.
    pub fn evsys10(&mut self, generator: &GClock) -> Option<Evsys10Clock> {
        let bits: u64 = 1 << u8::from(ClockId::EVSYS10) as u64;
        if (self.used_clocks & bits) != 0 { return None; }
        self.used_clocks |= bits;
        self.state.enable_clock_generator(ClockId::EVSYS10, generator.gclk);
        let freq = self.gclks[u8::from(generator.gclk) as usize];
        Some(Evsys10Clock { freq })
    }
    /// Configure the clock for peripheral(s) that match the name
    /// of this function to use the specific clock generator.
    /// The `GClock` parameter may be one of default clocks
    /// return from `gclk0()`, `gclk1()` or a clock configured
    /// by the host application using the `configure_gclk_divider_and_source`
    /// method.
    /// Returns a typed token that proves that the clock has been configured;
    /// the peripheral initialization code will typically require that this
    /// clock token be passed in to ensure that the clock has been initialized
    /// appropriately.
    /// Returns `None` is the specified generic clock has already been
    /// configured.
    pub fn evsys11(&mut self, generator: &GClock) -> Option<Evsys11Clock> {
        let bits: u64 = 1 << u8::from(ClockId::EVSYS11) as u64;
        if (self.used_clocks & bits) != 0 { return None; }
        self.used_clocks |= bits;
        self.state.enable_clock_generator(ClockId::EVSYS11, generator.gclk);
        let freq = self.gclks[u8::from(generator.gclk) as usize];
        Some(Evsys11Clock { freq })
    }
    /// Configure the clock for peripheral(s) that match the name
    /// of this function to use the specific clock generator.
    /// The `GClock` parameter may be one of default clocks
    /// return from `gclk0()`, `gclk1()` or a clock configured
    /// by the host application using the `configure_gclk_divider_and_source`
    /// method.
    /// Returns a typed token that proves that the clock has been configured;
    /// the peripheral initialization code will typically require that this
    /// clock token be passed in to ensure that the clock has been initialized
    /// appropriately.
    /// Returns `None` is the specified generic clock has already been
    /// configured.
    pub fn can0(&mut self, generator: &GClock) -> Option<Can0Clock> {
        let bits: u64 = 1 << u8::from(ClockId::CAN0) as u64;
        if (self.used_clocks & bits) != 0 { return None; }
        self.used_clocks |= bits;
        self.state.enable_clock_generator(ClockId::CAN0, generator.gclk);
        let freq = self.gclks[u8::from(generator.gclk) as usize];
        Some(Can0Clock { freq })
    }
    /// Configure the clock for peripheral(s) that match the name
    /// of this function to use the specific clock generator.
    /// The `GClock` parameter may be one of default clocks
    /// return from `gclk0()`, `gclk1()` or a clock configured
    /// by the host application using the `configure_gclk_divider_and_source`
    /// method.
    /// Returns a typed token that proves that the clock has been configured;
    /// the peripheral initialization code will typically require that this
    /// clock token be passed in to ensure that the clock has been initialized
    /// appropriately.
    /// Returns `None` is the specified generic clock has already been
    /// configured.
    pub fn can1(&mut self, generator: &GClock) -> Option<Can1Clock> {
        let bits: u64 = 1 << u8::from(ClockId::CAN1) as u64;
        if (self.used_clocks & bits) != 0 { return None; }
        self.used_clocks |= bits;
        self.state.enable_clock_generator(ClockId::CAN1, generator.gclk);
        let freq = self.gclks[u8::from(generator.gclk) as usize];
        Some(Can1Clock { freq })
    }
    /// Configure the clock for peripheral(s) that match the name
    /// of this function to use the specific clock generator.
    /// The `GClock` parameter may be one of default clocks
    /// return from `gclk0()`, `gclk1()` or a clock configured
    /// by the host application using the `configure_gclk_divider_and_source`
    /// method.
    /// Returns a typed token that proves that the clock has been configured;
    /// the peripheral initialization code will typically require that this
    /// clock token be passed in to ensure that the clock has been initialized
    /// appropriately.
    /// Returns `None` is the specified generic clock has already been
    /// configured.
    pub fn pdec(&mut self, generator: &GClock) -> Option<PdecClock> {
        let bits: u64 = 1 << u8::from(ClockId::PDEC) as u64;
        if (self.used_clocks & bits) != 0 { return None; }
        self.used_clocks |= bits;
        self.state.enable_clock_generator(ClockId::PDEC, generator.gclk);
        let freq = self.gclks[u8::from(generator.gclk) as usize];
        Some(PdecClock { freq })
    }
    /// Configure the clock for peripheral(s) that match the name
    /// of this function to use the specific clock generator.
    /// The `GClock` parameter may be one of default clocks
    /// return from `gclk0()`, `gclk1()` or a clock configured
    /// by the host application using the `configure_gclk_divider_and_source`
    /// method.
    /// Returns a typed token that proves that the clock has been configured;
    /// the peripheral initialization code will typically require that this
    /// clock token be passed in to ensure that the clock has been initialized
    /// appropriately.
    /// Returns `None` is the specified generic clock has already been
    /// configured.
    pub fn ac(&mut self, generator: &GClock) -> Option<AcClock> {
        let bits: u64 = 1 << u8::from(ClockId::AC) as u64;
        if (self.used_clocks & bits) != 0 { return None; }
        self.used_clocks |= bits;
        self.state.enable_clock_generator(ClockId::AC, generator.gclk);
        let freq = self.gclks[u8::from(generator.gclk) as usize];
        Some(AcClock { freq })
    }
    /// Configure the clock for peripheral(s) that match the name
    /// of this function to use the specific clock generator.
    /// The `GClock` parameter may be one of default clocks
    /// return from `gclk0()`, `gclk1()` or a clock configured
    /// by the host application using the `configure_gclk_divider_and_source`
    /// method.
    /// Returns a typed token that proves that the clock has been configured;
    /// the peripheral initialization code will typically require that this
    /// clock token be passed in to ensure that the clock has been initialized
    /// appropriately.
    /// Returns `None` is the specified generic clock has already been
    /// configured.
    pub fn ccl(&mut self, generator: &GClock) -> Option<CclClock> {
        let bits: u64 = 1 << u8::from(ClockId::CCL) as u64;
        if (self.used_clocks & bits) != 0 { return None; }
        self.used_clocks |= bits;
        self.state.enable_clock_generator(ClockId::CCL, generator.gclk);
        let freq = self.gclks[u8::from(generator.gclk) as usize];
        Some(CclClock { freq })
    }
    /// Configure the clock for peripheral(s) that match the name
    /// of this function to use the specific clock generator.
    /// The `GClock` parameter may be one of default clocks
    /// return from `gclk0()`, `gclk1()` or a clock configured
    /// by the host application using the `configure_gclk_divider_and_source`
    /// method.
    /// Returns a typed token that proves that the clock has been configured;
    /// the peripheral initialization code will typically require that this
    /// clock token be passed in to ensure that the clock has been initialized
    /// appropriately.
    /// Returns `None` is the specified generic clock has already been
    /// configured.
    pub fn dac(&mut self, generator: &GClock) -> Option<DacClock> {
        let bits: u64 = 1 << u8::from(ClockId::DAC) as u64;
        if (self.used_clocks & bits) != 0 { return None; }
        self.used_clocks |= bits;
        self.state.enable_clock_generator(ClockId::DAC, generator.gclk);
        let freq = self.gclks[u8::from(generator.gclk) as usize];
        Some(DacClock { freq })
    }
    /// Configure the clock for peripheral(s) that match the name
    /// of this function to use the specific clock generator.
    /// The `GClock` parameter may be one of default clocks
    /// return from `gclk0()`, `gclk1()` or a clock configured
    /// by the host application using the `configure_gclk_divider_and_source`
    /// method.
    /// Returns a typed token that proves that the clock has been configured;
    /// the peripheral initialization code will typically require that this
    /// clock token be passed in to ensure that the clock has been initialized
    /// appropriately.
    /// Returns `None` is the specified generic clock has already been
    /// configured.
    pub fn i2s0(&mut self, generator: &GClock) -> Option<I2S0Clock> {
        let bits: u64 = 1 << u8::from(ClockId::I2S0) as u64;
        if (self.used_clocks & bits) != 0 { return None; }
        self.used_clocks |= bits;
        self.state.enable_clock_generator(ClockId::I2S0, generator.gclk);
        let freq = self.gclks[u8::from(generator.gclk) as usize];
        Some(I2S0Clock { freq })
    }
    /// Configure the clock for peripheral(s) that match the name
    /// of this function to use the specific clock generator.
    /// The `GClock` parameter may be one of default clocks
    /// return from `gclk0()`, `gclk1()` or a clock configured
    /// by the host application using the `configure_gclk_divider_and_source`
    /// method.
    /// Returns a typed token that proves that the clock has been configured;
    /// the peripheral initialization code will typically require that this
    /// clock token be passed in to ensure that the clock has been initialized
    /// appropriately.
    /// Returns `None` is the specified generic clock has already been
    /// configured.
    pub fn i2s1(&mut self, generator: &GClock) -> Option<I2S1Clock> {
        let bits: u64 = 1 << u8::from(ClockId::I2S1) as u64;
        if (self.used_clocks & bits) != 0 { return None; }
        self.used_clocks |= bits;
        self.state.enable_clock_generator(ClockId::I2S1, generator.gclk);
        let freq = self.gclks[u8::from(generator.gclk) as usize];
        Some(I2S1Clock { freq })
    }
    /// Configure the clock for peripheral(s) that match the name
    /// of this function to use the specific clock generator.
    /// The `GClock` parameter may be one of default clocks
    /// return from `gclk0()`, `gclk1()` or a clock configured
    /// by the host application using the `configure_gclk_divider_and_source`
    /// method.
    /// Returns a typed token that proves that the clock has been configured;
    /// the peripheral initialization code will typically require that this
    /// clock token be passed in to ensure that the clock has been initialized
    /// appropriately.
    /// Returns `None` is the specified generic clock has already been
    /// configured.
    pub fn sdhc0(&mut self, generator: &GClock) -> Option<Sdhc0Clock> {
        let bits: u64 = 1 << u8::from(ClockId::SDHC0) as u64;
        if (self.used_clocks & bits) != 0 { return None; }
        self.used_clocks |= bits;
        self.state.enable_clock_generator(ClockId::SDHC0, generator.gclk);
        let freq = self.gclks[u8::from(generator.gclk) as usize];
        Some(Sdhc0Clock { freq })
    }
    /// Configure the clock for peripheral(s) that match the name
    /// of this function to use the specific clock generator.
    /// The `GClock` parameter may be one of default clocks
    /// return from `gclk0()`, `gclk1()` or a clock configured
    /// by the host application using the `configure_gclk_divider_and_source`
    /// method.
    /// Returns a typed token that proves that the clock has been configured;
    /// the peripheral initialization code will typically require that this
    /// clock token be passed in to ensure that the clock has been initialized
    /// appropriately.
    /// Returns `None` is the specified generic clock has already been
    /// configured.
    pub fn sdhc1(&mut self, generator: &GClock) -> Option<Sdhc1Clock> {
        let bits: u64 = 1 << u8::from(ClockId::SDHC1) as u64;
        if (self.used_clocks & bits) != 0 { return None; }
        self.used_clocks |= bits;
        self.state.enable_clock_generator(ClockId::SDHC1, generator.gclk);
        let freq = self.gclks[u8::from(generator.gclk) as usize];
        Some(Sdhc1Clock { freq })
    }
    /// Configure the clock for peripheral(s) that match the name
    /// of this function to use the specific clock generator.
    /// The `GClock` parameter may be one of default clocks
    /// return from `gclk0()`, `gclk1()` or a clock configured
    /// by the host application using the `configure_gclk_divider_and_source`
    /// method.
    /// Returns a typed token that proves that the clock has been configured;
    /// the peripheral initialization code will typically require that this
    /// clock token be passed in to ensure that the clock has been initialized
    /// appropriately.
    /// Returns `None` is the specified generic clock has already been
    /// configured.
    pub fn cm4_trace(&mut self, generator: &GClock) -> Option<Cm4TraceClock> {
        let bits: u64 = 1 << u8::from(ClockId::CM4_TRACE) as u64;
        if (self.used_clocks & bits) != 0 { return None; }
        self.used_clocks |= bits;
        self.state.enable_clock_generator(ClockId::CM4_TRACE, generator.gclk);
        let freq = self.gclks[u8::from(generator.gclk) as usize];
        Some(Cm4TraceClock { freq })
    }
}
//clock_generator!(
//    (tc0_tc1, Tc0Tc1Clock, TC0_TC1),
//    (tcc0_tcc1, Tcc0Tcc1Clock, TCC0_TCC1),
//    (tc2_tc3, Tc2Tc3Clock, TC2_TC3),
//    (tcc2_tcc3, Tcc2Tcc3Clock, TCC2_TCC3),
//    (tc4_tc5, Tc4Tc5Clock, TC4_TC5),
//    (tcc4, Tcc4Clock, TCC4),
//    (tc6_tc7, Tc6Tc7Clock, TC6_TC7),
//    (sercom0_core, Sercom0CoreClock, SERCOM0_CORE),
//    (sercom1_core, Sercom1CoreClock, SERCOM1_CORE),
//    (sercom2_core, Sercom2CoreClock, SERCOM2_CORE),
//    (sercom3_core, Sercom3CoreClock, SERCOM3_CORE),
//    (sercom4_core, Sercom4CoreClock, SERCOM4_CORE),
//    (sercom5_core, Sercom5CoreClock, SERCOM5_CORE),
//    #[cfg(feature = "min-samd51n")]
//    (sercom6_core, Sercom6CoreClock, SERCOM6_CORE),
//    #[cfg(feature = "min-samd51n")]
//    (sercom7_core, Sercom7CoreClock, SERCOM7_CORE),
//    (usb, UsbClock, USB),
//    (adc0, Adc0Clock, ADC0),
//    (adc1, Adc1Clock, ADC1),
//    (eic, EicClock, EIC),
//    (freq_m_msr, FreqmMsrClock, FREQM_MSR),
//    (freq_m_ref, FreqmRefClock, FREQM_REF),
//    (evsys0, Evsys0Clock, EVSYS0),
//    (evsys1, Evsys1Clock, EVSYS1),
//    (evsys2, Evsys2Clock, EVSYS2),
//    (evsys3, Evsys3Clock, EVSYS3),
//    (evsys4, Evsys4Clock, EVSYS4),
//    (evsys5, Evsys5Clock, EVSYS5),
//    (evsys6, Evsys6Clock, EVSYS6),
//    (evsys7, Evsys7Clock, EVSYS7),
//    (evsys8, Evsys8Clock, EVSYS8),
//    (evsys9, Evsys9Clock, EVSYS9),
//    (evsys10, Evsys10Clock, EVSYS10),
//    (evsys11, Evsys11Clock, EVSYS11),
//    (can0, Can0Clock, CAN0),
//    (can1, Can1Clock, CAN1),
//    (pdec, PdecClock, PDEC),
//    (ac, AcClock, AC),
//    (ccl, CclClock, CCL),
//    (dac, DacClock, DAC),
//    (i2s0, I2S0Clock, I2S0),
//    (i2s1, I2S1Clock, I2S1),
//    (sdhc0, Sdhc0Clock, SDHC0),
//    (sdhc1, Sdhc1Clock, SDHC1),
//    (cm4_trace, Cm4TraceClock, CM4_TRACE),
//);

/// The frequency of the 48Mhz source.
pub const OSC48M_FREQ: Hertz = Hertz(48_000_000);
/// The frequency of the 32Khz source.
pub const OSC32K_FREQ: Hertz = Hertz(32_768);
/// The frequency of the 120Mhz source.
pub const OSC120M_FREQ: Hertz = Hertz(120_000_000);

fn set_flash_to_half_auto_wait_state(nvmctrl: &mut NVMCTRL) {
    // Zero indicates zero wait states, one indicates one wait state, etc.,
    // up to 15 wait states.
    nvmctrl.ctrla.modify(|_, w| unsafe { w.rws().bits(0b0111) });
}

fn enable_gclk_apb(mclk: &mut MCLK) {
    mclk.apbamask.modify(|_, w| w.gclk_().set_bit());
}

/// Turn on the internal 32hkz oscillator
fn enable_internal_32kosc(osc32kctrl: &mut OSC32KCTRL) {
    osc32kctrl.osculp32k.modify(|_, w| {
        w.en32k().set_bit();
        w.en1k().set_bit()
    });
    osc32kctrl.rtcctrl.write(|w| w.rtcsel().ulp1k());
}

/// Turn on the external 32hkz oscillator
fn enable_external_32kosc(osc32kctrl: &mut OSC32KCTRL) {
    osc32kctrl.xosc32k.modify(|_, w| {
        w.ondemand().clear_bit();
        // Enable 32khz output
        w.en32k().set_bit();
        w.en1k().set_bit();
        // Crystal connected to xin32/xout32
        w.xtalen().set_bit();
        w.enable().set_bit();
        w.cgm().xt();
        w.runstdby().set_bit()
    });

    osc32kctrl.rtcctrl.write(|w| w.rtcsel().xosc1k());

    // Wait for the oscillator to stabilize
    while osc32kctrl.status.read().xosc32krdy().bit_is_clear() {}
}

fn wait_for_dpllrdy(oscctrl: &mut OSCCTRL) {
    while oscctrl.dpll[0].dpllstatus.read().lock().bit_is_clear()
        || oscctrl.dpll[0].dpllstatus.read().clkrdy().bit_is_clear()
    {}
}

/// Configure the dpll0 to run at 120MHz
fn configure_and_enable_dpll0(oscctrl: &mut OSCCTRL, gclk: &mut GCLK) {
    gclk.pchctrl[ClockId::FDPLL0 as usize].write(|w| {
        w.chen().set_bit();
        w.gen().gclk5()
    });
    unsafe {
        oscctrl.dpll[0].dpllratio.write(|w| {
            w.ldr().bits(59);
            w.ldrfrac().bits(0)
        });
    }
    oscctrl.dpll[0].dpllctrlb.write(|w| w.refclk().gclk());
    oscctrl.dpll[0].dpllctrla.write(|w| {
        w.enable().set_bit();
        w.ondemand().clear_bit()
    });
}

#[cfg(feature = "usb")]
/// Configure the dfll48m to calibrate against the 1Khz USB SOF reference.
fn configure_usb_correction(oscctrl: &mut OSCCTRL) {
    oscctrl.dfllmul.write(|w| unsafe {
        w.cstep().bits(0x1)
        .fstep().bits(0x1)
        // scaling factor for 1Khz SOF signal.
        .mul().bits((48_000_000u32 / 1000) as u16)
    });
    while oscctrl.dfllsync.read().dfllmul().bit_is_set() {}

    oscctrl.dfllctrlb.write(|w| {
        // closed loop mode
        w.mode().set_bit()
        // chill cycle disable
        .ccdis().set_bit()
        // usb correction
        .usbcrm().set_bit()
    });
    while oscctrl.dfllsync.read().dfllctrlb().bit_is_set() {}
}
