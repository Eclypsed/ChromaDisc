use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// S.M.A.R.T. (0101h): the ability to perform Self Monitoring Analysis and Reporting Technology.
///
/// Laid out as version 0.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct Smart {
    byte4: u8,
    _bytes5_7: [u8; 3],
}

impl Smart {
    super::bit!(#[doc(alias = "PP")] page_present: byte4[0]);
}

super::debug_fields!(Smart { page_present });
