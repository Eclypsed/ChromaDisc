use super::FeatureHeader;
use deku::DekuRead;

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead)]
#[deku(id = "header.version", ctx = "header: FeatureHeader")]
pub enum BdWriteDescriptor {
    #[deku(id = "0b0000")]
    V0 {
        #[deku(pad_bits_before = "7", bits = 1, pad_bytes_after = "3")]
        svnr: bool,
        #[deku(pad_bits_before = "13", bits = 1, pad_bits_after = "50")]
        re2: bool,
        #[deku(pad_bits_before = "14", bits = 1, pad_bits_after = "49")]
        r: bool,
    },
}
