use num_enum::IntoPrimitive;
use thiserror::Error;

use crate::scsi::mmc::features::{FeatureParser, MmcFeature};

use super::{Command, Control};

const FEATURE_HEADER_LENGTH: usize = 8;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Received {0} bytes of GET CONFIGURATION response, expected at least {min}", min = FEATURE_HEADER_LENGTH)]
    IncompleteHeader(usize),
    #[error(
        "Received {received} bytes of GET CONFIGURATION data, 'Data Length' expected: {data_length}"
    )]
    LengthMismatch { received: usize, data_length: u32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u8)]
pub enum RTField {
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

#[derive(Debug, Clone, Copy)]
pub struct GetConfiguration {
    rt: RTField,
    starting_feature_number: u16,
    allocation_length: u16,
    control: Control,
}

impl GetConfiguration {
    pub fn new(
        rt: RTField,
        starting_feature_number: u16,
        allocation_length: u16,
        control: Control,
    ) -> Self {
        Self {
            rt,
            starting_feature_number,
            allocation_length,
            control,
        }
    }
}

impl Command<10> for GetConfiguration {
    const OP_CODE: u8 = 0x46;

    type Response = GetConfigurationResponse;

    fn as_cdb(&self) -> [u8; 10] {
        let mut bytes = [0u8; 10];

        bytes[0] = GetConfiguration::OP_CODE;
        bytes[1] = self.rt.into();
        bytes[2] = (self.starting_feature_number >> 8) as u8;
        bytes[3] = self.starting_feature_number as u8;
        bytes[7] = (self.allocation_length >> 8) as u8;
        bytes[8] = self.allocation_length as u8;
        bytes[9] = self.control.into();

        bytes
    }

    fn allocation_len(&self) -> usize {
        self.allocation_length.into()
    }
}

#[derive(Debug)]
pub struct GetConfigurationResponse {
    /// The number of bytes in the response following this field, which comprises the first 4 bytes
    // data_length: u32,
    /// The drive's current profile
    pub current_profile: u16,
    /// The list of defined Feature Descriptors this drive is capable of
    pub descriptors: Vec<Box<dyn MmcFeature>>,
}

impl TryFrom<Vec<u8>> for GetConfigurationResponse {
    type Error = Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let response_len = value.len();

        if response_len < FEATURE_HEADER_LENGTH {
            return Err(Error::IncompleteHeader(response_len));
        }

        let data_length = u32::from_be_bytes([value[0], value[1], value[2], value[3]]);
        let current_profile = u16::from_be_bytes([value[6], value[7]]);

        if response_len - 4 != data_length as usize {
            return Err(Error::LengthMismatch {
                received: response_len - 4,
                data_length,
            });
        }

        let descriptor_bytes = value.get(FEATURE_HEADER_LENGTH..).unwrap_or(&[]);
        let descriptors =
            FeatureParser::new(descriptor_bytes).collect::<Vec<Box<dyn MmcFeature>>>();

        Ok(Self {
            // data_length,
            current_profile,
            descriptors,
        })
    }
}
