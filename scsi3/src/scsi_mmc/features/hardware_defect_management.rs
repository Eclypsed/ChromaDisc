use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// Hardware Defect Management (0024h): the ability of the Drive/media system to provide an
/// apparently defect-free space.
///
/// Laid out as version 1. Version 0 has no Feature Dependent Data.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct HardwareDefectManagement {
    byte4: u8,
    _bytes5_7: [u8; 3],
}

impl HardwareDefectManagement {
    super::bit!(
        /// Since version 1.
        #[doc(alias = "SSA")]
        supports_spare_area_info: byte4[7]
    );
}

super::debug_fields!(HardwareDefectManagement {
    supports_spare_area_info
});
