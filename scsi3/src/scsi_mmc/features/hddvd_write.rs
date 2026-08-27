use super::FeatureHeader;
use deku::DekuRead;

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead)]
#[deku(id = "header.version", ctx = "header: FeatureHeader")]
pub enum HddvdWriteDescriptor {
    #[deku(id = "0b0000")]
    V0 {
        #[deku(pad_bits_before = "7", bits = 1, pad_bytes_after = "1")]
        hddvdr: bool,
        #[deku(pad_bits_before = "7", bits = 1, pad_bytes_after = "1")]
        hddvdram: bool,
    },
}
