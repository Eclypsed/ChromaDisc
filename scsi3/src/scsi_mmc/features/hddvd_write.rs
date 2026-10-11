use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// HD DVD Write (0051h): the ability to write control structures and user data to certain HD DVD
/// discs.
///
/// Laid out as version 0.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct HddvdWrite {
    byte4: u8,
    _byte5: u8,
    byte6: u8,
    _byte7: u8,
}

impl HddvdWrite {
    super::bit!(#[doc(alias = "HD DVD-R")] hd_dvd_r: byte4[0]);
    super::bit!(#[doc(alias = "HD DVD-RAM")] hd_dvd_ram: byte6[0]);
}

super::debug_fields!(HddvdWrite {
    hd_dvd_r,
    hd_dvd_ram
});
