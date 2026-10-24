use arbitrary_int::u3;
use bitfield::{Bits, BitsEnum};
use zerocopy::{Immutable, IntoBytes, KnownLayout, TryFromBytes, Unaligned};

use self::bytes::SpecialInformation1M;
use crate::core::{
    util::{bit, debug_fields},
    validity::{Msb0, Msb1},
};

/// A 3-bit value representing the recommended recording speed of a DDCD-R or DDCD-RW.
///
/// Taken from mmc4r05a's description of the READ TOC/PMA/ATIP ATIP format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitsEnum)]
#[bits(3, repr = u3, reserved = 0b000..=0b001 | 0b100..=0b111)]
pub enum ReferenceSpeed {
    X4 = 0b010,
    X8 = 0b011,
}

// See `crate::core::validity` for why this is `pub` in a private module.
mod bytes {
    use crate::core::validity::byte_pattern;

    byte_pattern! {
        /// Byte M of a DDCD's Special Information 1: ID bit 1, and the DDCD bit, bit 3, set.
        pub enum SpecialInformation1M = 1 x x x _ 1 x x x
    }
}

/// The first three bytes of a DDCD's ATIP, as mmc4r05a describes them: the counterpart of a CD's
/// Special Information 1.
///
/// It accepts only the ID bits of a CD's Special Information 1, 101, with the DDCD bit set.
// NOTE: The ID bits are assumed to be a CD's. Whether mmc4r05a gives a DDCD's Special Information
// 1 the same ID bits hasn't been checked.
//
// TODO: Read the Indicative Target Writing Power, which mmc4r05a says "is contained within the
// entire field" for DDCD media, without saying which bits that covers.
#[derive(
    TryFromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct SpecialInformation1 {
    m: SpecialInformation1M,
    s: Msb0,
    f: Msb1,
}

impl SpecialInformation1 {
    #[inline]
    pub const fn reference_speed(&self) -> Bits<ReferenceSpeed> {
        Bits::new(u3::extract_u8(self.m as u8, 0))
    }

    bit!(
        #[doc(alias = "URU")]
        unrestricted_use_disc: s[6]
    );
    bit!(
        /// Clear for a DDCD-R, and set for a DDCD-RW.
        disc_type: f[6]
    );
    bit!(a1_valid: f[2]);
    bit!(a2_valid: f[1]);
    bit!(a3_valid: f[0]);
}

debug_fields!(SpecialInformation1 {
    reference_speed,
    unrestricted_use_disc,
    disc_type,
    a1_valid,
    a2_valid,
    a3_valid,
});
