use super::FeatureHeader;
use deku::DekuRead;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead)]
#[deku(id = "header.version", ctx = "header: FeatureHeader")]
pub enum CdReadDescriptor {
    #[deku(id = "0b0000")]
    V0,
    #[deku(id = "0b0001")]
    V1 {
        #[deku(pad_bits_before = "6", bits = 1)]
        c2_flags: bool,
        #[deku(bits = 1, pad_bytes_after = "3")]
        cd_text: bool,
    },
    #[deku(id = "0b0010")]
    V2 {
        #[deku(bits = 1, pad_bits_after = "5")]
        dap: bool,
        #[deku(bits = 1)]
        c2_flags: bool,
        #[deku(bits = 1, pad_bytes_after = "3")]
        cd_text: bool,
    },
}
