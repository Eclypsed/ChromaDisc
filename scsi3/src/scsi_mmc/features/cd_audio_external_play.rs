use super::FeatureHeader;
use deku::DekuRead;

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead)]
#[deku(id = "header.version", ctx = "header: FeatureHeader")]
pub enum CdAudioExternalPlayDescriptor {
    #[deku(id = "0b0000")]
    V0 {
        #[deku(pad_bits_before = "5", bits = 1)]
        scan: bool,
        #[deku(bits = 1)]
        separate_channel_mute: bool,
        #[deku(bits = 1)]
        separate_volume: bool,
        #[deku(pad_bytes_before = "1", endian = "big")]
        highest_slot_number: u16,
    },
}
