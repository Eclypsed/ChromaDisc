use num_enum::{IntoPrimitive, TryFromPrimitive};
use thiserror::Error;

use super::{Command, Control};

const MIN_RESPONSE_LENGTH: usize = 46;

#[derive(Debug, Error)]
pub enum Error {
    #[error("Encountered invalid CD Data Mode {0:X}")]
    InvalidDataMode(u8),
    #[error("Received {0} bytes of READ TRACK INFORMATION response, expected at least {min}", min = MIN_RESPONSE_LENGTH)]
    IncompleteResponse(usize),
}

#[allow(clippy::upper_case_acronyms)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive)]
#[repr(u8)]
pub enum AddressType {
    LBA = 0b00,
    LTN = 0b01,
    SessionNum = 0b10,
}

#[derive(Debug, Clone, Copy)]
pub struct ReadTrackInfo {
    open: bool,
    address_type: AddressType,
    address_number: u32,
    allocation_length: u16,
    control: Control,
}

impl ReadTrackInfo {
    pub fn new(
        open: bool,
        address_type: AddressType,
        address_number: u32,
        control: Control,
    ) -> Self {
        Self {
            open,
            address_type,
            address_number,
            allocation_length: 50,
            control,
        }
    }
}

impl Command<10> for ReadTrackInfo {
    const OP_CODE: u8 = 0x52;

    type Response = ReadTrackInfoResponse;

    fn allocation_len(&self) -> usize {
        self.allocation_length.into()
    }

    fn as_cdb(&self) -> [u8; 10] {
        let mut bytes = [0u8; 10];

        bytes[0] = Self::OP_CODE;
        bytes[1] |= u8::from(self.open) << 2;
        bytes[1] |= u8::from(self.address_type);
        bytes[2] = (self.address_number >> 24) as u8;
        bytes[3] = (self.address_number >> 16) as u8;
        bytes[4] = (self.address_number >> 8) as u8;
        bytes[5] = self.address_number as u8;
        bytes[7] = (self.allocation_length >> 8) as u8;
        bytes[8] = self.allocation_length as u8;
        bytes[9] = self.control.into();

        bytes
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[repr(u8)]
pub enum LayerJumpRecordingStatus {
    None = 0b00,
    Unspecified = 0b01,
    Manual = 0b10,
    RegularInterval = 0b11,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, IntoPrimitive, TryFromPrimitive)]
#[num_enum(error_type(name = Error, constructor = Error::InvalidDataMode))]
#[repr(u8)]
pub enum DataMode {
    /// Mode 1 (ISO/IEC 10149)
    Mode1 = 0x1,
    /// Mode 2 (ISO/IEC 10149 or CD-ROM XA)
    Mode2 = 0x2,
    /// Data Block Type unknown (no track descriptor block)
    Unknown = 0xF,
}

#[allow(dead_code)]
#[derive(Debug)]
pub struct ReadTrackInfoResponse {
    pub logical_track_number: u16,
    pub session_number: u16,
    pub ljrs: LayerJumpRecordingStatus,
    pub damage: bool,
    pub copy: bool,
    pub track_mode: u8,
    pub rt: bool,
    pub blank: bool,
    pub packet_inc: bool,
    pub fp: bool,
    pub data_mode: DataMode,
    pub lra_v: bool,
    pub nwa_v: bool,
    pub logical_track_start_addr: i32,
    pub next_writable_addr: i32,
    pub free_blocks: u32,
    pub fixed_packet_size: u32,
    pub logical_track_size: u32,
    pub last_recorded_addr: i32,
    pub read_compatibility_lba: i32,
    pub next_layer_jump_addr: i32,
    pub last_layer_jump_addr: i32,
}

impl ReadTrackInfoResponse {
    const LJRS_MASK: u8 = 0b11000000;
    const DAMAGE_MASK: u8 = 0b00100000;
    const COPY_MASK: u8 = 0b00010000;
    const TRACK_MODE_MASK: u8 = 0b00001111;
    const RT_MASK: u8 = 0b10000000;
    const BLANK_MASK: u8 = 0b01000000;
    const PACKET_INC_MASK: u8 = 0b00100000;
    const FP_MASK: u8 = 0b00010000;
    const DATA_MODE_MASK: u8 = 0b00001111;
    const LRA_V_MASK: u8 = 0b00000010;
    const NWA_V_MASK: u8 = 0b00000001;
}

impl TryFrom<Vec<u8>> for ReadTrackInfoResponse {
    type Error = Error;

    fn try_from(value: Vec<u8>) -> Result<Self, Self::Error> {
        let data_len = value.len();
        if data_len < MIN_RESPONSE_LENGTH {
            return Err(Error::IncompleteResponse(data_len));
        }

        let logical_track_number: u16 = u16::from_be_bytes([value[32], value[2]]);
        let session_number: u16 = u16::from_be_bytes([value[33], value[3]]);

        let ljrs = LayerJumpRecordingStatus::try_from((value[5] & Self::LJRS_MASK) >> 6).unwrap();
        let damage = (value[5] & Self::DAMAGE_MASK) >> 5 != 0;
        let copy = (value[5] & Self::COPY_MASK) >> 4 != 0;
        let track_mode = value[5] & Self::TRACK_MODE_MASK;
        let rt = (value[6] & Self::RT_MASK) >> 7 != 0;
        let blank = (value[6] & Self::BLANK_MASK) >> 6 != 0;
        let packet_inc = (value[6] & Self::PACKET_INC_MASK) >> 5 != 0;
        let fp = (value[6] & Self::FP_MASK) >> 4 != 0;
        let data_mode = DataMode::try_from_primitive(value[6] & Self::DATA_MODE_MASK)?;
        let lra_v = (value[7] & Self::LRA_V_MASK) >> 1 != 0;
        let nwa_v = (value[7] & Self::NWA_V_MASK) != 0;

        let logical_track_start_addr =
            i32::from_be_bytes([value[8], value[9], value[10], value[11]]);
        let next_writable_addr = i32::from_be_bytes([value[12], value[13], value[14], value[15]]);
        let free_blocks = u32::from_be_bytes([value[16], value[17], value[18], value[19]]);
        let fixed_packet_size = u32::from_be_bytes([value[20], value[21], value[22], value[23]]);
        let logical_track_size = u32::from_be_bytes([value[24], value[25], value[26], value[27]]);
        let last_recorded_addr = i32::from_be_bytes([value[28], value[29], value[30], value[31]]);
        let read_compatibility_lba =
            i32::from_be_bytes([value[36], value[37], value[38], value[39]]);
        let next_layer_jump_addr = i32::from_be_bytes([value[40], value[41], value[42], value[43]]);
        let last_layer_jump_addr = i32::from_be_bytes([value[44], value[45], value[46], value[47]]);

        Ok(Self {
            logical_track_number,
            session_number,
            ljrs,
            damage,
            copy,
            track_mode,
            rt,
            blank,
            packet_inc,
            fp,
            data_mode,
            lra_v,
            nwa_v,
            logical_track_start_addr,
            next_writable_addr,
            free_blocks,
            fixed_packet_size,
            logical_track_size,
            last_recorded_addr,
            read_compatibility_lba,
            next_layer_jump_addr,
            last_layer_jump_addr,
        })
    }
}
