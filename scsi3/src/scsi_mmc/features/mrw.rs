use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// MRW (0028h): the ability to recognize and read and optionally write MRW formatted media.
///
/// Laid out as version 1.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct Mrw {
    byte4: u8,
    _bytes5_7: [u8; 3],
}

impl Mrw {
    super::bit!(
        /// Since version 1.
        #[doc(alias = "DVD+Write")]
        dvd_plus_write: byte4[2]
    );
    super::bit!(
        /// Since version 1.
        #[doc(alias = "DVD+Read")]
        dvd_plus_read: byte4[1]
    );
    super::bit!(#[doc(alias = "Write")] cd_write: byte4[0]);
}

super::debug_fields!(Mrw {
    dvd_plus_write,
    dvd_plus_read,
    cd_write
});
