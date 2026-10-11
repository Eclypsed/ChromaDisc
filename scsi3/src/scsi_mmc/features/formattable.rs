use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// Formattable (0023h): support for formatting of media.
///
/// Laid out as version 2. Version 0 has no Feature Dependent Data.
// NOTE: Drafts up to MMC-5r01a give version 1 a 4-byte layout without RRM; later drafts extend it
// to this one without changing the version. Data in the earlier layout is shorter than this one, so
// `data()` is `None` for it rather than misread.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct Formattable {
    byte4: u8,
    byte5: u8,
    _bytes6_7: [u8; 2],
    byte8: u8,
    _bytes9_11: [u8; 3],
}

impl Formattable {
    super::bit!(
        /// Since version 1.
        #[doc(alias = "RENoSA")]
        bd_re_no_spares_allocated: byte4[3]
    );
    super::bit!(
        /// Since version 1.
        #[doc(alias = "Expand")]
        spare_area_expansion: byte4[2]
    );
    super::bit!(
        /// Since version 1.
        #[doc(alias = "QCert")]
        quick_certification: byte4[1]
    );
    super::bit!(
        /// Since version 1.
        #[doc(alias = "Cert")]
        full_certification: byte4[0]
    );
    super::bit!(
        /// Since version 2.
        #[doc(alias = "FRF")]
        fast_re_format: byte5[7]
    );
    super::bit!(
        /// Since version 1.
        #[doc(alias = "RRM")]
        random_recording_mode: byte8[0]
    );
}

super::debug_fields!(Formattable {
    bd_re_no_spares_allocated,
    spare_area_expansion,
    quick_certification,
    full_certification,
    fast_re_format,
    random_recording_mode
});
