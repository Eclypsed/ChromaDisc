use super::FeatureHeader;
use deku::DekuRead;

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead)]
#[deku(id = "header.version", ctx = "header: FeatureHeader")]
pub enum MrwDescriptor {
    #[deku(id = "0b0000")]
    V0 {
        #[deku(pad_bits_before = "7", bits = 1, pad_bytes_after = "3")]
        write: bool,
    },
    #[deku(id = "0b0001")]
    V1 {
        #[deku(pad_bits_before = "5", bits = 1)]
        dvd_plus_write: bool,
        #[deku(bits = 1)]
        dvd_plus_read: bool,
        #[deku(bits = 1, pad_bytes_after = "3")]
        cd_write: bool,
    },
}
