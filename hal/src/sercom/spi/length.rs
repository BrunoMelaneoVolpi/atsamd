//! Define a trait to track the transaction [`Length`], which represents the
//! [`Config`] [`Size`] for SAMx5x chips
//!
//! This module defines the `Size` trait for SAMx5x chips. These chips always
//! operate in 32-bit extension mode and use the hardware `LENGTH` counter to
//! track the length of each transaction, in bytes. See the [`Length`]
//! documentation for more details.
//!
//! [`Config`]: super::Config
//! [`Size`]: super::Size

//use seq_macro::seq;       //  Melabr:     no longer needed as macro has been replaced by the expanded code...
use typenum::{Unsigned, U0, U1, U2, U3, U4};

use crate::typelevel::Sealed;

//=============================================================================
// Transaction length
//=============================================================================

/// Type-level enum representing the SPI transaction length, in bytes
///
/// This trait acts as both a [type-level enum], forming a type class for
/// transaction lengths, as well as a [type-level function] mapping the `Length`
/// to the corresponding [`Word`] size.
///
/// The SPI transaction length is represented in the type domain using
/// [`Unsigned`] types from the [`typenum`] crate. The length can be set
/// statically, using a length from `U1` to `U255`, or it can be set
/// dynamically, using the [`DynLength`] marker type. All valid `Length` types
/// are re-exported in this module.
///
/// The SPI transaction length affects the word size for the embedded HAL
/// traits, as well as other aspects of the SPI API. Transaction lengths of 1-4
/// only require a single read/write of the `DATA` register, so they have an
/// [`AtomicSize`] behave differently than longer transaction lengths.
///
/// [type-level enum]: crate::typelevel#type-level-enums
/// [type-level function]: crate::typelevel#type-level-functions
/// [`OpMode`]: super::OpMode
/// [`AtomicSize`]: super::AtomicSize
pub trait Length: Sealed + Unsigned + 'static {
    /// Word size for the transaction length
    ///
    /// For lengths 1-4, this type is `u8`, `u16` or `u32`. For longer
    /// transactions, this type is `[u8, Self::USIZE]`.
    type Word: 'static;
}

/// Type alias to recover the [`Word`](Length::Word) type from an
/// implementation of [`Length`]
pub type Word<L> = <L as Length>::Word;

/// Marker type for a run-time dynamic [`Length`]
pub type DynLength = U0;

impl Length for DynLength {
    type Word = ();
}

/// Marker trait for statically known transaction [`Length`]s
pub trait StaticLength: Length {}

impl StaticLength for U1 {}
impl Length for U1 {
    type Word = u8;
}

impl StaticLength for U2 {}
impl Length for U2 {
    type Word = u16;
}

impl StaticLength for U3 {}
impl Length for U3 {
    type Word = u32;
}

impl StaticLength for U4 {}
impl Length for U4 {
    type Word = u32;
}

/// Marker trait for transaction [`Length`]s greater than four
pub trait GreaterThan4: Length {}

//  Melabr: macro replaced by the expanded code...
//seq!(N in 5..=255 {
//    impl StaticLength for typenum::U~N {}
//    impl GreaterThan4 for typenum::U~N {}
//    impl Length for typenum::U~N {
//        type Word = [u8; typenum::U~N::USIZE];
//    }
//});
// Recursive expansion of seq! macro
// ==================================

impl StaticLength for typenum::U5 {}

impl GreaterThan4 for typenum::U5 {}

impl Length for typenum::U5 {
    type Word = [u8; typenum::U5::USIZE];
}
impl StaticLength for typenum::U6 {}

impl GreaterThan4 for typenum::U6 {}

impl Length for typenum::U6 {
    type Word = [u8; typenum::U6::USIZE];
}
impl StaticLength for typenum::U7 {}

impl GreaterThan4 for typenum::U7 {}

impl Length for typenum::U7 {
    type Word = [u8; typenum::U7::USIZE];
}
impl StaticLength for typenum::U8 {}

impl GreaterThan4 for typenum::U8 {}

impl Length for typenum::U8 {
    type Word = [u8; typenum::U8::USIZE];
}
impl StaticLength for typenum::U9 {}

impl GreaterThan4 for typenum::U9 {}

impl Length for typenum::U9 {
    type Word = [u8; typenum::U9::USIZE];
}
impl StaticLength for typenum::U10 {}

impl GreaterThan4 for typenum::U10 {}

impl Length for typenum::U10 {
    type Word = [u8; typenum::U10::USIZE];
}
impl StaticLength for typenum::U11 {}

impl GreaterThan4 for typenum::U11 {}

impl Length for typenum::U11 {
    type Word = [u8; typenum::U11::USIZE];
}
impl StaticLength for typenum::U12 {}

impl GreaterThan4 for typenum::U12 {}

impl Length for typenum::U12 {
    type Word = [u8; typenum::U12::USIZE];
}
impl StaticLength for typenum::U13 {}

impl GreaterThan4 for typenum::U13 {}

impl Length for typenum::U13 {
    type Word = [u8; typenum::U13::USIZE];
}
impl StaticLength for typenum::U14 {}

impl GreaterThan4 for typenum::U14 {}

impl Length for typenum::U14 {
    type Word = [u8; typenum::U14::USIZE];
}
impl StaticLength for typenum::U15 {}

impl GreaterThan4 for typenum::U15 {}

impl Length for typenum::U15 {
    type Word = [u8; typenum::U15::USIZE];
}
impl StaticLength for typenum::U16 {}

impl GreaterThan4 for typenum::U16 {}

impl Length for typenum::U16 {
    type Word = [u8; typenum::U16::USIZE];
}
impl StaticLength for typenum::U17 {}

impl GreaterThan4 for typenum::U17 {}

impl Length for typenum::U17 {
    type Word = [u8; typenum::U17::USIZE];
}
impl StaticLength for typenum::U18 {}

impl GreaterThan4 for typenum::U18 {}

impl Length for typenum::U18 {
    type Word = [u8; typenum::U18::USIZE];
}
impl StaticLength for typenum::U19 {}

impl GreaterThan4 for typenum::U19 {}

impl Length for typenum::U19 {
    type Word = [u8; typenum::U19::USIZE];
}
impl StaticLength for typenum::U20 {}

impl GreaterThan4 for typenum::U20 {}

impl Length for typenum::U20 {
    type Word = [u8; typenum::U20::USIZE];
}
impl StaticLength for typenum::U21 {}

impl GreaterThan4 for typenum::U21 {}

impl Length for typenum::U21 {
    type Word = [u8; typenum::U21::USIZE];
}
impl StaticLength for typenum::U22 {}

impl GreaterThan4 for typenum::U22 {}

impl Length for typenum::U22 {
    type Word = [u8; typenum::U22::USIZE];
}
impl StaticLength for typenum::U23 {}

impl GreaterThan4 for typenum::U23 {}

impl Length for typenum::U23 {
    type Word = [u8; typenum::U23::USIZE];
}
impl StaticLength for typenum::U24 {}

impl GreaterThan4 for typenum::U24 {}

impl Length for typenum::U24 {
    type Word = [u8; typenum::U24::USIZE];
}
impl StaticLength for typenum::U25 {}

impl GreaterThan4 for typenum::U25 {}

impl Length for typenum::U25 {
    type Word = [u8; typenum::U25::USIZE];
}
impl StaticLength for typenum::U26 {}

impl GreaterThan4 for typenum::U26 {}

impl Length for typenum::U26 {
    type Word = [u8; typenum::U26::USIZE];
}
impl StaticLength for typenum::U27 {}

impl GreaterThan4 for typenum::U27 {}

impl Length for typenum::U27 {
    type Word = [u8; typenum::U27::USIZE];
}
impl StaticLength for typenum::U28 {}

impl GreaterThan4 for typenum::U28 {}

impl Length for typenum::U28 {
    type Word = [u8; typenum::U28::USIZE];
}
impl StaticLength for typenum::U29 {}

impl GreaterThan4 for typenum::U29 {}

impl Length for typenum::U29 {
    type Word = [u8; typenum::U29::USIZE];
}
impl StaticLength for typenum::U30 {}

impl GreaterThan4 for typenum::U30 {}

impl Length for typenum::U30 {
    type Word = [u8; typenum::U30::USIZE];
}
impl StaticLength for typenum::U31 {}

impl GreaterThan4 for typenum::U31 {}

impl Length for typenum::U31 {
    type Word = [u8; typenum::U31::USIZE];
}
impl StaticLength for typenum::U32 {}

impl GreaterThan4 for typenum::U32 {}

impl Length for typenum::U32 {
    type Word = [u8; typenum::U32::USIZE];
}
impl StaticLength for typenum::U33 {}

impl GreaterThan4 for typenum::U33 {}

impl Length for typenum::U33 {
    type Word = [u8; typenum::U33::USIZE];
}
impl StaticLength for typenum::U34 {}

impl GreaterThan4 for typenum::U34 {}

impl Length for typenum::U34 {
    type Word = [u8; typenum::U34::USIZE];
}
impl StaticLength for typenum::U35 {}

impl GreaterThan4 for typenum::U35 {}

impl Length for typenum::U35 {
    type Word = [u8; typenum::U35::USIZE];
}
impl StaticLength for typenum::U36 {}

impl GreaterThan4 for typenum::U36 {}

impl Length for typenum::U36 {
    type Word = [u8; typenum::U36::USIZE];
}
impl StaticLength for typenum::U37 {}

impl GreaterThan4 for typenum::U37 {}

impl Length for typenum::U37 {
    type Word = [u8; typenum::U37::USIZE];
}
impl StaticLength for typenum::U38 {}

impl GreaterThan4 for typenum::U38 {}

impl Length for typenum::U38 {
    type Word = [u8; typenum::U38::USIZE];
}
impl StaticLength for typenum::U39 {}

impl GreaterThan4 for typenum::U39 {}

impl Length for typenum::U39 {
    type Word = [u8; typenum::U39::USIZE];
}
impl StaticLength for typenum::U40 {}

impl GreaterThan4 for typenum::U40 {}

impl Length for typenum::U40 {
    type Word = [u8; typenum::U40::USIZE];
}
impl StaticLength for typenum::U41 {}

impl GreaterThan4 for typenum::U41 {}

impl Length for typenum::U41 {
    type Word = [u8; typenum::U41::USIZE];
}
impl StaticLength for typenum::U42 {}

impl GreaterThan4 for typenum::U42 {}

impl Length for typenum::U42 {
    type Word = [u8; typenum::U42::USIZE];
}
impl StaticLength for typenum::U43 {}

impl GreaterThan4 for typenum::U43 {}

impl Length for typenum::U43 {
    type Word = [u8; typenum::U43::USIZE];
}
impl StaticLength for typenum::U44 {}

impl GreaterThan4 for typenum::U44 {}

impl Length for typenum::U44 {
    type Word = [u8; typenum::U44::USIZE];
}
impl StaticLength for typenum::U45 {}

impl GreaterThan4 for typenum::U45 {}

impl Length for typenum::U45 {
    type Word = [u8; typenum::U45::USIZE];
}
impl StaticLength for typenum::U46 {}

impl GreaterThan4 for typenum::U46 {}

impl Length for typenum::U46 {
    type Word = [u8; typenum::U46::USIZE];
}
impl StaticLength for typenum::U47 {}

impl GreaterThan4 for typenum::U47 {}

impl Length for typenum::U47 {
    type Word = [u8; typenum::U47::USIZE];
}
impl StaticLength for typenum::U48 {}

impl GreaterThan4 for typenum::U48 {}

impl Length for typenum::U48 {
    type Word = [u8; typenum::U48::USIZE];
}
impl StaticLength for typenum::U49 {}

impl GreaterThan4 for typenum::U49 {}

impl Length for typenum::U49 {
    type Word = [u8; typenum::U49::USIZE];
}
impl StaticLength for typenum::U50 {}

impl GreaterThan4 for typenum::U50 {}

impl Length for typenum::U50 {
    type Word = [u8; typenum::U50::USIZE];
}
impl StaticLength for typenum::U51 {}

impl GreaterThan4 for typenum::U51 {}

impl Length for typenum::U51 {
    type Word = [u8; typenum::U51::USIZE];
}
impl StaticLength for typenum::U52 {}

impl GreaterThan4 for typenum::U52 {}

impl Length for typenum::U52 {
    type Word = [u8; typenum::U52::USIZE];
}
impl StaticLength for typenum::U53 {}

impl GreaterThan4 for typenum::U53 {}

impl Length for typenum::U53 {
    type Word = [u8; typenum::U53::USIZE];
}
impl StaticLength for typenum::U54 {}

impl GreaterThan4 for typenum::U54 {}

impl Length for typenum::U54 {
    type Word = [u8; typenum::U54::USIZE];
}
impl StaticLength for typenum::U55 {}

impl GreaterThan4 for typenum::U55 {}

impl Length for typenum::U55 {
    type Word = [u8; typenum::U55::USIZE];
}
impl StaticLength for typenum::U56 {}

impl GreaterThan4 for typenum::U56 {}

impl Length for typenum::U56 {
    type Word = [u8; typenum::U56::USIZE];
}
impl StaticLength for typenum::U57 {}

impl GreaterThan4 for typenum::U57 {}

impl Length for typenum::U57 {
    type Word = [u8; typenum::U57::USIZE];
}
impl StaticLength for typenum::U58 {}

impl GreaterThan4 for typenum::U58 {}

impl Length for typenum::U58 {
    type Word = [u8; typenum::U58::USIZE];
}
impl StaticLength for typenum::U59 {}

impl GreaterThan4 for typenum::U59 {}

impl Length for typenum::U59 {
    type Word = [u8; typenum::U59::USIZE];
}
impl StaticLength for typenum::U60 {}

impl GreaterThan4 for typenum::U60 {}

impl Length for typenum::U60 {
    type Word = [u8; typenum::U60::USIZE];
}
impl StaticLength for typenum::U61 {}

impl GreaterThan4 for typenum::U61 {}

impl Length for typenum::U61 {
    type Word = [u8; typenum::U61::USIZE];
}
impl StaticLength for typenum::U62 {}

impl GreaterThan4 for typenum::U62 {}

impl Length for typenum::U62 {
    type Word = [u8; typenum::U62::USIZE];
}
impl StaticLength for typenum::U63 {}

impl GreaterThan4 for typenum::U63 {}

impl Length for typenum::U63 {
    type Word = [u8; typenum::U63::USIZE];
}
impl StaticLength for typenum::U64 {}

impl GreaterThan4 for typenum::U64 {}

impl Length for typenum::U64 {
    type Word = [u8; typenum::U64::USIZE];
}
impl StaticLength for typenum::U65 {}

impl GreaterThan4 for typenum::U65 {}

impl Length for typenum::U65 {
    type Word = [u8; typenum::U65::USIZE];
}
impl StaticLength for typenum::U66 {}

impl GreaterThan4 for typenum::U66 {}

impl Length for typenum::U66 {
    type Word = [u8; typenum::U66::USIZE];
}
impl StaticLength for typenum::U67 {}

impl GreaterThan4 for typenum::U67 {}

impl Length for typenum::U67 {
    type Word = [u8; typenum::U67::USIZE];
}
impl StaticLength for typenum::U68 {}

impl GreaterThan4 for typenum::U68 {}

impl Length for typenum::U68 {
    type Word = [u8; typenum::U68::USIZE];
}
impl StaticLength for typenum::U69 {}

impl GreaterThan4 for typenum::U69 {}

impl Length for typenum::U69 {
    type Word = [u8; typenum::U69::USIZE];
}
impl StaticLength for typenum::U70 {}

impl GreaterThan4 for typenum::U70 {}

impl Length for typenum::U70 {
    type Word = [u8; typenum::U70::USIZE];
}
impl StaticLength for typenum::U71 {}

impl GreaterThan4 for typenum::U71 {}

impl Length for typenum::U71 {
    type Word = [u8; typenum::U71::USIZE];
}
impl StaticLength for typenum::U72 {}

impl GreaterThan4 for typenum::U72 {}

impl Length for typenum::U72 {
    type Word = [u8; typenum::U72::USIZE];
}
impl StaticLength for typenum::U73 {}

impl GreaterThan4 for typenum::U73 {}

impl Length for typenum::U73 {
    type Word = [u8; typenum::U73::USIZE];
}
impl StaticLength for typenum::U74 {}

impl GreaterThan4 for typenum::U74 {}

impl Length for typenum::U74 {
    type Word = [u8; typenum::U74::USIZE];
}
impl StaticLength for typenum::U75 {}

impl GreaterThan4 for typenum::U75 {}

impl Length for typenum::U75 {
    type Word = [u8; typenum::U75::USIZE];
}
impl StaticLength for typenum::U76 {}

impl GreaterThan4 for typenum::U76 {}

impl Length for typenum::U76 {
    type Word = [u8; typenum::U76::USIZE];
}
impl StaticLength for typenum::U77 {}

impl GreaterThan4 for typenum::U77 {}

impl Length for typenum::U77 {
    type Word = [u8; typenum::U77::USIZE];
}
impl StaticLength for typenum::U78 {}

impl GreaterThan4 for typenum::U78 {}

impl Length for typenum::U78 {
    type Word = [u8; typenum::U78::USIZE];
}
impl StaticLength for typenum::U79 {}

impl GreaterThan4 for typenum::U79 {}

impl Length for typenum::U79 {
    type Word = [u8; typenum::U79::USIZE];
}
impl StaticLength for typenum::U80 {}

impl GreaterThan4 for typenum::U80 {}

impl Length for typenum::U80 {
    type Word = [u8; typenum::U80::USIZE];
}
impl StaticLength for typenum::U81 {}

impl GreaterThan4 for typenum::U81 {}

impl Length for typenum::U81 {
    type Word = [u8; typenum::U81::USIZE];
}
impl StaticLength for typenum::U82 {}

impl GreaterThan4 for typenum::U82 {}

impl Length for typenum::U82 {
    type Word = [u8; typenum::U82::USIZE];
}
impl StaticLength for typenum::U83 {}

impl GreaterThan4 for typenum::U83 {}

impl Length for typenum::U83 {
    type Word = [u8; typenum::U83::USIZE];
}
impl StaticLength for typenum::U84 {}

impl GreaterThan4 for typenum::U84 {}

impl Length for typenum::U84 {
    type Word = [u8; typenum::U84::USIZE];
}
impl StaticLength for typenum::U85 {}

impl GreaterThan4 for typenum::U85 {}

impl Length for typenum::U85 {
    type Word = [u8; typenum::U85::USIZE];
}
impl StaticLength for typenum::U86 {}

impl GreaterThan4 for typenum::U86 {}

impl Length for typenum::U86 {
    type Word = [u8; typenum::U86::USIZE];
}
impl StaticLength for typenum::U87 {}

impl GreaterThan4 for typenum::U87 {}

impl Length for typenum::U87 {
    type Word = [u8; typenum::U87::USIZE];
}
impl StaticLength for typenum::U88 {}

impl GreaterThan4 for typenum::U88 {}

impl Length for typenum::U88 {
    type Word = [u8; typenum::U88::USIZE];
}
impl StaticLength for typenum::U89 {}

impl GreaterThan4 for typenum::U89 {}

impl Length for typenum::U89 {
    type Word = [u8; typenum::U89::USIZE];
}
impl StaticLength for typenum::U90 {}

impl GreaterThan4 for typenum::U90 {}

impl Length for typenum::U90 {
    type Word = [u8; typenum::U90::USIZE];
}
impl StaticLength for typenum::U91 {}

impl GreaterThan4 for typenum::U91 {}

impl Length for typenum::U91 {
    type Word = [u8; typenum::U91::USIZE];
}
impl StaticLength for typenum::U92 {}

impl GreaterThan4 for typenum::U92 {}

impl Length for typenum::U92 {
    type Word = [u8; typenum::U92::USIZE];
}
impl StaticLength for typenum::U93 {}

impl GreaterThan4 for typenum::U93 {}

impl Length for typenum::U93 {
    type Word = [u8; typenum::U93::USIZE];
}
impl StaticLength for typenum::U94 {}

impl GreaterThan4 for typenum::U94 {}

impl Length for typenum::U94 {
    type Word = [u8; typenum::U94::USIZE];
}
impl StaticLength for typenum::U95 {}

impl GreaterThan4 for typenum::U95 {}

impl Length for typenum::U95 {
    type Word = [u8; typenum::U95::USIZE];
}
impl StaticLength for typenum::U96 {}

impl GreaterThan4 for typenum::U96 {}

impl Length for typenum::U96 {
    type Word = [u8; typenum::U96::USIZE];
}
impl StaticLength for typenum::U97 {}

impl GreaterThan4 for typenum::U97 {}

impl Length for typenum::U97 {
    type Word = [u8; typenum::U97::USIZE];
}
impl StaticLength for typenum::U98 {}

impl GreaterThan4 for typenum::U98 {}

impl Length for typenum::U98 {
    type Word = [u8; typenum::U98::USIZE];
}
impl StaticLength for typenum::U99 {}

impl GreaterThan4 for typenum::U99 {}

impl Length for typenum::U99 {
    type Word = [u8; typenum::U99::USIZE];
}
impl StaticLength for typenum::U100 {}

impl GreaterThan4 for typenum::U100 {}

impl Length for typenum::U100 {
    type Word = [u8; typenum::U100::USIZE];
}
impl StaticLength for typenum::U101 {}

impl GreaterThan4 for typenum::U101 {}

impl Length for typenum::U101 {
    type Word = [u8; typenum::U101::USIZE];
}
impl StaticLength for typenum::U102 {}

impl GreaterThan4 for typenum::U102 {}

impl Length for typenum::U102 {
    type Word = [u8; typenum::U102::USIZE];
}
impl StaticLength for typenum::U103 {}

impl GreaterThan4 for typenum::U103 {}

impl Length for typenum::U103 {
    type Word = [u8; typenum::U103::USIZE];
}
impl StaticLength for typenum::U104 {}

impl GreaterThan4 for typenum::U104 {}

impl Length for typenum::U104 {
    type Word = [u8; typenum::U104::USIZE];
}
impl StaticLength for typenum::U105 {}

impl GreaterThan4 for typenum::U105 {}

impl Length for typenum::U105 {
    type Word = [u8; typenum::U105::USIZE];
}
impl StaticLength for typenum::U106 {}

impl GreaterThan4 for typenum::U106 {}

impl Length for typenum::U106 {
    type Word = [u8; typenum::U106::USIZE];
}
impl StaticLength for typenum::U107 {}

impl GreaterThan4 for typenum::U107 {}

impl Length for typenum::U107 {
    type Word = [u8; typenum::U107::USIZE];
}
impl StaticLength for typenum::U108 {}

impl GreaterThan4 for typenum::U108 {}

impl Length for typenum::U108 {
    type Word = [u8; typenum::U108::USIZE];
}
impl StaticLength for typenum::U109 {}

impl GreaterThan4 for typenum::U109 {}

impl Length for typenum::U109 {
    type Word = [u8; typenum::U109::USIZE];
}
impl StaticLength for typenum::U110 {}

impl GreaterThan4 for typenum::U110 {}

impl Length for typenum::U110 {
    type Word = [u8; typenum::U110::USIZE];
}
impl StaticLength for typenum::U111 {}

impl GreaterThan4 for typenum::U111 {}

impl Length for typenum::U111 {
    type Word = [u8; typenum::U111::USIZE];
}
impl StaticLength for typenum::U112 {}

impl GreaterThan4 for typenum::U112 {}

impl Length for typenum::U112 {
    type Word = [u8; typenum::U112::USIZE];
}
impl StaticLength for typenum::U113 {}

impl GreaterThan4 for typenum::U113 {}

impl Length for typenum::U113 {
    type Word = [u8; typenum::U113::USIZE];
}
impl StaticLength for typenum::U114 {}

impl GreaterThan4 for typenum::U114 {}

impl Length for typenum::U114 {
    type Word = [u8; typenum::U114::USIZE];
}
impl StaticLength for typenum::U115 {}

impl GreaterThan4 for typenum::U115 {}

impl Length for typenum::U115 {
    type Word = [u8; typenum::U115::USIZE];
}
impl StaticLength for typenum::U116 {}

impl GreaterThan4 for typenum::U116 {}

impl Length for typenum::U116 {
    type Word = [u8; typenum::U116::USIZE];
}
impl StaticLength for typenum::U117 {}

impl GreaterThan4 for typenum::U117 {}

impl Length for typenum::U117 {
    type Word = [u8; typenum::U117::USIZE];
}
impl StaticLength for typenum::U118 {}

impl GreaterThan4 for typenum::U118 {}

impl Length for typenum::U118 {
    type Word = [u8; typenum::U118::USIZE];
}
impl StaticLength for typenum::U119 {}

impl GreaterThan4 for typenum::U119 {}

impl Length for typenum::U119 {
    type Word = [u8; typenum::U119::USIZE];
}
impl StaticLength for typenum::U120 {}

impl GreaterThan4 for typenum::U120 {}

impl Length for typenum::U120 {
    type Word = [u8; typenum::U120::USIZE];
}
impl StaticLength for typenum::U121 {}

impl GreaterThan4 for typenum::U121 {}

impl Length for typenum::U121 {
    type Word = [u8; typenum::U121::USIZE];
}
impl StaticLength for typenum::U122 {}

impl GreaterThan4 for typenum::U122 {}

impl Length for typenum::U122 {
    type Word = [u8; typenum::U122::USIZE];
}
impl StaticLength for typenum::U123 {}

impl GreaterThan4 for typenum::U123 {}

impl Length for typenum::U123 {
    type Word = [u8; typenum::U123::USIZE];
}
impl StaticLength for typenum::U124 {}

impl GreaterThan4 for typenum::U124 {}

impl Length for typenum::U124 {
    type Word = [u8; typenum::U124::USIZE];
}
impl StaticLength for typenum::U125 {}

impl GreaterThan4 for typenum::U125 {}

impl Length for typenum::U125 {
    type Word = [u8; typenum::U125::USIZE];
}
impl StaticLength for typenum::U126 {}

impl GreaterThan4 for typenum::U126 {}

impl Length for typenum::U126 {
    type Word = [u8; typenum::U126::USIZE];
}
impl StaticLength for typenum::U127 {}

impl GreaterThan4 for typenum::U127 {}

impl Length for typenum::U127 {
    type Word = [u8; typenum::U127::USIZE];
}
impl StaticLength for typenum::U128 {}

impl GreaterThan4 for typenum::U128 {}

impl Length for typenum::U128 {
    type Word = [u8; typenum::U128::USIZE];
}
impl StaticLength for typenum::U129 {}

impl GreaterThan4 for typenum::U129 {}

impl Length for typenum::U129 {
    type Word = [u8; typenum::U129::USIZE];
}
impl StaticLength for typenum::U130 {}

impl GreaterThan4 for typenum::U130 {}

impl Length for typenum::U130 {
    type Word = [u8; typenum::U130::USIZE];
}
impl StaticLength for typenum::U131 {}

impl GreaterThan4 for typenum::U131 {}

impl Length for typenum::U131 {
    type Word = [u8; typenum::U131::USIZE];
}
impl StaticLength for typenum::U132 {}

impl GreaterThan4 for typenum::U132 {}

impl Length for typenum::U132 {
    type Word = [u8; typenum::U132::USIZE];
}
impl StaticLength for typenum::U133 {}

impl GreaterThan4 for typenum::U133 {}

impl Length for typenum::U133 {
    type Word = [u8; typenum::U133::USIZE];
}
impl StaticLength for typenum::U134 {}

impl GreaterThan4 for typenum::U134 {}

impl Length for typenum::U134 {
    type Word = [u8; typenum::U134::USIZE];
}
impl StaticLength for typenum::U135 {}

impl GreaterThan4 for typenum::U135 {}

impl Length for typenum::U135 {
    type Word = [u8; typenum::U135::USIZE];
}
impl StaticLength for typenum::U136 {}

impl GreaterThan4 for typenum::U136 {}

impl Length for typenum::U136 {
    type Word = [u8; typenum::U136::USIZE];
}
impl StaticLength for typenum::U137 {}

impl GreaterThan4 for typenum::U137 {}

impl Length for typenum::U137 {
    type Word = [u8; typenum::U137::USIZE];
}
impl StaticLength for typenum::U138 {}

impl GreaterThan4 for typenum::U138 {}

impl Length for typenum::U138 {
    type Word = [u8; typenum::U138::USIZE];
}
impl StaticLength for typenum::U139 {}

impl GreaterThan4 for typenum::U139 {}

impl Length for typenum::U139 {
    type Word = [u8; typenum::U139::USIZE];
}
impl StaticLength for typenum::U140 {}

impl GreaterThan4 for typenum::U140 {}

impl Length for typenum::U140 {
    type Word = [u8; typenum::U140::USIZE];
}
impl StaticLength for typenum::U141 {}

impl GreaterThan4 for typenum::U141 {}

impl Length for typenum::U141 {
    type Word = [u8; typenum::U141::USIZE];
}
impl StaticLength for typenum::U142 {}

impl GreaterThan4 for typenum::U142 {}

impl Length for typenum::U142 {
    type Word = [u8; typenum::U142::USIZE];
}
impl StaticLength for typenum::U143 {}

impl GreaterThan4 for typenum::U143 {}

impl Length for typenum::U143 {
    type Word = [u8; typenum::U143::USIZE];
}
impl StaticLength for typenum::U144 {}

impl GreaterThan4 for typenum::U144 {}

impl Length for typenum::U144 {
    type Word = [u8; typenum::U144::USIZE];
}
impl StaticLength for typenum::U145 {}

impl GreaterThan4 for typenum::U145 {}

impl Length for typenum::U145 {
    type Word = [u8; typenum::U145::USIZE];
}
impl StaticLength for typenum::U146 {}

impl GreaterThan4 for typenum::U146 {}

impl Length for typenum::U146 {
    type Word = [u8; typenum::U146::USIZE];
}
impl StaticLength for typenum::U147 {}

impl GreaterThan4 for typenum::U147 {}

impl Length for typenum::U147 {
    type Word = [u8; typenum::U147::USIZE];
}
impl StaticLength for typenum::U148 {}

impl GreaterThan4 for typenum::U148 {}

impl Length for typenum::U148 {
    type Word = [u8; typenum::U148::USIZE];
}
impl StaticLength for typenum::U149 {}

impl GreaterThan4 for typenum::U149 {}

impl Length for typenum::U149 {
    type Word = [u8; typenum::U149::USIZE];
}
impl StaticLength for typenum::U150 {}

impl GreaterThan4 for typenum::U150 {}

impl Length for typenum::U150 {
    type Word = [u8; typenum::U150::USIZE];
}
impl StaticLength for typenum::U151 {}

impl GreaterThan4 for typenum::U151 {}

impl Length for typenum::U151 {
    type Word = [u8; typenum::U151::USIZE];
}
impl StaticLength for typenum::U152 {}

impl GreaterThan4 for typenum::U152 {}

impl Length for typenum::U152 {
    type Word = [u8; typenum::U152::USIZE];
}
impl StaticLength for typenum::U153 {}

impl GreaterThan4 for typenum::U153 {}

impl Length for typenum::U153 {
    type Word = [u8; typenum::U153::USIZE];
}
impl StaticLength for typenum::U154 {}

impl GreaterThan4 for typenum::U154 {}

impl Length for typenum::U154 {
    type Word = [u8; typenum::U154::USIZE];
}
impl StaticLength for typenum::U155 {}

impl GreaterThan4 for typenum::U155 {}

impl Length for typenum::U155 {
    type Word = [u8; typenum::U155::USIZE];
}
impl StaticLength for typenum::U156 {}

impl GreaterThan4 for typenum::U156 {}

impl Length for typenum::U156 {
    type Word = [u8; typenum::U156::USIZE];
}
impl StaticLength for typenum::U157 {}

impl GreaterThan4 for typenum::U157 {}

impl Length for typenum::U157 {
    type Word = [u8; typenum::U157::USIZE];
}
impl StaticLength for typenum::U158 {}

impl GreaterThan4 for typenum::U158 {}

impl Length for typenum::U158 {
    type Word = [u8; typenum::U158::USIZE];
}
impl StaticLength for typenum::U159 {}

impl GreaterThan4 for typenum::U159 {}

impl Length for typenum::U159 {
    type Word = [u8; typenum::U159::USIZE];
}
impl StaticLength for typenum::U160 {}

impl GreaterThan4 for typenum::U160 {}

impl Length for typenum::U160 {
    type Word = [u8; typenum::U160::USIZE];
}
impl StaticLength for typenum::U161 {}

impl GreaterThan4 for typenum::U161 {}

impl Length for typenum::U161 {
    type Word = [u8; typenum::U161::USIZE];
}
impl StaticLength for typenum::U162 {}

impl GreaterThan4 for typenum::U162 {}

impl Length for typenum::U162 {
    type Word = [u8; typenum::U162::USIZE];
}
impl StaticLength for typenum::U163 {}

impl GreaterThan4 for typenum::U163 {}

impl Length for typenum::U163 {
    type Word = [u8; typenum::U163::USIZE];
}
impl StaticLength for typenum::U164 {}

impl GreaterThan4 for typenum::U164 {}

impl Length for typenum::U164 {
    type Word = [u8; typenum::U164::USIZE];
}
impl StaticLength for typenum::U165 {}

impl GreaterThan4 for typenum::U165 {}

impl Length for typenum::U165 {
    type Word = [u8; typenum::U165::USIZE];
}
impl StaticLength for typenum::U166 {}

impl GreaterThan4 for typenum::U166 {}

impl Length for typenum::U166 {
    type Word = [u8; typenum::U166::USIZE];
}
impl StaticLength for typenum::U167 {}

impl GreaterThan4 for typenum::U167 {}

impl Length for typenum::U167 {
    type Word = [u8; typenum::U167::USIZE];
}
impl StaticLength for typenum::U168 {}

impl GreaterThan4 for typenum::U168 {}

impl Length for typenum::U168 {
    type Word = [u8; typenum::U168::USIZE];
}
impl StaticLength for typenum::U169 {}

impl GreaterThan4 for typenum::U169 {}

impl Length for typenum::U169 {
    type Word = [u8; typenum::U169::USIZE];
}
impl StaticLength for typenum::U170 {}

impl GreaterThan4 for typenum::U170 {}

impl Length for typenum::U170 {
    type Word = [u8; typenum::U170::USIZE];
}
impl StaticLength for typenum::U171 {}

impl GreaterThan4 for typenum::U171 {}

impl Length for typenum::U171 {
    type Word = [u8; typenum::U171::USIZE];
}
impl StaticLength for typenum::U172 {}

impl GreaterThan4 for typenum::U172 {}

impl Length for typenum::U172 {
    type Word = [u8; typenum::U172::USIZE];
}
impl StaticLength for typenum::U173 {}

impl GreaterThan4 for typenum::U173 {}

impl Length for typenum::U173 {
    type Word = [u8; typenum::U173::USIZE];
}
impl StaticLength for typenum::U174 {}

impl GreaterThan4 for typenum::U174 {}

impl Length for typenum::U174 {
    type Word = [u8; typenum::U174::USIZE];
}
impl StaticLength for typenum::U175 {}

impl GreaterThan4 for typenum::U175 {}

impl Length for typenum::U175 {
    type Word = [u8; typenum::U175::USIZE];
}
impl StaticLength for typenum::U176 {}

impl GreaterThan4 for typenum::U176 {}

impl Length for typenum::U176 {
    type Word = [u8; typenum::U176::USIZE];
}
impl StaticLength for typenum::U177 {}

impl GreaterThan4 for typenum::U177 {}

impl Length for typenum::U177 {
    type Word = [u8; typenum::U177::USIZE];
}
impl StaticLength for typenum::U178 {}

impl GreaterThan4 for typenum::U178 {}

impl Length for typenum::U178 {
    type Word = [u8; typenum::U178::USIZE];
}
impl StaticLength for typenum::U179 {}

impl GreaterThan4 for typenum::U179 {}

impl Length for typenum::U179 {
    type Word = [u8; typenum::U179::USIZE];
}
impl StaticLength for typenum::U180 {}

impl GreaterThan4 for typenum::U180 {}

impl Length for typenum::U180 {
    type Word = [u8; typenum::U180::USIZE];
}
impl StaticLength for typenum::U181 {}

impl GreaterThan4 for typenum::U181 {}

impl Length for typenum::U181 {
    type Word = [u8; typenum::U181::USIZE];
}
impl StaticLength for typenum::U182 {}

impl GreaterThan4 for typenum::U182 {}

impl Length for typenum::U182 {
    type Word = [u8; typenum::U182::USIZE];
}
impl StaticLength for typenum::U183 {}

impl GreaterThan4 for typenum::U183 {}

impl Length for typenum::U183 {
    type Word = [u8; typenum::U183::USIZE];
}
impl StaticLength for typenum::U184 {}

impl GreaterThan4 for typenum::U184 {}

impl Length for typenum::U184 {
    type Word = [u8; typenum::U184::USIZE];
}
impl StaticLength for typenum::U185 {}

impl GreaterThan4 for typenum::U185 {}

impl Length for typenum::U185 {
    type Word = [u8; typenum::U185::USIZE];
}
impl StaticLength for typenum::U186 {}

impl GreaterThan4 for typenum::U186 {}

impl Length for typenum::U186 {
    type Word = [u8; typenum::U186::USIZE];
}
impl StaticLength for typenum::U187 {}

impl GreaterThan4 for typenum::U187 {}

impl Length for typenum::U187 {
    type Word = [u8; typenum::U187::USIZE];
}
impl StaticLength for typenum::U188 {}

impl GreaterThan4 for typenum::U188 {}

impl Length for typenum::U188 {
    type Word = [u8; typenum::U188::USIZE];
}
impl StaticLength for typenum::U189 {}

impl GreaterThan4 for typenum::U189 {}

impl Length for typenum::U189 {
    type Word = [u8; typenum::U189::USIZE];
}
impl StaticLength for typenum::U190 {}

impl GreaterThan4 for typenum::U190 {}

impl Length for typenum::U190 {
    type Word = [u8; typenum::U190::USIZE];
}
impl StaticLength for typenum::U191 {}

impl GreaterThan4 for typenum::U191 {}

impl Length for typenum::U191 {
    type Word = [u8; typenum::U191::USIZE];
}
impl StaticLength for typenum::U192 {}

impl GreaterThan4 for typenum::U192 {}

impl Length for typenum::U192 {
    type Word = [u8; typenum::U192::USIZE];
}
impl StaticLength for typenum::U193 {}

impl GreaterThan4 for typenum::U193 {}

impl Length for typenum::U193 {
    type Word = [u8; typenum::U193::USIZE];
}
impl StaticLength for typenum::U194 {}

impl GreaterThan4 for typenum::U194 {}

impl Length for typenum::U194 {
    type Word = [u8; typenum::U194::USIZE];
}
impl StaticLength for typenum::U195 {}

impl GreaterThan4 for typenum::U195 {}

impl Length for typenum::U195 {
    type Word = [u8; typenum::U195::USIZE];
}
impl StaticLength for typenum::U196 {}

impl GreaterThan4 for typenum::U196 {}

impl Length for typenum::U196 {
    type Word = [u8; typenum::U196::USIZE];
}
impl StaticLength for typenum::U197 {}

impl GreaterThan4 for typenum::U197 {}

impl Length for typenum::U197 {
    type Word = [u8; typenum::U197::USIZE];
}
impl StaticLength for typenum::U198 {}

impl GreaterThan4 for typenum::U198 {}

impl Length for typenum::U198 {
    type Word = [u8; typenum::U198::USIZE];
}
impl StaticLength for typenum::U199 {}

impl GreaterThan4 for typenum::U199 {}

impl Length for typenum::U199 {
    type Word = [u8; typenum::U199::USIZE];
}
impl StaticLength for typenum::U200 {}

impl GreaterThan4 for typenum::U200 {}

impl Length for typenum::U200 {
    type Word = [u8; typenum::U200::USIZE];
}
impl StaticLength for typenum::U201 {}

impl GreaterThan4 for typenum::U201 {}

impl Length for typenum::U201 {
    type Word = [u8; typenum::U201::USIZE];
}
impl StaticLength for typenum::U202 {}

impl GreaterThan4 for typenum::U202 {}

impl Length for typenum::U202 {
    type Word = [u8; typenum::U202::USIZE];
}
impl StaticLength for typenum::U203 {}

impl GreaterThan4 for typenum::U203 {}

impl Length for typenum::U203 {
    type Word = [u8; typenum::U203::USIZE];
}
impl StaticLength for typenum::U204 {}

impl GreaterThan4 for typenum::U204 {}

impl Length for typenum::U204 {
    type Word = [u8; typenum::U204::USIZE];
}
impl StaticLength for typenum::U205 {}

impl GreaterThan4 for typenum::U205 {}

impl Length for typenum::U205 {
    type Word = [u8; typenum::U205::USIZE];
}
impl StaticLength for typenum::U206 {}

impl GreaterThan4 for typenum::U206 {}

impl Length for typenum::U206 {
    type Word = [u8; typenum::U206::USIZE];
}
impl StaticLength for typenum::U207 {}

impl GreaterThan4 for typenum::U207 {}

impl Length for typenum::U207 {
    type Word = [u8; typenum::U207::USIZE];
}
impl StaticLength for typenum::U208 {}

impl GreaterThan4 for typenum::U208 {}

impl Length for typenum::U208 {
    type Word = [u8; typenum::U208::USIZE];
}
impl StaticLength for typenum::U209 {}

impl GreaterThan4 for typenum::U209 {}

impl Length for typenum::U209 {
    type Word = [u8; typenum::U209::USIZE];
}
impl StaticLength for typenum::U210 {}

impl GreaterThan4 for typenum::U210 {}

impl Length for typenum::U210 {
    type Word = [u8; typenum::U210::USIZE];
}
impl StaticLength for typenum::U211 {}

impl GreaterThan4 for typenum::U211 {}

impl Length for typenum::U211 {
    type Word = [u8; typenum::U211::USIZE];
}
impl StaticLength for typenum::U212 {}

impl GreaterThan4 for typenum::U212 {}

impl Length for typenum::U212 {
    type Word = [u8; typenum::U212::USIZE];
}
impl StaticLength for typenum::U213 {}

impl GreaterThan4 for typenum::U213 {}

impl Length for typenum::U213 {
    type Word = [u8; typenum::U213::USIZE];
}
impl StaticLength for typenum::U214 {}

impl GreaterThan4 for typenum::U214 {}

impl Length for typenum::U214 {
    type Word = [u8; typenum::U214::USIZE];
}
impl StaticLength for typenum::U215 {}

impl GreaterThan4 for typenum::U215 {}

impl Length for typenum::U215 {
    type Word = [u8; typenum::U215::USIZE];
}
impl StaticLength for typenum::U216 {}

impl GreaterThan4 for typenum::U216 {}

impl Length for typenum::U216 {
    type Word = [u8; typenum::U216::USIZE];
}
impl StaticLength for typenum::U217 {}

impl GreaterThan4 for typenum::U217 {}

impl Length for typenum::U217 {
    type Word = [u8; typenum::U217::USIZE];
}
impl StaticLength for typenum::U218 {}

impl GreaterThan4 for typenum::U218 {}

impl Length for typenum::U218 {
    type Word = [u8; typenum::U218::USIZE];
}
impl StaticLength for typenum::U219 {}

impl GreaterThan4 for typenum::U219 {}

impl Length for typenum::U219 {
    type Word = [u8; typenum::U219::USIZE];
}
impl StaticLength for typenum::U220 {}

impl GreaterThan4 for typenum::U220 {}

impl Length for typenum::U220 {
    type Word = [u8; typenum::U220::USIZE];
}
impl StaticLength for typenum::U221 {}

impl GreaterThan4 for typenum::U221 {}

impl Length for typenum::U221 {
    type Word = [u8; typenum::U221::USIZE];
}
impl StaticLength for typenum::U222 {}

impl GreaterThan4 for typenum::U222 {}

impl Length for typenum::U222 {
    type Word = [u8; typenum::U222::USIZE];
}
impl StaticLength for typenum::U223 {}

impl GreaterThan4 for typenum::U223 {}

impl Length for typenum::U223 {
    type Word = [u8; typenum::U223::USIZE];
}
impl StaticLength for typenum::U224 {}

impl GreaterThan4 for typenum::U224 {}

impl Length for typenum::U224 {
    type Word = [u8; typenum::U224::USIZE];
}
impl StaticLength for typenum::U225 {}

impl GreaterThan4 for typenum::U225 {}

impl Length for typenum::U225 {
    type Word = [u8; typenum::U225::USIZE];
}
impl StaticLength for typenum::U226 {}

impl GreaterThan4 for typenum::U226 {}

impl Length for typenum::U226 {
    type Word = [u8; typenum::U226::USIZE];
}
impl StaticLength for typenum::U227 {}

impl GreaterThan4 for typenum::U227 {}

impl Length for typenum::U227 {
    type Word = [u8; typenum::U227::USIZE];
}
impl StaticLength for typenum::U228 {}

impl GreaterThan4 for typenum::U228 {}

impl Length for typenum::U228 {
    type Word = [u8; typenum::U228::USIZE];
}
impl StaticLength for typenum::U229 {}

impl GreaterThan4 for typenum::U229 {}

impl Length for typenum::U229 {
    type Word = [u8; typenum::U229::USIZE];
}
impl StaticLength for typenum::U230 {}

impl GreaterThan4 for typenum::U230 {}

impl Length for typenum::U230 {
    type Word = [u8; typenum::U230::USIZE];
}
impl StaticLength for typenum::U231 {}

impl GreaterThan4 for typenum::U231 {}

impl Length for typenum::U231 {
    type Word = [u8; typenum::U231::USIZE];
}
impl StaticLength for typenum::U232 {}

impl GreaterThan4 for typenum::U232 {}

impl Length for typenum::U232 {
    type Word = [u8; typenum::U232::USIZE];
}
impl StaticLength for typenum::U233 {}

impl GreaterThan4 for typenum::U233 {}

impl Length for typenum::U233 {
    type Word = [u8; typenum::U233::USIZE];
}
impl StaticLength for typenum::U234 {}

impl GreaterThan4 for typenum::U234 {}

impl Length for typenum::U234 {
    type Word = [u8; typenum::U234::USIZE];
}
impl StaticLength for typenum::U235 {}

impl GreaterThan4 for typenum::U235 {}

impl Length for typenum::U235 {
    type Word = [u8; typenum::U235::USIZE];
}
impl StaticLength for typenum::U236 {}

impl GreaterThan4 for typenum::U236 {}

impl Length for typenum::U236 {
    type Word = [u8; typenum::U236::USIZE];
}
impl StaticLength for typenum::U237 {}

impl GreaterThan4 for typenum::U237 {}

impl Length for typenum::U237 {
    type Word = [u8; typenum::U237::USIZE];
}
impl StaticLength for typenum::U238 {}

impl GreaterThan4 for typenum::U238 {}

impl Length for typenum::U238 {
    type Word = [u8; typenum::U238::USIZE];
}
impl StaticLength for typenum::U239 {}

impl GreaterThan4 for typenum::U239 {}

impl Length for typenum::U239 {
    type Word = [u8; typenum::U239::USIZE];
}
impl StaticLength for typenum::U240 {}

impl GreaterThan4 for typenum::U240 {}

impl Length for typenum::U240 {
    type Word = [u8; typenum::U240::USIZE];
}
impl StaticLength for typenum::U241 {}

impl GreaterThan4 for typenum::U241 {}

impl Length for typenum::U241 {
    type Word = [u8; typenum::U241::USIZE];
}
impl StaticLength for typenum::U242 {}

impl GreaterThan4 for typenum::U242 {}

impl Length for typenum::U242 {
    type Word = [u8; typenum::U242::USIZE];
}
impl StaticLength for typenum::U243 {}

impl GreaterThan4 for typenum::U243 {}

impl Length for typenum::U243 {
    type Word = [u8; typenum::U243::USIZE];
}
impl StaticLength for typenum::U244 {}

impl GreaterThan4 for typenum::U244 {}

impl Length for typenum::U244 {
    type Word = [u8; typenum::U244::USIZE];
}
impl StaticLength for typenum::U245 {}

impl GreaterThan4 for typenum::U245 {}

impl Length for typenum::U245 {
    type Word = [u8; typenum::U245::USIZE];
}
impl StaticLength for typenum::U246 {}

impl GreaterThan4 for typenum::U246 {}

impl Length for typenum::U246 {
    type Word = [u8; typenum::U246::USIZE];
}
impl StaticLength for typenum::U247 {}

impl GreaterThan4 for typenum::U247 {}

impl Length for typenum::U247 {
    type Word = [u8; typenum::U247::USIZE];
}
impl StaticLength for typenum::U248 {}

impl GreaterThan4 for typenum::U248 {}

impl Length for typenum::U248 {
    type Word = [u8; typenum::U248::USIZE];
}
impl StaticLength for typenum::U249 {}

impl GreaterThan4 for typenum::U249 {}

impl Length for typenum::U249 {
    type Word = [u8; typenum::U249::USIZE];
}
impl StaticLength for typenum::U250 {}

impl GreaterThan4 for typenum::U250 {}

impl Length for typenum::U250 {
    type Word = [u8; typenum::U250::USIZE];
}
impl StaticLength for typenum::U251 {}

impl GreaterThan4 for typenum::U251 {}

impl Length for typenum::U251 {
    type Word = [u8; typenum::U251::USIZE];
}
impl StaticLength for typenum::U252 {}

impl GreaterThan4 for typenum::U252 {}

impl Length for typenum::U252 {
    type Word = [u8; typenum::U252::USIZE];
}
impl StaticLength for typenum::U253 {}

impl GreaterThan4 for typenum::U253 {}

impl Length for typenum::U253 {
    type Word = [u8; typenum::U253::USIZE];
}
impl StaticLength for typenum::U254 {}

impl GreaterThan4 for typenum::U254 {}

impl Length for typenum::U254 {
    type Word = [u8; typenum::U254::USIZE];
}
impl StaticLength for typenum::U255 {}

impl GreaterThan4 for typenum::U255 {}

impl Length for typenum::U255 {
    type Word = [u8; typenum::U255::USIZE];
}