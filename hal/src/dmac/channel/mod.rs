//! # Abstractions over individual DMA channels
//!
//! # Initializing
//!
//! Individual channels should be initialized through the
//! [`Channel::init`] method. This will return a `Channel<Id, Ready>` ready for
//! use by a [`Transfer`](super::transfer::Transfer). Initializing a channel
//! requires setting a priority level, as well as enabling or disabling
//! interrupt requests (only for the specific channel being initialized).
#![cfg_attr(
    not(any(feature = "samd11", feature = "samd21")),
    doc = "# Burst Length and FIFO Threshold (SAMD51/SAME5x only)

The transfer burst length can be configured through the
[`Channel::burst_length`] method. A burst is an atomic,
uninterruptible transfer which length corresponds to a number of beats. See
SAMD5x/E5x datasheet section 22.6.1.1 for more information. The FIFO
threshold can be configured through the
[`Channel::fifo_threshold`] method. This enables the channel
to wait for multiple Beats before sending a Burst. See SAMD5x/E5x datasheet
section 22.6.2.8 for more information."
)]
//!
//! # Channel status
//!
//! Channels can be in any of three statuses: [`Uninitialized`], [`Ready`], and
//! [`Busy`]. These statuses are checked at compile time to ensure they are
//! properly initialized before launching DMA transfers.
//!
//! # Resetting
//!
//! Calling the [`Channel::reset`] method will reset the channel to its
//! `Uninitialized` state. You will be required to call [`Channel::init`]
//! again before being able to use it with a `Transfer`.

use super::dma_controller::{ChId, PriorityLevel, TriggerAction, TriggerSource};
use crate::typelevel::{Is, Sealed};
use core::marker::PhantomData;
use modular_bitfield::prelude::*;

mod reg;

use reg::RegisterBlock;

#[cfg(feature = "min-samd51g")]
use super::dma_controller::{BurstLength, FifoThreshold};

//==============================================================================
// Channel Status
//==============================================================================
pub trait Status: Sealed {}

/// Uninitialized channel
pub enum Uninitialized {}
impl Sealed for Uninitialized {}
impl Status for Uninitialized {}
/// Initialized and ready to transfer channel
pub enum Ready {}
impl Sealed for Ready {}
impl Status for Ready {}
/// Busy channel
pub enum Busy {}
impl Sealed for Busy {}
impl Status for Busy {}

//==============================================================================
// AnyChannel
//==============================================================================
pub trait AnyChannel: Sealed + Is<Type = SpecificChannel<Self>> {
    type Status: Status;
    type Id: ChId;
}

pub type SpecificChannel<C> = Channel<<C as AnyChannel>::Id, <C as AnyChannel>::Status>;

pub type ChannelStatus<C> = <C as AnyChannel>::Status;
pub type ChannelId<C> = <C as AnyChannel>::Id;

impl<Id, S> Sealed for Channel<Id, S>
where
    Id: ChId,
    S: Status,
{
}

impl<Id, S> AnyChannel for Channel<Id, S>
where
    Id: ChId,
    S: Status,
{
    type Id = Id;
    type Status = S;
}

impl<Id, S> AsRef<Self> for Channel<Id, S>
where
    Id: ChId,
    S: Status,
{
    #[inline]
    fn as_ref(&self) -> &Self {
        self
    }
}

impl<Id, S> AsMut<Self> for Channel<Id, S>
where
    Id: ChId,
    S: Status,
{
    #[inline]
    fn as_mut(&mut self) -> &mut Self {
        self
    }
}

//==============================================================================
// Channel
//==============================================================================
/// DMA channel, capable of executing
/// [`Transfer`](super::transfer::Transfer)s.
pub struct Channel<Id: ChId, S: Status> {
    regs: RegisterBlock<Id>,
    _status: PhantomData<S>,
}

#[inline]
pub(super) fn new_chan<Id: ChId>(_id: PhantomData<Id>) -> Channel<Id, Uninitialized> {
    Channel {
        regs: RegisterBlock::new(_id),
        _status: PhantomData,
    }
}

/// These methods may be used on any DMA channel in any configuration
impl<Id: ChId, S: Status> Channel<Id, S> {
    /// Configure the DMA channel so that it is ready to be used by a
    /// [`Transfer`](super::transfer::Transfer).
    ///
    /// # Return
    ///
    /// A `Channel` with a `Ready` status
    #[inline]
    pub fn init(mut self, lvl: PriorityLevel) -> Channel<Id, Ready> {
        // Software reset the channel for good measure
        self._reset_private();

        #[cfg(any(feature = "samd11", feature = "samd21"))]
        // Setup priority level
        self.regs.chctrlb.modify(|_, w| w.lvl().bits(lvl as u8));

        #[cfg(feature = "min-samd51g")]
        self.regs.chprilvl.modify(|_, w| w.prilvl().bits(lvl as u8));

        Channel {
            regs: self.regs,
            _status: PhantomData,
        }
    }

    /// Selectively enable interrupts
    #[inline]
    pub fn enable_interrupts(&mut self, flags: InterruptFlags) {
        // SAFETY: This is safe as InterruptFlags is only capable of writing in
        // non-reserved bits
        self.regs
            .chintenset
            .write(|w| unsafe { w.bits(flags.into()) });
    }

    /// Selectively disable interrupts
    #[inline]
    pub fn disable_interrupts(&mut self, flags: InterruptFlags) {
        // SAFETY: This is safe as InterruptFlags is only capable of writing in
        // non-reserved bits
        self.regs
            .chintenclr
            .write(|w| unsafe { w.bits(flags.into()) });
    }

    /// Check the specified `flags`, clear then return any that were set
    #[inline]
    pub fn check_and_clear_interrupts(&mut self, flags: InterruptFlags) -> InterruptFlags {
        let mut cleared = 0;
        self.regs.chintflag.modify(|r, w| {
            cleared = r.bits() & flags.into_bytes()[0];
            unsafe { w.bits(cleared) }
        });

        InterruptFlags::from_bytes([cleared])
    }

//----  Hack  ------------------------------------
    #[inline]
    pub fn interrupt_trigger_source(&mut self) -> usize {
        return self.regs.chctrla.read().trigsrc().variant().unwrap() as usize;
    }
//------------------------------------------------

    #[inline]
    fn _reset_private(&mut self) {
        // Reset the channel to its startup state and wait for reset to complete
        self.regs.chctrla.modify(|_, w| w.swrst().set_bit());
        while self.regs.chctrla.read().swrst().bit_is_set() {}
    }

    #[inline]
    fn _trigger_private(&mut self) {
        self.regs.swtrigctrl.set_bit();
    }
}

/// These methods may only be used on a `Ready` DMA channel
impl<Id: ChId> Channel<Id, Ready> {
    /// Issue a software reset to the channel. This will return the channel to
    /// its startup state
    #[inline]
    pub fn reset(mut self) -> Channel<Id, Uninitialized> {
        self._reset_private();

        Channel {
            regs: self.regs,
            _status: PhantomData,
        }
    }

    /// Set the FIFO threshold length. The channel will wait until it has
    /// received the selected number of Beats before triggering the Burst
    /// transfer, reducing the DMA transfer latency.
    #[cfg(feature = "min-samd51g")]
    #[inline]
    pub fn fifo_threshold(&mut self, threshold: FifoThreshold) {
        self.regs
            .chctrla
            .modify(|_, w| w.threshold().bits(threshold as u8));
    }

    /// Set burst length for the channel, in number of beats. A burst transfer
    /// is an atomic, uninterruptible operation.
    #[cfg(feature = "min-samd51g")]
    #[inline]
    pub fn burst_length(&mut self, burst_length: BurstLength) {
        self.regs
            .chctrla
            .modify(|_, w| w.burstlen().bits(burst_length as u8));
    }

    /// Start transfer on channel using the specified trigger source.
    ///
    /// # Return
    ///
    /// A `Channel` with a `Busy` status.
    #[inline]
    pub(crate) fn start(
        mut self,
        trig_src: TriggerSource,
        trig_act: TriggerAction,
    ) -> Channel<Id, Busy> {
        // Configure the trigger source and trigger action
        #[cfg(any(feature = "samd11", feature = "samd21"))]
        self.regs.chctrlb.modify(|_, w| {
            w.trigsrc().variant(trig_src);
            w.trigact().variant(trig_act)
        });

        #[cfg(feature = "min-samd51g")]
        self.regs.chctrla.modify(|_, w| {
            w.trigsrc().variant(trig_src);
            w.trigact().variant(trig_act)
        });

        // Start channel
        self.regs.chctrla.modify(|_, w| w.enable().set_bit());

        // If trigger source is DISABLE, manually trigger transfer
        if trig_src == TriggerSource::DISABLE {
            self._trigger_private();
        }

        Channel {
            regs: self.regs,
            _status: PhantomData,
        }
    }
}

/// These methods may only be used on a `Busy` DMA channel
impl<Id: ChId> Channel<Id, Busy> {
    /// Issue a software trigger to the channel
    #[inline]
    pub(crate) fn software_trigger(&mut self) {
        self._trigger_private();
    }

    /// Returns whether or not the transfer is complete.
    #[inline]
    pub(crate) fn xfer_complete(&mut self) -> bool {
        !self.regs.chctrla.read().enable().bit_is_set()
    }

    /// Stop transfer on channel whether or not the transfer has completed
    ///
    /// # Return
    ///
    /// A `Channel` with a `Ready` status, ready to be reused by a new
    /// [`Transfer`](super::transfer::Transfer)
    #[inline]
    pub(crate) fn free(mut self) -> Channel<Id, Ready> {
        self.regs.chctrla.modify(|_, w| w.enable().clear_bit());
        while !self.xfer_complete() {}
        Channel {
            regs: self.regs,
            _status: PhantomData,
        }
    }

    #[inline]
    pub(super) fn callback(&mut self) -> CallbackStatus {
        // Transfer complete
        if self.regs.chintflag.read().tcmpl().bit_is_set() {
            self.regs.chintflag.modify(|_, w| w.tcmpl().set_bit());
            return CallbackStatus::TransferComplete;
        }
        // Transfer error
        else if self.regs.chintflag.read().terr().bit_is_set() {
            self.regs.chintflag.modify(|_, w| w.terr().set_bit());
            return CallbackStatus::TransferError;
        }
        // Channel suspended
        else if self.regs.chintflag.read().susp().bit_is_set() {
            self.regs.chintflag.modify(|_, w| w.susp().set_bit());
            return CallbackStatus::TransferSuspended;
        }
        // Default to error if for some reason there was in interrupt
        // flag raised
        CallbackStatus::TransferError
    }

    /// Restart transfer using previously-configured trigger source and action
    #[inline]
    pub(crate) fn restart(&mut self) {
        self.regs.chctrla.modify(|_, w| w.enable().set_bit());
    }
}

impl<Id: ChId> From<Channel<Id, Ready>> for Channel<Id, Uninitialized> {
    fn from(item: Channel<Id, Ready>) -> Self {
        Channel {
            regs: item.regs,
            _status: PhantomData,
        }
    }
}

/// Status of a transfer callback
#[derive(Clone, Copy)]
pub enum CallbackStatus {
    /// Transfer Complete
    TransferComplete,
    /// Transfer Error
    TransferError,
    /// Transfer Suspended
    TransferSuspended,
}

/// Interrupt sources available to a DMA channel
#[bitfield]
#[repr(u8)]
#[derive(Clone, Copy)]
pub struct InterruptFlags {
    /// Transfer error
    pub terr: bool,
    /// Transfer complete
    pub tcmpl: bool,
    /// Transfer suspended
    pub susp: bool,
    #[skip]
    _reserved: B5,
}


/*

#[doc = " Interrupt sources available to a DMA channel"]
#[derive(Clone, Copy)]
#[allow(clippy::identity_op)]
pub struct InterruptFlags {
    bytes: [::core::primitive::u8; {
        ((({
            0usize
                + <bool as ::modular_bitfield::Specifier>::BITS
                + <bool as ::modular_bitfield::Specifier>::BITS
                + <bool as ::modular_bitfield::Specifier>::BITS
                + <B5 as ::modular_bitfield::Specifier>::BITS
        } - 1)
            / 8)
            + 1)
            * 8
    } / 8usize],
}
#[allow(clippy::identity_op)]
const _: () = {
    impl ::modular_bitfield::private::checks::CheckTotalSizeMultipleOf8 for InterruptFlags {
        type Size = ::modular_bitfield::private::checks::TotalSize<
            [(); {
                0usize
                    + <bool as ::modular_bitfield::Specifier>::BITS
                    + <bool as ::modular_bitfield::Specifier>::BITS
                    + <bool as ::modular_bitfield::Specifier>::BITS
                    + <B5 as ::modular_bitfield::Specifier>::BITS
            } % 8usize],
        >;
    }
};
impl InterruptFlags {
    #[doc = r" Returns an instance with zero initialized data."]
    #[allow(clippy::identity_op)]
    pub const fn new() -> Self {
        Self {
            bytes: [0u8; {
                ((({
                    0usize
                        + <bool as ::modular_bitfield::Specifier>::BITS
                        + <bool as ::modular_bitfield::Specifier>::BITS
                        + <bool as ::modular_bitfield::Specifier>::BITS
                        + <B5 as ::modular_bitfield::Specifier>::BITS
                } - 1)
                    / 8)
                    + 1)
                    * 8
            } / 8usize],
        }
    }
}
impl InterruptFlags {
    #[doc = r" Returns the underlying bits."]
    #[doc = r""]
    #[doc = r" # Layout"]
    #[doc = r""]
    #[doc = r" The returned byte array is layed out in the same way as described"]
    #[doc = r" [here](https://docs.rs/modular-bitfield/#generated-structure)."]
    #[inline]
    #[allow(clippy::identity_op)]
    pub const fn into_bytes(
        self,
    ) -> [::core::primitive::u8; {
           ((({
               0usize
                   + <bool as ::modular_bitfield::Specifier>::BITS
                   + <bool as ::modular_bitfield::Specifier>::BITS
                   + <bool as ::modular_bitfield::Specifier>::BITS
                   + <B5 as ::modular_bitfield::Specifier>::BITS
           } - 1)
               / 8)
               + 1)
               * 8
       } / 8usize] {
        self.bytes
    }
    #[doc = r" Converts the given bytes directly into the bitfield struct."]
    #[inline]
    #[allow(clippy::identity_op)]
    pub const fn from_bytes(
        bytes: [::core::primitive::u8; {
            ((({
                0usize
                    + <bool as ::modular_bitfield::Specifier>::BITS
                    + <bool as ::modular_bitfield::Specifier>::BITS
                    + <bool as ::modular_bitfield::Specifier>::BITS
                    + <B5 as ::modular_bitfield::Specifier>::BITS
            } - 1)
                / 8)
                + 1)
                * 8
        } / 8usize],
    ) -> Self {
        Self { bytes }
    }
}
const _: () = {
    const _: () = {};
    const _: () = {};
    const _: () = {};
    const _: () = {};
};
impl InterruptFlags {
    #[doc = "Returns the value of terr."]
    #[inline]
    #[doc = " Transfer error"]
    pub fn terr(&self) -> <bool as ::modular_bitfield::Specifier>::InOut {
        self.terr_or_err()
            .expect("value contains invalid bit pattern for field InterruptFlags.terr")
    }
    #[doc = "Returns the value of terr.\n\n#Errors\n\nIf the returned value contains an invalid bit pattern for terr."]
    #[inline]
    #[allow(dead_code)]
    #[doc = " Transfer error"]
    pub fn terr_or_err(
        &self,
    ) -> ::core::result::Result<
        <bool as ::modular_bitfield::Specifier>::InOut,
        ::modular_bitfield::error::InvalidBitPattern<
            <bool as ::modular_bitfield::Specifier>::Bytes,
        >,
    > {
        let __bf_read: <bool as ::modular_bitfield::Specifier>::Bytes =
            { ::modular_bitfield::private::read_specifier::<bool>(&self.bytes[..], 0usize) };
        <bool as ::modular_bitfield::Specifier>::from_bytes(__bf_read)
    }
    #[doc = "Returns a copy of the bitfield with the value of terr set to the given value.\n\n#Panics\n\nIf the given value is out of bounds for terr."]
    #[inline]
    #[allow(dead_code)]
    #[doc = " Transfer error"]
    pub fn with_terr(mut self, new_val: <bool as ::modular_bitfield::Specifier>::InOut) -> Self {
        self.set_terr(new_val);
        self
    }
    #[doc = "Returns a copy of the bitfield with the value of terr set to the given value.\n\n#Errors\n\nIf the given value is out of bounds for terr."]
    #[inline]
    #[allow(dead_code)]
    #[doc = " Transfer error"]
    pub fn with_terr_checked(
        mut self,
        new_val: <bool as ::modular_bitfield::Specifier>::InOut,
    ) -> ::core::result::Result<Self, ::modular_bitfield::error::OutOfBounds> {
        self.set_terr_checked(new_val)?;
        ::core::result::Result::Ok(self)
    }
    #[doc = "Sets the value of terr to the given value.\n\n#Panics\n\nIf the given value is out of bounds for terr."]
    #[inline]
    #[allow(dead_code)]
    #[doc = " Transfer error"]
    pub fn set_terr(&mut self, new_val: <bool as ::modular_bitfield::Specifier>::InOut) {
        self.set_terr_checked(new_val)
            .expect("value out of bounds for field InterruptFlags.terr")
    }
    #[doc = "Sets the value of terr to the given value.\n\n#Errors\n\nIf the given value is out of bounds for terr."]
    #[inline]
    #[doc = " Transfer error"]
    pub fn set_terr_checked(
        &mut self,
        new_val: <bool as ::modular_bitfield::Specifier>::InOut,
    ) -> ::core::result::Result<(), ::modular_bitfield::error::OutOfBounds> {
        let __bf_base_bits: ::core::primitive::usize =
            8usize * ::core::mem::size_of::<<bool as ::modular_bitfield::Specifier>::Bytes>();
        let __bf_max_value: <bool as ::modular_bitfield::Specifier>::Bytes =
            { !0 >> (__bf_base_bits - <bool as ::modular_bitfield::Specifier>::BITS) };
        let __bf_spec_bits: ::core::primitive::usize =
            <bool as ::modular_bitfield::Specifier>::BITS;
        let __bf_raw_val: <bool as ::modular_bitfield::Specifier>::Bytes =
            { <bool as ::modular_bitfield::Specifier>::into_bytes(new_val) }?;
        if !(__bf_base_bits == __bf_spec_bits || __bf_raw_val <= __bf_max_value) {
            return ::core::result::Result::Err(::modular_bitfield::error::OutOfBounds);
        }
        ::modular_bitfield::private::write_specifier::<bool>(
            &mut self.bytes[..],
            0usize,
            __bf_raw_val,
        );
        ::core::result::Result::Ok(())
    }
    #[doc = "Returns the value of tcmpl."]
    #[inline]
    #[doc = " Transfer complete"]
    pub fn tcmpl(&self) -> <bool as ::modular_bitfield::Specifier>::InOut {
        self.tcmpl_or_err()
            .expect("value contains invalid bit pattern for field InterruptFlags.tcmpl")
    }
    #[doc = "Returns the value of tcmpl.\n\n#Errors\n\nIf the returned value contains an invalid bit pattern for tcmpl."]
    #[inline]
    #[allow(dead_code)]
    #[doc = " Transfer complete"]
    pub fn tcmpl_or_err(
        &self,
    ) -> ::core::result::Result<
        <bool as ::modular_bitfield::Specifier>::InOut,
        ::modular_bitfield::error::InvalidBitPattern<
            <bool as ::modular_bitfield::Specifier>::Bytes,
        >,
    > {
        let __bf_read: <bool as ::modular_bitfield::Specifier>::Bytes = {
            ::modular_bitfield::private::read_specifier::<bool>(
                &self.bytes[..],
                0usize + <bool as ::modular_bitfield::Specifier>::BITS,
            )
        };
        <bool as ::modular_bitfield::Specifier>::from_bytes(__bf_read)
    }
    #[doc = "Returns a copy of the bitfield with the value of tcmpl set to the given value.\n\n#Panics\n\nIf the given value is out of bounds for tcmpl."]
    #[inline]
    #[allow(dead_code)]
    #[doc = " Transfer complete"]
    pub fn with_tcmpl(mut self, new_val: <bool as ::modular_bitfield::Specifier>::InOut) -> Self {
        self.set_tcmpl(new_val);
        self
    }
    #[doc = "Returns a copy of the bitfield with the value of tcmpl set to the given value.\n\n#Errors\n\nIf the given value is out of bounds for tcmpl."]
    #[inline]
    #[allow(dead_code)]
    #[doc = " Transfer complete"]
    pub fn with_tcmpl_checked(
        mut self,
        new_val: <bool as ::modular_bitfield::Specifier>::InOut,
    ) -> ::core::result::Result<Self, ::modular_bitfield::error::OutOfBounds> {
        self.set_tcmpl_checked(new_val)?;
        ::core::result::Result::Ok(self)
    }
    #[doc = "Sets the value of tcmpl to the given value.\n\n#Panics\n\nIf the given value is out of bounds for tcmpl."]
    #[inline]
    #[allow(dead_code)]
    #[doc = " Transfer complete"]
    pub fn set_tcmpl(&mut self, new_val: <bool as ::modular_bitfield::Specifier>::InOut) {
        self.set_tcmpl_checked(new_val)
            .expect("value out of bounds for field InterruptFlags.tcmpl")
    }
    #[doc = "Sets the value of tcmpl to the given value.\n\n#Errors\n\nIf the given value is out of bounds for tcmpl."]
    #[inline]
    #[doc = " Transfer complete"]
    pub fn set_tcmpl_checked(
        &mut self,
        new_val: <bool as ::modular_bitfield::Specifier>::InOut,
    ) -> ::core::result::Result<(), ::modular_bitfield::error::OutOfBounds> {
        let __bf_base_bits: ::core::primitive::usize =
            8usize * ::core::mem::size_of::<<bool as ::modular_bitfield::Specifier>::Bytes>();
        let __bf_max_value: <bool as ::modular_bitfield::Specifier>::Bytes =
            { !0 >> (__bf_base_bits - <bool as ::modular_bitfield::Specifier>::BITS) };
        let __bf_spec_bits: ::core::primitive::usize =
            <bool as ::modular_bitfield::Specifier>::BITS;
        let __bf_raw_val: <bool as ::modular_bitfield::Specifier>::Bytes =
            { <bool as ::modular_bitfield::Specifier>::into_bytes(new_val) }?;
        if !(__bf_base_bits == __bf_spec_bits || __bf_raw_val <= __bf_max_value) {
            return ::core::result::Result::Err(::modular_bitfield::error::OutOfBounds);
        }
        ::modular_bitfield::private::write_specifier::<bool>(
            &mut self.bytes[..],
            0usize + <bool as ::modular_bitfield::Specifier>::BITS,
            __bf_raw_val,
        );
        ::core::result::Result::Ok(())
    }
    #[doc = "Returns the value of susp."]
    #[inline]
    #[doc = " Transfer suspended"]
    pub fn susp(&self) -> <bool as ::modular_bitfield::Specifier>::InOut {
        self.susp_or_err()
            .expect("value contains invalid bit pattern for field InterruptFlags.susp")
    }
    #[doc = "Returns the value of susp.\n\n#Errors\n\nIf the returned value contains an invalid bit pattern for susp."]
    #[inline]
    #[allow(dead_code)]
    #[doc = " Transfer suspended"]
    pub fn susp_or_err(
        &self,
    ) -> ::core::result::Result<
        <bool as ::modular_bitfield::Specifier>::InOut,
        ::modular_bitfield::error::InvalidBitPattern<
            <bool as ::modular_bitfield::Specifier>::Bytes,
        >,
    > {
        let __bf_read: <bool as ::modular_bitfield::Specifier>::Bytes = {
            ::modular_bitfield::private::read_specifier::<bool>(
                &self.bytes[..],
                0usize
                    + <bool as ::modular_bitfield::Specifier>::BITS
                    + <bool as ::modular_bitfield::Specifier>::BITS,
            )
        };
        <bool as ::modular_bitfield::Specifier>::from_bytes(__bf_read)
    }
    #[doc = "Returns a copy of the bitfield with the value of susp set to the given value.\n\n#Panics\n\nIf the given value is out of bounds for susp."]
    #[inline]
    #[allow(dead_code)]
    #[doc = " Transfer suspended"]
    pub fn with_susp(mut self, new_val: <bool as ::modular_bitfield::Specifier>::InOut) -> Self {
        self.set_susp(new_val);
        self
    }
    #[doc = "Returns a copy of the bitfield with the value of susp set to the given value.\n\n#Errors\n\nIf the given value is out of bounds for susp."]
    #[inline]
    #[allow(dead_code)]
    #[doc = " Transfer suspended"]
    pub fn with_susp_checked(
        mut self,
        new_val: <bool as ::modular_bitfield::Specifier>::InOut,
    ) -> ::core::result::Result<Self, ::modular_bitfield::error::OutOfBounds> {
        self.set_susp_checked(new_val)?;
        ::core::result::Result::Ok(self)
    }
    #[doc = "Sets the value of susp to the given value.\n\n#Panics\n\nIf the given value is out of bounds for susp."]
    #[inline]
    #[allow(dead_code)]
    #[doc = " Transfer suspended"]
    pub fn set_susp(&mut self, new_val: <bool as ::modular_bitfield::Specifier>::InOut) {
        self.set_susp_checked(new_val)
            .expect("value out of bounds for field InterruptFlags.susp")
    }
    #[doc = "Sets the value of susp to the given value.\n\n#Errors\n\nIf the given value is out of bounds for susp."]
    #[inline]
    #[doc = " Transfer suspended"]
    pub fn set_susp_checked(
        &mut self,
        new_val: <bool as ::modular_bitfield::Specifier>::InOut,
    ) -> ::core::result::Result<(), ::modular_bitfield::error::OutOfBounds> {
        let __bf_base_bits: ::core::primitive::usize =
            8usize * ::core::mem::size_of::<<bool as ::modular_bitfield::Specifier>::Bytes>();
        let __bf_max_value: <bool as ::modular_bitfield::Specifier>::Bytes =
            { !0 >> (__bf_base_bits - <bool as ::modular_bitfield::Specifier>::BITS) };
        let __bf_spec_bits: ::core::primitive::usize =
            <bool as ::modular_bitfield::Specifier>::BITS;
        let __bf_raw_val: <bool as ::modular_bitfield::Specifier>::Bytes =
            { <bool as ::modular_bitfield::Specifier>::into_bytes(new_val) }?;
        if !(__bf_base_bits == __bf_spec_bits || __bf_raw_val <= __bf_max_value) {
            return ::core::result::Result::Err(::modular_bitfield::error::OutOfBounds);
        }
        ::modular_bitfield::private::write_specifier::<bool>(
            &mut self.bytes[..],
            0usize
                + <bool as ::modular_bitfield::Specifier>::BITS
                + <bool as ::modular_bitfield::Specifier>::BITS,
            __bf_raw_val,
        );
        ::core::result::Result::Ok(())
    }
}
impl ::core::convert::From<::core::primitive::u8> for InterruptFlags
where
    [(); {
        0usize
            + <bool as ::modular_bitfield::Specifier>::BITS
            + <bool as ::modular_bitfield::Specifier>::BITS
            + <bool as ::modular_bitfield::Specifier>::BITS
            + <B5 as ::modular_bitfield::Specifier>::BITS
    }]: ::modular_bitfield::private::IsU8Compatible,
{
    #[inline]
    fn from(__bf_prim: ::core::primitive::u8) -> Self {
        Self {
            bytes: <::core::primitive::u8>::to_le_bytes(__bf_prim),
        }
    }
}
impl ::core::convert::From<InterruptFlags> for ::core::primitive::u8
where
    [(); {
        0usize
            + <bool as ::modular_bitfield::Specifier>::BITS
            + <bool as ::modular_bitfield::Specifier>::BITS
            + <bool as ::modular_bitfield::Specifier>::BITS
            + <B5 as ::modular_bitfield::Specifier>::BITS
    }]: ::modular_bitfield::private::IsU8Compatible,
{
    #[inline]
    fn from(__bf_bitfield: InterruptFlags) -> Self {
        <Self>::from_le_bytes(__bf_bitfield.bytes)
    }
}
*/