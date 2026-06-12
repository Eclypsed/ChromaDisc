use std::io::Cursor;

use deku::{deku_derive, reader::Reader, DekuError, DekuRead, DekuReader};

use crate::scsi::mmc::commands::Response;
use rainbow_books::q_subcode;

pub use super::formatted_toc::TrackStartAddress;

#[deku_derive(DekuRead)]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct MultiSessionInformation<A: TrackStartAddress> {
    #[deku(temp, bytes = "2", endian = "big")]
    _toc_data_length: usize,

    pub first_complete_session_number: u8,
    pub last_complete_session_number: u8,

    #[deku(count = "_toc_data_length.saturating_sub(2) / 8")]
    pub toc_track_descriptors: Vec<TocTrackDescriptor<A>>,
}

impl<A: TrackStartAddress> Response for MultiSessionInformation<A> {
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
    pub first_track_number_last_complete_session: u8,

    #[deku(bytes = 4, reader = "A::read_track_start_address(deku::reader)")]
    pub track_start_address: A,
}
