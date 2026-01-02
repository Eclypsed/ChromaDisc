use std::{cmp, fs::File, io};

use bitflags::bitflags;
use derive_more::{From, Into};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use thiserror::Error;

use crate::{
    addressing::Lba,
    cdb::Cdb,
    sgio::{DxferDirection, run_sgio},
};

#[derive(Error, Debug)]
pub enum ReadCDError {
    #[error("Invalid sector type: {_0:03b}")]
    InvalidSectorType(u8),
    #[error("Transfer length exceeded 16,777,215: {0}")]
    InvalidTransferLength(u32),
    #[error("Invalid C2 error code: {_0:02b}")]
    InvalidC2ErrorCode(u8),
    #[error("Invalid sub-channel selection: {_0:03b}")]
    InvalidSubChannelSelection(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[num_enum(error_type(name = ReadCDError, constructor = ReadCDError::InvalidSectorType))]
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
#[num_enum(error_type(name = ReadCDError, constructor = ReadCDError::InvalidC2ErrorCode))]
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
#[num_enum(error_type(name = ReadCDError, constructor = ReadCDError::InvalidSubChannelSelection))]
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

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct ReadCD([u8; 12]);

impl Cdb<12> for ReadCD {
    const OP_CODE: u8 = 0xBE;

    #[inline]
    fn to_bytes(&self) -> [u8; 12] {
        self.0
    }
}

#[allow(dead_code)]
impl ReadCD {
    const SECTOR_TYPE_MASK: u8 = 0b0001_1100;
    const DAP_MASK: u8 = 0b0000_0010;
    const C2_ERROR_MASK: u8 = 0b0000_0110;
    const SUB_CHANNEL_MASK: u8 = 0b0000_0111;

    pub fn new() -> Self {
        let mut bytes = [0u8; 12];

        bytes[0] = Self::OP_CODE;

        Self(bytes)
    }

    #[inline]
    pub fn sector_type(&mut self, sector_type: SectorType) -> &mut Self {
        self.0[1] = (self.0[1] & !Self::SECTOR_TYPE_MASK) | u8::from(sector_type) << 2;
        self
    }

    #[inline]
    #[allow(dead_code)]
    pub fn dap(&mut self, dap: bool) -> &mut Self {
        self.0[1] = (self.0[1] & !Self::DAP_MASK) | u8::from(dap) << 1;
        self
    }

    #[inline]
    pub fn start_lba(&mut self, start: Lba) -> &mut Self {
        let v = i32::from(start);

        self.0[2] = (v >> 24) as u8;
        self.0[3] = (v >> 16) as u8;
        self.0[4] = (v >> 8) as u8;
        self.0[5] = v as u8;

        self
    }

    #[inline]
    pub fn transfer_length(&mut self, transfer_length: u32) -> Result<&mut Self, ReadCDError> {
        if transfer_length > 0x00FF_FFFF {
            return Err(ReadCDError::InvalidTransferLength(transfer_length));
        }

        self.0[6] = (transfer_length >> 16) as u8;
        self.0[7] = (transfer_length >> 8) as u8;
        self.0[8] = transfer_length as u8;

        Ok(self)
    }

    #[inline]
    pub fn set_main_channel(&mut self, flags: MainChannelFlags) -> &mut Self {
        self.0[9] |= flags.bits();
        self
    }

    #[inline]
    #[allow(dead_code)]
    pub fn c2_error_info(&mut self, c2_error_info: C2ErrorCode) -> &mut Self {
        self.0[9] = (self.0[9] & !Self::C2_ERROR_MASK) | u8::from(c2_error_info) << 1;
        self
    }

    #[inline]
    #[allow(dead_code)]
    pub fn sub_channel(&mut self, selection: SubChannelSelection) -> &mut Self {
        self.0[10] = (self.0[10] & !Self::SUB_CHANNEL_MASK) | u8::from(selection);
        self
    }

    #[inline]
    #[allow(dead_code)]
    pub fn control(&mut self, control: Control) -> &mut Self {
        self.0[11] = control.into();
        self
    }

    #[inline]
    pub fn as_bytes_mut(&mut self) -> &mut [u8; 12] {
        &mut self.0
    }
}

#[allow(dead_code)]
pub fn read_audio_range(file: &File, start: Lba, sectors: u32) -> io::Result<Vec<u8>> {
    const SECTOR_BYTES: usize = 2352;

    // 2352 * 27 = 63531 ~ 64 KBs common CD firmware limit
    const MAX_SECTORS_PER: u8 = 27;

    let mut out = Vec::<u8>::with_capacity(SECTOR_BYTES * sectors as usize);

    let mut remaining = sectors;

    let mut start = start;

    let mut cdb = ReadCD::new();
    cdb.sector_type(SectorType::CdDa)
        .start_lba(start)
        .set_main_channel(MainChannelFlags::USER_DATA);
    // .sub_channel(SubChannelSelection::QSubChannel);

    while remaining > 0 {
        let sectors_to_read = cmp::min(remaining, MAX_SECTORS_PER.into());
        let bytes_to_read = sectors_to_read as usize * SECTOR_BYTES;

        cdb.transfer_length(sectors_to_read).unwrap();

        let mut data = vec![0u8; bytes_to_read];

        let received = run_sgio(file, DxferDirection::FromDev, cdb.as_bytes_mut(), &mut data)
            .map_err(io::Error::other)?;

        data.truncate(received);
        out.extend_from_slice(&data);

        // This conversion is mathematically safe
        start += Lba::from(i32::try_from(sectors_to_read).unwrap());
        cdb.start_lba(start);

        remaining -= sectors_to_read;
    }

    Ok(out)
}
