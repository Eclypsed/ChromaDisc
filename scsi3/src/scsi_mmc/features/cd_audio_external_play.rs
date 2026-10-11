use zerocopy::{BigEndian, FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned, U16};

/// CD Audio External Play (0103h): the ability to play audio CDs via the Logical Unit's own analog
/// output.
///
/// Laid out as version 0.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct CdAudioExternalPlay {
    byte4: u8,
    _byte5: u8,
    number_of_volume_levels: U16<BigEndian>,
}

impl CdAudioExternalPlay {
    super::bit!(scan: byte4[2]);
    super::bit!(#[doc(alias = "SCM")] separate_channel_mute: byte4[1]);
    super::bit!(#[doc(alias = "SV")] separate_volume: byte4[0]);

    #[inline]
    pub const fn number_of_volume_levels(&self) -> u16 {
        self.number_of_volume_levels.get()
    }
}

super::debug_fields!(CdAudioExternalPlay {
    scan,
    separate_channel_mute,
    separate_volume,
    number_of_volume_levels
});
