use std::io::Cursor;

use deku::{deku_derive, reader::Reader, DekuError, DekuRead, DekuReader};

use crate::scsi::mmc::commands::Response;

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

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead)]
pub struct PmaDescriptor {
    // TODO
}
