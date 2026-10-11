use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// Sector Erasable (0022h): write support for erasable media and media that requires an erase
/// pass before overwrite.
///
/// Laid out as version 0, which has no Feature Dependent Data.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct SectorErasable;
