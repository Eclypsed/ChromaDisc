use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// Morphing (0002h): the Drive is able to report operational changes to the Host and accept Host
/// requests to prevent operational changes.
///
/// Laid out as version 1.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct Morphing {
    byte4: u8,
    _bytes5_7: [u8; 3],
}

impl Morphing {
    super::bit!(
        /// Since version 1.
        oc_event: byte4[1]
    );
    super::bit!(r#async: byte4[0]);
}

super::debug_fields!(Morphing { oc_event, r#async });
