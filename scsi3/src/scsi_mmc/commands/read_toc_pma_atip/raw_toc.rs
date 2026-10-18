use core::fmt;

use arbitrary_int::u4;
use thiserror::Error;
use zerocopy::{
    BigEndian, CastError, FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned, U16,
};

use super::{ReadTocPmaAtip, ReadTocPmaAtipOpCode};
use crate::{
    core::{util::debug_fields, ReadCommand, TruncationError},
    mmc::device_models::cd::q_subcode,
};

/// The length of the header that precedes the TOC Track Descriptors.
const RAW_TOC_HEADER_LEN: usize = 4;

/// Why a Raw TOC response couldn't be parsed.
#[derive(Debug, Error)]
pub enum RawTocError {
    /// The response is shorter than its header.
    #[error(transparent)]
    Truncated(#[from] TruncationError<RAW_TOC_HEADER_LEN>),
}

impl ReadCommand<ReadTocPmaAtipOpCode> for ReadTocPmaAtip<RawToc> {
    type Len = u16;
    type Response<'a> = (&'a RawToc, &'a [u8]);
    type Error = RawTocError;

    fn response_len(&self) -> Self::Len {
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
            .min(buf.len().saturating_sub(RAW_TOC_HEADER_LEN))
            / size_of::<TocTrackDescriptor>();

        match RawToc::ref_from_prefix_with_elems(buf, elems) {
            Ok(res) => Ok(res),
            Err(CastError::Size(_)) => Err(TruncationError(buf.len()).into()),
            Err(CastError::Alignment(_)) => unreachable!("`RawToc` should be `Unaligned`"),
        }
    }
}

/// The response to READ TOC/PMA/ATIP with Format 0010b: the Q sub-channel entries of the Lead-in
/// TOC, from the requested session on.
#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct RawToc {
    toc_data_length: U16<BigEndian>,
    first_complete_session_number: u8,
    last_complete_session_number: u8,
    track_descriptors: [TocTrackDescriptor],
}

impl RawToc {
    /// The length of the response that follows this field.
    ///
    /// Not cut by Allocation Length, so it can exceed the bytes the Drive transferred.
    #[inline]
    pub const fn toc_data_length(&self) -> u16 {
        self.toc_data_length.get()
    }

    /// The number of the first complete session.
    #[inline]
    pub const fn first_complete_session_number(&self) -> u8 {
        self.first_complete_session_number
    }

    /// The number of the last complete session.
    #[inline]
    pub const fn last_complete_session_number(&self) -> u8 {
        self.last_complete_session_number
    }

    /// The TOC Track Descriptors, in the order they appear in the response.
    #[inline]
    pub const fn track_descriptors(&self) -> &[TocTrackDescriptor] {
        &self.track_descriptors
    }
}

debug_fields!(RawToc {
    toc_data_length,
    first_complete_session_number,
    last_complete_session_number,
    track_descriptors
});

/// One entry of a [`RawToc`]: a Q sub-channel entry of the Lead-in TOC, and the session it was
/// read from.
///
/// The Drive converts TNO, POINT, MIN, SEC, FRAME and ZERO to binary when the medium holds a
/// value from 00 to 99 in BCD.
///
/// What MIN through PFRAME hold depends on ADR and POINT, so they have no accessors yet. Their
/// bytes are readable through [`IntoBytes::as_bytes`].
// NOTE: mmc4r05a adds that other values are returned unmodified, and that for DDCD the Drive
// also converts bytes 7-10 (PHOUR, PMIN, PSEC, PFRAME). For a CD, neither revision says whether
// the Drive converts PMIN, PSEC and PFRAME, which the medium records in BCD. The ASUS DRW-24B1ST
// j (firmware 1.11) converts them to binary.
// TODO: Decide how to read MIN through PFRAME, whose meaning depends on ADR and POINT.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct TocTrackDescriptor {
    session_number: u8,
    adr_control: u8,
    tno: u8,
    point: u8,
    min: u8,
    sec: u8,
    frame: u8,
    // NOTE: mmc4r05a defines HOUR in bits 7-4 and PHOUR in bits 3-0 of this byte for DDCD media,
    // yet lists the byte for conversion to binary both as ZERO and as PHOUR. Converting it whole
    // would mix the two nibbles.
    zero: u8,
    pmin: u8,
    psec: u8,
    pframe: u8,
}

impl TocTrackDescriptor {
    /// The session this entry was read from.
    #[inline]
    pub const fn session_number(&self) -> u8 {
        self.session_number
    }

    super::q_sub_channel_fields!();
}

/// Prints the Q sub-channel bytes in hex, as the spec's tables write them, including those
/// without accessors.
impl fmt::Debug for TocTrackDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("TocTrackDescriptor");
        s.field("session_number", &self.session_number());
        self.debug_q_sub_channel(&mut s);
        s.finish()
    }
}
