//! Working with timer counter hardware
use crate::ehal::timer::{CountDown, Periodic};
use crate::pac::tc0::COUNT16;
#[allow(unused)]
use crate::pac::{MCLK, TC2, TC3};
use crate::timer_params::TimerParams;
// Only the G variants are missing these timers
#[cfg(feature = "min-samd51j")]
use crate::pac::{TC4, TC5};
use crate::timer_traits::InterruptDrivenTimer;

use crate::clock;
use crate::time::{Hertz, Nanoseconds};
use void::Void;

// Note:
// TC3 + TC4 can be paired to make a 32-bit counter
// TC5 + TC6 can be paired to make a 32-bit counter

/// A generic hardware timer counter.
/// The counters are exposed in 16-bit mode only.
/// The hardware allows configuring the 8-bit mode
/// and pairing up some instances to run in 32-bit
/// mode, but that functionality is not currently
/// exposed by this hal implementation.
/// TimerCounter implements both the `Periodic` and
/// the `CountDown` embedded_hal timer traits.
/// Before a hardware timer can be used, it must first
/// have a clock configured.
pub struct TimerCounter<TC> {
    freq: Hertz,
    tc: TC,
}

/// This is a helper trait to make it easier to make most of the
/// TimerCounter impl generic.  It doesn't make too much sense to
/// to try to implement this trait outside of this module.
pub trait Count16 {
    fn count_16(&self) -> &COUNT16;
}

impl<TC> Periodic for TimerCounter<TC> {}
impl<TC> CountDown for TimerCounter<TC>
where
    TC: Count16,
{
    type Time = Nanoseconds;

    fn start<T>(&mut self, timeout: T)
    where
        T: Into<Self::Time>,
    {
        let params = TimerParams::new_us(timeout, self.freq.0);
        let divider = params.divider;
        let cycles = params.cycles;
        let count = self.tc.count_16();

        // Disable the timer while we reconfigure it
        count.ctrla.modify(|_, w| w.enable().clear_bit());
        while count.status.read().perbufv().bit_is_set() {}

        // Now that we have a clock routed to the peripheral, we
        // can ask it to perform a reset.
        count.ctrla.write(|w| w.swrst().set_bit());

        while count.status.read().perbufv().bit_is_set() {}
        // the SVD erroneously marks swrst as write-only, so we
        // need to manually read the bit here
        while count.ctrla.read().bits() & 1 != 0 {}

        count.ctrlbset.write(|w| {
            // Count up when the direction bit is zero
            w.dir().clear_bit();
            // Periodic
            w.oneshot().clear_bit()
        });

        // Set TOP value for mfrq mode
        count.cc[0].write(|w| unsafe { w.cc().bits(cycles as u16) });

        // Enable Match Frequency Waveform generation
        count.wave.modify(|_, w| w.wavegen().mfrq());

        count.ctrla.modify(|_, w| {
            match divider {
                1 => w.prescaler().div1(),
                2 => w.prescaler().div2(),
                4 => w.prescaler().div4(),
                8 => w.prescaler().div8(),
                16 => w.prescaler().div16(),
                64 => w.prescaler().div64(),
                256 => w.prescaler().div256(),
                1024 => w.prescaler().div1024(),
                _ => unreachable!(),
            };
            w.enable().set_bit();
            w.runstdby().set_bit()
        });
    }

    fn wait(&mut self) -> nb::Result<(), Void> {
        let count = self.tc.count_16();
        if count.intflag.read().ovf().bit_is_set() {
            // Writing a 1 clears the flag
            count.intflag.modify(|_, w| w.ovf().set_bit());
            Ok(())
        } else {
            Err(nb::Error::WouldBlock)
        }
    }
}

impl<TC> InterruptDrivenTimer for TimerCounter<TC>
where
    TC: Count16,
{
    /// Enable the interrupt generation for this hardware timer.
    /// This method only sets the clock configuration to trigger
    /// the interrupt; it does not configure the interrupt controller
    /// or define an interrupt handler.
    fn enable_interrupt(&mut self) {
        self.tc.count_16().intenset.write(|w| w.ovf().set_bit());
    }

    /// Disables interrupt generation for this hardware timer.
    /// This method only sets the clock configuration to prevent
    /// triggering the interrupt; it does not configure the interrupt
    /// controller.
    fn disable_interrupt(&mut self) {
        self.tc.count_16().intenclr.write(|w| w.ovf().set_bit());
    }
}
pub type TimerCounter2 = TimerCounter<TC2>;
impl Count16 for TC2 {
    fn count_16(&self) -> &COUNT16 {
        self.count16()
    }
}
impl TimerCounter<TC2> {
    /// Configure this timer counter instance.
    /// The clock is obtained from the `GenericClockController` instance
    /// and its frequency impacts the resolution and maximum range of
    /// the timeout values that can be passed to the `start` method.
    /// Note that some hardware timer instances share the same clock
    /// generator instance and thus will be clocked at the same rate.
    pub fn tc2_(clock: &clock::Tc2Tc3Clock, tc: TC2, mclk: &mut MCLK) -> Self {
        mclk.apbbmask.modify(|_, w| w.tc2_().set_bit());
        {
            let count = tc.count16();
            count.ctrla.modify(|_, w| w.enable().clear_bit());
            while count.status.read().perbufv().bit_is_set() {}
        }
        Self { freq: clock.freq(), tc }
    }
}
pub type TimerCounter3 = TimerCounter<TC3>;
impl Count16 for TC3 {
    fn count_16(&self) -> &COUNT16 {
        self.count16()
    }
}
impl TimerCounter<TC3> {
    /// Configure this timer counter instance.
    /// The clock is obtained from the `GenericClockController` instance
    /// and its frequency impacts the resolution and maximum range of
    /// the timeout values that can be passed to the `start` method.
    /// Note that some hardware timer instances share the same clock
    /// generator instance and thus will be clocked at the same rate.
    pub fn tc3_(clock: &clock::Tc2Tc3Clock, tc: TC3, mclk: &mut MCLK) -> Self {
        mclk.apbbmask.modify(|_, w| w.tc3_().set_bit());
        {
            let count = tc.count16();
            count.ctrla.modify(|_, w| w.enable().clear_bit());
            while count.status.read().perbufv().bit_is_set() {}
        }
        Self { freq: clock.freq(), tc }
    }
}
pub type TimerCounter4 = TimerCounter<TC4>;
impl Count16 for TC4 {
    fn count_16(&self) -> &COUNT16 {
        self.count16()
    }
}
impl TimerCounter<TC4> {
    /// Configure this timer counter instance.
    /// The clock is obtained from the `GenericClockController` instance
    /// and its frequency impacts the resolution and maximum range of
    /// the timeout values that can be passed to the `start` method.
    /// Note that some hardware timer instances share the same clock
    /// generator instance and thus will be clocked at the same rate.
    pub fn tc4_(clock: &clock::Tc4Tc5Clock, tc: TC4, mclk: &mut MCLK) -> Self {
        mclk.apbcmask.modify(|_, w| w.tc4_().set_bit());
        {
            let count = tc.count16();
            count.ctrla.modify(|_, w| w.enable().clear_bit());
            while count.status.read().perbufv().bit_is_set() {}
        }
        Self { freq: clock.freq(), tc }
    }

    pub fn clear_all_irq(&self) {
        let count = self.tc.count16();
        count.intflag.modify(|_, w| w.ovf().set_bit());
        count.intflag.modify(|_, w| w.err().set_bit());
        count.intflag.modify(|_, w| w.mc0().set_bit());
        count.intflag.modify(|_, w| w.mc1().set_bit());
    }

    pub fn clear_ovf_irq(&self) {
        let count = self.tc.count16();
        count.intflag.modify(|_, w| w.ovf().set_bit());
        while count.intflag.read().ovf().bit_is_set() {}
    }

    pub fn clear_err_irq(&self) {
        let count = self.tc.count16();
        count.intflag.modify(|_, w| w.err().set_bit());
        while count.intflag.read().err().bit_is_set() {}
    }

    pub fn clear_mc0_irq(&self) {
        let count = self.tc.count16();
        count.intflag.modify(|_, w| w.mc0().set_bit());
        while count.intflag.read().mc0().bit_is_set() {}
    }

    pub fn clear_mc1_irq(&self) {
        let count = self.tc.count16();
        count.intflag.modify(|_, w| w.mc1().set_bit());
        while count.intflag.read().mc1().bit_is_set() {}
    }



    pub fn is_ovf_int_flag_set(&self) -> bool {
        let count = self.tc.count16();
        count.intflag.read().ovf().bit_is_set()
    }

    pub fn is_err_int_flag_set(&self) -> bool {
        let count = self.tc.count16();
        count.intflag.read().err().bit_is_set()
    }

    pub fn is_mc0_int_flag_set(&self) -> bool {
        let count = self.tc.count16();
        count.intflag.read().mc0().bit_is_set()
    }

    pub fn is_mc1_int_flag_set(&self) -> bool {
        let count = self.tc.count16();
        count.intflag.read().mc1().bit_is_set()
    }
}
pub type TimerCounter5 = TimerCounter<TC5>;
impl Count16 for TC5 {
    fn count_16(&self) -> &COUNT16 {
        self.count16()
    }
}
impl TimerCounter<TC5> {
    /// Configure this timer counter instance.
    /// The clock is obtained from the `GenericClockController` instance
    /// and its frequency impacts the resolution and maximum range of
    /// the timeout values that can be passed to the `start` method.
    /// Note that some hardware timer instances share the same clock
    /// generator instance and thus will be clocked at the same rate.
    pub fn tc5_(clock: &clock::Tc4Tc5Clock, tc: TC5, mclk: &mut MCLK) -> Self {
        mclk.apbcmask.modify(|_, w| w.tc5_().set_bit());
        {
            let count = tc.count16();
            count.ctrla.modify(|_, w| w.enable().clear_bit());
            while count.status.read().perbufv().bit_is_set() {}
        }
        Self { freq: clock.freq(), tc }
    }
}
