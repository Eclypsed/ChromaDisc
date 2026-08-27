use super::FeatureHeader;
use deku::DekuRead;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead)]
#[deku(id = "header.version", ctx = "header: FeatureHeader")]
pub enum RandomWritableDescriptor {
    #[deku(id = "0b0000")]
    V0 {
        #[deku(endian = "big")]
        logical_block_address: u32,
    },
    #[deku(id = "0b0001")]
    V1 {
        #[deku(endian = "big")]
        last_logical_block_address: u32,
        #[deku(endian = "big")]
        logical_block_size: u32,
        #[deku(endian = "big")]
        blocking: u16,
        #[deku(pad_bits_before = "7", bits = 1, pad_bytes_after = "1")]
        page_present: bool,
    },
}
