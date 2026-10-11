use arbitrary_int::u4;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// AACS (010Dh): the ability to decode and optionally encode AACS protected information.
///
/// Laid out as version 2.
// NOTE: Currently don't have access to a revision with a version 1 byte diagram (somewhere between
// mmc6r02 and mmc6r02g). Going off of: https://www.t10.org/ftp/t10/document.08/08-375r1.pdf
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct Aacs {
    byte4: u8,
    block_count_for_binding_nonce: u8,
    byte6: u8,
    aacs_version: u8,
}

impl Aacs {
    super::bit!(
        /// Since version 2.
        #[doc(alias = "RDC")]
        read_drive_certificate: byte4[4]
    );
    super::bit!(
        /// Since version 2.
        #[doc(alias = "RMC")]
        read_media_key_block_cprm: byte4[3]
    );
    super::bit!(
        /// Since version 1.
        #[doc(alias = "WBE")]
        write_bus_encryption: byte4[2]
    );
    super::bit!(
        /// Since version 1.
        #[doc(alias = "BEC")]
        bus_encryption_capable: byte4[1]
    );
    super::bit!(#[doc(alias = "BNG")] binding_nonce_generation: byte4[0]);

    #[inline]
    pub const fn block_count_for_binding_nonce(&self) -> u8 {
        self.block_count_for_binding_nonce
    }

    #[inline]
    pub const fn number_of_agids(&self) -> u4 {
        u4::extract_u8(self.byte6, 0)
    }

    #[inline]
    pub const fn aacs_version(&self) -> u8 {
        self.aacs_version
    }
}

super::debug_fields!(Aacs {
    read_drive_certificate,
    read_media_key_block_cprm,
    write_bus_encryption,
    bus_encryption_capable,
    binding_nonce_generation,
    block_count_for_binding_nonce,
    number_of_agids,
    aacs_version
});
