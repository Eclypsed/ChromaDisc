use arbitrary_int::{u2, u3, u4, u7};
use bitfield::{Bits, BitsEnum};
use zerocopy::{Immutable, IntoBytes, KnownLayout, TryFromBytes, Unaligned};

use self::bytes::SpecialInformation1M;
use crate::core::{
    util::{bit, debug_fields},
    validity::{Msb0, Msb1},
};

pub mod cdr;
pub mod cdrw;

/// A 3-bit value representing the different CD-R sub-types
///
/// See CD-R System Description (Orange Book Part II Volume 1) §IV.4.1.5.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitsEnum)]
#[bits(3, repr = u3, reserved = 0b001)]
pub enum CdRSubtype {
    Normal = 0b000,
    TypeALowBeta = 0b010,
    TypeAHighBeta = 0b011,
    TypeBLowBeta = 0b100,
    TypeBHighBeta = 0b101,
    TypeCLowBeta = 0b110,
    TypeCHighBeta = 0b111,
}

/// A 3-bit value representing the different CD-RW sub-types
///
/// See CD-RW System Description (Orange Book Part III Volume 3) §I.2, Table 1.
#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitsEnum)]
#[bits(3, repr = u3, reserved = 0b100..=0b111)]
pub enum CdRwSubtype {
    Standard = 0b000,
    HighSpeed = 0b001,
    UltraSpeed = 0b010,
    UltraSpeedPlus = 0b011,
}

/// A 7-bit value distinguishing between discs used for different applications
///
/// See CD-R System Description (Orange Book Part II Volume 2) §4.4.1.3
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitsEnum)]
#[bits(7, repr = u7, reserved = 0b1000001..=0b1111111)]
pub enum DiscApplicationCode {
    GeneralPurposeDisc = 0b0000000,
    #[bits(alt = 0b0000010..=0b0111111)]
    SpecialPurposeDisc = 0b0000001,
    UnrestrictedUse = 0b1000000,
}

/// A 16-bit value that contains a unique identifying code for the Disc Manufacturer and the type
/// of disc.
///
/// See CD-R System Description (Orange Book Part II Volume 2) §4.4.6.2
#[derive(derive_more::Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct MediaIdentificationCode(#[debug("{_0:04X}h")] u16);

impl MediaIdentificationCode {
    #[inline]
    pub const fn from_u16(code: u16) -> Self {
        Self(code)
    }

    #[inline]
    pub const fn into_u16(self) -> u16 {
        self.0
    }

    /// Reads the code from bits 4-0 of M, 6-0 of S and 6-3 of F, most significant first.
    const fn from_section(m: u8, s: u8, f: u8) -> Self {
        Self(((m as u16 & 0x1F) << 11) | ((s as u16 & 0x7F) << 4) | ((f as u16 >> 3) & 0x0F))
    }
}

// Each section of the ATIP is three bytes, M, S and F, and bit 7 of each byte is one of the
// section's three ID bits. A section's type accepts only bytes with its own ID: 101 for Special
// Information 1, 001 for Additional Information 1, 010 for Additional Information 2 and 011 for
// Additional Information 3. Special Information 1 also fixes the bits that select the medium: the
// DDCD bit here, and Disc Type and Disc Sub-Type in each medium's module.
//
// See `crate::core::validity` for why these types are `pub` in a private module.
mod bytes {
    use crate::core::validity::byte_pattern;

    byte_pattern! {
        /// Byte M of a CD's Special Information 1: ID bit 1, and the DDCD bit, bit 3, clear.
        pub enum SpecialInformation1M = 1 x x x _ 0 x x x
    }
}
