use std::io::Cursor;

use deku::{deku_derive, reader::Reader, DekuError, DekuRead, DekuReader};

use crate::scsi::mmc::commands::Response;

#[deku_derive(DekuRead)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CdText {
    #[deku(bytes = "2", temp, endian = "big", pad_bytes_after = "2")]
    _cd_text_data_length: usize,

    #[deku(count = "_cd_text_data_length.saturating_sub(2) / 18")]
    pub cd_text_descriptors: Vec<CdTextDescriptor>,
}

impl Response for CdText {
    type Error = DekuError;

    fn from_bytes(bytes: &[u8]) -> Result<Self, Self::Error> {
        Self::from_reader_with_ctx(&mut Reader::new(Cursor::new(bytes)), ())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, DekuRead)]
pub struct CdTextDescriptor {
    // TODO
    cd_text_data: [u8; 18],
}
