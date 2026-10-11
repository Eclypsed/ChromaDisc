use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// CD-RW CAV Write (0027h): the ability to write high speed CD-RW media.
///
/// Laid out as version 0, whose Feature Dependent Data is reserved.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct CdrwCavWrite {
    _bytes4_7: [u8; 4],
}

super::debug_fields!(CdrwCavWrite {});
