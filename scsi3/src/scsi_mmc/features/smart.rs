use super::FeatureHeader;
use deku::DekuRead;

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead)]
#[deku(id = "header.version", ctx = "header: FeatureHeader")]
pub enum SmartDescriptor {
    #[deku(id = "0b0000")]
    V0 {
        #[deku(pad_bits_before = "7", bits = 1, pad_bits_after = "3")]
        page_present: bool,
    },
}
