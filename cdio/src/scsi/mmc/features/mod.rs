use deku::{ctx::ByteSize, deku_derive, DekuRead};

pub mod core_feature;
pub mod profile_list;

#[deku_derive(DekuRead)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Feature {
    #[deku(temp, endian = "big")]
    _feature_code: u16,
    #[deku(temp, pad_bits_before = "2", bits = 4)]
    _version: u8,
    #[deku(bits = 1)]
    persistent: bool,
    #[deku(bits = 1)]
    current: bool,
    #[deku(temp)]
    _additional_length: u8,
    #[deku(
        bytes = "*_additional_length as usize",
        ctx = "*_feature_code, *_version"
    )]
    feature_data: FeatureData,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead)]
#[deku(
    ctx = "bytes: ByteSize, feature_code: u16, version: u8",
    id = "feature_code"
)]
pub enum FeatureData {
    #[deku(id = "0x0000")]
    ProfileList(#[deku(bytes_read = "bytes.0")] Vec<profile_list::ProfileDescriptor>),
    #[deku(id = "0x0001")]
    Core(#[deku(ctx = "version")] core_feature::CoreDescriptor),
}

struct FeatureDescriptor<'a> {
    feature_code: u16,
    version: u8,
    persistent: bool,
    current: bool,
    feature_dependent_data: &'a [u8],
}
