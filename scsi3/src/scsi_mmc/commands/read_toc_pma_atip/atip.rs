use core::fmt;

use thiserror::Error;
use zerocopy::{
    BigEndian, CastError, FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned, U16,
};

use super::{ReadTocPmaAtip, ReadTocPmaAtipOpCode};
use crate::core::{ReadCommand, TruncationError};

/// The length of the response, which ends after the A3 Values.
const ATIP_LEN: usize = 28;

/// Why an ATIP response couldn't be parsed.
#[derive(Debug, Error)]
pub enum AtipError {
    /// `buf` is shorter than the response's fixed length.
    #[error(transparent)]
    Truncated(#[from] TruncationError<ATIP_LEN>),
}

impl ReadCommand<ReadTocPmaAtipOpCode> for ReadTocPmaAtip<Atip> {
    type Len = u16;
    type Response<'a> = (&'a Atip, &'a [u8]);
    type Error = AtipError;

    fn response_len(&self) -> Self::Len {
        self.allocation_length
    }

    /// Reads the first 28 bytes of `buf` as the response, and returns the bytes after them.
    ///
    /// The response has a fixed length, so ATIP Data Length isn't used to find where it ends. The
    /// bytes of `buf` past byte 27 are returned, including the S4 Values when a Drive sends them.
    fn parse<'a>(&self, buf: &'a [u8]) -> Result<Self::Response<'a>, Self::Error> {
        match Atip::ref_from_prefix(buf) {
            Ok(res) => Ok(res),
            Err(CastError::Size(_)) => Err(TruncationError(buf.len()).into()),
            Err(CastError::Alignment(_)) => unreachable!("`Atip` should be `Unaligned`"),
        }
    }
}

/// The response to READ TOC/PMA/ATIP with Format 0100b: the ATIP of a CD-R or CD-RW, as read
/// from its Lead-in.
///
/// Each group of fields is returned as its three raw bytes.
///
/// The response ends after the A3 Values. MMC-6 also places S4 Values in bytes 28-30, which no
/// revision of MMC explains. When a Drive returns them, they begin the bytes after the response.
// NOTE: MMC-6 lays out 32 bytes, but this layout stops at byte 27, so `size_of::<Atip>()` is 28.
// mmc3r02 added bytes 28-31 without saying what S4 holds or that the response grew. It's also the
// first revision to describe DDCD media in this response, and mmc4r05a says S4 is available for
// both DDCD-R and DDCD-RW. The ASUS DRW-24B1ST j (firmware 1.11) returns an ATIP Data Length of
// 26, ending the response at byte 27.
//
// NOTE: MMC-6 doesn't say how the two start times are encoded. Earlier revisions say they're
// returned in "hex format", meaning binary, and mmc4r05a says that for DDCD media they're binary
// and not in the HMSF format. The ASUS DRW-24B1ST j (firmware 1.11) returns them in binary.
// TODO: Decode the fields of each group, whose meaning depends on Disc Type and Disc Sub-Type.
#[derive(
    FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned, Clone, Copy, PartialEq, Eq, Hash,
)]
#[repr(C)]
pub struct Atip {
    atip_data_length: U16<BigEndian>,
    _bytes2_3: [u8; 2],
    // NOTE: MMC defines only bit 6 of the second byte, as Unrestricted Use Disc (URU), and reserves
    // bits 5-0. The Orange Book defines bits 6-0 as the Disc Application Code.
    special_information_1: [u8; 3],
    _byte7: u8,
    start_time_of_lead_in: [u8; 3],
    _byte11: u8,
    last_possible_start_time_of_lead_out: [u8; 3],
    _byte15: u8,
    a1_values: [u8; 3],
    _byte19: u8,
    a2_values: [u8; 3],
    _byte23: u8,
    a3_values: [u8; 3],
    _byte27: u8,
}

impl Atip {
    /// The length of the response that follows this field.
    #[inline]
    pub const fn atip_data_length(&self) -> u16 {
        self.atip_data_length.get()
    }

    /// The fields the Orange Book calls Special Information 1: Indicative Target Writing Power,
    /// Reference Speed, Unrestricted Use Disc, Disc Type, Disc Sub-Type, and A1, A2 and A3 Valid.
    #[inline]
    pub const fn special_information_1(&self) -> &[u8; 3] {
        &self.special_information_1
    }

    /// The ATIP Start Time of Lead-in, as Min, Sec and Frame in binary.
    #[inline]
    pub const fn start_time_of_lead_in(&self) -> &[u8; 3] {
        &self.start_time_of_lead_in
    }

    /// The ATIP Last Possible Start Time of Lead-out, as Min, Sec and Frame in binary.
    #[inline]
    pub const fn last_possible_start_time_of_lead_out(&self) -> &[u8; 3] {
        &self.last_possible_start_time_of_lead_out
    }

    /// The A1 Values, which are valid only when A1 Valid is set.
    #[inline]
    pub const fn a1_values(&self) -> &[u8; 3] {
        &self.a1_values
    }

    /// The A2 Values, which are valid only when A2 Valid is set.
    #[inline]
    pub const fn a2_values(&self) -> &[u8; 3] {
        &self.a2_values
    }

    /// The A3 Values, which are valid only when A3 Valid is set.
    #[inline]
    pub const fn a3_values(&self) -> &[u8; 3] {
        &self.a3_values
    }
}

/// Prints each group of fields as its three bytes in hex, until the fields are decoded.
impl fmt::Debug for Atip {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let hex = |&[a, b, c]: &[u8; 3]| {
            fmt::from_fn(move |f| write!(f, "[{a:02X}h, {b:02X}h, {c:02X}h]"))
        };

        f.debug_struct("Atip")
            .field("atip_data_length", &self.atip_data_length())
            .field("special_information_1", &hex(self.special_information_1()))
            .field("start_time_of_lead_in", &hex(self.start_time_of_lead_in()))
            .field(
                "last_possible_start_time_of_lead_out",
                &hex(self.last_possible_start_time_of_lead_out()),
            )
            .field("a1_values", &hex(self.a1_values()))
            .field("a2_values", &hex(self.a2_values()))
            .field("a3_values", &hex(self.a3_values()))
            .finish()
    }
}
