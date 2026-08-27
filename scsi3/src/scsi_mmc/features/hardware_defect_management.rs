use super::FeatureHeader;
use deku::DekuRead;

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead)]
#[deku(id = "header.version", ctx = "header: FeatureHeader")]
pub enum HardwareDefectManagementDescriptor {
    #[deku(id = "0b0000")]
    V0,
    #[deku(id = "0b0001")]
    V1 {
        #[deku(bits = 1, pad_bits_after = "31")]
        ssa: bool,
    },
}
