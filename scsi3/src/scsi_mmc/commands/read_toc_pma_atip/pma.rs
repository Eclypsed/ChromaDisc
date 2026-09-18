use alloc::vec::Vec;
use arbitrary_int::u4;
use thiserror::Error;

use super::{ReadTocPmaAtip, ReadTocPmaAtipOpCode};
use crate::{
    core::{ReadCommand, TruncationError},
    mmc::device_models::cd::q_subcode,
};

const PMA_MIN_BYTES: usize = 4;
const TRACK_DESCRIPTOR_SIZE: usize = 11;

#[derive(Debug, Error)]
pub enum PmaError {
    #[error(transparent)]
    Truncated(#[from] TruncationError<PMA_MIN_BYTES>),
}

impl ReadCommand<ReadTocPmaAtipOpCode> for ReadTocPmaAtip<Pma> {
    type Len = u16;
    type Response<'a> = Pma;
    type Error = PmaError;

    fn response_len(&self) -> Self::Len {
        self.allocation_length
    }

    fn parse<'a>(&self, buf: &'a [u8]) -> Result<Self::Response<'a>, Self::Error> {
        if buf.len() < PMA_MIN_BYTES {
            return Err(TruncationError(buf.len()).into());
        }

        let toc_data_length: usize = u16::from_be_bytes([buf[0], buf[1]]).into();

        let max_bytes = buf.len().min(toc_data_length - 2);
        let desc_bytes = buf.get(4..max_bytes).unwrap_or_default();

        let pma_descriptors = desc_bytes
            .chunks_exact(TRACK_DESCRIPTOR_SIZE)
            .map(|chunk| PmaDescriptor {
                adr: u4::extract_u8(chunk[1], 4),
                control: q_subcode::Control::from_bits_truncate(chunk[1] & 0xF),
                tno: chunk[2],
                point: chunk[3],
                min: chunk[4],
                sec: chunk[5],
                frame: chunk[6],
                zero: chunk[7],
                pmin: chunk[8],
                psec: chunk[9],
                pframe: chunk[10],
            })
            .collect::<Vec<_>>();

        Ok(Pma { pma_descriptors })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Pma {
    pub pma_descriptors: Vec<PmaDescriptor>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct PmaDescriptor {
    adr: u4,
    control: q_subcode::Control,
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
