use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// DVD+R (002Bh): the ability to read DVD+R recorded media formats.
///
/// Laid out as version 0.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct DvdPlusR {
    byte4: u8,
    _bytes5_7: [u8; 3],
}

impl DvdPlusR {
    super::bit!(write: byte4[0]);
}

super::debug_fields!(DvdPlusR { write });
