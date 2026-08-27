use super::FeatureHeader;
use deku::DekuRead;

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead)]
#[deku(id = "header.version", ctx = "header: FeatureHeader")]
pub enum DvdPlusRWDualLayerDescriptor {
    #[deku(id = "0b0000")]
    V0 {
        #[deku(pad_bits_before = "7", bits = 1)]
        write: bool,
        #[deku(pad_bits_before = "6", bits = 1)]
        quick_start: bool,
        #[deku(bits = 1, pad_bytes_after = "2")]
        close_only: bool,
    },
}
