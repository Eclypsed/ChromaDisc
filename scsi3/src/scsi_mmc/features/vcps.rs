use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// VCPS (0110h): the ability to decode and optionally encode VCPS protected information.
///
/// Laid out as version 0, whose Feature Dependent Data is reserved.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct Vcps {
    _bytes4_7: [u8; 4],
}

super::debug_fields!(Vcps {});
