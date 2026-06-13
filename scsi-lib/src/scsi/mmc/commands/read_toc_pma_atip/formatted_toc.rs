use std::io::{Cursor, Seek, SeekFrom};

use crate::core::msf::Msf;
use deku::{ctx::Endian, deku_derive, reader::Reader, DekuError, DekuRead, DekuReader};

use super::AddressingMode;
use crate::rainbow_books::q_subcode;
use crate::{core::addressing::Lba, scsi::mmc::commands::Response};

mod sealed {
    pub trait Sealed {}
}

pub trait TrackStartAddress: sealed::Sealed + AddressingMode + Sized {
    fn read_track_start_address<R: deku::no_std_io::Read + deku::no_std_io::Seek>(
        reader: &mut Reader<R>,
    ) -> Result<Self, DekuError>;
}

impl sealed::Sealed for Msf {}
impl TrackStartAddress for Msf {
    fn read_track_start_address<R: deku::no_std_io::Read + deku::no_std_io::Seek>(
        reader: &mut Reader<R>,
    ) -> Result<Self, DekuError> {
        reader.seek(SeekFrom::Current(1))?;
        Msf::from_reader_with_ctx(reader, ())
    }
}

impl sealed::Sealed for Lba {}
impl TrackStartAddress for Lba {
    fn read_track_start_address<R: deku::no_std_io::Read + deku::no_std_io::Seek>(
        reader: &mut Reader<R>,
    ) -> Result<Self, DekuError> {
        Ok(Self::from(i32::from_reader_with_ctx(reader, Endian::Big)?))
    }
}

#[deku_derive(DekuRead)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct FormattedToc<A: TrackStartAddress> {
    #[deku(temp, bytes = "2", endian = "big")]
    _toc_data_length: usize,

    pub first_track_number: u8,
    pub last_track_number: u8,

    #[deku(count = "_toc_data_length.saturating_sub(2) / 8")]
    pub toc_track_descriptors: Vec<TocTrackDescriptor<A>>,
}

impl<A: TrackStartAddress> Response for FormattedToc<A> {
    type Error = DekuError;

    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error> {
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
