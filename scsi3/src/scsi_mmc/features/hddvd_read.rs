use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// HD DVD Read (0050h): the ability to read control structures and user data from a HD DVD disc.
///
/// Laid out as version 0.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct HddvdRead {
    byte4: u8,
    _byte5: u8,
    byte6: u8,
    _byte7: u8,
}

impl HddvdRead {
    super::bit!(#[doc(alias = "HD DVD-R")] hd_dvd_r: byte4[0]);
    super::bit!(#[doc(alias = "HD DVD-RAM")] hd_dvd_ram: byte6[0]);
}

super::debug_fields!(HddvdRead {
    hd_dvd_r,
    hd_dvd_ram
});
