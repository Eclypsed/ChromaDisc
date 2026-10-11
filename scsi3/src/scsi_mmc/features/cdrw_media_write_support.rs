use core::fmt;

use bitflags::bitflags;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// CD-RW Media Write Support (0037h): the ability to report CD-RW media sub-types that are
/// supported for write.
///
/// Laid out as version 0.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct CdrwMediaWriteSupport {
    _byte4: u8,
    cd_rw_media_subtype_support: CdrwMediaSubtypeSupport,
    _bytes6_7: [u8; 2],
}

impl CdrwMediaWriteSupport {
    #[inline]
    pub const fn cd_rw_media_subtype_support(&self) -> CdrwMediaSubtypeSupport {
        self.cd_rw_media_subtype_support
    }
}

super::debug_fields!(CdrwMediaWriteSupport {
    cd_rw_media_subtype_support
});

/// A field to identify the CD-RW subtypes a drive supports writing for.
///
/// See MMC-6 §5.3.28 for field definition
/// See Orange Book Part III Vol.3 §I.2, Table 1 for subtype definitions
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(transparent)]
pub struct CdrwMediaSubtypeSupport(u8);

/// Shows the set flags, as the `Debug` of a type that `bitflags!` declares does.
impl fmt::Debug for CdrwMediaSubtypeSupport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("CdrwMediaSubtypeSupport(")?;
        if self.is_empty() {
            f.write_str("0x0")?;
        } else {
            bitflags::parser::to_writer(self, &mut *f)?;
        }
        f.write_str(")")
    }
}

bitflags! {
    impl CdrwMediaSubtypeSupport: u8 {
        /// Disc type 1-4x
        const STANDARD = 1 << 0;
        /// Disc type 4-10x (High Speed)
        const HIGH_SPEED = 1 << 1;
        /// Disc type 8-24x (Ultra Speed 24)
        const ULTRA_SPEED_24 = 1 << 2;
        /// Disc type 8-32x (Ultra Speed 32)
        const ULTRA_SPEED_32 = 1 << 3;
    }
}
