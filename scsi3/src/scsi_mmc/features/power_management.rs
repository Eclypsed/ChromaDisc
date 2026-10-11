use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// Power Management (0100h): Host and device directed power management.
///
/// Laid out as version 0, which has no Feature Dependent Data.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct PowerManagement;
