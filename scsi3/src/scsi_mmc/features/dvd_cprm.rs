use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// DVD CPRM (010Bh): the Drive supports DVD CPRM authentication.
///
/// Laid out as version 0.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct DvdCprm {
    _bytes4_6: [u8; 3],
    cprm_version: u8,
}

impl DvdCprm {
    #[inline]
    pub const fn cprm_version(&self) -> u8 {
        self.cprm_version
    }
}

super::debug_fields!(DvdCprm { cprm_version });
