use super::FeatureHeader;
use deku::DekuRead;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead)]
#[deku(id = "header.version", ctx = "header: FeatureHeader")]
pub enum DvdReadDescriptor {
    #[deku(id = "0b0000")]
    V0,
    #[deku(id = "0b0001")]
    V1 {
        #[deku(pad_bits_before = "7", bits = 1, pad_bytes_after = "1")]
        multi110: bool,
        #[deku(pad_bits_before = "7", bits = 1, pad_bytes_after = "1")]
        dual_r: bool,
    },
    #[deku(id = "0b0010")]
    V2 {
        #[deku(pad_bits_before = "7", bits = 1, pad_bytes_after = "1")]
        multi110: bool,
        #[deku(pad_bits_before = "6", bits = 1)]
        dual_rw: bool,
        #[deku(bits = 1, pad_bytes_after = "1")]
        dual_r: bool,
    },
}
