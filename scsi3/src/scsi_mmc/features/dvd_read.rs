use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// DVD Read (001Fh): the ability to read DVD specific structures.
///
/// Laid out as version 2. Version 0 has no Feature Dependent Data.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct DvdRead {
    byte4: u8,
    _byte5: u8,
    byte6: u8,
    _byte7: u8,
}

impl DvdRead {
    super::bit!(
        /// Since version 1.
        multi_110: byte4[0]
    );
    super::bit!(
        /// Since version 2.
        dual_rw: byte6[1]
    );
    super::bit!(
        /// Since version 1.
        dual_r: byte6[0]
    );
}

super::debug_fields!(DvdRead {
    multi_110,
    dual_rw,
    dual_r
});
