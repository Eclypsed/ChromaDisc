use std::io::Cursor;

use deku::{ctx::Endian, deku_derive, reader::Reader, DekuError, DekuRead, DekuReader};
use rainbow_books::core::RawMsf;

use super::AddressingMode;
use crate::{core::addressing::Lba, scsi::mmc::commands::Response};
use rainbow_books::red_book::q_subcode;

mod sealed {
    pub trait Sealed {}
}

pub trait TrackStartAddress: sealed::Sealed + AddressingMode + Sized {
    fn read_track_start_address<R: deku::no_std_io::Read + deku::no_std_io::Seek>(
        reader: &mut Reader<R>,
    ) -> Result<Self, DekuError>;
}

impl sealed::Sealed for RawMsf {}
impl TrackStartAddress for RawMsf {
    fn read_track_start_address<R: deku::no_std_io::Read + deku::no_std_io::Seek>(
        reader: &mut Reader<R>,
    ) -> Result<Self, DekuError> {
        let bytes = <[u8; 4]>::from_reader_with_ctx(reader, Endian::Big)?;
        Ok(Self::new(bytes[1], bytes[2], bytes[3]))
    }
}

impl sealed::Sealed for Lba {}
impl TrackStartAddress for Lba {
    fn read_track_start_address<R: deku::no_std_io::Read + deku::no_std_io::Seek>(
        reader: &mut Reader<R>,
    ) -> Result<Self, DekuError> {
        let addr = i32::from_reader_with_ctx(reader, Endian::Big)?;
        Ok(Self::from(addr))
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

#[cfg(test)]
mod tests {
    use std::io::Cursor;

    use deku::{reader::Reader, DekuReader};
    use rainbow_books::core::RawMsf;

    use crate::scsi::mmc::commands::read_toc_pma_atip::formatted_toc::{
        FormattedToc, TocTrackDescriptor,
    };
    use rainbow_books::red_book::q_subcode;

    #[test]
    fn parse_formatted_toc_response() {
        let data: &[u8] = &[
            0b0000_0000,
            0b0000_1010,
            0b0000_0001,
            0b0000_1000,
            // Track Descriptor 1
            0b0000_0000,
            0b0010_0100,
            0b0000_0010,
            0b0000_0000,
            0b0000_0000,
            0b0000_0100,
            0b0000_1011,
            0b0010_1001,
        ];

        let mut reader = Reader::new(Cursor::new(data));
        let val = FormattedToc::<RawMsf>::from_reader_with_ctx(&mut reader, ()).unwrap();

        assert_eq!(
            FormattedToc {
                first_track_number: 1,
                last_track_number: 8,
                toc_track_descriptors: vec![TocTrackDescriptor {
                    adr: 0b0010,
                    control: q_subcode::Control::IS_DATA,
                    track_number: 2,
                    track_start_address: RawMsf::new(0b0000_0100, 0b0000_1011, 0b0010_1001)
                },]
            },
            val
        );
    }
}
