use super::FeatureHeader;
use deku::DekuRead;

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead)]
#[deku(id = "header.version", ctx = "header: FeatureHeader")]
pub enum DvdRRwWriteDescriptor {
    #[deku(id = "0b0000")]
    V0 {
        #[deku(pad_bits_before = "1", bits = 1)]
        buf: bool,
        #[deku(pad_bits_before = "3", bits = 1, pad_bits_after = "26")]
        test_write: bool,
    },
    #[deku(id = "0b0001")]
    V1 {
        #[deku(pad_bits_before = "1", bits = 1)]
        buf: bool,
        #[deku(pad_bits_before = "3", bits = 1)]
        test_write: bool,
        #[deku(bits = 1, pad_bits_after = "25")]
        dvd_rw: bool,
    },
    #[deku(id = "0b0010")]
    V2 {
        #[deku(pad_bits_before = "1", bits = 1)]
        buf: bool,
        #[deku(pad_bits_before = "2", bits = 1)]
        rdl: bool,
        #[deku(bits = 1)]
        test_write: bool,
        #[deku(bits = 1, pad_bits_after = "25")]
        dvd_rw_sl: bool,
    },
}
