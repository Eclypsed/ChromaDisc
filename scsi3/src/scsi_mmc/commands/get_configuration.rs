use core::{fmt, iter::FusedIterator};

use thiserror::Error;
use zerocopy::{
    BigEndian, CastError, FromBytes, Immutable, IntoBytes, KnownLayout, U32, Unaligned,
};

use crate::{
    core::{Command, Control, OpCode, OpCodeDef, ReadCommand, TruncationError},
    mmc::{
        features::{Feature, FeatureCode, FeatureDescriptor, GenericFeatureDescriptor},
        profiles::ProfileNumber,
    },
};

/// The RT field: which Feature Descriptors the Drive returns.
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

/// GET CONFIGURATION (46h): requests the Feature Header and the Feature Descriptors that describe
/// what the Drive and its current medium can do.
///
/// A Drive returns Feature Descriptors in ascending Feature Code order, starting at Starting
/// Feature Number. A response longer than Allocation Length is cut short. To get the rest, issue
/// the command again with Starting Feature Number one past the last complete descriptor's
/// Feature Code.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct GetConfiguration {
    rt: RequestType,
    starting_feature_number: FeatureCode,
    allocation_length: u16,
    control: Control,
}

impl GetConfiguration {
    /// Requests every Feature Descriptor the Drive supports, current or not, from
    /// `starting_feature_number` up.
    pub const fn all(
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

    /// Requests the Feature Descriptors of current Features, from `starting_feature_number` up.
    pub const fn current(
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

    /// Requests the Feature Descriptor of `feature_number` alone.
    ///
    /// The response holds no Feature Descriptor if the Drive doesn't support that Feature.
    pub const fn single(
        feature_number: FeatureCode,
        allocation_length: u16,
        control: Control,
    ) -> Self {
        Self {
            rt: RequestType::SingleFeature,
            starting_feature_number: feature_number,
            allocation_length,
            control,
        }
    }
}

type GetConfigurationOpCode = OpCode<0x46>;

impl Command<GetConfigurationOpCode> for GetConfiguration {
    fn as_cdb(&self) -> <GetConfigurationOpCode as OpCodeDef>::Cdb {
        [
            GetConfigurationOpCode::OP_CODE,
            self.rt as u8,
            (self.starting_feature_number.into_u16() >> 8) as u8,
            self.starting_feature_number.into_u16() as u8,
            0,
            0,
            0,
            (self.allocation_length >> 8) as u8,
            self.allocation_length as u8,
            self.control.into(),
        ]
    }
}

/// The length of the Feature Header.
const GET_CONFIGURATION_HEADER_LEN: usize = 8;

/// Why a GET CONFIGURATION response couldn't be parsed.
#[derive(Debug, Error)]
pub enum GetConfigurationError {
    /// The response is shorter than the Feature Header.
    #[error(transparent)]
    Truncated(#[from] TruncationError),
}

/// The response to GET CONFIGURATION: the Feature Header, then the Feature Descriptors.
#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct GetConfigurationResponse {
    data_length: U32<BigEndian>,
    _reserved: [u8; 2],
    current_profile: ProfileNumber,
    feature_descriptor_data: [u8],
}

impl GetConfigurationResponse {
    /// The length of the response that follows this field.
    ///
    /// Not cut by Allocation Length, so it can exceed the bytes the Drive transferred.
    #[inline]
    pub const fn data_length(&self) -> u32 {
        self.data_length.get()
    }

    /// The Profile the Drive is operating in, or [`ProfileNumber::NO_CURRENT_PROFILE`] if none
    /// is.
    #[inline]
    pub const fn current_profile(&self) -> ProfileNumber {
        self.current_profile
    }

    /// The Feature Descriptors, cut to Data Length.
    ///
    /// A descriptor is yielded only if all of its Additional Length bytes are present, so each
    /// one is exactly as the Drive reported it, provided the response holds only the bytes the
    /// Drive transferred. A descriptor cut short ends iteration and is left in
    /// [`FeatureDescriptors::remainder`].
    pub fn feature_descriptors(&self) -> FeatureDescriptors<'_> {
        let data = &self.feature_descriptor_data;
        let len = usize::try_from(self.data_length().saturating_sub(4)).unwrap_or(usize::MAX);
        FeatureDescriptors {
            buf: data.get(..len).unwrap_or(data),
        }
    }

    /// The first Feature Descriptor of `F`, or `None` if there isn't one.
    pub fn get<F: Feature + ?Sized>(&self) -> Option<&FeatureDescriptor<F>> {
        self.feature_descriptors()
            .find_map(|descriptor| descriptor.get::<F>())
    }
}

impl fmt::Debug for GetConfigurationResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut walked = self.feature_descriptors();
        walked.by_ref().for_each(drop);

        f.debug_struct("GetConfigurationResponse")
            .field("data_length", &self.data_length())
            .field("current_profile", &self.current_profile())
            .field(
                "feature_descriptors",
                &fmt::from_fn(|f| f.debug_list().entries(self.feature_descriptors()).finish()),
            )
            .field("remainder", &walked.remainder())
            .finish()
    }
}

/// The Feature Descriptors of a [`GetConfigurationResponse`], in the order they appear in the
/// response.
#[derive(Debug, Clone)]
pub struct FeatureDescriptors<'a> {
    buf: &'a [u8],
}

impl<'a> FeatureDescriptors<'a> {
    /// The bytes not yet walked.
    ///
    /// Non-empty after iteration only if the response ends partway through a Feature Descriptor.
    #[inline]
    pub const fn remainder(&self) -> &'a [u8] {
        self.buf
    }
}

impl<'a> Iterator for FeatureDescriptors<'a> {
    type Item = &'a GenericFeatureDescriptor;

    fn next(&mut self) -> Option<Self::Item> {
        // Additional Length, the last byte of the descriptor's header
        let additional_length = *self.buf.get(3)?;

        let elems = additional_length.into();
        let (descriptor, rest) =
            match GenericFeatureDescriptor::ref_from_prefix_with_elems(self.buf, elems) {
                Ok(res) => res,
                Err(CastError::Size(_)) => return None, // Cut short, left in `remainder()`
                Err(CastError::Alignment(_)) => {
                    unreachable!("`GenericFeatureDescriptor` should be `Unaligned`")
                }
            };

        self.buf = rest;
        Some(descriptor)
    }
}

impl<'a> FusedIterator for FeatureDescriptors<'a> {}

impl ReadCommand<GetConfigurationOpCode> for GetConfiguration {
    type Len = u16;
    type Response<'a> = (&'a GetConfigurationResponse, &'a [u8]);
    type Error = GetConfigurationError;

    fn response_len(&self) -> Self::Len {
        self.allocation_length
    }

    /// Reads `buf` as a response cut to Data Length, and returns the bytes after it.
    ///
    /// `buf` should hold only the bytes the Drive transferred. Any bytes past the transfer that
    /// still fall within Data Length, such as an allocated buffer's unused tail, are read as part
    /// of the response.
    fn parse<'a>(&self, buf: &'a [u8]) -> Result<Self::Response<'a>, Self::Error> {
        // Data Length counts the bytes following its own 4-byte field. A `buf` too short to hold
        // it holds no Feature Descriptors either, so 0 stands in, and the cast reports the
        // truncation.
        let data_length = buf.first_chunk().map_or(0, |&len| u32::from_be_bytes(len));
        let elems = usize::try_from(data_length.saturating_sub(4))
            .unwrap_or(usize::MAX)
            .min(buf.len().saturating_sub(GET_CONFIGURATION_HEADER_LEN));

        match GetConfigurationResponse::ref_from_prefix_with_elems(buf, elems) {
            Ok(res) => Ok(res),
            Err(CastError::Size(_)) => Err(TruncationError {
                expected: GET_CONFIGURATION_HEADER_LEN,
                received: buf.len(),
            }
            .into()),
            Err(CastError::Alignment(_)) => {
                unreachable!("`GetConfigurationResponse` should be `Unaligned`")
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::mmc::features::{cd_read::CdRead, core::Core};

    fn command() -> GetConfiguration {
        GetConfiguration::all(FeatureCode::PROFILE_LIST, 0xFFFF, Control::default())
    }

    #[test]
    fn truncated_header_is_an_error() {
        assert!(matches!(
            command().parse(&[0; 7]),
            Err(GetConfigurationError::Truncated(TruncationError {
                expected: 8,
                received: 7
            }))
        ));
    }

    #[test]
    fn parse_cuts_at_data_length() {
        let buf = [
            0, 0, 0, 16, 0, 0, 0x00, 0x08, // Feature Header: CD-ROM
            0x00, 0x22, 0b0011, 0, // Sector Erasable
            0x00, 0x1E, 0b1011, 4, 0b1, 0, 0, 0, // CD Read v2
            0xEE, 0xEE, 0xEE,
        ];
        let (response, rest) = command().parse(&buf).unwrap();
        assert_eq!(rest, [0xEE; 3]);
        assert_eq!(response.data_length(), 16);
        assert_eq!(response.current_profile(), ProfileNumber::CD_ROM);

        let mut descriptors = response.feature_descriptors();
        assert_eq!(
            descriptors.next().unwrap().feature_code(),
            FeatureCode::SECTOR_ERASABLE
        );
        assert_eq!(
            descriptors.next().unwrap().feature_code(),
            FeatureCode::CD_READ
        );
        assert!(descriptors.next().is_none());
        assert!(descriptors.remainder().is_empty());
    }

    #[test]
    fn cut_descriptor_is_left_in_remainder() {
        // Data Length larger than the bytes transferred
        let buf = [
            0, 0, 0, 40, 0, 0, 0x00, 0x08, // Feature Header: CD-ROM
            0x00, 0x22, 0b0011, 0, // Sector Erasable
            0x00, 0x1E, 0b1011, 4, 0b1, // CD Read v2, cut short
        ];
        let (response, rest) = command().parse(&buf).unwrap();
        assert!(rest.is_empty());
        assert_eq!(response.data_length(), 40);

        let mut descriptors = response.feature_descriptors();
        assert_eq!(
            descriptors.next().unwrap().feature_code(),
            FeatureCode::SECTOR_ERASABLE
        );
        assert!(descriptors.next().is_none());
        assert_eq!(descriptors.remainder(), [0x00, 0x1E, 0b1011, 4, 0b1]);

        // Cut within the header
        let (response, _) = command().parse(&buf[..14]).unwrap();
        let mut descriptors = response.feature_descriptors();
        assert_eq!(
            descriptors.next().unwrap().feature_code(),
            FeatureCode::SECTOR_ERASABLE
        );
        assert!(descriptors.next().is_none());
        assert_eq!(descriptors.remainder(), [0x00, 0x1E]);
    }

    #[test]
    fn get_finds_typed_descriptor() {
        let buf = [
            0, 0, 0, 16, 0, 0, 0x00, 0x08, // Feature Header: CD-ROM
            0x00, 0x22, 0b0011, 0, // Sector Erasable
            0x00, 0x1E, 0b1011, 4, 0b1, 0, 0, 0, // CD Read v2
        ];
        let (response, _) = command().parse(&buf).unwrap();
        assert!(response.get::<CdRead>().unwrap().data().unwrap().cd_text());
        assert!(response.get::<Core>().is_none());
    }

    #[test]
    fn direct_cast_is_cut_to_data_length() {
        let mut buf = [
            0, 0, 0, 8, 0, 0, 0x00, 0x08, // Feature Header: CD-ROM
            0x00, 0x22, 0b0011, 0, // Sector Erasable
            0x00, 0x1E, 0b1011, 0, // CD Read, past Data Length
        ];
        let response = GetConfigurationResponse::ref_from_bytes(&buf).unwrap();
        let mut descriptors = response.feature_descriptors();
        assert_eq!(
            descriptors.next().unwrap().feature_code(),
            FeatureCode::SECTOR_ERASABLE
        );
        assert!(descriptors.next().is_none());
        assert!(descriptors.remainder().is_empty());

        // Data Length too small to count the Feature Header's own fields
        buf[3] = 0;
        let response = GetConfigurationResponse::ref_from_bytes(&buf).unwrap();
        assert!(response.feature_descriptors().next().is_none());
    }
}
