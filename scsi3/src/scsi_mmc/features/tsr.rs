use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// Timely, Safe Recording (0042h): permits the Host to schedule defect management.
///
/// Laid out as version 0, which has no Feature Dependent Data.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct Tsr;
