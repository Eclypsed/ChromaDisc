use crate::mmc::features::FeatureCode;

use crate::core::Control;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
enum RequestType {
    /// The Drive shall return the Feature Header and all Feature Descriptors supported by the
    /// Drive without regard to currency.
    All = 0b00,
    /// The Drive shall return the Feature Header and only those Feature Descriptors in which the
    /// Current bit is set to one.
    Current = 0b01,
    /// The Feature Header and the Feature Descriptor identified by Starting Feature Number shall
    /// be returned. If the Drive does not support the specified feature, only the Feature Header
    /// shall be returned.
    SingleFeature = 0b10,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetConfiguration {
    rt: RequestType,
    starting_feature_number: FeatureCode,
    allocation_length: u16,
    control: Control,
}

impl GetConfiguration {
    pub fn all(
        starting_feature_number: FeatureCode,
        allocation_length: u16,
        control: Control,
    ) -> Self {
        Self {
            rt: RequestType::All,
            starting_feature_number,
            allocation_length,
            control,
        }
    }

    pub fn current(
        starting_feature_number: FeatureCode,
        allocation_length: u16,
        control: Control,
    ) -> Self {
        Self {
            rt: RequestType::Current,
            starting_feature_number,
            allocation_length,
            control,
        }
    }

    pub fn single(feature_number: FeatureCode, allocation_length: u16, control: Control) -> Self {
        Self {
            rt: RequestType::SingleFeature,
            starting_feature_number: feature_number,
            allocation_length,
            control,
        }
    }
}

// pub struct GetConfigurationResponse {
//     pub current_profile: ProfileNumber,
//     pub features: Truncatable<Vec<FeatureStatus<FeatureResult<Feature>>>>,
// }

// #[derive(Debug, Error)]
// pub enum GetConfigurationError {
//     #[error("Response header incomplete, received: {0} bytes")]
//     IncompleteHeader(usize),
// }

// pub struct Truncatable<T> {
//     pub result: T,
//     pub trailing_bytes: Option<Bytes>,
// }

// impl Response for GetConfigurationResponse {
//     type Error = GetConfigurationError;

//     fn from_bytes(bytes: Bytes) -> Result<Self, Self::Error> {
//         const MAX_BYTES: usize = 65534;
//         let mut bytes = bytes.take(MAX_BYTES);

//         const FEATURE_HEADER_LEN: usize = 8;

//         let response_size = bytes.remaining();
//         if response_size < FEATURE_HEADER_LEN {
//             return Err(GetConfigurationError::IncompleteHeader(response_size));
//         }

//         let data_length = bytes.get_u32();
//         let multiple_commands_needed = data_length > (MAX_BYTES - 4) as u32;

//         bytes.advance(2);
//         let current_profile = ProfileNumber::from(bytes.get_u16());

//         let mut descriptors = std::iter::from_fn(move || {
//             let feature_code = bytes.try_get_u16().map(FeatureCode::from).ok()?;
//             let (version, persistent, current) = bytes
//                 .try_get_u8()
//                 .map(|b| ((b & 0x3C) >> 2, b & 0b10 != 0, b & 0b01 != 0))
//                 .ok()?;
//             let additional_length = bytes.try_get_u8().ok()?;
//             let feature_data = bytes.take(additional_length.into()).chunk();

//             let reader = Reader::new(Cursor::new(feature_data));

//             todo!()
//         });

// let mut index: usize = 0;
// let mut descriptors = std::iter::from_fn(move || {
//     let feature_code =
//         FeatureCode::from(u16::from_be_bytes([bytes[index], bytes[index + 1]]));
//     let version = (bytes[index + 2] & 0x3C) >> 2;
//     let additional_length = bytes[index + 3] as usize;
//     let data_start = index + 4;
//     let data_end = data_start + additional_length;
//     let data = bytes.slice(data_start..data_end);
//     index += additional_length + 4;
//     Some(RawFeatureDescriptor {
//         feature_code,
//         version,
//         data,
//     })
// });
// todo!();
//     }
// }

// GET CONFIGURATION response invariant cases. These should not really be considered errors.
// - Unknown Feature Code
// - Unknown version
//
// GET CONFIGURATION response error cases:
// - Parsing the expected version does not consume all the data bytes (there is a mismatch between the additional length and the version parsing logic for the given feature)

// type GetConfigurationOpCode = OpCode<0x46>;

// impl Command<GetConfigurationOpCode> for GetConfiguration {
//     type Response = GetConfigurationResponse;

//     fn as_cdb(&self) -> <GetConfigurationOpCode as OpCodeDef>::Cdb {
//         [
//             GetConfigurationOpCode::OP_CODE,
//             (self.rt as u8) & 0x03,
//             (u16::from(self.starting_feature_number) >> 8) as u8,
//             u16::from(self.starting_feature_number) as u8,
//             0,
//             0,
//             0,
//             (self.allocation_length >> 8) as u8,
//             self.allocation_length as u8,
//             self.control.into(),
//         ]
//     }
// }
