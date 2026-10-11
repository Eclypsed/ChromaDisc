use zerocopy::{BigEndian, FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned, U16};

/// Enhanced Defect Reporting (0029h): the ability to control RECOVERED ERROR reporting.
///
/// Laid out as version 0.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct EnhancedDefectReporting {
    byte4: u8,
    number_of_dbi_cache_zones: u8,
    number_of_entries: U16<BigEndian>,
}

impl EnhancedDefectReporting {
    super::bit!(#[doc(alias = "DRT-DM")] drt_dm: byte4[0]);

    #[inline]
    pub const fn number_of_dbi_cache_zones(&self) -> u8 {
        self.number_of_dbi_cache_zones
    }

    #[inline]
    pub const fn number_of_entries(&self) -> u16 {
        self.number_of_entries.get()
    }
}

super::debug_fields!(EnhancedDefectReporting {
    drt_dm,
    number_of_dbi_cache_zones,
    number_of_entries
});
