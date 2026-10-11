use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// SecurDisc (0113h): the ability to encode and decode SecurDisc protected information.
///
/// Laid out as version 0, which has no Feature Dependent Data.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct SecurDisc;
