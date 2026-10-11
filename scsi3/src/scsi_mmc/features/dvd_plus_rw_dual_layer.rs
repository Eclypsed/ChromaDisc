use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// DVD+RW Dual Layer (003Ah): the ability to read DVD+RW Dual Layer recorded media formats.
///
/// Laid out as version 0.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct DvdPlusRwDualLayer {
    byte4: u8,
    byte5: u8,
    _bytes6_7: [u8; 2],
}

impl DvdPlusRwDualLayer {
    super::bit!(write: byte4[0]);
    super::bit!(quick_start: byte5[1]);
    super::bit!(close_only: byte5[0]);
}

super::debug_fields!(DvdPlusRwDualLayer {
    write,
    quick_start,
    close_only
});
