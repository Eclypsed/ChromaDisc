use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// Write Protect (0004h): the ability to control Write Protection status.
///
/// Laid out as version 2.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct WriteProtect {
    byte4: u8,
    _bytes5_7: [u8; 3],
}

impl WriteProtect {
    super::bit!(
        /// Since version 2.
        #[doc(alias = "DWP")]
        disc_write_protect_pac: byte4[3]
    );
    super::bit!(
        /// Since version 1.
        #[doc(alias = "WDCB")]
        write_inhibit_dcb: byte4[2]
    );
    super::bit!(#[doc(alias = "SPWP")] set_pwp_status: byte4[1]);
    super::bit!(#[doc(alias = "SSWPP")] supports_swpp: byte4[0]);
}

super::debug_fields!(WriteProtect {
    disc_write_protect_pac,
    write_inhibit_dcb,
    set_pwp_status,
    supports_swpp
});
