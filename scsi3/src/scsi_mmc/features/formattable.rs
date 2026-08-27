use super::FeatureHeader;
use deku::DekuRead;

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead)]
#[deku(id = "header.version", ctx = "header: FeatureHeader")]
pub enum FormattableDescriptor {
    #[deku(id = "0b0000")]
    V0,
    #[deku(id = "0b0001")]
    V1 {
        #[deku(pad_bits_before = "4", bits = 1)]
        renosa: bool,
        #[deku(bits = 1)]
        expand: bool,
        #[deku(bits = 1)]
        qcert: bool,
        #[deku(bits = 1, pad_bytes_after = "3")]
        cert: bool,
        #[deku(pad_bits_before = "7", bits = 1, pad_bytes_after = "3")]
        rrm: bool,
    },
    #[deku(id = "0b0010")]
    V2 {
        #[deku(pad_bits_before = "4", bits = 1)]
        renosa: bool,
        #[deku(bits = 1)]
        expand: bool,
        #[deku(bits = 1)]
        qcert: bool,
        #[deku(bits = 1)]
        cert: bool,
        #[deku(bits = 1, pad_bits_after = "23")]
        frf: bool,
        #[deku(pad_bits_before = "7", bits = 1, pad_bytes_after = "3")]
        rrm: bool,
    },
}
