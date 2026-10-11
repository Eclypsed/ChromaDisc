use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// Double Density CD-RW Write (0032h): the ability to write and read DDCD-RW media.
///
/// Laid out as version 0.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct DoubleDensityCdrwWrite {
    byte4: u8,
    _bytes5_7: [u8; 3],
}

impl DoubleDensityCdrwWrite {
    super::bit!(intermediate: byte4[1]);
    super::bit!(blank: byte4[0]);
}

super::debug_fields!(DoubleDensityCdrwWrite {
    intermediate,
    blank
});
