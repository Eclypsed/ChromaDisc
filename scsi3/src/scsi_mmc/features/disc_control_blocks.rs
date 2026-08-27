use super::FeatureHeader;
use deku::DekuRead;

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead)]
#[deku(id = "header.version", ctx = "header: FeatureHeader")]
pub enum DiscControlBlocksDescriptor {
    #[deku(id = "0b0000")]
    V0 {
        #[deku(bytes_read = "header.additional_length", endian = "big")]
        supported_dcb_entries: Vec<u32>,
    },
}
