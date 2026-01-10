use num_enum::{IntoPrimitive, TryFromPrimitive};
use thiserror::Error;

use crate::features::{
    self, FeatureDescriptor, parse_fature,
    profile_list::{self, Profile},
};

use super::{Command, Control};

const FEATURE_HEADER_LEN: usize = 8;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Received {0} bytes of GET CONFIGURATION response, expected at least {min}", min = FEATURE_HEADER_LEN)]
    IncompleteHeader(usize),
    #[error(
        "Received {received} bytes of GET CONFIGURATION data, 'Data Length' expected: {data_length}"
    )]
    LengthMismatch { received: usize, data_length: u32 },
    #[error(transparent)]
    UnknownProfile(#[from] profile_list::Error),
    #[error(transparent)]
    MalformedResponse(#[from] features::Error),
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

#[allow(dead_code)]
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

#[allow(dead_code)]
pub struct GetConfigurationResponse {
    /// The number of bytes in the response following this field, which comprises the first 4 bytes
    data_length: u32,
    /// The drive's current profile
    pub current_profile: Profile,
    /// The list of defined Feature Descriptors this drive is capable of
    pub descriptors: Vec<FeatureDescriptor>,
}

// Splits out a feature descriptor from a slice of bytes, returning the bytes that made up the
// descriptor, and the remaining bytes after. Currently this does not verify that there aren't any
// trailing bytes that would indicate an invalid response to the GET CONFIGURATION command.
fn next_descriptor(data: &[u8]) -> Option<(&[u8], &[u8])> {
    let generic_bytes = data.get(0..4)?;
    let end: usize = (generic_bytes[3] + 4).into();
    let descriptor = data.get(0..end)?;
    let remainder = data.get(end..).unwrap_or(&[]);

    Some((descriptor, remainder))
}

impl TryFrom<Vec<u8>> for GetConfigurationResponse {
    type Error = Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let response_len = value.len();

        if response_len < FEATURE_HEADER_LEN {
            return Err(Error::IncompleteHeader(response_len));
        }

        let data_length = u32::from_be_bytes([value[0], value[1], value[2], value[3]]);
        let current_profile =
            Profile::try_from_primitive(u16::from_be_bytes([value[6], value[7]]))?;

        if response_len - 4 != data_length as usize {
            return Err(Error::LengthMismatch {
                received: response_len - 4,
                data_length,
            });
        }

        let mut descriptor_bytes = value.get(FEATURE_HEADER_LEN..).unwrap_or(&[]);
        let mut descriptors = Vec::new();

        while let Some((chunk, remainder)) = next_descriptor(descriptor_bytes) {
            match parse_fature(chunk) {
                Ok(descriptor) => descriptors.push(descriptor),
                Err(e) => println!("{e}"),
            }

            descriptor_bytes = remainder;
        }

        Ok(Self {
            data_length,
            current_profile,
            descriptors,
        })
    }
}
