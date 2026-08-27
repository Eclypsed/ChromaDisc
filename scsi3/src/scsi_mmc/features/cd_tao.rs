use super::FeatureHeader;
use deku::DekuRead;

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead)]
#[deku(id = "header.version", ctx = "header: FeatureHeader")]
pub enum CdTaoDescriptor {
    #[deku(id = "0b0000")]
    V0 {
        #[deku(pad_bits_before = "5", bits = 1)]
        test_write: bool,
        #[deku(bits = 1)]
        cdrw: bool,
        #[deku(bits = 1, pad_bytes_after = "1")]
        r_w_subcode: bool,
        #[deku(endian = "big")]
        data_type_supported: u16,
    },
    #[deku(id = "0b0001")]
    V1 {
        #[deku(pad_bits_before = "3", bits = 1)]
        r_w_raw: bool,
        #[deku(bits = 1)]
        r_w_pack: bool,
        #[deku(bits = 1)]
        test_write: bool,
        #[deku(bits = 1)]
        cdrw: bool,
        #[deku(bits = 1, pad_bytes_after = "1")]
        r_w_subcode: bool,
        #[deku(endian = "big")]
        data_type_supported: u16,
    },
    #[deku(id = "0b0010")]
    V2 {
        #[deku(pad_bits_before = "1", bits = 1)]
        buf: bool,
        #[deku(pad_bits_before = "1", bits = 1)]
        r_w_raw: bool,
        #[deku(bits = 1)]
        r_w_pack: bool,
        #[deku(bits = 1)]
        test_write: bool,
        #[deku(bits = 1)]
        cdrw: bool,
        #[deku(bits = 1, pad_bytes_after = "1")]
        r_w_subcode: bool,
        #[deku(endian = "big")]
        data_type_supported: u16,
    },
}
