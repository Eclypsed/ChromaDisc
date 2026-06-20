use crate::scsi::mmc::features::{Feature, FeatureCodeDef, FeatureData};

use super::{Command, Control, OpCode};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum RtField {
    /// The Drive shall return the Feature Header and all Feature Descriptors supported by the
    /// Drive without regard to currency.
    All = 0b00,
    /// The Drive shall return the Feature Header and only those Feature Descriptors in which the
    /// Current bit is set to one.
    Current = 0b01,
    /// The Feature Header and the Feature Descriptor identified by Starting Feature Number shall
    /// be returned. If the Drive does not support the specified feature, only the Feature Header
    /// shall be returned.
    Supported = 0b10,
}

pub struct GetConfiguration {
    rt: RtField,
    starting_feature_number: u16,
    allocation_length: u16,
    control: Control,
}

pub struct GetConfigurationResponse {
    current_profile: u16,
    // feature_descriptors: Vec<Feature<dyn FeatureCodeDef<FeatureData = >>>,
}

type GetConfigurationOpCode = OpCode<0x46>;

// impl Command<GetConfigurationOpCode> for GetConfiguration {
//     fn as_cdb(&self) -> <GetConfigurationOpCode as super::OpCodeDef>::Cdb {}
// }
