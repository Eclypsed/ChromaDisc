use core::fmt;

use arbitrary_int::u4;
use derive_where::derive_where;
use thiserror::Error;
use zerocopy::{
    BigEndian, CastError, FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned, I32, U16,
};

use super::{ReadTocPmaAtip, ReadTocPmaAtipOpCode};
use crate::{
    core::{addressing::Lba, ReadCommand, TruncationError},
    mmc::device_models::cd::{
        addressing::{Msf, UnvalidatedMsf},
        q_subcode,
    },
};

const FORMATTED_TOC_MIN_BYTES: usize = 4;

mod private {
    use super::*;

    pub trait AddressingMode {
        const MSF: bool;
        type ResponseAddressType: FromBytes + IntoBytes + KnownLayout + Immutable + Unaligned;
    }
}

pub trait TrackStartAddress: private::AddressingMode + 'static {}

impl private::AddressingMode for Lba {
    const MSF: bool = false;
    type ResponseAddressType = I32<BigEndian>;
}
impl TrackStartAddress for Lba {}

impl private::AddressingMode for Msf {
    const MSF: bool = true;
    // TODO Add a properly padded type here
    // CONFIRM: My drive returns the addresses as binary, but I don't think the spec specifies
    // if it should be BCD or binary.
    type ResponseAddressType = [u8; 4];
}
impl TrackStartAddress for Msf {}

#[derive(Debug, Error)]
pub enum FormattedTocError {
    #[error(transparent)]
    Truncated(#[from] TruncationError<FORMATTED_TOC_MIN_BYTES>),
}

impl<A: TrackStartAddress> ReadCommand<ReadTocPmaAtipOpCode> for ReadTocPmaAtip<FormattedToc<A>> {
    type Len = u16;
    type Response<'a> = (&'a FormattedToc<A>, &'a [u8]);
    type Error = FormattedTocError;

    fn response_len(&self) -> u16 {
        self.allocation_length
    }

    fn parse<'a>(&self, buf: &'a [u8]) -> Result<Self::Response<'a>, Self::Error> {
        let buf_len = buf.len();

        let Some(max_num_data_bytes) = buf_len.checked_sub(FORMATTED_TOC_MIN_BYTES) else {
            return Err(TruncationError(buf_len).into());
        };

        // TOC Data Length counts the bytes following its own 2-byte field
        let toc_data_length = u16::from_be_bytes([buf[0], buf[1]]);
        let elems = usize::from(toc_data_length.saturating_sub(2)).min(max_num_data_bytes)
            / size_of::<TocTrackDescriptor<A>>();

        match FormattedToc::<A>::ref_from_prefix_with_elems(buf, elems) {
            Ok(res) => Ok(res),
            Err(CastError::Size(_)) => {
                unreachable!("`buf` should hold the header plus `elems` descriptors")
            }
            Err(CastError::Alignment(_)) => unreachable!("`FormattedToc` should be `Unaligned`"),
        }
    }
}

#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned)]
#[derive_where(PartialEq, Eq, Hash; A::ResponseAddressType)]
#[repr(C)]
pub struct FormattedToc<A: TrackStartAddress> {
    data_length: U16<BigEndian>,
    pub first_track_number: u8,
    pub last_track_number: u8,
    pub track_descriptors: [TocTrackDescriptor<A>],
}

impl<A: TrackStartAddress> fmt::Debug for FormattedToc<A>
where
    TocTrackDescriptor<A>: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("FormattedToc")
            .field("data_length", &self.data_length.get())
            .field("first_track_number", &self.first_track_number)
            .field("last_track_number", &self.last_track_number)
            .field("track_descriptors", &&self.track_descriptors)
            .finish()
    }
}

#[derive(FromBytes, IntoBytes, KnownLayout, Immutable, Unaligned)]
#[derive_where(Debug, Clone, Copy, PartialEq, Eq, Hash; A::ResponseAddressType)]
#[repr(C)]
pub struct TocTrackDescriptor<A: TrackStartAddress> {
    _reserved_1: u8,
    adr_control: u8,
    pub track_number: u8,
    _reserved_2: u8,
    track_start_address: A::ResponseAddressType,
}

impl<A: TrackStartAddress> TocTrackDescriptor<A> {
    #[inline]
    pub const fn adr(&self) -> u4 {
        u4::extract_u8(self.adr_control, 4)
    }

    #[inline]
    pub const fn control(&self) -> q_subcode::Control {
        q_subcode::Control::from_bits_truncate(self.adr_control)
    }
}

impl TocTrackDescriptor<Lba> {
    #[inline]
    pub fn track_start_address(&self) -> Lba {
        self.track_start_address.get().into()
    }
}

impl TocTrackDescriptor<Msf> {
    #[inline]
    pub const fn track_start_address(&self) -> UnvalidatedMsf {
        UnvalidatedMsf::new(
            self.track_start_address[1],
            self.track_start_address[2],
            self.track_start_address[3],
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::Control;

    fn command() -> ReadTocPmaAtip<FormattedToc<Lba>> {
        ReadTocPmaAtip::<FormattedToc<Lba>>::new(0, 0xFFFF, Control::default())
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
        assert_eq!(toc.track_descriptors.len(), 2);
        assert_eq!(toc.track_descriptors[1].track_number, 0xAA);
        assert_eq!(
            toc.track_descriptors[1].track_start_address(),
            Lba::from(4096)
        );
        assert_eq!(rest, [0xEE, 0xEE]);

        // Cut partway through the lead-out descriptor
        let (toc, rest) = command().parse(&buf[..16]).unwrap();
        assert_eq!(toc.track_descriptors.len(), 1);
        assert_eq!(rest, &buf[12..16]);
    }

    #[test]
    fn truncated_header_is_an_error() {
        assert!(matches!(
            command().parse(&[0; 3]),
            Err(FormattedTocError::Truncated(TruncationError(3)))
        ));
    }
}
