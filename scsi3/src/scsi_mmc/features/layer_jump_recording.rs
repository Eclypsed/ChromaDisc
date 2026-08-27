use super::FeatureHeader;
use deku::deku_derive;

#[deku_derive(DekuRead)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[deku(id = "header.version", ctx = "header: FeatureHeader")]
pub enum LayerJumpRecordingDescriptor {
    #[deku(id = "0b0000")]
    V0 {
        #[deku(temp, pad_bytes_before = "3")]
        _L: u8,
        #[deku(count = "*_L", pad_bytes_after = "3 - ((*_L + 3) % 4)")]
        link_sizes: Vec<u8>,
    },
}
