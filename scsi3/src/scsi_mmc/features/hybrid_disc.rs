use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// Hybrid Disc (0080h): the ability to access some Hybrid Discs.
///
/// Laid out as version 0.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct HybridDisc {
    byte4: u8,
    _bytes5_7: [u8; 3],
}

impl HybridDisc {
    super::bit!(#[doc(alias = "RI")] reset_immunity: byte4[0]);
}

super::debug_fields!(HybridDisc { reset_immunity });
