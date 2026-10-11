use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// Double Density CD-R Write (0031h): the ability to write and read DDCD-R media.
///
/// Laid out as version 0.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct DoubleDensityCdrWrite {
    byte4: u8,
    _bytes5_7: [u8; 3],
}

impl DoubleDensityCdrWrite {
    super::bit!(#[doc(alias = "Test Write")] test_write: byte4[2]);
}

super::debug_fields!(DoubleDensityCdrWrite { test_write });
