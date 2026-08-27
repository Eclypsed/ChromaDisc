use super::FeatureHeader;
use deku::DekuRead;

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead)]
#[deku(id = "header.version", ctx = "header: FeatureHeader")]
pub enum DoubleDensityCdrWriteDescriptor {
    #[deku(id = "0b0000")]
    V0 {
        #[deku(pad_bits_before = "5", bits = 1, pad_bits_after = "26")]
        test_wr: bool,
    },
}
