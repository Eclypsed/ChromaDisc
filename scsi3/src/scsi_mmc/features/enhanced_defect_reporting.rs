use super::FeatureHeader;
use deku::DekuRead;

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead)]
#[deku(id = "header.version", ctx = "header: FeatureHeader")]
pub enum EnhancedDefectReportingDescriptor {
    #[deku(id = "0b0000")]
    V0 {
        #[deku(pad_bits_before = "7", bits = 1)]
        drt_dm: bool,
        number_dbi_cache_zones: u8,
        #[deku(endian = "big")]
        number_entries: u16,
    },
}
