use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// Restricted Overwrite (0026h): write support for media that shall be written from Blocking
/// boundaries only.
///
/// Laid out as version 0, which has no Feature Dependent Data.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct RestrictedOverwrite;
