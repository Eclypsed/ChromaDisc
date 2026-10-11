use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// BD-R Pseudo-Overwrite (0038h): Logical Block overwrite service on BD-R discs formatted as
/// SRM+POW.
///
/// Laid out as version 0, whose Feature Dependent Data is reserved.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct BdrPseudoOverwrite {
    _bytes4_7: [u8; 4],
}

super::debug_fields!(BdrPseudoOverwrite {});
