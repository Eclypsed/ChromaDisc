use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// Layer Jump Recording (0033h): the ability to record in layer jump mode.
///
/// Laid out as version 0.
#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct LayerJumpRecording {
    _bytes4_6: [u8; 3],
    number_of_link_sizes: u8,
    link_sizes_and_pad: [u8],
}

impl LayerJumpRecording {
    super::incremental_streaming_writable::link_sizes!();
}

super::debug_fields!(LayerJumpRecording {
    number_of_link_sizes,
    link_sizes
});
