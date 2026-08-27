use super::FeatureHeader;
use deku::DekuRead;

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead)]
#[deku(id = "header.version", ctx = "header: FeatureHeader")]
pub enum DvdCssManagedRecordingDescriptor {
    #[deku(id = "0b0000")]
    V0 {
        #[deku(pad_bits_after = "3")]
        maximum_number_of_scramble_extent_information_entries: u8,
    },
}
