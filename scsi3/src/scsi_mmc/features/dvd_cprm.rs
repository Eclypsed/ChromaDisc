use super::FeatureHeader;
use deku::DekuRead;

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead)]
#[deku(id = "header.version", ctx = "header: FeatureHeader")]
pub enum DvdCprmDescriptor {
    #[deku(id = "0b0000")]
    V0 {
        #[deku(pad_bytes_before = "3")]
        cprm_version: u8,
    },
}
