use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// Stop Long Operation (0035h): the ability to stop the long immediate operation by a command.
///
/// Laid out as version 0, which has no Feature Dependent Data.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Debug, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct StopLongOperation;
