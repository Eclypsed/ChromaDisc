use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// Embedded Changer (0102h): a single mechanism multiple disc changer.
///
/// Laid out as version 0.
// NOTE: The Embedded Changer Feature Descriptor Format diagram is incorrect in MMC-6 and was
// amended by MMC-6 Amendment 1.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct EmbeddedChanger {
    byte4: u8,
    _bytes5_6: [u8; 2],
    highest_slot_number: u8,
}

impl EmbeddedChanger {
    super::bit!(#[doc(alias = "SCC")] side_change_capable: byte4[4]);
    super::bit!(#[doc(alias = "SDP")] supports_disc_present: byte4[2]);

    #[inline]
    pub const fn highest_slot_number(&self) -> u8 {
        self.highest_slot_number
    }
}

super::debug_fields!(EmbeddedChanger {
    side_change_capable,
    supports_disc_present,
    highest_slot_number
});
