use super::FeatureHeader;
use deku::deku_derive;

#[deku_derive(DekuRead)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[deku(id = "header.version", ctx = "header: FeatureHeader")]
pub enum IncrementalStreamingWritableDescriptor {
    #[deku(id = "0b0000")]
    V0 {
        #[deku(temp, pad_bytes_before = "3")]
        _L: u8, // number of link sizes
        #[deku(count = "*_L", pad_bytes_after = "3 - ((*_L + 3) % 4)")]
        link_sizes: Vec<u8>,
    },
    #[deku(id = "0b0001")]
    V1 {
        #[deku(endian = "big")]
        data_block_types_supported: u16,
        #[deku(pad_bits_before = "7", bits = 1)]
        buf: bool,
        #[deku(temp)]
        _L: u8, // number of link sizes
        #[deku(count = "*_L", pad_bytes_after = "3 - ((*_L + 3) % 4)")]
        link_sizes: Vec<u8>,
    },
    // TODO: Missing V2, should be somewhere between MMC-5r01e and MMC-5r02b
    // #[deku(id = "0b0010")]
    // V2 {},
    #[deku(id = "0b0011")]
    V3 {
        #[deku(endian = "big")]
        data_block_types_supported: u16,
        #[deku(pad_bits_before = "5", bits = 1)]
        trio: bool,
        #[deku(bits = 1)]
        arsv: bool,
        #[deku(bits = 1)]
        buf: bool,
        #[deku(temp)]
        _L: u8, // number of link sizes
        #[deku(count = "*_L", pad_bytes_after = "3 - ((*_L + 3) % 4)")]
        link_sizes: Vec<u8>,
    },
}
