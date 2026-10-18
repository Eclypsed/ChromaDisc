use zerocopy::{BigEndian, FromBytes, Immutable, IntoBytes, KnownLayout, U16, Unaligned};

/// CD Track at Once (002Dh): the ability to write CD with Track at Once recording.
///
/// Laid out as version 2.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct CdTao {
    byte4: u8,
    _byte5: u8,
    data_type_supported: U16<BigEndian>,
}

impl CdTao {
    super::bit!(
        /// Since version 2.
        #[doc(alias = "BUF")]
        buf: byte4[6]
    );
    super::bit!(
        /// Since version 1.
        #[doc(alias = "R-W Raw")]
        rw_raw: byte4[4]
    );
    super::bit!(
        /// Since version 1.
        #[doc(alias = "R-W Pack")]
        rw_pack: byte4[3]
    );
    super::bit!(#[doc(alias = "Test Write")] test_write: byte4[2]);
    super::bit!(#[doc(alias = "CD-RW")] cd_rw: byte4[1]);
    super::bit!(#[doc(alias = "R-W Sub-code")] rw_subcode: byte4[0]);

    #[inline]
    pub const fn data_type_supported(&self) -> u16 {
        self.data_type_supported.get()
    }
}

super::debug_fields!(CdTao {
    buf,
    rw_raw,
    rw_pack,
    test_write,
    cd_rw,
    rw_subcode,
    data_type_supported
});
