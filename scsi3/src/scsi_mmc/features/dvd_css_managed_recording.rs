use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// DVD CSS Managed Recording (010Eh): the ability to perform DVD CSS managed recording.
///
/// Laid out as version 0.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct DvdCssManagedRecording {
    maximum_number_of_scramble_extent_information_entries: u8,
    _bytes5_7: [u8; 3],
}

impl DvdCssManagedRecording {
    #[inline]
    pub const fn maximum_number_of_scramble_extent_information_entries(&self) -> u8 {
        self.maximum_number_of_scramble_extent_information_entries
    }
}

super::debug_fields!(DvdCssManagedRecording {
    maximum_number_of_scramble_extent_information_entries
});
