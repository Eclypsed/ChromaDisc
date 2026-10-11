use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// DVD+R Dual Layer (003Bh): the ability to read DVD+R Dual Layer recorded media formats.
///
/// Laid out as version 0.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct DvdPlusRDualLayer {
    byte4: u8,
    _bytes5_7: [u8; 3],
}

impl DvdPlusRDualLayer {
    super::bit!(write: byte4[0]);
}

super::debug_fields!(DvdPlusRDualLayer { write });
