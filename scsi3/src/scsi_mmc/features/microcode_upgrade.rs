use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

/// Microcode Upgrade (0104h): the ability of the device to accept new microcode via the interface.
///
/// Laid out as version 1. Version 0 has no Feature Dependent Data.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct MicrocodeUpgrade {
    byte4: u8,
    _bytes5_7: [u8; 3],
}

impl MicrocodeUpgrade {
    super::bit!(
        /// Since version 1.
        #[doc(alias = "M5")]
        m5: byte4[0]
    );
}

super::debug_fields!(MicrocodeUpgrade { m5 });
