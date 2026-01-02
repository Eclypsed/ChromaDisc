#![doc = include_str!("../docs/scsi.md")]

use derive_more::{Debug, Eq};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum CDBError {
    #[error("Invalid group code: {_0:03b}")]
    InvalidGroupCode(u8),
}

pub trait Cdb<const N: usize> {
    /// OPERATION CODE enum for valid MMCs
    /// ```text
    ///   7   6   5   4   3   2   1   0
    /// +---+---+---+---+---+---+---+---+
    /// | GROUPCODE |    COMMAND CODE   |
    /// +---+---+---+---+---+---+---+---+
    /// ```
    /// See: [SAM-2]
    const OP_CODE: u8;

    fn to_bytes(&self) -> [u8; N];
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[num_enum(error_type(name = CDBError, constructor = CDBError::InvalidGroupCode))]
#[repr(u8)]
pub enum GroupCode {
    Cmd6 = 0b000,
    #[num_enum(alternatives = [0b010])]
    Cmd10 = 0b001,
    Cmd12 = 0b101,
    Cmd16 = 0b100,
}

impl GroupCode {
    pub const fn cdb_len(self) -> usize {
        match self {
            Self::Cmd6 => 6,
            Self::Cmd10 => 10,
            Self::Cmd12 => 12,
            Self::Cmd16 => 16,
        }
    }
}
