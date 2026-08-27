use super::FeatureHeader;
use deku::DekuRead;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead)]
#[deku(id = "header.version", ctx = "header: FeatureHeader")]
pub enum RemovableMediumDescriptor {
    #[deku(id = "0b0000")]
    V0 {
        loading_mechanism_type: LoadingMechanism,
        #[deku(pad_bits_before = "1", bits = 1)]
        eject: bool,
        #[deku(bits = 1)]
        prevent_jumper: bool,
        #[deku(pad_bits_before = "1", bits = 1, pad_bytes_after = "3")]
        lock: bool,
    },
    #[deku(id = "0b0001")]
    V1 {
        loading_mechanism_type: LoadingMechanism,
        #[deku(bits = 1)]
        load: bool,
        #[deku(bits = 1)]
        eject: bool,
        #[deku(bits = 1, pad_bits_after = "1")]
        prevent_jumper: bool,
        #[deku(bits = 1, pad_bytes_after = "3")]
        lock: bool,
    },
    #[deku(id = "0b0010")]
    V2 {
        loading_mechanism_type: LoadingMechanism,
        #[deku(bits = 1)]
        load: bool,
        #[deku(bits = 1)]
        eject: bool,
        #[deku(bits = 1)]
        prevent_jumper: bool,
        #[deku(bits = 1)]
        dbml: bool,
        #[deku(bits = 1, pad_bytes_after = "3")]
        lock: bool,
    },
}

/// A 3-bit value representing the phyical Loading Mechanism Type used by drives that support
/// Removable Mediums.
///
/// See MMC-6 §5.3.4, Table 102.
#[repr(u8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, DekuRead)]
#[deku(id_type = "u8", bits = 3)]
pub enum LoadingMechanism {
    CaddySlot = 0b000,
    Tray = 0b001,
    PopUp = 0b010,
    EmbeddedIndividuallyChangeable = 0b100,
    EmbeddedMagazine = 0b101,
    #[deku(id_pat = "_")]
    Reserved(u8),
}
