use core::marker::PhantomData;

use alloc::vec::Vec;
use arbitrary_int::u4;
use derive_where::derive_where;
use thiserror::Error;
use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

use super::{ReadTocPmaAtip, ReadTocPmaAtipOpCode};
use crate::{
    core::{addressing::Lba, ReadCommand, TruncationError},
    mmc::device_models::cd::{
        addressing::{Msf, UnvalidatedMsf},
        q_subcode,
    },
};

const FORMATTED_TOC_MIN_BYTES: usize = 4;
const TRACK_DESCRIPTOR_SIZE: usize = 8;

mod private {
    use super::*;

    pub trait AddressingMode {
        const MSF: bool;
        type ResponseAddressType: FromBytes + IntoBytes + KnownLayout + Immutable;
        fn read_track_start_address(bytes: [u8; 4]) -> Self::ResponseAddressType;
    }
}

pub trait TrackStartAddress: private::AddressingMode {}

impl private::AddressingMode for Lba {
    const MSF: bool = false;
    type ResponseAddressType = Lba;
    fn read_track_start_address(bytes: [u8; 4]) -> Self::ResponseAddressType {
        i32::from_be_bytes(bytes).into()
    }
}
impl TrackStartAddress for Lba {}

impl private::AddressingMode for Msf {
    const MSF: bool = true;
    // CONFIRM: My drive returns the addresses as binary, but I don't think the spec specifies
    // if it should be BCD or binary.
    type ResponseAddressType = UnvalidatedMsf;
    fn read_track_start_address(bytes: [u8; 4]) -> Self::ResponseAddressType {
        UnvalidatedMsf::new(bytes[1], bytes[2], bytes[3])
    }
}
impl TrackStartAddress for Msf {}

#[derive(Debug, Error)]
pub enum FormattedTocError {
    #[error(transparent)]
    Truncated(#[from] TruncationError<FORMATTED_TOC_MIN_BYTES>),
}

impl<A: TrackStartAddress> ReadCommand<ReadTocPmaAtipOpCode> for ReadTocPmaAtip<FormattedToc<A>> {
    type Len = u16;
    type Response<'a> = FormattedToc<A>;
    type Error = FormattedTocError;

    fn response_len(&self) -> u16 {
        self.allocation_length
    }

    fn parse<'a>(&self, buf: &'a [u8]) -> Result<Self::Response<'a>, Self::Error> {
        if buf.len() < FORMATTED_TOC_MIN_BYTES {
            return Err(TruncationError(buf.len()).into());
        }

        let toc_data_length: usize = u16::from_be_bytes([buf[0], buf[1]]).into();
        let first_track_number = buf[2];
        let last_track_number = buf[3];

        let max_bytes = buf.len().min(toc_data_length - 2);
        let desc_bytes = buf.get(4..max_bytes).unwrap_or_default();

        let toc_track_descriptors = desc_bytes
            .chunks_exact(TRACK_DESCRIPTOR_SIZE)
            .map(|chunk| TocTrackDescriptor {
                adr: u4::extract_u8(chunk[1], 4),
                control: q_subcode::Control::from_bits_truncate(chunk[1] & 0xF),
                track_number: chunk[2],
                track_start_address: A::read_track_start_address(chunk[4..=7].try_into().unwrap()),
            })
            .collect::<Vec<_>>();

        Ok(FormattedToc {
            first_track_number,
            last_track_number,
            toc_track_descriptors,
        })
    }
}

#[derive_where(Debug, Clone, PartialEq, Eq, Hash; A::ResponseAddressType)]
pub struct FormattedToc<A: TrackStartAddress> {
    pub first_track_number: u8,
    pub last_track_number: u8,
    pub toc_track_descriptors: Vec<TocTrackDescriptor<A>>,
}

#[derive_where(Debug, Clone, PartialEq, Eq, Hash; A::ResponseAddressType)]
pub struct TocTrackDescriptor<A: TrackStartAddress> {
    pub adr: u4,
    pub control: q_subcode::Control,
    pub track_number: u8,
    pub track_start_address: A::ResponseAddressType,
}
