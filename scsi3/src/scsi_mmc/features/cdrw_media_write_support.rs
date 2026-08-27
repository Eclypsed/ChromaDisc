use super::FeatureHeader;
use bitflags::bitflags;
use deku::{DekuError, DekuRead};

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead)]
#[deku(id = "header.version", ctx = "header: FeatureHeader")]
pub enum CdrwMediaWriteSupportDescriptor {
    #[deku(id = "0b0000")]
    V0 {
        #[deku(
            pad_bytes_before = "1",
            map = "|b: u8| -> Result<_, DekuError> { Ok(CdrwMediaSubtypeSupport::from_bits_retain(b)) }",
            pad_bytes_after = "2"
        )]
        cljb: CdrwMediaSubtypeSupport,
    },
}

bitflags! {
    /// A field to identify the CD-RW subtypes a drive supports writing for.
    ///
    /// See MMC-6 §5.3.28 for field definition
    /// See Orange Book Part III Vol.3 §I.2, Table 1 for subtype definitions
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct CdrwMediaSubtypeSupport: u8 {
        /// Disc type 1-4x
        const STANDARD = 1 << 0;
        /// Disc type 4-10x (High Speed)
        const HIGH_SPEED = 1 << 1;
        /// Disc type 8-24x (Ultra Speed 24)
        const ULTRA_SPEED_24 = 1 << 2;
        /// Disc type 8-32x (Ultra Speed 32)
        const ULTRA_SPEED_32 = 1 << 3;
    }
}
