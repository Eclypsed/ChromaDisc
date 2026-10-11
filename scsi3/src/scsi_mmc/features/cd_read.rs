use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// CD Read (001Eh): the ability to read CD specific structures.
///
/// Laid out as version 2. Version 0 has no Feature Dependent Data.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct CdRead {
    byte4: u8,
    _bytes5_7: [u8; 3],
}

impl CdRead {
    super::bit!(
        /// Since version 2.
        #[doc(alias = "DAP")]
        digital_audio_play: byte4[7]
    );
    super::bit!(
        /// Since version 1.
        c2_flags: byte4[1]
    );
    super::bit!(
        /// Since version 1.
        cd_text: byte4[0]
    );
}

super::debug_fields!(CdRead {
    digital_audio_play,
    c2_flags,
    cd_text
});
