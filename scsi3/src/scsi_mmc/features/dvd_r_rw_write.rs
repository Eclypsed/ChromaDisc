use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// DVD-R/-RW Write (002Fh): the ability to write DVD specific structures.
///
/// Laid out as version 2.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct DvdRRwWrite {
    byte4: u8,
    _bytes5_7: [u8; 3],
}

impl DvdRRwWrite {
    super::bit!(#[doc(alias = "BUF")] buf: byte4[6]);
    super::bit!(
        /// Since version 2.
        #[doc(alias = "RDL")]
        rdl: byte4[3]
    );
    super::bit!(#[doc(alias = "Test Write")] test_write: byte4[2]);
    super::bit!(
        /// Since version 1, which names it DVD-RW.
        #[doc(alias = "DVD-RW SL", alias = "DVD-RW")]
        dvd_rw_sl: byte4[1]
    );
}

super::debug_fields!(DvdRRwWrite {
    buf,
    rdl,
    test_write,
    dvd_rw_sl
});
