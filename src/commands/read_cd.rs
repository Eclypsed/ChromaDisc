use std::{cmp, fs::File};

use bitflags::bitflags;
use i24::{U24, u24};
use num_enum::{IntoPrimitive, TryFromPrimitive};
use thiserror::Error;

use crate::{
    addressing::{Lba, lba},
    sgio::{self, DxferDirection, run_sgio},
};

use super::{Command, Control};

const CDDA_USER_DATA_SIZE: usize = 2352;
const MODE1_USER_DATA_SIZE: usize = 2048;
const MODE2_FORMLESS_USER_DATA_SIZE: usize = 2336;
const MODE2_FORM1_USER_DATA_SIZE: usize = 2048;
const MODE2_FORM2_USER_DATA_SIZE: usize = 2324;

#[derive(Error, Debug)]
pub enum Error {
    #[error("Invalid sector type: {0:03b}")]
    InvalidSectorType(u8),
    #[error("Transfer length exceeded 16,777,215: {0}")]
    InvalidTransferLength(u32),
    #[error("Invalid C2 error code: {0:02b}")]
    InvalidC2ErrorCode(u8),
    #[error("Invalid sub-channel selection: {0:03b}")]
    InvalidSubChannelSelection(u8),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[num_enum(error_type(name = Error, constructor = Error::InvalidSectorType))]
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
#[num_enum(error_type(name = Error, constructor = Error::InvalidC2ErrorCode))]
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
#[num_enum(error_type(name = Error, constructor = Error::InvalidSubChannelSelection))]
#[repr(u8)]
pub enum SubChannelSelection {
    None = 0b000,
    QSubChannel = 0b010,
    RWSubChannel = 0b100,
}

#[derive(Debug, Clone, Copy)]
pub struct ReadCD {
    pub sector_type: SectorType,
    pub dap: bool,
    pub starting_lba: Lba,
    pub transfer_length: U24,
    pub main_channel: MainChannelFlags,
    pub c2_error_info: C2ErrorCode,
    pub sub_channel: SubChannelSelection,
    pub control: Control,
}

impl Command<12> for ReadCD {
    const OP_CODE: u8 = 0xBE;

    type Response = Vec<u8>;

    fn as_cdb(&self) -> [u8; 12] {
        let mut bytes = [0u8; 12];
        let lba_bytes: i32 = self.starting_lba.into();
        let transfer_length_bytes = self.transfer_length.to_be_bytes();

        bytes[0] = Self::OP_CODE;
        bytes[1] |= u8::from(self.sector_type) << 2;
        bytes[1] |= u8::from(self.dap) << 1;
        bytes[2] = (lba_bytes >> 24) as u8;
        bytes[3] = (lba_bytes >> 16) as u8;
        bytes[4] = (lba_bytes >> 8) as u8;
        bytes[5] = lba_bytes as u8;
        bytes[6] = transfer_length_bytes[0];
        bytes[7] = transfer_length_bytes[1];
        bytes[8] = transfer_length_bytes[2];
        bytes[9] |= self.main_channel.bits();
        bytes[9] |= u8::from(self.c2_error_info) << 1;
        bytes[10] |= u8::from(self.sub_channel);
        bytes[11] = self.control.into();

        bytes
    }

    fn allocation_len(&self) -> usize {
        let tl_bytes = self.transfer_length.to_be_bytes();
        let sectors = usize::from_be_bytes([0, 0, 0, 0, 0, tl_bytes[0], tl_bytes[1], tl_bytes[2]]);
        let user_data_size = match self.sector_type {
            SectorType::AllTypes | SectorType::CdDa => CDDA_USER_DATA_SIZE, // The largest one possible
            SectorType::Mode1 => MODE1_USER_DATA_SIZE,
            SectorType::Mode2Formless => MODE2_FORMLESS_USER_DATA_SIZE,
            SectorType::Mode2Form1 => MODE2_FORM1_USER_DATA_SIZE,
            SectorType::Mode2Form2 => MODE2_FORM2_USER_DATA_SIZE,
        };

        sectors * user_data_size
    }
}

impl ReadCD {
    pub fn new() -> Self {
        Self {
            sector_type: SectorType::AllTypes,
            dap: false,
            starting_lba: lba!(0),
            transfer_length: U24::ZERO,
            main_channel: MainChannelFlags::empty(),
            c2_error_info: C2ErrorCode::None,
            sub_channel: SubChannelSelection::None,
            control: 0.into(),
        }
    }
}

#[derive(Debug)]
pub struct SectorReader<'a> {
    file: &'a File,
    remaining: U24,
    command: ReadCD,
}

#[allow(dead_code)]
impl<'a> SectorReader<'a> {
    // 2352 * 27 = 63531 ~ 64 KBs common CD firmware limit
    const MAX_SECTORS_PER: U24 = u24!(27);

    pub fn new(file: &'a File, start: Lba, sectors: U24) -> Self {
        let mut command = ReadCD::new();
        command.sector_type = SectorType::AllTypes;
        command.starting_lba = start;
        command.main_channel |= MainChannelFlags::USER_DATA;

        Self {
            file,
            remaining: sectors,
            command,
        }
    }
}

impl<'a> Iterator for SectorReader<'a> {
    type Item = Result<(Vec<u8>, U24), sgio::SCSIError>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.remaining == U24::ZERO {
            return None;
        }

        let sectors_to_read = cmp::min(self.remaining, Self::MAX_SECTORS_PER);

        self.command.transfer_length = sectors_to_read;

        let data = run_sgio(self.file, self.command, DxferDirection::FromDev);

        self.command.starting_lba +=
            Lba::try_from(i32::try_from(sectors_to_read.to_u32()).unwrap()).unwrap();
        self.remaining -= sectors_to_read;

        Some(data.map(|v| (v, self.remaining)))
    }
}

#[allow(dead_code)]
pub fn read_audio_range(file: &File, start: Lba, sectors: U24) -> Result<Vec<u8>, sgio::SCSIError> {
    // 2352 * 27 = 63531 ~ 64 KBs common CD firmware limit
    const MAX_SECTORS_PER: U24 = u24!(27);

    let mut out = Vec::<u8>::with_capacity(CDDA_USER_DATA_SIZE * usize::from(sectors));

    let mut remaining = sectors;
    let mut start = start;

    let mut command = ReadCD::new();
    command.sector_type = SectorType::AllTypes;
    command.starting_lba = start;
    command.main_channel |= MainChannelFlags::USER_DATA;

    while remaining > U24::ZERO {
        let sectors_to_read = cmp::min(remaining, MAX_SECTORS_PER);

        command.transfer_length = sectors_to_read;

        let data = run_sgio(file, command, DxferDirection::FromDev)?;

        out.extend_from_slice(&data);

        // This conversion is mathematically safe
        start += Lba::try_from(i32::try_from(sectors_to_read.to_u32()).unwrap()).unwrap();
        command.starting_lba = start;

        remaining -= sectors_to_read;
    }

    Ok(out)
}
