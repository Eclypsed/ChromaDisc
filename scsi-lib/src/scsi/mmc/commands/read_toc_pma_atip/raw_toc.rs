use std::io::Cursor;

use bcd::Bcd;
use deku::{deku_derive, reader::Reader, DekuError, DekuRead, DekuReader};
use rainbow_books::q_subcode::Control;

use crate::scsi::mmc::commands::Response;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ProgramAreaFormat {
    CddaOrCdrom = 0x00,
    Cdi = 0x10,
    CdromXa = 0x20,
}

#[deku_derive(DekuRead)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct RawToc {
    #[deku(temp, bytes = "2", endian = "big")]
    _toc_data_length: usize,

    pub first_complete_session_number: u8,
    pub last_complete_session_number: u8,

    #[deku(count = "_toc_data_length.saturating_sub(2) / 11")]
    pub toc_track_descriptors: Vec<TempTocTrackDescriptor>,
}

impl Response for RawToc {
    type Error = DekuError;

    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error> {
        Self::from_reader_with_ctx(&mut Reader::new(Cursor::new(bytes)), ())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead)]
pub struct TempTocTrackDescriptor {
    session_number: u8,
    #[deku(bits = 4)]
    adr: u8,
    control: Control,
    tno: u8,
    point: u8,
    min: u8,
    sec: u8,
    frame: u8,
    zero: u8,
    pmin: u8,
    psec: u8,
    pframe: u8,
}

pub struct RawTocTrackDescriptor {
    session_numer: u8,
    adr: u8,
    control: Control,
    data_q: [u8; 9],
}

// fn parse_raw_toc_proto<R: deku::no_std_io::Read + deku::no_std_io::Seek>(
//     reader: &mut Reader<R>,
// ) -> Result<TocTrackDescriptor, DekuError> {
//     const CTRL4: q_subcode::Control = q_subcode::Control::from_bits_truncate(4);
//     const CTRL6: q_subcode::Control = q_subcode::Control::from_bits_truncate(6);

//     let session_num = u8::from_reader_with_ctx(reader, ())?;
//     let adr = u8::from_reader_with_ctx(reader, BitSize(4))?;
//     let control = q_subcode::Control::from_reader_with_ctx(reader, ())?;
//     let tno = u8::from_reader_with_ctx(reader, ())?;
//     let point = u8::from_reader_with_ctx(reader, ())?;
//     let min = u8::from_reader_with_ctx(reader, ())?;
//     let sec = u8::from_reader_with_ctx(reader, ())?;
//     let frame = u8::from_reader_with_ctx(reader, ())?;
//     let zero = u8::from_reader_with_ctx(reader, ())?;
//     let pmin = u8::from_reader_with_ctx(reader, ())?;
//     let psec = u8::from_reader_with_ctx(reader, ())?;
//     let pframe = u8::from_reader_with_ctx(reader, ())?;

//     match (control, adr, tno, point) {
//         (CTRL4 | CTRL6, 0b0001, 0x00, 0x01..=0x63) => todo!(),
//         (CTRL4 | CTRL6, 0b0001, 0x00, 0xA0) => todo!(),
//         (CTRL4 | CTRL6, 0b0001, 0x00, 0xA1) => todo!(),
//         (CTRL4 | CTRL6, 0b0001, 0x00, 0xA2) => todo!(),
//         (CTRL4 | CTRL6, 0b0101, 0x00, 0x01..=0x40) => todo!(),
//         (CTRL4 | CTRL6, 0b0101, 0x00, 0xB0) => todo!(),
//         (CTRL4 | CTRL6, 0b0101, 0x00, 0xB1) => todo!(),
//         (CTRL4 | CTRL6, 0b0101, 0x00, 0xB2..=0xB4) => todo!(),
//         (CTRL4 | CTRL6, 0b0101, 0x00, 0xC0) => todo!(),
//         (CTRL4 | CTRL6, 0b0101, 0x00, 0xC1) => todo!(),
//         _ => Err(deku_error!(
//             DekuError::Parse,
//             "Unknown descriptor type",
//             "CTRL={}, ADR={}, POINT={:X}h",
//             control,
//             adr,
//             point
//         )),
//     }
// }
