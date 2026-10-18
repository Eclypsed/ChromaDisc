use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

use crate::core::util::debug_fields;

/// A 4-byte address in hours, minutes, seconds and frames, as MMC responses report it.
///
/// MMC-4 calls this the TIME field, and the bit that selects it TIME. Hour is zero on a CD, and
/// up to 2 on a DDCD. Revisions after MMC-4 drop DDCD, call the bit MSF, and reserve the Hour
/// byte, leaving an MSF address. Unlike [`Msf`], which holds only minutes, seconds and frames,
/// this keeps all 4 bytes.
///
/// The fields are returned without checking their ranges.
///
/// [`Msf`]: crate::mmc::device_models::cd::addressing::Msf
// NOTE: mmc4r05a is the last revision that defines TIME, in its DDCD model. It applies to every
// disc type in the Formatted TOC format of READ TOC/PMA/ATIP, and READ SUB-CHANNEL uses it too.
#[doc(alias = "TIME")]
#[doc(alias = "MSF")]
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct Hmsf {
    hour: u8,
    minute: u8,
    second: u8,
    frame: u8,
}

impl Hmsf {
    /// Reserved after MMC-4, so zero from a Drive that follows a later revision.
    #[inline]
    pub const fn hour(&self) -> u8 {
        self.hour
    }

    #[inline]
    pub const fn minute(&self) -> u8 {
        self.minute
    }

    #[inline]
    pub const fn second(&self) -> u8 {
        self.second
    }

    #[inline]
    pub const fn frame(&self) -> u8 {
        self.frame
    }
}

debug_fields!(Hmsf {
    hour,
    minute,
    second,
    frame
});
