use crate::mmc::profiles::ProfileNumber;

use super::FeatureHeader;
use deku::deku_derive;

#[deku_derive(DekuRead)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
#[deku(id = "header.version", ctx = "header: FeatureHeader")]
pub enum OsscDescriptor {
    #[deku(id = "0b0000")]
    V0 {
        #[deku(bits = 1)]
        psa_updates: bool,
        #[deku(bits = 1)]
        linked_ospb: bool,
        #[deku(pad_bits_before = "5", bits = 1)]
        mandatory_encryption: bool,
        #[deku(temp)]
        _P: u8,
        #[deku(count = "*_P")]
        profile_numbers: Vec<ProfileNumber>,
    },
}
