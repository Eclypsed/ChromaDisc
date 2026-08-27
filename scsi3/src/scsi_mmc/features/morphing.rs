use super::FeatureHeader;
use deku::DekuRead;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead)]
#[deku(id = "header.version", ctx = "header: FeatureHeader")]
pub enum MorphingDescriptor {
    #[deku(id = "0b0000")]
    V0 {
        #[deku(pad_bits_before = "7", bits = 1, pad_bytes_after = "3")]
        asynchronous: bool,
    },
    #[deku(id = "0b0001")]
    V1 {
        #[deku(pad_bits_before = "6", bits = 1)]
        oc_event: bool,
        #[deku(bits = 1, pad_bytes_after = "3")]
        asynchronous: bool,
    },
}
