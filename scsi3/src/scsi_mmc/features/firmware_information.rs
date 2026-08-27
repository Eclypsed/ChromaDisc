use super::FeatureHeader;
use deku::DekuRead;

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead)]
#[deku(id = "header.version", ctx = "header: FeatureHeader")]
pub enum FirmwareInformationDescriptor {
    #[deku(id = "0b0000")]
    V0 {
        #[deku(endian = "big")]
        century: u16,
        #[deku(endian = "big")]
        year: u16,
        #[deku(endian = "big")]
        month: u16,
        #[deku(endian = "big")]
        day: u16,
        #[deku(endian = "big")]
        hour: u16,
        #[deku(endian = "big")]
        minute: u16,
        #[deku(endian = "big", pad_bytes_after = "2")]
        second: u16,
    },
}
