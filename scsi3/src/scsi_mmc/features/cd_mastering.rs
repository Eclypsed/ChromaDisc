use arbitrary_int::u24;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// CD Mastering (002Eh): the ability to write CD with Session at Once or Raw write methods.
///
/// Laid out as version 1.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct CdMastering {
    byte4: u8,
    maximum_cue_sheet_length: [u8; 3],
}

impl CdMastering {
    super::bit!(
        /// Since version 1.
        #[doc(alias = "BUF")]
        buf: byte4[6]
    );
    super::bit!(#[doc(alias = "SAO")] session_at_once: byte4[5]);
    super::bit!(#[doc(alias = "Raw MS")] raw_multi_session: byte4[4]);
    super::bit!(raw: byte4[3]);
    super::bit!(#[doc(alias = "Test Write")] test_write: byte4[2]);
    super::bit!(#[doc(alias = "CD-RW")] cd_rw: byte4[1]);
    super::bit!(#[doc(alias = "R-W")] rw: byte4[0]);

    #[inline]
    pub const fn maximum_cue_sheet_length(&self) -> u24 {
        u24::from_be_bytes(self.maximum_cue_sheet_length)
    }
}

super::debug_fields!(CdMastering {
    buf,
    session_at_once,
    raw_multi_session,
    raw,
    test_write,
    cd_rw,
    rw,
    maximum_cue_sheet_length
});
