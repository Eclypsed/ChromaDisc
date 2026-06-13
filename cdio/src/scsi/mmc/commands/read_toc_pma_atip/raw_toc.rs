use std::io::Cursor;

use crate::rainbow_books::q_subcode::Control;
use deku::{deku_derive, reader::Reader, DekuError, DekuRead, DekuReader};

use crate::scsi::mmc::commands::Response;

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
