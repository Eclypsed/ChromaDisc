use core::fmt;

use arbitrary_int::u4;
use thiserror::Error;
use zerocopy::{
    BigEndian, CastError, FromBytes, Immutable, IntoBytes, KnownLayout, U16, Unaligned,
};

use super::{ReadTocPmaAtip, ReadTocPmaAtipOpCode};
use crate::{
    core::{ReadCommand, TruncationError, util::debug_fields},
    mmc::device_models::cd::q_subcode,
};

/// The length of the header that precedes the PMA Descriptors.
const PMA_HEADER_LEN: usize = 4;

/// Why a PMA response couldn't be parsed.
#[derive(Debug, Error)]
pub enum PmaError {
    /// The response is shorter than its header.
    #[error(transparent)]
    Truncated(#[from] TruncationError),
}

impl ReadCommand<ReadTocPmaAtipOpCode> for ReadTocPmaAtip<Pma> {
    type Len = u16;
    type Response<'a> = (&'a Pma, &'a [u8]);
    type Error = PmaError;

    fn response_len(&self) -> Self::Len {
        self.allocation_length
    }

    /// Reads the PMA Descriptors that fit in both PMA Data Length and `buf`, and returns the
    /// bytes after them.
    ///
    /// Only whole descriptors are read, so a descriptor cut short begins the returned bytes.
    /// `buf` should hold only the bytes the Drive transferred. Any bytes past the transfer that
    /// still fall within PMA Data Length, such as an allocated buffer's unused tail, are read as
    /// descriptors.
    fn parse<'a>(&self, buf: &'a [u8]) -> Result<Self::Response<'a>, Self::Error> {
        // PMA Data Length counts the bytes following its own 2-byte field. A `buf` too short to
        // hold it holds no descriptors either, so 0 stands in, and the cast reports the truncation.
        let pma_data_length = buf.first_chunk().map_or(0, |&len| u16::from_be_bytes(len));
        let elems = usize::from(pma_data_length.saturating_sub(2))
            .min(buf.len().saturating_sub(PMA_HEADER_LEN))
            / size_of::<PmaDescriptor>();

        match Pma::ref_from_prefix_with_elems(buf, elems) {
            Ok(res) => Ok(res),
            Err(CastError::Size(_)) => Err(TruncationError {
                expected: PMA_HEADER_LEN,
                received: buf.len(),
            }
            .into()),
            Err(CastError::Alignment(_)) => unreachable!("`Pma` should be `Unaligned`"),
        }
    }
}

/// The response to READ TOC/PMA/ATIP with Format 0011b: the Q sub-channel entries of the PMA.
#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, PartialEq, Eq, Hash)]
#[repr(C)]
pub struct Pma {
    pma_data_length: U16<BigEndian>,
    _reserved: [u8; 2],
    pma_descriptors: [PmaDescriptor],
}

impl Pma {
    /// The length of the response that follows this field.
    ///
    /// Not cut by Allocation Length, so it can exceed the bytes the Drive transferred.
    #[inline]
    pub const fn pma_data_length(&self) -> u16 {
        self.pma_data_length.get()
    }

    /// The PMA Descriptors, in the order they appear in the response.
    #[inline]
    pub const fn pma_descriptors(&self) -> &[PmaDescriptor] {
        &self.pma_descriptors
    }
}

debug_fields!(Pma {
    pma_data_length,
    pma_descriptors
});

/// One entry of a [`Pma`]: a Q sub-channel entry of the PMA.
///
/// What MIN through PFRAME hold depends on ADR and POINT, so they have no accessors yet. Their
/// bytes are readable through [`IntoBytes::as_bytes`].
// TODO: Decide how to read MIN through PFRAME, whose meaning depends on ADR and POINT.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct PmaDescriptor {
    _reserved: u8,
    adr_control: u8,
    tno: u8,
    point: u8,
    min: u8,
    sec: u8,
    frame: u8,
    // NOTE: mmc4r05a defines HOUR in bits 7-4 and PHOUR in bits 3-0 of this byte for DDCD media.
    zero: u8,
    pmin: u8,
    psec: u8,
    pframe: u8,
}

impl PmaDescriptor {
    super::q_sub_channel_fields!();
}

/// Prints the Q sub-channel bytes in hex, as the spec's tables write them, including those
/// without accessors.
impl fmt::Debug for PmaDescriptor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut s = f.debug_struct("PmaDescriptor");
        self.debug_q_sub_channel(&mut s);
        s.finish()
    }
}
