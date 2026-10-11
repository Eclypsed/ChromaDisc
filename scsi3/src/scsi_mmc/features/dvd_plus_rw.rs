use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// DVD+RW (002Ah): the ability to recognize, read and optionally write DVD+RW media.
///
/// Laid out as version 1.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct DvdPlusRw {
    byte4: u8,
    byte5: u8,
    _bytes6_7: [u8; 2],
}

impl DvdPlusRw {
    super::bit!(write: byte4[0]);
    super::bit!(
        /// Since version 1.
        quick_start: byte5[1]
    );
    super::bit!(close_only: byte5[0]);
}

super::debug_fields!(DvdPlusRw {
    write,
    quick_start,
    close_only
});
