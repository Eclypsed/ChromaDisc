use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// Media Serial Number (0109h): the ability to return a unique Media Serial Number.
///
/// Laid out as version 0, which has no Feature Dependent Data.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct MediaSerialNumber;
