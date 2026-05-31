use num_enum::IntoPrimitive;
use thiserror::Error;

use super::features::{FeatureParser, MmcFeature};
use super::types::{q_subchannel, Profile};
use crate::transport::sgio::{run_sgio, DxferDirection, ScsiError};

#[derive(Debug, Error)]
pub enum CommandError {
    #[error("Response specified {expected} bytes of data, received {received}")]
    MissingData { expected: usize, received: usize },
    #[error(transparent)]
    ScsiError(#[from] ScsiError),
}

/// GET CONFIGURATION Response data
///
/// See MMC-6 §6.5.2.1
pub struct GetConfigurationResponse {
    /// The current profile the drive is occupying
    pub current_profile: Profile,
    /// The list of features the drive supports
    ///
    /// See MMC-6 §6.5.2.2
    pub feature_descriptors: Vec<MmcFeature>,
}

/// Identifies the type of data to be returned to the drive.
///
/// See MMM-6 §6.5.1.2, Table 283
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
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

pub fn get_configuration(fd: i32, rt: RtField) -> Result<GetConfigurationResponse, CommandError> {
    const OP_CODE: u8 = 0x046;
    const ALLOCATION_LENGTH: u16 = 0x0100;
    const CONTROL: u8 = 0;

    let mut cdb = [0u8; 10];
    cdb[0] = OP_CODE;
    cdb[1] |= u8::from(rt) & 0b11;
    cdb[7] = (ALLOCATION_LENGTH >> 8) as u8;
    cdb[8] = ALLOCATION_LENGTH as u8;
    cdb[9] = CONTROL;

    let bytes = run_sgio(
        fd,
        &mut cdb,
        ALLOCATION_LENGTH.into(),
        DxferDirection::FromDev,
    )?;
    let res_len = bytes.len();

    const FEATURE_HEADER_LENGTH: usize = 8;

    if res_len < FEATURE_HEADER_LENGTH {
        return Err(CommandError::MissingData {
            expected: FEATURE_HEADER_LENGTH,
            received: res_len,
        });
    }

    // let data_len = u32::from_be_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
    let current_profile = Profile::from(u16::from_be_bytes([bytes[6], bytes[7]]));

    // TODO: Implement case for when data_len > 65530
    // Note, this is an extreme edge case that isn't possible in reallity, but is defined in the
    // spec.

    let descriptor_bytes = bytes.get(8..).unwrap_or(&[]);
    let descriptors = FeatureParser::new(descriptor_bytes).collect::<Vec<MmcFeature>>();

    Ok(GetConfigurationResponse {
        current_profile,
        feature_descriptors: descriptors,
    })
}

mod read_toc_pma_atip {
    use crate::core::addressing::{RawLba, RawMsf};

    use super::*;

    const MIN_READ_TOC_PMA_ATIP_RESPONSE_LEN: usize = 4;

    /// General format of a response to the READ TOC/PMA/ATIP Command
    ///
    /// See MMC-6 §6.25.3.1, Table 498
    struct ReadTocPmaAtipResponse {
        pub first_track_session_reserved_field: u8,
        pub last_track_session_reserved_field: u8,
        /// Generic descriptor data, format specific.
        pub descriptor_data: Vec<u8>,
    }

    fn read_toc_pma_atip(
        fd: i32,
        msf: bool,
        format: Format,
        track_session_num: u8,
    ) -> Result<ReadTocPmaAtipResponse, CommandError> {
        const OP_CODE: u8 = 0x43;
        // Don't know if this should be fixed, need to investigate if "allocation length" is
        // something that should be consumer-determined
        const ALLOCATION_LEN: u16 = 0x0100;

        let mut cdb = [0u8; 10];
        cdb[0] = OP_CODE;
        cdb[1] |= u8::from(msf) << 1;
        cdb[2] |= u8::from(format) & 0x0F;
        cdb[6] = track_session_num;
        cdb[7] = (ALLOCATION_LEN >> 8) as u8;
        cdb[8] = ALLOCATION_LEN as u8;

        let mut bytes = run_sgio(fd, &mut cdb, ALLOCATION_LEN.into(), DxferDirection::FromDev)?;
        let res_len = bytes.len();

        if res_len > MIN_READ_TOC_PMA_ATIP_RESPONSE_LEN {
            return Err(CommandError::MissingData {
                expected: MIN_READ_TOC_PMA_ATIP_RESPONSE_LEN,
                received: res_len,
            });
        }

        let data_length = u16::from_be_bytes([bytes[0], bytes[1]]);
        bytes.truncate((data_length + 4).into());
        let first_track_session_reserved_field = bytes[2];
        let last_track_session_reserved_field = bytes[3];
        let descriptor_data = bytes.drain(0..4).collect::<Vec<_>>();

        Ok(ReadTocPmaAtipResponse {
            first_track_session_reserved_field,
            last_track_session_reserved_field,
            descriptor_data,
        })
    }

    mod sealed {
        pub trait Sealed {
            fn parse_track_start_address(value: [u8; 4]) -> Self;
        }
    }

    pub trait TocAddress {}
    impl sealed::Sealed for RawLba {
        fn parse_track_start_address(value: [u8; 4]) -> Self {
            Self::new(i32::from_be_bytes(value))
        }
    }
    impl TocAddress for RawLba {}
    impl sealed::Sealed for RawMsf {
        fn parse_track_start_address(value: [u8; 4]) -> Self {
            Self::new(value[1], value[2], value[3])
        }
    }
    impl TocAddress for RawMsf {}

    pub struct FormattedTocTrackDescriptor<A: TocAddress> {
        pub adr: q_subchannel::Adr,
        pub control: q_subchannel::Control,
        pub track_number: u8,
        pub track_start_address: A,
    }

    impl<A: TocAddress + sealed::Sealed> From<[u8; 8]> for FormattedTocTrackDescriptor<A> {
        fn from(value: [u8; 8]) -> Self {
            Self {
                adr: q_subchannel::Adr::from((value[1] & 0xF0) >> 4),
                control: q_subchannel::Control::from_bits_truncate(value[1] & 0x0F),
                track_number: value[2],
                track_start_address: A::parse_track_start_address(value[4..=7].try_into().unwrap()),
            }
        }
    }

    pub struct FormattedToc<A: TocAddress> {
        pub first_track_number: u8,
        pub last_track_number: u8,
        pub track_descriptors: Vec<FormattedTocTrackDescriptor<A>>,
    }

    impl<A: TocAddress + sealed::Sealed> From<ReadTocPmaAtipResponse> for FormattedToc<A> {
        fn from(value: ReadTocPmaAtipResponse) -> Self {
            Self {
                first_track_number: value.first_track_session_reserved_field,
                last_track_number: value.last_track_session_reserved_field,
                track_descriptors: value
                    .descriptor_data
                    .chunks_exact(8)
                    .map(|d| {
                        FormattedTocTrackDescriptor::<A>::from(<[u8; 8]>::try_from(d).unwrap())
                    })
                    .collect::<Vec<_>>(),
            }
        }
    }

    /// Execute a READ TOC/PMA/ATIP Command for repsonse format 0b0000 (Formatted TOC) with track descriptors containing LBA values.
    ///
    /// See MMC-6 §6.25.3.2
    pub fn read_formatted_toc_lba(
        fd: i32,
        track_number: u8,
    ) -> Result<FormattedToc<RawLba>, CommandError> {
        Ok(read_toc_pma_atip(fd, false, Format::FormattedToc, track_number)?.into())
    }

    /// Execute a READ TOC/PMA/ATIP Command for repsonse format 0b0000 (Formatted TOC) with track descriptors containing MSF values.
    ///
    /// See MMC-6 §6.25.3.2
    pub fn read_formatted_toc_msf(
        fd: i32,
        track_number: u8,
    ) -> Result<FormattedToc<RawMsf>, CommandError> {
        Ok(read_toc_pma_atip(fd, true, Format::FormattedToc, track_number)?.into())
    }

    mod multi_session_information {
        use super::{
            q_subchannel, read_toc_pma_atip, sealed, CommandError, Format, RawLba, RawMsf,
            ReadTocPmaAtipResponse, TocAddress,
        };

        const TRACK_DESCRIPTOR_LEN: usize = 8;

        pub struct TrackDescriptor<A: TocAddress> {
            pub adr: q_subchannel::Adr,
            pub control: q_subchannel::Control,
            pub last_complete_session_first_track_number: u8,
            pub last_sesssion_first_track_start_address: A,
        }

        impl<A: TocAddress + sealed::Sealed> From<[u8; 8]> for TrackDescriptor<A> {
            fn from(value: [u8; 8]) -> Self {
                Self {
                    adr: q_subchannel::Adr::from((value[1] & 0xF0) >> 4),
                    control: q_subchannel::Control::from_bits_truncate(value[1] & 0x0F),
                    last_complete_session_first_track_number: value[2],
                    last_sesssion_first_track_start_address: A::parse_track_start_address(
                        value[4..=7].try_into().unwrap(),
                    ),
                }
            }
        }

        pub struct MultiSessionInformation<A: TocAddress> {
            pub first_complete_session_number: u8,
            pub last_complete_session_number: u8,
            pub track_descriptor: TrackDescriptor<A>,
        }

        impl<A: TocAddress + sealed::Sealed> TryFrom<ReadTocPmaAtipResponse>
            for MultiSessionInformation<A>
        {
            type Error = CommandError;

            fn try_from(value: ReadTocPmaAtipResponse) -> Result<Self, Self::Error> {
                let descriptor_len = value.descriptor_data.len();
                if descriptor_len > TRACK_DESCRIPTOR_LEN {
                    return Err(CommandError::MissingData {
                        expected: TRACK_DESCRIPTOR_LEN,
                        received: descriptor_len,
                    });
                }

                let descriptor_bytes: [u8; TRACK_DESCRIPTOR_LEN] = value.descriptor_data
                    [..TRACK_DESCRIPTOR_LEN]
                    .try_into()
                    .unwrap();

                Ok(Self {
                    first_complete_session_number: value.first_track_session_reserved_field,
                    last_complete_session_number: value.last_track_session_reserved_field,
                    track_descriptor: TrackDescriptor::<A>::from(descriptor_bytes),
                })
            }
        }

        pub fn read_multi_session_information_lba(
            fd: i32,
        ) -> Result<MultiSessionInformation<RawLba>, CommandError> {
            read_toc_pma_atip(fd, false, Format::MultiSessionInformation, 0)?.try_into()
        }

        pub fn read_multi_session_information_msf(
            fd: i32,
        ) -> Result<MultiSessionInformation<RawMsf>, CommandError> {
            read_toc_pma_atip(fd, true, Format::MultiSessionInformation, 0)?.try_into()
        }
    }

    mod raw_toc {
        use super::{
            q_subchannel, read_toc_pma_atip, CommandError, Format, ReadTocPmaAtipResponse,
        };

        pub struct TrackDescriptor {
            pub session_number: u8,
            pub adr: q_subchannel::Adr,
            pub control: q_subchannel::Control,
            pub track_number: u8,
            pub point: u8,
            pub min: u8,
            pub sec: u8,
            pub frame: u8,
            pub pmin: u8,
            pub psec: u8,
            pub pframe: u8,
        }

        impl From<[u8; 11]> for TrackDescriptor {
            fn from(value: [u8; 11]) -> Self {
                Self {
                    session_number: value[0],
                    adr: q_subchannel::Adr::from((value[1] & 0xF0) >> 4),
                    control: q_subchannel::Control::from_bits_truncate(value[1] & 0x0F),
                    track_number: value[2],
                    point: value[3],
                    min: value[4],
                    sec: value[5],
                    frame: value[6],
                    pmin: value[8],
                    psec: value[9],
                    pframe: value[10],
                }
            }
        }

        pub struct RawToc {
            pub first_complete_session_number: u8,
            pub last_complete_session_number: u8,
            pub track_descriptors: Vec<TrackDescriptor>,
        }

        impl From<ReadTocPmaAtipResponse> for RawToc {
            fn from(value: ReadTocPmaAtipResponse) -> Self {
                Self {
                    first_complete_session_number: value.first_track_session_reserved_field,
                    last_complete_session_number: value.last_track_session_reserved_field,
                    track_descriptors: value
                        .descriptor_data
                        .chunks_exact(11)
                        .map(|d| TrackDescriptor::from(<[u8; 11]>::try_from(d).unwrap()))
                        .collect::<Vec<_>>(),
                }
            }
        }

        fn read_raw_toc(fd: i32, session_number: u8) -> Result<RawToc, CommandError> {
            Ok(read_toc_pma_atip(fd, true, Format::RawToc, session_number)?.into())
        }
    }

    mod pma {
        use super::{
            q_subchannel, read_toc_pma_atip, CommandError, Format, ReadTocPmaAtipResponse,
        };

        pub struct PmaDescriptor {
            pub adr: q_subchannel::Adr,
            pub control: q_subchannel::Control,
            pub track_number: u8,
            pub point: u8,
            pub min: u8,
            pub sec: u8,
            pub frame: u8,
            pub pmin: u8,
            pub psec: u8,
            pub pframe: u8,
        }

        impl From<[u8; 11]> for PmaDescriptor {
            fn from(value: [u8; 11]) -> Self {
                Self {
                    adr: q_subchannel::Adr::from((value[1] & 0xF0) >> 4),
                    control: q_subchannel::Control::from_bits_truncate(value[1] & 0x0F),
                    track_number: value[2],
                    point: value[3],
                    min: value[4],
                    sec: value[5],
                    frame: value[6],
                    pmin: value[8],
                    psec: value[9],
                    pframe: value[10],
                }
            }
        }

        pub struct Pma {
            pma_descriptors: Vec<PmaDescriptor>,
        }

        impl From<ReadTocPmaAtipResponse> for Pma {
            fn from(value: ReadTocPmaAtipResponse) -> Self {
                Self {
                    pma_descriptors: value
                        .descriptor_data
                        .chunks_exact(11)
                        .map(|d| PmaDescriptor::from(<[u8; 11]>::try_from(d).unwrap()))
                        .collect::<Vec<_>>(),
                }
            }
        }

        pub fn read_pma(fd: i32) -> Result<Pma, CommandError> {
            Ok(read_toc_pma_atip(fd, true, Format::Pma, 0)?.into())
        }
    }
}

pub use read_toc_pma_atip::{
    read_formatted_toc_lba, read_formatted_toc_msf, FormattedToc, FormattedTocTrackDescriptor,
    TocAddress,
};
