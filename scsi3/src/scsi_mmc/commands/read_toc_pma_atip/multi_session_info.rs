use core::fmt;

use arbitrary_int::u4;
use thiserror::Error;
use zerocopy::{
    BigEndian, CastError, FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned, U16,
};

use super::{ReadTocPmaAtip, ReadTocPmaAtipOpCode, TrackStartAddress};
use crate::{
    core::{ReadCommand, TruncationError},
    mmc::device_models::cd::q_subcode,
};

/// The length of the response: the header plus its one TOC Track Descriptor.
const MULTI_SESSION_INFORMATION_LEN: usize = 12;

/// Why a Multi-session Information response couldn't be parsed.
#[derive(Debug, Error)]
pub enum MultiSessionInformationError {
    /// The response is shorter than its fixed length.
    #[error(transparent)]
    Truncated(#[from] TruncationError<MULTI_SESSION_INFORMATION_LEN>),
}

impl<A: TrackStartAddress> ReadCommand<ReadTocPmaAtipOpCode>
    for ReadTocPmaAtip<MultiSessionInformation<A>>
{
    type Len = u16;
    type Response<'a> = (&'a MultiSessionInformation<A>, &'a [u8]);
    type Error = MultiSessionInformationError;

    fn response_len(&self) -> Self::Len {
        self.allocation_length
    }

    /// Reads the first 12 bytes of `buf` as the response, and returns the bytes after them.
    ///
    /// The response always holds exactly one TOC Track Descriptor, so TOC Data Length isn't used
    /// to find where it ends.
    fn parse<'a>(&self, buf: &'a [u8]) -> Result<Self::Response<'a>, Self::Error> {
        match MultiSessionInformation::<A>::ref_from_prefix(buf) {
            Ok(res) => Ok(res),
            Err(CastError::Size(_)) => Err(TruncationError(buf.len()).into()),
            Err(CastError::Alignment(_)) => {
                unreachable!("`MultiSessionInformation` should be `Unaligned`")
            }
        }
    }
}

/// The response to READ TOC/PMA/ATIP with Format 0001b: the first and last complete sessions, and
/// where the first track in the last complete session starts.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct MultiSessionInformation<A: TrackStartAddress> {
    toc_data_length: U16<BigEndian>,
    first_complete_session_number: u8,
    last_complete_session_number: u8,
    track_descriptor: TocTrackDescriptor<A>,
}

impl<A: TrackStartAddress> MultiSessionInformation<A> {
    /// The length of the response that follows this field, which the spec fixes at 0Ah.
    pub const fn toc_data_length(&self) -> u16 {
        self.toc_data_length.get()
    }

    /// The number of the first complete session.
    pub const fn first_complete_session_number(&self) -> u8 {
        self.first_complete_session_number
    }

    /// The number of the last complete session.
    pub const fn last_complete_session_number(&self) -> u8 {
        self.last_complete_session_number
    }

    /// The TOC Track Descriptor of the first track in the last complete session.
    pub const fn track_descriptor(&self) -> &TocTrackDescriptor<A> {
        &self.track_descriptor
    }
}

impl<A: TrackStartAddress> fmt::Debug for MultiSessionInformation<A>
where
    TocTrackDescriptor<A>: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("MultiSessionInformation")
            .field("toc_data_length", &self.toc_data_length())
            .field(
                "first_complete_session_number",
                &self.first_complete_session_number(),
            )
            .field(
                "last_complete_session_number",
                &self.last_complete_session_number(),
            )
            .field("track_descriptor", self.track_descriptor())
            .finish()
    }
}

/// The TOC Track Descriptor of a [`MultiSessionInformation`]: where the first track in the last
/// complete session starts.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct TocTrackDescriptor<A: TrackStartAddress> {
    _reserved_1: u8,
    adr_control: u8,
    first_track_number_in_last_complete_session: u8,
    _reserved_2: u8,
    start_address_of_first_track_in_last_session: A,
}

impl<A: TrackStartAddress + fmt::Debug> fmt::Debug for TocTrackDescriptor<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TocTrackDescriptor")
            .field("adr", &self.adr())
            .field("control", &self.control())
            .field(
                "first_track_number_in_last_complete_session",
                &self.first_track_number_in_last_complete_session(),
            )
            .field(
                "start_address_of_first_track_in_last_session",
                &self.start_address_of_first_track_in_last_session(),
            )
            .finish()
    }
}

impl<A: TrackStartAddress> TocTrackDescriptor<A> {
    /// The type of information in the Q sub-channel of the block this entry was found in.
    pub const fn adr(&self) -> u4 {
        u4::extract_u8(self.adr_control, 4)
    }

    /// The attributes of the track, as its Q sub-channel reports them.
    pub const fn control(&self) -> q_subcode::Control {
        q_subcode::Control::from_bits_truncate(self.adr_control)
    }

    /// The number of the first track in the last complete session.
    pub const fn first_track_number_in_last_complete_session(&self) -> u8 {
        self.first_track_number_in_last_complete_session
    }

    /// The address of the first block of the first track in the last complete session.
    pub const fn start_address_of_first_track_in_last_session(&self) -> A {
        self.start_address_of_first_track_in_last_session
    }
}
