use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// HD DVD-RW Fragment Recording (0052h): the ability to record HD DVD-RW in fragment recording
/// mode.
///
/// Laid out as version 0.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct HddvdrwFragmentRecording {
    byte4: u8,
    _bytes5_7: [u8; 3],
}

impl HddvdrwFragmentRecording {
    super::bit!(#[doc(alias = "BGP")] background_padding: byte4[0]);
}

super::debug_fields!(HddvdrwFragmentRecording { background_padding });
