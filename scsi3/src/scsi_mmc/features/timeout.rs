use super::FeatureHeader;
use deku::DekuRead;

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead)]
#[deku(id = "header.version", ctx = "header: FeatureHeader")]
pub enum TimeoutDescriptor {
    #[deku(id = "0b0000")]
    V0,
    #[deku(id = "0b0001")]
    V1 {
        #[deku(pad_bits_before = "7", bits = 1)]
        group3: bool,
        #[deku(pad_bytes_before = "1", endian = "big")]
        unit_length: u16,
    },
}
