use alloc::vec::Vec;
use arbitrary_int::u4;
use derive_where::derive_where;
use thiserror::Error;

use super::{ReadTocPmaAtip, ReadTocPmaAtipOpCode};
use crate::{
    core::{ReadCommand, TruncationError},
    mmc::device_models::cd::q_subcode,
};

pub use super::formatted_toc::TrackStartAddress;

const MULTI_SESSION_INFORMATION_MIN_BYTES: usize = 4;
const TRACK_DESCRIPTOR_SIZE: usize = 8;

#[derive(Debug, Error)]
pub enum MultiSessionInformationError {
    #[error(transparent)]
    Truncated(#[from] TruncationError<MULTI_SESSION_INFORMATION_MIN_BYTES>),
}

impl<A: TrackStartAddress> ReadCommand<ReadTocPmaAtipOpCode>
    for ReadTocPmaAtip<MultiSessionInformation<A>>
{
    type Len = u16;
    type Response<'a> = MultiSessionInformation<A>;
    type Error = MultiSessionInformationError;

    fn response_len(&self) -> Self::Len {
        self.allocation_length
    }

    fn parse<'a>(&self, buf: &'a [u8]) -> Result<Self::Response<'a>, Self::Error> {
        if buf.len() < MULTI_SESSION_INFORMATION_MIN_BYTES {
            return Err(TruncationError(buf.len()).into());
        }

        let toc_data_length: usize = u16::from_be_bytes([buf[0], buf[1]]).into();
        let first_complete_session_number = buf[2];
        let last_complete_session_number = buf[3];

        let max_bytes = buf.len().min(toc_data_length - 2);
        let desc_bytes = buf.get(4..max_bytes).unwrap_or_default();

        let toc_track_descriptors = desc_bytes
            .chunks_exact(TRACK_DESCRIPTOR_SIZE)
            .map(|chunk| TocTrackDescriptor {
                adr: u4::extract_u8(chunk[1], 4),
                control: q_subcode::Control::from_bits_truncate(chunk[1] & 0xF),
                first_track_number_last_complete_session: chunk[2],
                first_track_in_last_session_start_address: A::read_track_start_address(
                    chunk[4..=7].try_into().unwrap(),
                ),
            })
            .collect::<Vec<_>>();

        Ok(MultiSessionInformation {
            first_complete_session_number,
            last_complete_session_number,
            toc_track_descriptors,
        })
    }
}

#[derive_where(Debug, Clone, PartialEq, Eq, Hash; A::ResponseAddressType)]
pub struct MultiSessionInformation<A: TrackStartAddress> {
    pub first_complete_session_number: u8,
    pub last_complete_session_number: u8,
    pub toc_track_descriptors: Vec<TocTrackDescriptor<A>>,
}

#[derive_where(Debug, Clone, PartialEq, Eq, Hash; A::ResponseAddressType)]
pub struct TocTrackDescriptor<A: TrackStartAddress> {
    pub adr: u4,
    pub control: q_subcode::Control,
    pub first_track_number_last_complete_session: u8,
    pub first_track_in_last_session_start_address: A::ResponseAddressType,
}
