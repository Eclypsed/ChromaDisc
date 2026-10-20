use arbitrary_int::u3;
use bitfield::{Bits, BitsEnum};
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

use crate::core::util::{bit, debug_fields};

/// A 3-bit value representing the recommended recording speed of a DDCD-R or DDCD-RW.
///
/// Taken from mmc4r05a's description of the READ TOC/PMA/ATIP ATIP format.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, BitsEnum)]
#[bits(3, repr = u3, reserved = 0b000..=0b001 | 0b100..=0b111)]
pub enum ReferenceSpeed {
    X4 = 0b010,
    X8 = 0b011,
}

/// The first three bytes of a DDCD's ATIP, as mmc4r05a describes them: the counterpart of a CD's
/// Special Information 1.
// TODO: Read the Indicative Target Writing Power, which mmc4r05a says "is contained within the
// entire field" for DDCD media, without saying which bits that covers.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct SpecialInformation1 {
    m: u8,
    s: u8,
    f: u8,
}

impl SpecialInformation1 {
    #[inline]
    pub const fn reference_speed(&self) -> Bits<ReferenceSpeed> {
        Bits::new(u3::extract_u8(self.m, 0))
    }

    bit!(
        #[doc(alias = "URU")]
        unrestricted_use_disc: s[6]
    );
    bit!(a1_valid: f[2]);
    bit!(a2_valid: f[1]);
    bit!(a3_valid: f[0]);
}

debug_fields!(SpecialInformation1 {
    reference_speed,
    unrestricted_use_disc,
    a1_valid,
    a2_valid,
    a3_valid,
});
