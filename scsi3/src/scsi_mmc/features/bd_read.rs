use super::FeatureHeader;
use deku::DekuRead;

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead)]
#[deku(id = "header.version", ctx = "header: FeatureHeader")]
pub enum BdReadDescriptor {
    #[deku(id = "0b0000")]
    V0 {
        #[deku(pad_bits_before = "45", bits = 1)]
        re2: bool,
        #[deku(bits = 1, pad_bits_after = "49")]
        re1: bool,
        #[deku(pad_bits_before = "14", bits = 1, pad_bits_after = "49")]
        r: bool,
        #[deku(pad_bits_before = "14", bits = 1, pad_bits_after = "49")]
        rom: bool,
    },
    #[deku(id = "0b0001")]
    V1 {
        #[deku(pad_bits_before = "7", bits = 1, pad_bytes_after = "3")]
        bca: bool,
        #[deku(pad_bits_before = "13", bits = 1)]
        re2: bool,
        #[deku(bits = 1, pad_bits_after = "49")]
        re1: bool,
        #[deku(pad_bits_before = "14", bits = 1, pad_bits_after = "49")]
        r: bool,
        #[deku(pad_bits_before = "14", bits = 1, pad_bits_after = "49")]
        rom: bool,
    },
}
