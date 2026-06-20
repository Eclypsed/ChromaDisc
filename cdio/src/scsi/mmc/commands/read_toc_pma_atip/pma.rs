use std::io::Cursor;

use deku::{deku_derive, reader::Reader, DekuError, DekuReader};

use crate::{rainbow_books::q_subcode::Control, scsi::mmc::commands::Response};

#[deku_derive(DekuRead)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pma {
    #[deku(temp, bytes = "2", endian = "big", pad_bytes_after = "2")]
    _pma_data_length: usize,

    #[deku(count = "_pma_data_length.saturating_sub(2) / 11")]
    pub toc_track_descriptors: Vec<PmaDescriptor>,
}

impl Response for Pma {
    type Error = DekuError;

    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error> {
        Self::from_reader_with_ctx(&mut Reader::new(Cursor::new(bytes)), ())
    }
}

#[deku_derive(DekuRead)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PmaDescriptor {
    #[deku(pad_bytes_before = "1", bits = 4)]
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
