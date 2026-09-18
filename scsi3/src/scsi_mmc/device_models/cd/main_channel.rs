use core::fmt;

use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

use crate::mmc::device_models::cd::addressing::UnvalidatedMsf;

pub const SYNC_PATTERN: [u8; 12] = [
    0x00, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0xFF, 0x00,
];

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, FromBytes, IntoBytes, Immutable, KnownLayout, Unaligned,
)]
#[repr(C)]
pub struct SectorHeader {
    // TODO: Turn into `Raw<Msf>`, delete `UnvalidatedMsf`. Probably implement `Raw` in scsi-core.
    // TODO: Determine if this comes back in BCD or binary
    pub address: UnvalidatedMsf,
    pub mode: ModeByte,
}

/// The header's mode byte: a 3-bit block indicator, 3 reserved bits, and a
/// 2-bit data mode.
#[derive(Copy, Clone, PartialEq, Eq, FromBytes, IntoBytes, Immutable, KnownLayout, Unaligned)]
#[repr(transparent)]
pub struct ModeByte(u8);

/// Bits 7-5 of the mode byte. Per-sector, not per-mode: link, run-in and
/// run-out blocks appear inside a track of otherwise ordinary sectors.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum BlockIndicator {
    UserData = 0b000,
    FourthRunIn = 0b001,
    ThirdRunIn = 0b010,
    SecondRunIn = 0b011,
    FirstRunIn = 0b100,
    Link = 0b101,
    SecondRunOut = 0b110,
    FirstRunOut = 0b111,
}

/// Bits 1-0 of the mode byte: the part that identifies the sector mode.
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(u8)]
pub enum DataMode {
    Mode0 = 0b00,
    Mode1 = 0b01,
    Mode2 = 0b10,
    Reserved = 0b11,
}

impl ModeByte {
    #[inline]
    pub const fn from_bits(b: u8) -> Self {
        ModeByte(b)
    }

    /// The raw byte, exactly as recorded.
    #[inline]
    pub const fn bits(self) -> u8 {
        self.0
    }

    /// Build a well-formed mode byte with the reserved bits clear.
    #[inline]
    pub const fn new(block: BlockIndicator, mode: DataMode) -> Self {
        ModeByte(((block as u8) << 5) | mode as u8)
    }

    #[inline]
    pub const fn block_indicator(self) -> BlockIndicator {
        match (self.0 >> 5) & 0b111 {
            0b000 => BlockIndicator::UserData,
            0b001 => BlockIndicator::FourthRunIn,
            0b010 => BlockIndicator::ThirdRunIn,
            0b011 => BlockIndicator::SecondRunIn,
            0b100 => BlockIndicator::FirstRunIn,
            0b101 => BlockIndicator::Link,
            0b110 => BlockIndicator::SecondRunOut,
            _ => BlockIndicator::FirstRunOut,
        }
    }

    #[inline]
    pub const fn data_mode(self) -> DataMode {
        match self.0 & 0b11 {
            0b00 => DataMode::Mode0,
            0b01 => DataMode::Mode1,
            0b10 => DataMode::Mode2,
            _ => DataMode::Reserved, // 0b11
        }
    }
}

impl fmt::Debug for ModeByte {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ModeByte")
            .field("block", &self.block_indicator())
            .field("mode", &self.data_mode())
            .finish()
    }
}

pub mod cdda {}

pub mod mode0 {}

pub mod mode1 {}

pub mod mode2 {
    use core::fmt;

    use zerocopy::{FromBytes, Immutable, IntoBytes, KnownLayout, Unaligned};

    #[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
    pub enum Mode2Form {
        Form1,
        Form2,
    }

    /// One copy of a formed Mode 2 sub-header. Sectors carry two identical copies.
    #[derive(
        Debug, Copy, Clone, PartialEq, Eq, FromBytes, IntoBytes, Immutable, KnownLayout, Unaligned,
    )]
    #[repr(C)]
    pub struct Mode2FormedSubHeader {
        pub file_number: u8,
        pub channel_number: u8,
        pub sub_mode: Mode2FormedSubMode,
        pub coding_information: u8,
    }

    const _: () = assert!(size_of::<Mode2FormedSubHeader>() == 4);

    /// The sub-mode byte: eight independent flags, one of which selects the form.
    #[derive(
        Copy, Clone, PartialEq, Eq, FromBytes, IntoBytes, Immutable, KnownLayout, Unaligned,
    )]
    #[repr(transparent)]
    pub struct Mode2FormedSubMode(u8);

    impl Mode2FormedSubMode {
        #[inline]
        pub const fn from_bits(b: u8) -> Self {
            Mode2FormedSubMode(b)
        }
        #[inline]
        pub const fn bits(self) -> u8 {
            self.0
        }
        #[inline]
        pub const fn end_of_file(self) -> bool {
            self.0 & 1 << 7 != 0
        }
        #[inline]
        pub const fn real_time_block(self) -> bool {
            self.0 & 1 << 6 != 0
        }
        /// Bit 5. Clear means Form 1, set means Form 2.
        ///
        /// This is the only in-band statement of the form, which is why a `READ CD`
        /// that omits the sub-header cannot resolve Form 1 from Form 2 on its own.
        #[inline]
        pub const fn form(self) -> Mode2Form {
            if self.0 & 1 << 5 == 0 {
                Mode2Form::Form1
            } else {
                Mode2Form::Form2
            }
        }
        #[inline]
        pub const fn trigger_block(self) -> bool {
            self.0 & 1 << 4 != 0
        }
        #[inline]
        pub const fn data_block(self) -> bool {
            self.0 & 1 << 3 != 0
        }
        #[inline]
        pub const fn audio_block(self) -> bool {
            self.0 & 1 << 2 != 0
        }
        #[inline]
        pub const fn video_block(self) -> bool {
            self.0 & 1 << 1 != 0
        }
        #[inline]
        pub const fn end_of_record(self) -> bool {
            self.0 & 1 << 0 != 0
        }
    }

    impl fmt::Debug for Mode2FormedSubMode {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_struct("Mode2FormedSubMode")
                .field("form", &self.form())
                .field("eof", &self.end_of_file())
                .field("real_time", &self.real_time_block())
                .field("trigger", &self.trigger_block())
                .field("data", &self.data_block())
                .field("audio", &self.audio_block())
                .field("video", &self.video_block())
                .field("eor", &self.end_of_record())
                .finish()
        }
    }

    pub mod formless {}

    pub mod form1 {}

    pub mod form2 {}
}
