use std::io::{Cursor, Read, Seek, SeekFrom};

use bytes::Bytes;
use deku::{ctx::Endian, deku_derive, reader::Reader, DekuError, DekuRead, DekuReader};

use super::AddressingMode;
use crate::{
    core::{addressing::Lba, Response},
    mmc::msf::Msf,
    rainbow_books::q_subcode,
};

mod private {
    use super::*;

    pub trait ReadAddress: Sized {
        fn read_track_start_address<R: Read + Seek>(
            reader: &mut Reader<R>,
        ) -> Result<Self, DekuError>;
    }
}

pub trait TrackStartAddress: private::ReadAddress + AddressingMode {}

impl private::ReadAddress for Msf {
    fn read_track_start_address<R: Read + Seek>(reader: &mut Reader<R>) -> Result<Self, DekuError> {
        reader.seek(SeekFrom::Current(1))?;
        Self::from_reader_with_ctx(reader, ())
    }
}
impl TrackStartAddress for Msf {}

impl private::ReadAddress for Lba {
    fn read_track_start_address<R: Read + Seek>(reader: &mut Reader<R>) -> Result<Self, DekuError> {
        Ok(i32::from_reader_with_ctx(reader, Endian::Big)?.into())
    }
}
impl TrackStartAddress for Lba {}

#[deku_derive(DekuRead)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FormattedToc<A: TrackStartAddress> {
    #[deku(temp, bytes = 2, endian = "big")]
    _toc_data_length: usize,

    pub first_track_number: u8,
    pub last_track_number: u8,

    #[deku(count = "_toc_data_length.saturating_sub(2) / 8")]
    pub toc_track_descriptors: Vec<TocTrackDescriptor<A>>,
}

impl<A: TrackStartAddress> Response for FormattedToc<A> {
    type Error = DekuError;

    fn from_bytes(bytes: Bytes) -> Result<Self, Self::Error> {
        Self::from_reader_with_ctx(&mut Reader::new(Cursor::new(bytes)), ())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead)]
pub struct TocTrackDescriptor<A: TrackStartAddress> {
    #[deku(pad_bytes_before = "1", bits = 4)]
    pub adr: u8,
    pub control: q_subcode::Control,

    #[deku(pad_bytes_after = "1")]
    pub track_number: u8,

    #[deku(bytes = 4, reader = "A::read_track_start_address(deku::reader)")]
    pub track_start_address: A,
}
