use super::FeatureHeader;
use deku::DekuRead;

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead)]
#[deku(id = "header.version", ctx = "header: FeatureHeader")]
pub enum MicrocodeUpgradeDescriptor {
    #[deku(id = "0b0000")]
    V0,
    #[deku(id = "0b0001")]
    V1 {
        #[deku(pad_bits_before = "7", bits = 1, pad_bytes_after = "3")]
        m5: bool,
    },
}
