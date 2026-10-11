use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// Double Density CD Read (0030h): the ability to read user data from DDCD blocks.
///
/// Laid out as version 0, which has no Feature Dependent Data.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct DoubleDensityCdRead;
