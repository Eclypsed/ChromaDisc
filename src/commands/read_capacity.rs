use thiserror::Error;

use crate::addressing::{AddressError, Lba};

use super::{Command, Control};

const RESPONSE_LENGTH: usize = 8;

#[derive(Debug, Error)]
pub enum Error {
    #[error("READ CAPACITY Response must be at least {size} bytes long, received {0}", size = RESPONSE_LENGTH)]
    IncompleteResponse(usize),
    #[error(transparent)]
    InvalidLBA(#[from] AddressError<Lba>),
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ReadCapacity {
    control: Control,
}

#[allow(dead_code)]
impl ReadCapacity {
    pub fn new(control: Control) -> Self {
        Self { control }
    }
}

impl Command<10> for ReadCapacity {
    const OP_CODE: u8 = 0x25;

    type Response = ReadCapacityResponse;

    #[inline]
    fn allocation_len(&self) -> usize {
        8
    }

    fn as_cdb(&self) -> [u8; 10] {
        let mut bytes = [0u8; 10];

        bytes[0] = Self::OP_CODE;
        bytes[9] = self.control.into();

        bytes
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ReadCapacityResponse {
    lba: Lba,
    // This SHOULD be 2048
    block_length_bytes: u32,
}

impl TryFrom<Vec<u8>> for ReadCapacityResponse {
    type Error = Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let res_len = value.len();

        if res_len < RESPONSE_LENGTH {
            return Err(Error::IncompleteResponse(res_len));
        }

        let lba_bytes = i32::from_be_bytes([value[0], value[1], value[2], value[3]]);
        let lba = Lba::try_from(lba_bytes)?;

        let block_length_bytes = u32::from_be_bytes([value[4], value[5], value[6], value[7]]);

        Ok(Self {
            lba,
            block_length_bytes,
        })
    }
}
