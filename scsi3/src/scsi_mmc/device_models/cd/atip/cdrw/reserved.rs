//! A CD-RW whose Disc Sub-Type is reserved, from 100b to 111b. The Orange Book doesn't lay out its
//! sections, so only the fields MMC defines are read.

use self::bytes::SpecialInformation1F;
use super::*;

// See `crate::core::validity` for why this is `pub` in a private module.
mod bytes {
    use crate::core::validity::byte_pattern;

    byte_pattern! {
        /// Byte F of the Special Information 1 of a CD-RW whose Disc Sub-Type is reserved: ID bit 1,
        /// Disc Type 1 and Disc Sub-Type 1xxb.
        pub enum SpecialInformation1F = 1 1 1 x _ x x x x
    }
}

/// Special Information 1 of a CD-RW whose Disc Sub-Type is reserved, read as MMC defines it.
#[derive(
    TryFromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct SpecialInformation1 {
    m: SpecialInformation1M,
    s: Msb0,
    f: SpecialInformation1F,
}

impl SpecialInformation1 {
    /// The medium's recommended initial laser power setting, as a code whose meaning depends on
    /// the medium.
    #[inline]
    pub const fn indicative_target_writing_power(&self) -> u3 {
        u3::extract_u8(self.m as u8, 4)
    }

    /// The medium's recommended write speed, as a code whose meaning depends on the medium.
    #[inline]
    pub const fn reference_speed(&self) -> u3 {
        u3::extract_u8(self.m as u8, 0)
    }

    bit!(
        #[doc(alias = "URU")]
        unrestricted_use_disc: s[6]
    );

    /// The Disc Sub-Type, from 100b to 111b.
    #[inline]
    pub const fn disc_sub_type(&self) -> u3 {
        u3::extract_u8(self.f as u8, 3)
    }

    bit!(a1_valid: f[2]);
    bit!(a2_valid: f[1]);
    bit!(a3_valid: f[0]);
}

debug_fields!(SpecialInformation1 {
    indicative_target_writing_power,
    reference_speed,
    unrestricted_use_disc,
    disc_sub_type,
    a1_valid,
    a2_valid,
    a3_valid,
});
