use super::FeatureHeader;
use deku::DekuRead;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead)]
#[deku(id = "header.version", ctx = "header: FeatureHeader")]
pub enum WriteProtectDescriptor {
    #[deku(id = "0b0000")]
    V0 {
        #[deku(pad_bits_before = "6", bits = 1)]
        spwp: bool,
        #[deku(bits = 1, pad_bytes_after = "3")]
        sswpp: bool,
    },
    #[deku(id = "0b0001")]
    V1 {
        #[deku(pad_bits_before = "5", bits = 1)]
        wdcb: bool,
        #[deku(bits = 1)]
        spwp: bool,
        #[deku(bits = 1, pad_bytes_after = "3")]
        sswpp: bool,
    },
    #[deku(id = "0b0010")]
    V2 {
        #[deku(pad_bits_before = "4", bits = 1)]
        dwp: bool,
        #[deku(bits = 1)]
        wdcb: bool,
        #[deku(bits = 1)]
        spwp: bool,
        #[deku(bits = 1, pad_bytes_after = "3")]
        sswpp: bool,
    },
}
