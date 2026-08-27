use super::FeatureHeader;
use deku::DekuRead;

// NOTE: The Embedded Changer Feature Descriptor Format diagram is incorrect in MMC-6 and was ammended by MMC-6 Ammendment 1.
#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead)]
#[deku(id = "header.version", ctx = "header: FeatureHeader")]
pub enum EmbeddedChangerDescriptor {
    #[deku(id = "0b0000")]
    V0 {
        #[deku(pad_bits_before = "3", bits = 1)]
        side_change_capable: bool,
        #[deku(pad_bits_before = "1", bits = 1, pad_bits_after = "2")]
        supports_disc_present: bool,
        #[deku(pad_bytes_before = "2")]
        highest_slot_number: u8,
    },
}
