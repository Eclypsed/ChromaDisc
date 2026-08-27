use super::FeatureHeader;
use deku::DekuRead;

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead)]
#[deku(id = "header.version", ctx = "header: FeatureHeader")]
pub enum DoubleDensityCdReadDescriptor {
    #[deku(id = "0b0000")]
    V0,
}
