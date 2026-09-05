use std::marker::PhantomData;

use crate::{
    core::{
        addressing::{Lba, Span},
        Command, Control, OpCode, OpCodeDef, ReadCommand,
    },
    mmc::msf::Msf,
};
use arbitrary_int::{u2, u24, u3};
use bitfield::BitsEnum;

mod private {
    use super::*;

    pub trait SectorRange {
        fn sector_count(&self) -> u32;
    }

    pub trait AddressingModeSeal {
        type AddressingParams: SectorRange;
    }

    pub struct LbaAddressingParams {
        pub starting_lba: Lba,
        pub transfer_length: u24,
    }

    impl SectorRange for LbaAddressingParams {
        fn sector_count(&self) -> u32 {
            self.transfer_length.into()
        }
    }

    impl SectorRange for Span<Msf> {
        fn sector_count(&self) -> u32 {
            self.end().total_frames() - self.start().total_frames()
        }
    }

    pub trait MainChannelSelectionSeal {
        const MAIN_CHANNEL_SELECTION_VALUE: u8;

        fn user_data() -> bool {
            (Self::MAIN_CHANNEL_SELECTION_VALUE & MainChannelSelection::USER_DATA) != 0
        }
    }

    pub trait SectorTypeSeal {
        const SECTOR_TYPE: SectorType;
    }

    #[non_exhaustive]
    #[derive(Debug, Clone, Copy, PartialEq, Eq, BitsEnum)]
    #[bits(3, repr = u3, reserved = 0b110..=0b111)]
    pub enum SectorType {
        AllTypes = 0b000,
        CdDa = 0b001,
        Mode1 = 0b010,
        Mode2Formless = 0b011,
        Mode2Form1 = 0b100,
        Mode2Form2 = 0b101,
    }
}

pub trait ReadCdAddress: private::AddressingModeSeal {}

impl private::AddressingModeSeal for Lba {
    type AddressingParams = private::LbaAddressingParams;
}
impl ReadCdAddress for Lba {}

impl private::AddressingModeSeal for Msf {
    type AddressingParams = Span<Msf>;
}
impl ReadCdAddress for Msf {}

#[derive(Debug, Clone, Copy, PartialEq, Eq, BitsEnum)]
#[bits(2, repr = u2)]
pub enum HeaderCodes {
    None = 0b00,
    Header = 0b01,
    SubHeader = 0b10,
    AllHeaders = 0b11,
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, BitsEnum)]
#[bits(2, repr = u2, reserved = 0b11)]
pub enum C2ErrorInfo {
    None = 0b00,
    C2Pointers = 0b01,
    BlockC2Pointers = 0b10,
}

#[non_exhaustive]
#[derive(Debug, Clone, Copy, PartialEq, Eq, BitsEnum)]
#[bits(3, repr = u3, reserved = 0b011 | 0b101..=0b111)]
pub enum SubChannelSelection {
    None = 0b000,
    RawPW = 0b001,
    FormattedQ = 0b010,
    CorrectedDeinterleavedRW = 0b100,
}

pub trait SectorSelection: private::SectorTypeSeal + private::MainChannelSelectionSeal {}

pub struct ReadCd<A: ReadCdAddress, S: SectorSelection> {
    _sector_selection: PhantomData<S>,
    digital_audio_play: bool,
    // Interestingly, in the LBA version of this command, byte 1 bit 0 is an obsolete RELADDR flag.
    // However, every reference going back to MMC-1 says this flag should just be 0, so I don't
    // know where it came from but I'm choosing to omit it.
    addressing_params: A::AddressingParams,
    c2_error_info: C2ErrorInfo,
    sub_channel_selection: SubChannelSelection,
    control: Control,
}

type ReadCdOpcode = OpCode<0xBE>;
type ReadCdMsfOpcode = OpCode<0xB9>;

impl<S: SectorSelection> Command<ReadCdOpcode> for ReadCd<Lba, S> {
    fn as_cdb(&self) -> <ReadCdOpcode as OpCodeDef>::Cdb {
        let lba_bytes: [u8; 4] = i32::from(self.addressing_params.starting_lba).to_be_bytes();
        let transfer_bytes: [u8; 3] = self.addressing_params.transfer_length.to_be_bytes();

        [
            ReadCdOpcode::OP_CODE,
            (S::SECTOR_TYPE.to_bits().value() << 2) | ((self.digital_audio_play as u8) << 1),
            lba_bytes[0],
            lba_bytes[1],
            lba_bytes[2],
            lba_bytes[3],
            transfer_bytes[0],
            transfer_bytes[1],
            transfer_bytes[2],
            (S::MAIN_CHANNEL_SELECTION_VALUE & 0xF8) | (self.c2_error_info.to_bits().value() << 1),
            (self.sub_channel_selection.to_bits().value()),
            self.control.into(),
        ]
    }
}

impl<S: SectorSelection> Command<ReadCdMsfOpcode> for ReadCd<Msf, S> {
    fn as_cdb(&self) -> <ReadCdMsfOpcode as OpCodeDef>::Cdb {
        [
            ReadCdMsfOpcode::OP_CODE,
            (S::SECTOR_TYPE.to_bits().value() << 2) | ((self.digital_audio_play as u8) << 1),
            0,
            (*self.addressing_params.start().minute()).into(),
            (*self.addressing_params.start().second()).into(),
            (*self.addressing_params.start().frame()).into(),
            (*self.addressing_params.end().minute()).into(),
            (*self.addressing_params.end().second()).into(),
            (*self.addressing_params.end().frame()).into(),
            (S::MAIN_CHANNEL_SELECTION_VALUE & 0xF8) | (self.c2_error_info.to_bits().value() << 1),
            (self.sub_channel_selection.to_bits().value()),
            self.control.into(),
        ]
    }
}

// pub struct Sector<'a, S: SectorSelection> {
//     buf: &'a [u8], // Should be at most 2352 bytes total
//     _type_marker: PhantomData<S>,
// }

// Probably explain here why there is no way to construct an "All sector types" command.
// Either that or in the future add an "All sector types" option that has no Main Channel
// restrictions or useful parsing beyond chunking the raw sectors (since we don't know anything
// about the sectors returned, beyond them being either all CD-DA or all CD Data, without external
// information)

pub struct MainChannelSelection;

impl MainChannelSelection {
    pub const NO_FIELDS: u8 = 0;
    pub const SYNC: u8 = 1 << 7;
    pub const SUB_HEADER: u8 = 1 << 6;
    pub const HEADER: u8 = 1 << 5;
    pub const USER_DATA: u8 = 1 << 4;
    pub const EDC_ECC: u8 = 1 << 3;
}

macro_rules! impl_sector_selection {
    ($sector_selection:ident, $sector_type:expr, [$($flag:expr),+ $(,)?]) => {
        impl<const MAIN_CHANNEL_SELECTION: u8> private::SectorTypeSeal for $sector_selection<MAIN_CHANNEL_SELECTION> {
            const SECTOR_TYPE: private::SectorType = $sector_type;
        }

        $(
            impl private::MainChannelSelectionSeal for $sector_selection<{ $flag }> {
                const MAIN_CHANNEL_SELECTION_VALUE: u8 = $flag;
            }
        )+

        impl<const MAIN_CHANNEL_SELECTION: u8> SectorSelection for $sector_selection<MAIN_CHANNEL_SELECTION> where
            $sector_selection<MAIN_CHANNEL_SELECTION>: private::MainChannelSelectionSeal
        {
        }
    };
}

pub mod cd_da;
pub mod mode1;
pub mod mode2_form1;
pub mod mode2_form2;
pub mod mode2_formless;
