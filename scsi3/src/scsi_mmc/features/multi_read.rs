use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// MultiRead (001Dh): the Drive is able to read all CD media types; based on OSTA MultiRead.
///
/// Laid out as version 0, which has no Feature Dependent Data.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct MultiRead;
