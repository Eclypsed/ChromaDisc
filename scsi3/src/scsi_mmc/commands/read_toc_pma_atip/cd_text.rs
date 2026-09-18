use alloc::vec::Vec;
use thiserror::Error;

use super::{ReadTocPmaAtip, ReadTocPmaAtipOpCode};
use crate::core::{ReadCommand, TruncationError};

const CD_TEXT_MIN_BYTES: usize = 4;
const TRACK_DESCRIPTOR_SIZE: usize = 18;

#[derive(Debug, Error)]
pub enum CdTextError {
    #[error(transparent)]
    Truncated(#[from] TruncationError<CD_TEXT_MIN_BYTES>),
}

impl ReadCommand<ReadTocPmaAtipOpCode> for ReadTocPmaAtip<CdText> {
    type Len = u16;
    type Response<'a> = CdText;
    type Error = CdTextError;

    fn response_len(&self) -> Self::Len {
        self.allocation_length
    }

    fn parse<'a>(&self, buf: &'a [u8]) -> Result<Self::Response<'a>, Self::Error> {
        if buf.len() < CD_TEXT_MIN_BYTES {
            return Err(TruncationError(buf.len()).into());
        }

        let toc_data_length: usize = u16::from_be_bytes([buf[0], buf[1]]).into();

        let max_bytes = buf.len().min(toc_data_length - 2);
        let desc_bytes = buf.get(4..max_bytes).unwrap_or_default();

        let cd_text_descriptors = desc_bytes
            .chunks_exact(TRACK_DESCRIPTOR_SIZE)
            .map(|chunk| CdTextDescriptor {
                cd_text_data: chunk.try_into().unwrap(),
            })
            .collect::<Vec<_>>();

        Ok(CdText {
            cd_text_descriptors,
        })
    }
}

pub struct CdText {
    pub cd_text_descriptors: Vec<CdTextDescriptor>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct CdTextDescriptor {
    // TODO
    cd_text_data: [u8; 18],
}
