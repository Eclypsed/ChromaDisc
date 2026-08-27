use super::FeatureHeader;
use deku::DekuRead;

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead)]
#[deku(id = "header.version", ctx = "header: FeatureHeader")]
pub enum RigidRestrictedOverwriteDescriptor {
    #[deku(id = "0b0000")]
    V0 {
        #[deku(pad_bits_before = "4", bits = 1)]
        dsdg: bool,
        #[deku(bits = 1)]
        dsdr: bool,
        #[deku(bits = 1)]
        intermediate: bool,
        #[deku(bits = 1, pad_bytes_after = "3")]
        blank: bool,
    },
}
