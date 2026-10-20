use core::fmt;

use arbitrary_int::u4;
use thiserror::Error;
use zerocopy::{
    BigEndian, CastError, FromBytes, Immutable, IntoBytes, KnownLayout, U16, Unaligned,
};

use super::{ReadTocPmaAtip, ReadTocPmaAtipOpCode, TrackStartAddress};
use crate::{
    core::{ReadCommand, TruncationError},
    mmc::device_models::cd::q_subcode,
};

/// The length of the header that precedes the TOC Track Descriptors.
const FORMATTED_TOC_HEADER_LEN: usize = 4;

/// Why a Formatted TOC response couldn't be parsed.
#[derive(Debug, Error)]
pub enum FormattedTocError {
    /// The response is shorter than its header.
    #[error(transparent)]
    Truncated(#[from] TruncationError),
}

impl<A: TrackStartAddress> ReadCommand<ReadTocPmaAtipOpCode> for ReadTocPmaAtip<FormattedToc<A>> {
    type Len = u16;
    type Response<'a> = (&'a FormattedToc<A>, &'a [u8]);
    type Error = FormattedTocError;

    fn response_len(&self) -> u16 {
        self.allocation_length
    }

    /// Reads the TOC Track Descriptors that fit in both TOC Data Length and `buf`, and returns
    /// the bytes after them.
    ///
    /// Only whole descriptors are read, so a descriptor cut short begins the returned bytes.
    /// `buf` should hold only the bytes the Drive transferred. Any bytes past the transfer that
    /// still fall within TOC Data Length, such as an allocated buffer's unused tail, are read as
    /// descriptors.
    fn parse<'a>(&self, buf: &'a [u8]) -> Result<Self::Response<'a>, Self::Error> {
        // TOC Data Length counts the bytes following its own 2-byte field. A `buf` too short to
        // hold it holds no descriptors either, so 0 stands in, and the cast reports the truncation.
        let toc_data_length = buf.first_chunk().map_or(0, |&len| u16::from_be_bytes(len));
        let elems = usize::from(toc_data_length.saturating_sub(2))
            .min(buf.len().saturating_sub(FORMATTED_TOC_HEADER_LEN))
            / size_of::<TocTrackDescriptor<A>>();

        match FormattedToc::<A>::ref_from_prefix_with_elems(buf, elems) {
            Ok(res) => Ok(res),
            Err(CastError::Size(_)) => Err(TruncationError {
                expected: FORMATTED_TOC_HEADER_LEN,
                received: buf.len(),
            }
            .into()),
            Err(CastError::Alignment(_)) => unreachable!("`FormattedToc` should be `Unaligned`"),
        }
    }
}

/// The response to READ TOC/PMA/ATIP with Format 0000b: a TOC Track Descriptor for each track,
/// then one for the Lead-out.
#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct FormattedToc<A: TrackStartAddress> {
    toc_data_length: U16<BigEndian>,
    first_track_number: u8,
    last_track_number: u8,
    track_descriptors: [TocTrackDescriptor<A>],
}

impl<A: TrackStartAddress> FormattedToc<A> {
    /// The length of the response that follows this field.
    ///
    /// Not cut by Allocation Length, so it can exceed the bytes the Drive transferred.
    pub const fn toc_data_length(&self) -> u16 {
        self.toc_data_length.get()
    }

    /// The number of the first track in the first complete session.
    pub const fn first_track_number(&self) -> u8 {
        self.first_track_number
    }

    /// The number of the last track in the last complete session.
    pub const fn last_track_number(&self) -> u8 {
        self.last_track_number
    }

    /// The TOC Track Descriptors, in the order they appear in the response.
    ///
    /// The last one is the Lead-out's, unless the response was cut short.
    pub const fn track_descriptors(&self) -> &[TocTrackDescriptor<A>] {
        &self.track_descriptors
    }
}

impl<A: TrackStartAddress> fmt::Debug for FormattedToc<A>
where
    TocTrackDescriptor<A>: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormattedToc")
            .field("toc_data_length", &self.toc_data_length())
            .field("first_track_number", &self.first_track_number())
            .field("last_track_number", &self.last_track_number())
            .field("track_descriptors", &self.track_descriptors())
            .finish()
    }
}

/// One entry of a [`FormattedToc`]: where a track, or the Lead-out, starts.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct TocTrackDescriptor<A: TrackStartAddress> {
    _reserved_1: u8,
    adr_control: u8,
    track_number: u8,
    _reserved_2: u8,
    track_start_address: A,
}

impl<A: TrackStartAddress + fmt::Debug> fmt::Debug for TocTrackDescriptor<A> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TocTrackDescriptor")
            .field("adr", &self.adr())
            .field("control", &self.control())
            .field("track_number", &self.track_number())
            .field("track_start_address", &self.track_start_address())
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

    /// The number of the track, or AAh for the Lead-out.
    pub const fn track_number(&self) -> u8 {
        self.track_number
    }

    /// The address of the track's first block, or of the Lead-out's.
    pub const fn track_start_address(&self) -> A {
        self.track_start_address
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Control, addressing::Lba};

    fn command() -> ReadTocPmaAtip<FormattedToc<Lba>> {
        ReadTocPmaAtip::formatted_toc(0, 0xFFFF, Control::default())
    }

    #[test]
    fn parse_cuts_at_toc_data_length() {
        let buf = [
            0, 18, 1, 1, // TOC Data Length, First and Last Track Number
            0, 0x14, 1, 0, 0, 0, 0, 0, // Track 1 at LBA 0
            0, 0x10, 0xAA, 0, 0, 0, 0x10, 0, // Lead-out at LBA 4096
            0xEE, 0xEE,
        ];
        let (toc, rest) = command().parse(&buf).unwrap();
        assert_eq!(toc.track_descriptors().len(), 2);
        assert_eq!(toc.track_descriptors()[1].track_number(), 0xAA);
        assert_eq!(
            toc.track_descriptors()[1].track_start_address(),
            Lba::from(4096)
        );
        assert_eq!(rest, [0xEE, 0xEE]);

        // Cut partway through the lead-out descriptor
        let (toc, rest) = command().parse(&buf[..16]).unwrap();
        assert_eq!(toc.track_descriptors().len(), 1);
        assert_eq!(rest, &buf[12..16]);
    }

    #[test]
    fn truncated_header_is_an_error() {
        assert!(matches!(
            command().parse(&[0; 3]),
            Err(FormattedTocError::Truncated(TruncationError {
                expected: 4,
                received: 3
            }))
        ));
    }
}
