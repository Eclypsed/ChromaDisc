#![doc = include_str!("../docs/scsi.md")]

use bitflags::bitflags;
use derive_more::{Debug, Eq, From, Into};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use thiserror::Error;

use crate::addressing::Lba;

#[derive(Error, Debug)]
pub enum CDBError {
    #[error("Incorrect operation code, received 0x{_0:02X} expected 0xBE")]
    IncorrectOpCode(u8),
    #[error("Invalid group code: {_0:03b}")]
    InvalidGroupCode(u8),
    #[error("Invalid sector type: {_0:03b}")]
    InvalidSectorType(u8),
    #[error("Invalid LBA: {0}")]
    InvalidLBA(i32),
    #[error("Transfer length exceeded 16,777,215: {0}")]
    InvalidTransferLength(u32),
    #[error("Invalid C2 error code: {_0:02b}")]
    InvalidC2ErrorCode(u8),
    #[error("Invalid sub-channel selection: {_0:03b}")]
    InvalidSubChannelSelection(u8),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[num_enum(error_type(name = CDBError, constructor = CDBError::InvalidSectorType))]
#[repr(u8)]
pub enum SectorType {
    AllTypes = 0b000,
    CdDa = 0b001,
    Mode1 = 0b010,
    Mode2Formless = 0b011,
    Mode2Form1 = 0b100,
    Mode2Form2 = 0b101,
}

bitflags! {
    #[repr(transparent)]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub struct MainChannelFlags: u8 {
        const SYNC = 1 << 7;
        const SUBHEADER = 1 << 6;
        const HEADER = 1 << 5;
        const USER_DATA = 1 << 4;
        const EDC_ECC = 1 << 3;
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[num_enum(error_type(name = CDBError, constructor = CDBError::InvalidC2ErrorCode))]
#[repr(u8)]
pub enum C2ErrorCode {
    None = 0b00,
    /// A bit is associated with each of the 2 352 bytes of main channel where: 0 = No C2 error
    /// and 1 = C2 error. This results in 294 bytes of C2 error bits. Return the 294 bytes of C2
    /// error bits in the data stream.
    ErrorBits = 0b01,
    /// The Block Error Byte = Logical OR of all of the 294 bytes of C2 error bits. First return
    /// Block Error Byte, then a pad byte of zero and finally the 294 bytes of C2 error bits.
    BlockErrorByte = 0b10,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[num_enum(error_type(name = CDBError, constructor = CDBError::InvalidSubChannelSelection))]
#[repr(u8)]
pub enum SubChannelSelection {
    None = 0b000,
    QSubChannel = 0b010,
    RWSubChannel = 0b100,
}

/// CONTROL byte newtype
/// ```text
///   7   6   5   4   3   2   1   0
/// +---+---+---+---+---+---+---+---+
/// |   VS  |  Reserved | N | O | L |
/// +---+---+---+---+---+---+---+---+
/// ```
/// * **VS** - Vendor Specific
/// * **N**  - NACA (Normal ACA)
/// * **O**  - Obsolete
/// * **L**  - Link
///
/// See: [SAM-2]
#[repr(transparent)]
#[derive(Debug, Clone, Copy, From, Into)]
pub struct Control(u8);

/// Struct Representing the 12-Byte variant of the Command Descriptor Block (CDB).
#[derive(Debug)]
#[allow(dead_code)]
pub struct ReadCD {
    pub sector_type: SectorType,
    pub dap: bool,
    pub start_lba: Lba,
    pub transfer_length: u32,
    pub main_channel: MainChannelFlags,
    pub c2_error_info: C2ErrorCode,
    pub sub_channel: SubChannelSelection,
    pub control: Control,
}

impl Cdb<{ GroupCode::Cmd12.cdb_len() }> for ReadCD {
    const OP_CODE: u8 = 0xBE;

    fn to_bytes(&self) -> [u8; GroupCode::Cmd12.cdb_len()] {
        let mut bytes = [0u8; 12];

        bytes[0] = Self::OP_CODE;

        bytes[1] |= u8::from(self.sector_type) << 2;
        bytes[1] |= u8::from(self.dap) << 1;

        bytes[2..=5].copy_from_slice(&i32::from(self.start_lba).to_be_bytes());

        bytes[6..=8].copy_from_slice(&self.transfer_length.to_be_bytes()[1..4]);

        bytes[9] |= self.main_channel.bits();
        bytes[9] |= u8::from(self.c2_error_info) << 1;

        bytes[10] |= u8::from(self.sub_channel);

        bytes[11] = self.control.into();

        bytes
    }
}

impl TryFrom<&[u8; 12]> for ReadCD {
    type Error = CDBError;

    fn try_from(value: &[u8; 12]) -> Result<Self, Self::Error> {
        if value[0] != Self::OP_CODE {
            return Err(CDBError::IncorrectOpCode(value[0]));
        }

        let sector_type = SectorType::try_from_primitive(value[1] >> 2)?;

        let dap = (value[1] >> 1) & 0b1 == 1;

        let lba_val = i32::from_be_bytes(value[2..=5].try_into().unwrap());
        let start_lba = Lba::from(lba_val);

        let transfer_length = u32::from_be_bytes([0, value[6], value[7], value[8]]);

        let main_channel = MainChannelFlags::from_bits_truncate(value[9]);

        let c2_error_info = C2ErrorCode::try_from_primitive((value[9] & 0b00000110) >> 1)?;

        let sub_channel = SubChannelSelection::try_from(value[10] & 0b111)?;

        let control = Control::from(value[11]);

        Ok(Self {
            sector_type,
            dap,
            start_lba,
            transfer_length,
            main_channel,
            c2_error_info,
            sub_channel,
            control,
        })
    }
}
